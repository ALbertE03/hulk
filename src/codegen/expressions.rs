use std::collections::HashMap;
use crate::ast::nodes::{Expr, Op, UnOp, TypeAnnotation, Pattern};
use crate::utils::Spanned;
use super::context::{Ctx, ClassLayout};
use super::utils::{fmt_double, ValTy, val_ty_from_annotation};
use super::functions::mangle_fn;

pub fn gen_expr(ctx: &mut Ctx, expr: &Spanned<Expr>) -> String {
    match &expr.node {
        // ── Primitivos ──────────────────────────────────────────────────────
        Expr::Number(v) => fmt_double(*v),

        Expr::Boolean(v) => if *v { "1.0".into() } else { "0.0".into() },

        Expr::String(s) => {
            let id = ctx.add_global_string(s);
            let len = s.len() + 1;
            let gep = ctx.gep_string(&id, len);
            let i = ctx.tmp(); ctx.emit(&format!("{} = ptrtoint i8* {} to i64", i, gep));
            let d = ctx.tmp(); ctx.emit(&format!("{} = bitcast i64 {} to double", d, i));
            d
        }

        Expr::Identifier(name) => {
            // 1. Local / parámetro
            if let Some((ptr, _ty)) = ctx.get_var(name) {
                let r = ctx.tmp();
                ctx.emit(&format!("{} = load double, double* {}", r, ptr));
                return r;
            }
            // 2. Referencia a `self` dentro de un método — codificar i8* como double
            if name == "self" && ctx.current_class.is_some() {
                let i = ctx.tmp();
                ctx.emit(&format!("{} = ptrtoint i8* %self to i64", i));
                let d = ctx.tmp();
                ctx.emit(&format!("{} = bitcast i64 {} to double", d, i));
                return d;
            }
            // 3. Atributo de self (dentro del cuerpo de un método)
            if let Some(cls) = ctx.current_class.clone() {
                if let Some(layout) = ctx.classes.get(&cls) {
                    if let Some(&idx) = layout.attr_indices.get(name.as_str()) {
                        let sn = layout.struct_name.clone();
                        let cast = ctx.tmp();
                        ctx.emit(&format!("{} = bitcast i8* %self to {}*", cast, sn));
                        let gep = ctx.tmp();
                        ctx.emit(&format!("{} = getelementptr inbounds {}, {}* {}, i32 0, i32 {}",
                            gep, sn, sn, cast, idx));
                        let v = ctx.tmp();
                        ctx.emit(&format!("{} = load double, double* {}", v, gep));
                        return v;
                    }
                }
            }
            ctx.emit(&format!("; WARNING: variable '{}' not found", name));
            "0.0".into()
        }

        // ── Binario ──────────────────────────────────────────────────────────
        Expr::Binary(lhs_ast, op, rhs_ast) => {
            let l = gen_expr(ctx, lhs_ast);
            let r = gen_expr(ctx, rhs_ast);
            let res = ctx.tmp();

            match op {
                Op::Add => ctx.emit(&format!("{} = fadd double {}, {}", res, l, r)),
                Op::Sub => ctx.emit(&format!("{} = fsub double {}, {}", res, l, r)),
                Op::Mul => ctx.emit(&format!("{} = fmul double {}, {}", res, l, r)),
                Op::Div => ctx.emit(&format!("{} = fdiv double {}, {}", res, l, r)),
                Op::Mod => ctx.emit(&format!("{} = frem double {}, {}", res, l, r)),
                Op::Pow => ctx.emit(&format!("{} = call double @llvm.pow.f64(double {}, double {})", res, l, r)),

                Op::Eq | Op::Neq | Op::Lt | Op::Gt | Op::Le | Op::Ge => {
                    let pred = match op {
                        Op::Eq  => "oeq", Op::Neq => "one",
                        Op::Lt  => "olt", Op::Gt  => "ogt",
                        Op::Le  => "ole", Op::Ge  => "oge",
                        _ => unreachable!(),
                    };
                    let c = ctx.tmp();
                    ctx.emit(&format!("{} = fcmp {} double {}, {}", c, pred, l, r));
                    ctx.emit(&format!("{} = uitofp i1 {} to double", res, c));
                }
                Op::And => {
                    let lb = ctx.tmp(); ctx.emit(&format!("{} = fcmp one double {}, 0.0", lb, l));
                    let rb = ctx.tmp(); ctx.emit(&format!("{} = fcmp one double {}, 0.0", rb, r));
                    let ab = ctx.tmp(); ctx.emit(&format!("{} = and i1 {}, {}", ab, lb, rb));
                    ctx.emit(&format!("{} = uitofp i1 {} to double", res, ab));
                }
                Op::Or => {
                    let lb = ctx.tmp(); ctx.emit(&format!("{} = fcmp one double {}, 0.0", lb, l));
                    let rb = ctx.tmp(); ctx.emit(&format!("{} = fcmp one double {}, 0.0", rb, r));
                    let ob = ctx.tmp(); ctx.emit(&format!("{} = or i1 {}, {}", ob, lb, rb));
                    ctx.emit(&format!("{} = uitofp i1 {} to double", res, ob));
                }
                Op::Concat | Op::ConcatSpace => {
                    // Convertir cada operando a puntero i8* de cadena.
                    // Si el operando ya es una cadena (puntero-como-double), bitcast.
                    // Si es un número, llamar a @__hulk_num_to_str.
                    // Si es un bool, llamar a @__hulk_bool_to_str.
                    // Si es desconocido, usar @__hulk_num_to_str como heurística por defecto.
                    let lp = gen_to_str_ptr(ctx, &lhs_ast.node, &l);
                    let rp = gen_to_str_ptr(ctx, &rhs_ast.node, &r);

                    let ll = ctx.tmp(); ctx.emit(&format!("{} = call i64 @strlen(i8* {})", ll, lp));
                    let rl = ctx.tmp(); ctx.emit(&format!("{} = call i64 @strlen(i8* {})", rl, rp));
                    let total = ctx.tmp(); ctx.emit(&format!("{} = add i64 {}, {}", total, ll, rl));
                    let extra = if *op == Op::ConcatSpace { 2i64 } else { 1 };
                    let total2 = ctx.tmp(); ctx.emit(&format!("{} = add i64 {}, {}", total2, total, extra));

                    let buf = ctx.tmp(); ctx.emit(&format!("{} = call i8* @malloc(i64 {})", buf, total2));
                    ctx.emit(&format!("call i8* @strcpy(i8* {}, i8* {})", buf, lp));
                    if *op == Op::ConcatSpace {
                        ctx.emit(&format!("call i8* @strcat(i8* {}, i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.space_s, i64 0, i64 0))", buf));
                    }
                    ctx.emit(&format!("call i8* @strcat(i8* {}, i8* {})", buf, rp));

                    let pi = ctx.tmp(); ctx.emit(&format!("{} = ptrtoint i8* {} to i64", pi, buf));
                    ctx.emit(&format!("{} = bitcast i64 {} to double", res, pi));
                }
            }
            res
        }

        // ── Unario ───────────────────────────────────────────────────────────
        Expr::Unary(op, operand) => {
            let v = gen_expr(ctx, operand);
            let res = ctx.tmp();
            match op {
                UnOp::Neg => ctx.emit(&format!("{} = fneg double {}", res, v)),
                UnOp::Not => {
                    let c = ctx.tmp();
                    ctx.emit(&format!("{} = fcmp oeq double {}, 0.0", c, v));
                    ctx.emit(&format!("{} = uitofp i1 {} to double", res, c));
                }
            }
            res
        }

        // ── If / Else (Condicional) ────────────────────────────────────────
        Expr::If { cond, then_expr, else_expr } => {
            let cv = gen_expr(ctx, cond);
            let cb = ctx.tmp();
            ctx.emit(&format!("{} = fcmp one double {}, 0.0", cb, cv));

            let res_ptr = ctx.tmp();
            ctx.emit(&format!("{} = alloca double", res_ptr));

            let tl = ctx.lbl("then"); let el = ctx.lbl("else"); let ml = ctx.lbl("merge");
            ctx.emit(&format!("br i1 {}, label %{}, label %{}", cb, tl, el));

            ctx.emit_label(&tl);
            let tv = gen_expr(ctx, then_expr);
            ctx.emit(&format!("store double {}, double* {}", tv, res_ptr));
            ctx.emit(&format!("br label %{}", ml));

            ctx.emit_label(&el);
            let ev = gen_expr(ctx, else_expr);
            ctx.emit(&format!("store double {}, double* {}", ev, res_ptr));
            ctx.emit(&format!("br label %{}", ml));

            ctx.emit_label(&ml);
            let r = ctx.tmp();
            ctx.emit(&format!("{} = load double, double* {}", r, res_ptr));
            r
        }

        // ── Mientras (While) ────────────────────────────────────────────────
        Expr::While { cond, body } => {
            let lc = ctx.lbl("wcond"); let lb = ctx.lbl("wbody"); let le = ctx.lbl("wend");
            ctx.emit(&format!("br label %{}", lc));

            ctx.emit_label(&lc);
            let cv = gen_expr(ctx, cond);
            let cb = ctx.tmp();
            ctx.emit(&format!("{} = fcmp one double {}, 0.0", cb, cv));
            ctx.emit(&format!("br i1 {}, label %{}, label %{}", cb, lb, le));

            ctx.emit_label(&lb);
            gen_expr(ctx, body);
            ctx.emit(&format!("br label %{}", lc));

            ctx.emit_label(&le);
            "0.0".into()
        }

        // ── Para (For) ──────────────────────────────────────────────────────
        // Transpila automáticamente según el tipo del iterable:
        //   - Si es un objeto con métodos next()/get_current() (protocolo Iterable),
        //     se transpila a: while(iterable.next()) let var = iterable.get_current() in { body }
        //   - Si es un vector, se itera por índice como antes.
        Expr::For { var, iterable, body } => {
            // Intentar resolver la clase del iterable para ver si implementa Iterable
            let iter_vty = infer_val_ty(ctx, &iterable.node);
            let iterable_class = match &iter_vty {
                ValTy::Obj(cls) => Some(cls.clone()),
                _ => None,
            };

            // Determinar nombre del método "current" (get_current o current)
            let current_method = if let Some(ref cls) = iterable_class {
                if let Some(layout) = ctx.classes.get(cls.as_str()) {
                    if layout.method_names.contains_key("get_current") {
                        Some("get_current")
                    } else if layout.method_names.contains_key("current") {
                        Some("current")
                    } else {
                        None
                    }
                } else { None }
            } else { None };

            let is_iterable = if let Some(ref cls) = iterable_class {
                if let Some(layout) = ctx.classes.get(cls.as_str()) {
                    layout.method_names.contains_key("next") && current_method.is_some()
                } else { false }
            } else { false };

            if is_iterable {
                // ── Protocolo Iterable: transpilación a while(iter.next()) ──
                let cls = iterable_class.unwrap();
                let cur_method = current_method.unwrap();
                let layout = ctx.classes.get(&cls).unwrap();
                let next_fn = layout.method_names.get("next").unwrap().clone();
                let current_fn = layout.method_names.get(cur_method).unwrap().clone();

                let iter_val = gen_expr(ctx, iterable);
                let iter_ptr = ctx.tmp();
                ctx.emit(&format!("{} = alloca double", iter_ptr));
                ctx.emit(&format!("store double {}, double* {}", iter_val, iter_ptr));

                let lc = ctx.lbl("icond"); let lb = ctx.lbl("ibody"); let le = ctx.lbl("iend");
                ctx.emit(&format!("br label %{}", lc));

                // Condición: call next()
                ctx.emit_label(&lc);
                let iv = ctx.tmp(); ctx.emit(&format!("{} = load double, double* {}", iv, iter_ptr));
                let op = ctx.decode_ptr(&iv, "i8*");
                let next_r = ctx.tmp();
                ctx.emit(&format!("{} = call double {}(i8* {})", next_r, next_fn, op));
                let cond = ctx.tmp();
                ctx.emit(&format!("{} = fcmp one double {}, 0.0", cond, next_r));
                ctx.emit(&format!("br i1 {}, label %{}, label %{}", cond, lb, le));

                // Cuerpo: let var = iter.get_current() in body
                ctx.emit_label(&lb);
                ctx.enter_scope();
                let iv2 = ctx.tmp(); ctx.emit(&format!("{} = load double, double* {}", iv2, iter_ptr));
                let op2 = ctx.decode_ptr(&iv2, "i8*");
                let cur_r = ctx.tmp();
                ctx.emit(&format!("{} = call double {}(i8* {})", cur_r, current_fn, op2));
                let vp = ctx.tmp(); ctx.emit(&format!("{} = alloca double", vp));
                ctx.emit(&format!("store double {}, double* {}", cur_r, vp));
                ctx.def_var(var, &vp, ValTy::Num);

                gen_expr(ctx, body);
                ctx.exit_scope();
                ctx.emit(&format!("br label %{}", lc));

                ctx.emit_label(&le);
                "0.0".into()
            } else {
                // ── Vector: iteración por índice ──
                let iter_val = gen_expr(ctx, iterable);
                let vp = ctx.decode_ptr(&iter_val, "double*");

                let len_ptr = vp.clone();
                let len_d = ctx.tmp(); ctx.emit(&format!("{} = load double, double* {}", len_d, len_ptr));
                let len_i = ctx.tmp(); ctx.emit(&format!("{} = fptosi double {} to i64", len_i, len_d));

                let idx_ptr = ctx.tmp(); ctx.emit(&format!("{} = alloca i64", idx_ptr));
                ctx.emit(&format!("store i64 0, i64* {}", idx_ptr));

                let lc = ctx.lbl("fcond"); let lb = ctx.lbl("fbody"); let le = ctx.lbl("fend");
                ctx.emit(&format!("br label %{}", lc));

                ctx.emit_label(&lc);
                let ci = ctx.tmp(); ctx.emit(&format!("{} = load i64, i64* {}", ci, idx_ptr));
                let cc = ctx.tmp(); ctx.emit(&format!("{} = icmp slt i64 {}, {}", cc, ci, len_i));
                ctx.emit(&format!("br i1 {}, label %{}, label %{}", cc, lb, le));

                ctx.emit_label(&lb);
                ctx.enter_scope();
                let ci2 = ctx.tmp(); ctx.emit(&format!("{} = load i64, i64* {}", ci2, idx_ptr));
                let off = ctx.tmp(); ctx.emit(&format!("{} = add i64 {}, 1", off, ci2));
                let ep = ctx.tmp(); ctx.emit(&format!("{} = getelementptr double, double* {}, i64 {}", ep, vp, off));
                let ev = ctx.tmp(); ctx.emit(&format!("{} = load double, double* {}", ev, ep));
                let vp2 = ctx.tmp(); ctx.emit(&format!("{} = alloca double", vp2));
                ctx.emit(&format!("store double {}, double* {}", ev, vp2));
                ctx.def_var(var, &vp2, ValTy::Num);

                gen_expr(ctx, body);

                let ni = ctx.tmp(); ctx.emit(&format!("{} = load i64, i64* {}", ni, idx_ptr));
                let ni2 = ctx.tmp(); ctx.emit(&format!("{} = add i64 {}, 1", ni2, ni));
                ctx.emit(&format!("store i64 {}, i64* {}", ni2, idx_ptr));
                ctx.exit_scope();
                ctx.emit(&format!("br label %{}", lc));

                ctx.emit_label(&le);
                "0.0".into()
            }
        }

        // ── Let (Enlace de variables) ───────────────────────────────────────
        Expr::Let { bindings, body } => {
            ctx.enter_scope();
            for (name, _ann, init_expr) in bindings {
                let vty = infer_val_ty(ctx, &init_expr.node);
                let v = gen_expr(ctx, init_expr);
                let p = ctx.tmp();
                ctx.emit(&format!("{} = alloca double", p));
                ctx.emit(&format!("store double {}, double* {}", v, p));
                ctx.def_var(name, &p, vty);
            }
            let res = gen_expr(ctx, body);
            ctx.exit_scope();
            res
        }

        // ── Asignación ──────────────────────────────────────────────────────
        Expr::Assignment { target, value } => {
            let v = gen_expr(ctx, value);
            if let Some((ptr, _)) = ctx.get_var(target) {
                ctx.emit(&format!("store double {}, double* {}", v, ptr));
            }
            v
        }

        // ── Asignación a atributo (obj.attr := expr) ────────────────────────
        Expr::AttributeAssignment { obj, attribute, value } => {
            let v = gen_expr(ctx, value);
            // Determinar la clase del objeto
            let obj_val = gen_expr(ctx, obj);
            let op = ctx.decode_ptr(&obj_val, "i8*");
            let cls_name = resolve_obj_class_from_expr(ctx, &obj.node);
            if let Some(cls) = cls_name {
                if let Some(layout) = ctx.classes.get(&cls) {
                    if let Some(&idx) = layout.attr_indices.get(attribute.as_str()) {
                        let sn = layout.struct_name.clone();
                        let cast = ctx.tmp();
                        ctx.emit(&format!("{} = bitcast i8* {} to {}*", cast, op, sn));
                        let gep = ctx.tmp();
                        ctx.emit(&format!("{} = getelementptr inbounds {}, {}* {}, i32 0, i32 {}",
                            gep, sn, sn, cast, idx));
                        ctx.emit(&format!("store double {}, double* {}", v, gep));
                    }
                }
            }
            v
        }

        // ── Bloque ───────────────────────────────────────────────────────────
        Expr::Block(exprs) => {
            let mut last = "0.0".to_string();
            for e in exprs { last = gen_expr(ctx, e); }
            last
        }

        // ── Llamada (función) ────────────────────────────────────────────────
        Expr::Call { func, args } => {
            if func == "print" {
                gen_print(ctx, args);
                return "0.0".into();
            }
            let mut vals = Vec::new();
            for a in args { vals.push(gen_expr(ctx, a)); }

            // Verificar si 'func' es una variable (lambda/clausura) en el ámbito
            if let Some((ptr, vty)) = ctx.get_var(func) {
                // ── Llamada functor: si la variable contiene un objeto con método `invoke`,
                // llamar obj.invoke(args) en vez de tratar como clausura.
                if let ValTy::Obj(ref class_name) = vty {
                    if let Some(layout) = ctx.classes.get(class_name.as_str()) {
                        if let Some(invoke_fname) = layout.method_names.get("invoke") {
                            let invoke_fname = invoke_fname[1..].to_string(); // quitar @ inicial
                            let ov = ctx.tmp();
                            ctx.emit(&format!("{} = load double, double* {}", ov, ptr));
                            let op = ctx.decode_ptr(&ov, "i8*");
                            let mut arg_s = format!("i8* {}", op);
                            for v in &vals {
                                arg_s.push_str(&format!(", double {}", v));
                            }
                            let r = ctx.tmp();
                            ctx.emit(&format!("{} = call double @{}({})", r, invoke_fname, arg_s));
                            return r;
                        }
                    }
                }

                // Llamada a clausura: la variable contiene un puntero double-encoded a [fn_ptr, env_ptr]
                let cval = ctx.tmp();
                ctx.emit(&format!("{} = load double, double* {}", cval, ptr));
                let cp = ctx.decode_ptr(&cval, "double*");
                // Cargar fn_ptr (slot 0)
                let fp_d = ctx.tmp();
                ctx.emit(&format!("{} = load double, double* {}", fp_d, cp));
                // Construir tipo fn: double(double*, double, double, ...)
                let param_types: String = std::iter::once("double*".to_string())
                    .chain(vals.iter().map(|_| "double".to_string()))
                    .collect::<Vec<_>>().join(", ");
                let fn_ty = format!("double ({})*", param_types);
                let fp = ctx.decode_ptr(&fp_d, &fn_ty);
                // Cargar env_ptr (slot 1)
                let env_slot = ctx.tmp();
                ctx.emit(&format!("{} = getelementptr double, double* {}, i64 1", env_slot, cp));
                let env_d = ctx.tmp();
                ctx.emit(&format!("{} = load double, double* {}", env_d, env_slot));
                let env_p = ctx.decode_ptr(&env_d, "double*");
                // Construir args: env_ptr, val0, val1, ...
                let mut arg_s = format!("double* {}", env_p);
                for v in &vals {
                    arg_s.push_str(&format!(", double {}", v));
                }
                let r = ctx.tmp();
                ctx.emit(&format!("{} = call double {}({})", r, fp, arg_s));
                r
            } else {
                // Llamada directa a función
                let mut arg_s = String::new();
                for (i, v) in vals.iter().enumerate() {
                    if i > 0 { arg_s.push_str(", "); }
                    arg_s.push_str("double "); arg_s.push_str(v);
                }
                let r = ctx.tmp();
                let ir_func = mangle_fn(func);
                ctx.emit(&format!("{} = call double @{}({})", r, ir_func, arg_s));
                r
            }
        }

        // ── Instanciación ───────────────────────────────────────────────────
        Expr::Instantiation { ty, args } => {
            let mut vals = Vec::new();
            for a in args { vals.push(gen_expr(ctx, a)); }
            let mut arg_s = String::new();
            for (i, v) in vals.iter().enumerate() {
                if i > 0 { arg_s.push_str(", "); }
                arg_s.push_str("double "); arg_s.push_str(v);
            }
            let raw = ctx.tmp();
            ctx.emit(&format!("{} = call i8* @{}_new({})", raw, ty, arg_s));
            let pi = ctx.tmp(); ctx.emit(&format!("{} = ptrtoint i8* {} to i64", pi, raw));
            let d = ctx.tmp(); ctx.emit(&format!("{} = bitcast i64 {} to double", d, pi));
            d
        }

        // ── Llamada a método ─────────────────────────────────────────────────
        Expr::MethodCall { obj, method, args } => {
            // Resolver la clase del objeto en tiempo de compilación
            let obj_class = resolve_obj_class_from_expr(ctx, &obj.node);
            let ov = gen_expr(ctx, obj);
            let op = ctx.decode_ptr(&ov, "i8*");

            let mut vals = Vec::new();
            for a in args { vals.push(gen_expr(ctx, a)); }

            let mut func_name = format!("unknown_{}", method);
            // Primero intentar con la clase resuelta
            if let Some(ref cls) = obj_class {
                if let Some(layout) = ctx.classes.get(cls.as_str()) {
                    if let Some(fname) = layout.method_names.get(method.as_str()) {
                        func_name = fname[1..].to_string();
                    }
                }
            }
            // Respaldo: buscar en todas las clases
            if func_name.starts_with("unknown_") {
                for (_, layout) in &ctx.classes {
                    if let Some(fname) = layout.method_names.get(method.as_str()) {
                        func_name = fname[1..].to_string();
                        break;
                    }
                }
            }

            let mut arg_s = format!("i8* {}", op);
            for v in &vals {
                arg_s.push_str(&format!(", double {}", v));
            }
            let r = ctx.tmp();
            ctx.emit(&format!("{} = call double @{}({})", r, func_name, arg_s));
            r
        }

        // ── Acceso a atributo ────────────────────────────────────────────────
        Expr::AttributeAccess { obj, attribute } => {
            // Resolver clase del objeto en tiempo de compilación para layout correcto del struct
            let obj_class = resolve_obj_class_from_expr(ctx, &obj.node);
            let ov = gen_expr(ctx, obj);
            let op = ctx.decode_ptr(&ov, "i8*");

            let mut found_idx: Option<(u32, String)> = None;
            // Primero intentar con la clase resuelta
            if let Some(ref cls) = obj_class {
                if let Some(layout) = ctx.classes.get(cls.as_str()) {
                    if let Some(&idx) = layout.attr_indices.get(attribute.as_str()) {
                        found_idx = Some((idx, layout.struct_name.clone()));
                    }
                }
            }
            // Respaldo: buscar en todas las clases
            if found_idx.is_none() {
                for (_, layout) in &ctx.classes {
                    if let Some(&idx) = layout.attr_indices.get(attribute.as_str()) {
                        found_idx = Some((idx, layout.struct_name.clone()));
                        break;
                    }
                }
            }

            if let Some((idx, sn)) = found_idx {
                let cast = ctx.tmp(); ctx.emit(&format!("{} = bitcast i8* {} to {}*", cast, op, sn));
                let gep = ctx.tmp(); ctx.emit(&format!("{} = getelementptr inbounds {}, {}* {}, i32 0, i32 {}",
                    gep, sn, sn, cast, idx));
                let v = ctx.tmp(); ctx.emit(&format!("{} = load double, double* {}", v, gep));
                v
            } else {
                ctx.emit(&format!("; WARNING: attribute '{}' not found", attribute));
                "0.0".into()
            }
        }

        // ── Llamada base (llama al constructor o método del padre) ───────────
        Expr::BaseCall { args } => {
            // Resolver la clase padre desde current_class
            let parent_info = ctx.current_class.as_ref().and_then(|cls| {
                ctx.classes.get(cls).and_then(|layout| {
                    layout.parent_name.as_ref().map(|pn| pn.clone())
                })
            });

            if let Some(parent_class) = parent_info {
                // Evaluar argumentos
                let mut vals = Vec::new();
                for a in args { vals.push(gen_expr(ctx, a)); }
                let mut arg_s = String::new();
                for (i, v) in vals.iter().enumerate() {
                    if i > 0 { arg_s.push_str(", "); }
                    arg_s.push_str("double "); arg_s.push_str(v);
                }
                // Llamar al constructor del padre
                let raw = ctx.tmp();
                ctx.emit(&format!("{} = call i8* @{}_new({})", raw, parent_class, arg_s));
                let pi = ctx.tmp(); ctx.emit(&format!("{} = ptrtoint i8* {} to i64", pi, raw));
                let d = ctx.tmp(); ctx.emit(&format!("{} = bitcast i64 {} to double", d, pi));
                d
            } else {
                for a in args { gen_expr(ctx, a); }
                ctx.emit("; BaseCall – no parent class found");
                "0.0".into()
            }
        }

        // ── Lambda (con captura de clausura) ────────────────────────────────
        Expr::Lambda { params, body, .. } => {
            // 1. Identificar variables libres en el cuerpo que no son parámetros de la lambda
            let param_names: Vec<String> = params.iter().map(|p| p.name.clone()).collect();
            let free_vars = collect_free_vars(body, &param_names);

            // 2. Capturar valores actuales de variables libres
            let mut captured: Vec<(String, String)> = Vec::new(); // (nombre, valor_actual)
            for fv in &free_vars {
                if let Some((ptr, _)) = ctx.get_var(fv) {
                    let val = ctx.tmp();
                    ctx.emit(&format!("{} = load double, double* {}", val, ptr));
                    captured.push((fv.clone(), val));
                }
            }

            // 3. Asignar entorno de clausura en el heap: [cap0, cap1, ...]
            let env_size = captured.len();
            let env_ptr = if env_size > 0 {
                let bytes = env_size as u64 * 8;
                let raw = ctx.tmp();
                ctx.emit(&format!("{} = call i8* @malloc(i64 {})", raw, bytes));
                let arr = ctx.tmp();
                ctx.emit(&format!("{} = bitcast i8* {} to double*", arr, raw));
                // Almacenar valores capturados
                for (i, (_, val)) in captured.iter().enumerate() {
                    let gep = ctx.tmp();
                    ctx.emit(&format!("{} = getelementptr double, double* {}, i64 {}", gep, arr, i));
                    ctx.emit(&format!("store double {}, double* {}", val, gep));
                }
                Some(arr.clone())
            } else {
                None
            };

            // 4. Emitir el cuerpo de la función lambda: @__lambda_N(double* %env, double %p0, ...)
            let fname = format!("__lambda_{}", ctx.counter); ctx.counter += 1;
            let mut sig = String::from("double* %__env");
            for p in params {
                sig.push_str(&format!(", double %{}", p.name));
            }

            // Guardar buffer de funciones actual e iniciar uno nuevo para la lambda
            let saved_functions = std::mem::take(&mut ctx.functions);
            ctx.functions.push_str(&format!("define double @{}({}) {{\nentry:\n", fname, sig));
            ctx.enter_scope();

            // Cargar variables capturadas desde env
            for (i, (name, _)) in captured.iter().enumerate() {
                let gep = ctx.tmp();
                ctx.emit(&format!("{} = getelementptr double, double* %__env, i64 {}", gep, i));
                let val = ctx.tmp();
                ctx.emit(&format!("{} = load double, double* {}", val, gep));
                let ptr = ctx.tmp();
                ctx.emit(&format!("{} = alloca double", ptr));
                ctx.emit(&format!("store double {}, double* {}", val, ptr));
                ctx.def_var(name, &ptr, ValTy::Num);
            }

            // Parámetros de la lambda
            for p in params {
                let ptr = ctx.tmp();
                ctx.emit(&format!("{} = alloca double", ptr));
                ctx.emit(&format!("store double %{}, double* {}", p.name, ptr));
                ctx.def_var(&p.name, &ptr, val_ty_from_annotation(&p.type_annotation));
            }

            let r = gen_expr(ctx, body);
            ctx.emit(&format!("ret double {}", r));
            ctx.functions.push_str("}\n\n");
            ctx.exit_scope();

            // Mover la definición lambda a lambda_defs y restaurar el buffer del llamador
            let lambda_code = std::mem::replace(&mut ctx.functions, saved_functions);
            ctx.lambda_defs.push_str(&lambda_code);

            // 5. Codificar clausura como par: { fn_ptr, env_ptr } empaquetado en dos doubles
            // Por simplicidad empaquetamos [fn_ptr_as_double, env_ptr_as_double] en un buffer en el heap
            let closure_buf = ctx.tmp();
            ctx.emit(&format!("{} = call i8* @malloc(i64 16)", closure_buf)); // 2 * 8 bytes
            let closure_arr = ctx.tmp();
            ctx.emit(&format!("{} = bitcast i8* {} to double*", closure_arr, closure_buf));

            // Almacenar puntero de función como double
            let fn_param_types: String = std::iter::once("double*".to_string())
                .chain(params.iter().map(|_| "double".to_string()))
                .collect::<Vec<_>>().join(", ");
            let fp_i = ctx.tmp();
            ctx.emit(&format!("{} = ptrtoint double ({})* @{} to i64", fp_i, fn_param_types, fname));
            let fp_d = ctx.tmp();
            ctx.emit(&format!("{} = bitcast i64 {} to double", fp_d, fp_i));
            ctx.emit(&format!("store double {}, double* {}", fp_d, closure_arr));

            // Almacenar puntero de entorno como double
            let env_val = if let Some(ref ep) = env_ptr {
                let ei = ctx.tmp();
                ctx.emit(&format!("{} = ptrtoint double* {} to i64", ei, ep));
                let ed = ctx.tmp();
                ctx.emit(&format!("{} = bitcast i64 {} to double", ed, ei));
                ed
            } else {
                "0.0".to_string()
            };
            let env_slot = ctx.tmp();
            ctx.emit(&format!("{} = getelementptr double, double* {}, i64 1", env_slot, closure_arr));
            ctx.emit(&format!("store double {}, double* {}", env_val, env_slot));

            // Retornar buffer de clausura como double
            let pi = ctx.tmp(); ctx.emit(&format!("{} = ptrtoint double* {} to i64", pi, closure_arr));
            let d = ctx.tmp(); ctx.emit(&format!("{} = bitcast i64 {} to double", d, pi));
            d
        }

        // ── Is (verificación de tipo en tiempo de ejecución) ─────────────────
        Expr::Is(expr, type_name) => {
            let v = gen_expr(ctx, expr);
            // Buscar el type-id del tipo destino y todos sus descendientes
            if let Some(target_layout) = ctx.classes.get(type_name.as_str()) {
                let target_tid = target_layout.type_id;
                // Recopilar destino + todos los type-ids hijos
                let mut valid_ids = vec![target_tid];
                // Recorrer todas las clases y verificar si heredan de type_name
                for (_, layout) in &ctx.classes {
                    if layout.type_id != target_tid && class_inherits_from(&ctx.classes, layout, type_name) {
                        valid_ids.push(layout.type_id);
                    }
                }

                // Decodificar puntero de objeto y leer type-id desde slot 0
                let op = ctx.decode_ptr(&v, "i64*");
                let tid = ctx.tmp(); ctx.emit(&format!("{} = load i64, i64* {}", tid, op));

                // Verificar si tid coincide con algún id válido
                let res_ptr = ctx.tmp();
                ctx.emit(&format!("{} = alloca double", res_ptr));
                ctx.emit(&format!("store double 0.0, double* {}", res_ptr));

                for vid in &valid_ids {
                    let c = ctx.tmp();
                    ctx.emit(&format!("{} = icmp eq i64 {}, {}", c, tid, vid));
                    let match_lbl = ctx.lbl("is_match");
                    let next_lbl = ctx.lbl("is_next");
                    ctx.emit(&format!("br i1 {}, label %{}, label %{}", c, match_lbl, next_lbl));
                    ctx.emit_label(&match_lbl);
                    ctx.emit(&format!("store double 1.0, double* {}", res_ptr));
                    // No podemos hacer break fácilmente, pero sobreescribir con 1.0 está bien
                    ctx.emit(&format!("br label %{}", next_lbl));
                    ctx.emit_label(&next_lbl);
                }

                let r = ctx.tmp();
                ctx.emit(&format!("{} = load double, double* {}", r, res_ptr));
                r
            } else {
                // Tipo no encontrado en clases – verificar primitivos
                // Para Number/Boolean/String no podemos verificar en runtime (todo es double)
                // Retornar 1.0 como mejor esfuerzo
                gen_expr(ctx, expr);
                "1.0".into()
            }
        }

        // ── As (cast de tipo con verificación en runtime) ────────────────────
        Expr::As(expr, type_name) => {
            let v = gen_expr(ctx, expr);
            // Si el tipo tiene un type-id, verificar y abortar en caso de fallo
            if let Some(target_layout) = ctx.classes.get(type_name.as_str()) {
                let target_tid = target_layout.type_id;
                let mut valid_ids = vec![target_tid];
                for (_, layout) in &ctx.classes {
                    if layout.type_id != target_tid && class_inherits_from(&ctx.classes, layout, type_name) {
                        valid_ids.push(layout.type_id);
                    }
                }

                let op = ctx.decode_ptr(&v, "i64*");
                let tid = ctx.tmp(); ctx.emit(&format!("{} = load i64, i64* {}", tid, op));
                let mut any = None;
                for vid in &valid_ids {
                    let c = ctx.tmp();
                    ctx.emit(&format!("{} = icmp eq i64 {}, {}", c, tid, vid));
                    any = Some(if let Some(prev) = any {
                        let combined = ctx.tmp();
                        ctx.emit(&format!("{} = or i1 {}, {}", combined, prev, c));
                        combined
                    } else {
                        c
                    });
                }

                if let Some(ok_cond) = any {
                    let ok_lbl = ctx.lbl("as_ok");
                    let fail_lbl = ctx.lbl("as_fail");
                    ctx.emit(&format!("br i1 {}, label %{}, label %{}", ok_cond, ok_lbl, fail_lbl));
                    ctx.emit_label(&fail_lbl);
                    // Imprimir error y abortar
                    let msg_id = ctx.add_global_string("Runtime error: invalid cast");
                    let msg_len = "Runtime error: invalid cast".len() + 1;
                    let msg_gep = ctx.gep_string(&msg_id, msg_len);
                    ctx.emit(&format!("call i32 @puts(i8* {})", msg_gep));
                    ctx.emit("call void @abort()");
                    ctx.emit("unreachable");
                    ctx.emit_label(&ok_lbl);
                }
            }
            // Pasar el valor (sigue siendo el mismo objeto)
            v
        }

        // ── Literal de vector ────────────────────────────────────────────────
        Expr::VectorLiteral(elems) => {
            let count = elems.len();
            let total = (count + 1) as u64 * 8;
            let raw = ctx.tmp();
            ctx.emit(&format!("{} = call i8* @malloc(i64 {})", raw, total));
            let arr = ctx.tmp();
            ctx.emit(&format!("{} = bitcast i8* {} to double*", arr, raw));
            ctx.emit(&format!("store double {}, double* {}", fmt_double(count as f64), arr));
            for (i, e) in elems.iter().enumerate() {
                let v = gen_expr(ctx, e);
                let gep = ctx.tmp();
                ctx.emit(&format!("{} = getelementptr double, double* {}, i64 {}", gep, arr, i + 1));
                ctx.emit(&format!("store double {}, double* {}", v, gep));
            }
            let pi = ctx.tmp(); ctx.emit(&format!("{} = ptrtoint double* {} to i64", pi, arr));
            let d = ctx.tmp(); ctx.emit(&format!("{} = bitcast i64 {} to double", d, pi));
            d
        }

        // ── VectorGenerator [expr || var in iterable] ───────────────────────
        Expr::VectorGenerator { expr: elem_expr, var, iterable } => {
            let iter_val = gen_expr(ctx, iterable);

            let sp = ctx.decode_ptr(&iter_val, "double*");
            let len_d = ctx.tmp(); ctx.emit(&format!("{} = load double, double* {}", len_d, sp));
            let len_i = ctx.tmp(); ctx.emit(&format!("{} = fptosi double {} to i64", len_i, len_d));

            let one = ctx.tmp(); ctx.emit(&format!("{} = add i64 {}, 1", one, len_i));
            let bytes = ctx.tmp(); ctx.emit(&format!("{} = mul i64 {}, 8", bytes, one));
            let raw = ctx.tmp(); ctx.emit(&format!("{} = call i8* @malloc(i64 {})", raw, bytes));
            let dp = ctx.tmp(); ctx.emit(&format!("{} = bitcast i8* {} to double*", dp, raw));
            ctx.emit(&format!("store double {}, double* {}", len_d, dp));

            let idx_ptr = ctx.tmp(); ctx.emit(&format!("{} = alloca i64", idx_ptr));
            ctx.emit(&format!("store i64 0, i64* {}", idx_ptr));

            let lc = ctx.lbl("vgc"); let lb = ctx.lbl("vgb"); let le = ctx.lbl("vge");
            ctx.emit(&format!("br label %{}", lc));
            ctx.emit_label(&lc);
            let ci = ctx.tmp(); ctx.emit(&format!("{} = load i64, i64* {}", ci, idx_ptr));
            let cc = ctx.tmp(); ctx.emit(&format!("{} = icmp slt i64 {}, {}", cc, ci, len_i));
            ctx.emit(&format!("br i1 {}, label %{}, label %{}", cc, lb, le));

            ctx.emit_label(&lb);
            ctx.enter_scope();
            let off = ctx.tmp(); ctx.emit(&format!("{} = add i64 {}, 1", off, ci));
            let ep = ctx.tmp(); ctx.emit(&format!("{} = getelementptr double, double* {}, i64 {}", ep, sp, off));
            let ev = ctx.tmp(); ctx.emit(&format!("{} = load double, double* {}", ev, ep));
            let vp = ctx.tmp(); ctx.emit(&format!("{} = alloca double", vp));
            ctx.emit(&format!("store double {}, double* {}", ev, vp));
            ctx.def_var(var, &vp, ValTy::Num);

            let mapped = gen_expr(ctx, elem_expr);
            let dp2 = ctx.tmp(); ctx.emit(&format!("{} = getelementptr double, double* {}, i64 {}", dp2, dp, off));
            ctx.emit(&format!("store double {}, double* {}", mapped, dp2));

            let ni = ctx.tmp(); ctx.emit(&format!("{} = add i64 {}, 1", ni, ci));
            ctx.emit(&format!("store i64 {}, i64* {}", ni, idx_ptr));
            ctx.exit_scope();
            ctx.emit(&format!("br label %{}", lc));

            ctx.emit_label(&le);
            let pi = ctx.tmp(); ctx.emit(&format!("{} = ptrtoint double* {} to i64", pi, dp));
            let d = ctx.tmp(); ctx.emit(&format!("{} = bitcast i64 {} to double", d, pi));
            d
        }

        // ── Indexing (with bounds checking) ─────────────────────────────────
        Expr::Indexing { obj, index } => {
            let ov = gen_expr(ctx, obj);
            let iv = gen_expr(ctx, index);
            let op = ctx.decode_ptr(&ov, "double*");
            let ii = ctx.tmp(); ctx.emit(&format!("{} = fptosi double {} to i64", ii, iv));

            // ── Verificación de límites: 0 <= ii < len ───────────────────────────
            let len_d = ctx.tmp(); ctx.emit(&format!("{} = load double, double* {}", len_d, op));
            let len_i = ctx.tmp(); ctx.emit(&format!("{} = fptosi double {} to i64", len_i, len_d));
            let neg_check = ctx.tmp(); ctx.emit(&format!("{} = icmp slt i64 {}, 0", neg_check, ii));
            let upper_check = ctx.tmp(); ctx.emit(&format!("{} = icmp sge i64 {}, {}", upper_check, ii, len_i));
            let oob = ctx.tmp(); ctx.emit(&format!("{} = or i1 {}, {}", oob, neg_check, upper_check));

            let ok_lbl = ctx.lbl("idx_ok");
            let fail_lbl = ctx.lbl("idx_oob");
            ctx.emit(&format!("br i1 {}, label %{}, label %{}", oob, fail_lbl, ok_lbl));

            ctx.emit_label(&fail_lbl);
            ctx.emit("call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([36 x i8], [36 x i8]* @.oob_msg, i64 0, i64 0))");
            ctx.emit("call void @abort()");
            ctx.emit("unreachable");

            ctx.emit_label(&ok_lbl);
            let off = ctx.tmp(); ctx.emit(&format!("{} = add i64 {}, 1", off, ii));
            let ep = ctx.tmp(); ctx.emit(&format!("{} = getelementptr double, double* {}, i64 {}", ep, op, off));
            let v = ctx.tmp(); ctx.emit(&format!("{} = load double, double* {}", v, ep));
            v
        }

        // ── Match (Coincidencia de patrones) ─────────────────────────────────
        Expr::Match { expr: match_expr, cases, default } => {
            let mv = gen_expr(ctx, match_expr);
            let res_ptr = ctx.tmp();
            ctx.emit(&format!("{} = alloca double", res_ptr));
            let end_lbl = ctx.lbl("mend");

            for case in cases {
                let next_lbl = ctx.lbl("mnext");
                match &case.pattern {
                    Pattern::Literal(lit) => {
                        let lv = match lit {
                            Expr::Number(n) => fmt_double(*n),
                            Expr::Boolean(b) => if *b { "1.0".into() } else { "0.0".into() },
                            _ => "0.0".into(),
                        };
                        let c = ctx.tmp();
                        ctx.emit(&format!("{} = fcmp oeq double {}, {}", c, mv, lv));
                        let body_lbl = ctx.lbl("mbody");
                        ctx.emit(&format!("br i1 {}, label %{}, label %{}", c, body_lbl, next_lbl));
                        ctx.emit_label(&body_lbl);
                    }
                    Pattern::Wildcard => { /* always match */ }
                    Pattern::Variable { name, .. } => {
                        ctx.enter_scope();
                        let p = ctx.tmp();
                        ctx.emit(&format!("{} = alloca double", p));
                        ctx.emit(&format!("store double {}, double* {}", mv, p));
                        ctx.def_var(name, &p, ValTy::Num);
                    }
                    _ => {
                        ctx.emit(&format!("br label %{}", next_lbl));
                    }
                }
                let cv = gen_expr(ctx, &case.expr);
                ctx.emit(&format!("store double {}, double* {}", cv, res_ptr));

                if matches!(&case.pattern, Pattern::Variable { .. }) {
                    ctx.exit_scope();
                }

                ctx.emit(&format!("br label %{}", end_lbl));
                ctx.emit_label(&next_lbl);
            }

            if let Some(def) = default {
                let dv = gen_expr(ctx, def);
                ctx.emit(&format!("store double {}, double* {}", dv, res_ptr));
            } else {
                ctx.emit(&format!("store double 0.0, double* {}", res_ptr));
            }
            ctx.emit(&format!("br label %{}", end_lbl));

            ctx.emit_label(&end_lbl);
            let r = ctx.tmp();
            ctx.emit(&format!("{} = load double, double* {}", r, res_ptr));
            r
        }

        // ── Funciones matemáticas integradas ─────────────────────────────────
        Expr::Sqrt(a) => {
            let v = gen_expr(ctx, a);
            let r = ctx.tmp();
            ctx.emit(&format!("{} = call double @llvm.sqrt.f64(double {})", r, v));
            r
        }
        Expr::Sin(a) => {
            let v = gen_expr(ctx, a);
            let r = ctx.tmp();
            ctx.emit(&format!("{} = call double @llvm.sin.f64(double {})", r, v));
            r
        }
        Expr::Cos(a) => {
            let v = gen_expr(ctx, a);
            let r = ctx.tmp();
            ctx.emit(&format!("{} = call double @llvm.cos.f64(double {})", r, v));
            r
        }
        Expr::Exp(a) => {
            let v = gen_expr(ctx, a);
            let r = ctx.tmp();
            ctx.emit(&format!("{} = call double @llvm.exp.f64(double {})", r, v));
            r
        }
        Expr::Log(base, val) => {
            let bv = gen_expr(ctx, base);
            let vv = gen_expr(ctx, val);
            let ln_v = ctx.tmp(); ctx.emit(&format!("{} = call double @llvm.log.f64(double {})", ln_v, vv));
            let ln_b = ctx.tmp(); ctx.emit(&format!("{} = call double @llvm.log.f64(double {})", ln_b, bv));
            let r = ctx.tmp(); ctx.emit(&format!("{} = fdiv double {}, {}", r, ln_v, ln_b));
            r
        }
        Expr::Rand => {
            // Sembrar una vez, luego llamar rand() de libc
            let seeded = ctx.tmp();
            ctx.emit(&format!("{} = load i1, i1* @.rand_seeded", seeded));
            let seed_lbl = ctx.lbl("rand_seed");
            let call_lbl = ctx.lbl("rand_call");
            ctx.emit(&format!("br i1 {}, label %{}, label %{}", seeded, call_lbl, seed_lbl));

            ctx.emit_label(&seed_lbl);
            let t = ctx.tmp();
            ctx.emit(&format!("{} = call i64 @time(i64* null)", t));
            let t32 = ctx.tmp();
            ctx.emit(&format!("{} = trunc i64 {} to i32", t32, t));
            ctx.emit(&format!("call void @srand(i32 {})", t32));
            ctx.emit("store i1 true, i1* @.rand_seeded");
            ctx.emit(&format!("br label %{}", call_lbl));

            ctx.emit_label(&call_lbl);
            let ri = ctx.tmp();
            ctx.emit(&format!("{} = call i32 @rand()", ri));
            // Normalizar a [0, 1): ri / RAND_MAX (2147483647)
            let rf = ctx.tmp();
            ctx.emit(&format!("{} = sitofp i32 {} to double", rf, ri));
            let r = ctx.tmp();
            ctx.emit(&format!("{} = fdiv double {}, 2.147483647e9", r, rf));
            r
        }
        Expr::PI => fmt_double(std::f64::consts::PI),
        Expr::E  => fmt_double(std::f64::consts::E),

        // ── Nodo de error ────────────────────────────────────────────────────
        Expr::Error => { "0.0".into() }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Auxiliar: verificar si una clase hereda de una clase destino
// ─────────────────────────────────────────────────────────────────────────────
fn class_inherits_from(
    classes: &HashMap<String, ClassLayout>,
    layout: &ClassLayout,
    target: &str,
) -> bool {
    let mut current = layout.parent_name.as_ref().map(|s| s.as_str());
    while let Some(parent) = current {
        if parent == target { return true; }
        current = classes.get(parent).and_then(|l| l.parent_name.as_ref().map(|s| s.as_str()));
    }
    false
}

// ─────────────────────────────────────────────────────────────────────────────
// Auxiliar: recopilar variables libres en una expresión
// ─────────────────────────────────────────────────────────────────────────────
fn collect_free_vars(expr: &Spanned<Expr>, bound: &[String]) -> Vec<String> {
    let mut free = Vec::new();
    collect_free_vars_inner(&expr.node, bound, &mut free);
    // Deduplicar
    let mut seen = std::collections::HashSet::new();
    free.retain(|v| seen.insert(v.clone()));
    free
}

fn collect_free_vars_inner(expr: &Expr, bound: &[String], free: &mut Vec<String>) {
    match expr {
        Expr::Identifier(name) => {
            if !bound.contains(name) && name != "self" {
                free.push(name.clone());
            }
        }
        Expr::Binary(l, _, r) => {
            collect_free_vars_inner(&l.node, bound, free);
            collect_free_vars_inner(&r.node, bound, free);
        }
        Expr::Unary(_, e) => collect_free_vars_inner(&e.node, bound, free),
        Expr::If { cond, then_expr, else_expr } => {
            collect_free_vars_inner(&cond.node, bound, free);
            collect_free_vars_inner(&then_expr.node, bound, free);
            collect_free_vars_inner(&else_expr.node, bound, free);
        }
        Expr::While { cond, body } => {
            collect_free_vars_inner(&cond.node, bound, free);
            collect_free_vars_inner(&body.node, bound, free);
        }
        Expr::For { var, iterable, body } => {
            collect_free_vars_inner(&iterable.node, bound, free);
            let mut inner_bound: Vec<String> = bound.to_vec();
            inner_bound.push(var.clone());
            collect_free_vars_inner(&body.node, &inner_bound, free);
        }
        Expr::Block(exprs) => {
            for e in exprs { collect_free_vars_inner(&e.node, bound, free); }
        }
        Expr::Let { bindings, body } => {
            let mut inner_bound: Vec<String> = bound.to_vec();
            for (name, _, init) in bindings {
                collect_free_vars_inner(&init.node, &inner_bound, free);
                inner_bound.push(name.clone());
            }
            collect_free_vars_inner(&body.node, &inner_bound, free);
        }
        Expr::Assignment { target, value } => {
            if !bound.contains(target) { free.push(target.clone()); }
            collect_free_vars_inner(&value.node, bound, free);
        }
        Expr::AttributeAssignment { obj, value, .. } => {
            collect_free_vars_inner(&obj.node, bound, free);
            collect_free_vars_inner(&value.node, bound, free);
        }
        Expr::Call { args, .. } => {
            for a in args { collect_free_vars_inner(&a.node, bound, free); }
        }
        Expr::BaseCall { args } => {
            for a in args { collect_free_vars_inner(&a.node, bound, free); }
        }
        Expr::MethodCall { obj, args, .. } => {
            collect_free_vars_inner(&obj.node, bound, free);
            for a in args { collect_free_vars_inner(&a.node, bound, free); }
        }
        Expr::AttributeAccess { obj, .. } => {
            collect_free_vars_inner(&obj.node, bound, free);
        }
        Expr::Instantiation { args, .. } => {
            for a in args { collect_free_vars_inner(&a.node, bound, free); }
        }
        Expr::Lambda { params, body, .. } => {
            let mut inner_bound: Vec<String> = bound.to_vec();
            for p in params { inner_bound.push(p.name.clone()); }
            collect_free_vars_inner(&body.node, &inner_bound, free);
        }
        Expr::Is(e, _) | Expr::As(e, _) => collect_free_vars_inner(&e.node, bound, free),
        Expr::VectorLiteral(elems) => {
            for e in elems { collect_free_vars_inner(&e.node, bound, free); }
        }
        Expr::VectorGenerator { expr: e, var, iterable } => {
            collect_free_vars_inner(&iterable.node, bound, free);
            let mut inner_bound: Vec<String> = bound.to_vec();
            inner_bound.push(var.clone());
            collect_free_vars_inner(&e.node, &inner_bound, free);
        }
        Expr::Indexing { obj, index } => {
            collect_free_vars_inner(&obj.node, bound, free);
            collect_free_vars_inner(&index.node, bound, free);
        }
        Expr::Match { expr: e, cases, default } => {
            collect_free_vars_inner(&e.node, bound, free);
            for c in cases {
                // Los patrones pueden enlazar variables
                let mut inner_bound: Vec<String> = bound.to_vec();
                if let Pattern::Variable { name, .. } = &c.pattern {
                    inner_bound.push(name.clone());
                }
                collect_free_vars_inner(&c.expr.node, &inner_bound, free);
            }
            if let Some(d) = default {
                collect_free_vars_inner(&d.node, bound, free);
            }
        }
        Expr::Sqrt(a) | Expr::Sin(a) | Expr::Cos(a) | Expr::Exp(a) => {
            collect_free_vars_inner(&a.node, bound, free);
        }
        Expr::Log(a, b) => {
            collect_free_vars_inner(&a.node, bound, free);
            collect_free_vars_inner(&b.node, bound, free);
        }
        // Literales, constantes – sin variables libres
        Expr::Number(_) | Expr::String(_) | Expr::Boolean(_) |
        Expr::Rand | Expr::PI | Expr::E | Expr::Error => {}
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Auxiliares: inferir tipos de retorno y resolver clase de objeto en tiempo de compilación
// ─────────────────────────────────────────────────────────────────────────────

/// Heurística: inferir el nombre del tipo de retorno de una expresión del cuerpo de un método.
/// Usa tipos de atributos para accesos `self.attr`, y análisis de literales/operaciones.
pub fn infer_return_type_from_body(body: &Expr, attr_types: &HashMap<String, String>) -> Option<String> {
    match body {
        Expr::String(_) => Some("String".into()),
        Expr::Number(_) | Expr::PI | Expr::E | Expr::Rand => Some("Number".into()),
        Expr::Boolean(_) => Some("Boolean".into()),
        Expr::Binary(_, Op::Concat, _) | Expr::Binary(_, Op::ConcatSpace, _) => Some("String".into()),
        Expr::Binary(_, op, _) if matches!(op, Op::Add | Op::Sub | Op::Mul | Op::Div | Op::Mod | Op::Pow) => Some("Number".into()),
        Expr::Binary(_, op, _) if matches!(op, Op::Eq | Op::Neq | Op::Lt | Op::Gt | Op::Le | Op::Ge | Op::And | Op::Or) => Some("Boolean".into()),
        Expr::Unary(UnOp::Neg, _) => Some("Number".into()),
        Expr::Unary(UnOp::Not, _) => Some("Boolean".into()),
        Expr::Sqrt(_) | Expr::Sin(_) | Expr::Cos(_) | Expr::Exp(_) | Expr::Log(_, _) => Some("Number".into()),
        Expr::Is(_, _) => Some("Boolean".into()),
        Expr::AttributeAccess { attribute, .. } => {
            attr_types.get(attribute.as_str()).cloned()
        }
        Expr::Identifier(name) => {
            // Atajo self.attr: buscar en attr_types
            attr_types.get(name.as_str()).cloned()
        }
        Expr::Block(stmts) => {
            stmts.last().and_then(|s| infer_return_type_from_body(&s.node, attr_types))
        }
        Expr::If { then_expr, .. } => infer_return_type_from_body(&then_expr.node, attr_types),
        Expr::Let { body, .. } => infer_return_type_from_body(&body.node, attr_types),
        _ => None,
    }
}

/// Resolver el nombre de clase HULK de una expresión en tiempo de compilación.
/// Retorna Some("NombreClase") si es determinable, None en caso contrario.
fn resolve_obj_class_from_expr(ctx: &Ctx, expr: &Expr) -> Option<String> {
    match expr {
        Expr::Instantiation { ty, .. } => Some(ty.clone()),
        Expr::Identifier(name) => {
            if let Some((_, vt)) = ctx.get_var(name) {
                if let ValTy::Obj(cls) = vt {
                    return Some(cls);
                }
            }
            // Si estamos dentro de un método y name es "self", retornar la clase actual
            if name == "self" {
                return ctx.current_class.clone();
            }
            None
        }
        Expr::As(_, ty_name) => Some(ty_name.clone()),
        _ => None,
    }
}

/// Inferir el ValTy correcto para una variable que se enlaza a una expresión.
fn infer_val_ty(ctx: &Ctx, init_expr: &Expr) -> ValTy {
    match init_expr {
        Expr::Instantiation { ty, .. } => ValTy::Obj(ty.clone()),
        Expr::String(_) => ValTy::Str,
        Expr::Boolean(_) => ValTy::Bool,
        Expr::Number(_) | Expr::PI | Expr::E | Expr::Rand => ValTy::Num,
        Expr::Binary(_, Op::Concat, _) | Expr::Binary(_, Op::ConcatSpace, _) => ValTy::Str,
        Expr::Binary(_, op, _) if matches!(op, Op::Add | Op::Sub | Op::Mul | Op::Div | Op::Mod | Op::Pow) => ValTy::Num,
        Expr::Binary(_, op, _) if matches!(op, Op::Eq | Op::Neq | Op::Lt | Op::Gt | Op::Le | Op::Ge | Op::And | Op::Or) => ValTy::Bool,
        Expr::Unary(UnOp::Neg, _) => ValTy::Num,
        Expr::Unary(UnOp::Not, _) => ValTy::Bool,
        Expr::Sqrt(_) | Expr::Sin(_) | Expr::Cos(_) | Expr::Exp(_) | Expr::Log(_, _) => ValTy::Num,
        Expr::Is(_, _) => ValTy::Bool,
        Expr::As(_, ty_name) => ValTy::Obj(ty_name.clone()),
        Expr::Identifier(name) => {
            if let Some((_, vt)) = ctx.get_var(name) {
                return vt;
            }
            ValTy::Num
        }
        Expr::If { then_expr, .. } => infer_val_ty(ctx, &then_expr.node),
        Expr::Block(stmts) => {
            stmts.last().map_or(ValTy::Num, |s| infer_val_ty(ctx, &s.node))
        }
        Expr::Let { body, .. } => infer_val_ty(ctx, &body.node),
        Expr::Lambda { body, .. } => {
            let ret_ty = infer_val_ty(ctx, &body.node);
            ValTy::Fn(Box::new(ret_ty))
        }
        Expr::Call { func, .. } => {
            // Verificar funciones con nombre
            if let Some(ret_vty) = ctx.func_ret_types.get(func.as_str()) {
                return ret_vty.clone();
            }
            // Verificar lambdas
            if let Some((_, ValTy::Fn(ret_ty))) = ctx.get_var(func) {
                return *ret_ty;
            }
            ValTy::Num
        }
        _ => ValTy::Num,
    }
}

/// Convertir la anotación de tipo de un parámetro a un ValTy.
// ─────────────────────────────────────────────────────────────────────────────
// Auxiliar de impresión – detecta Number vs Boolean vs String desde el AST + metadatos de clase
// ─────────────────────────────────────────────────────────────────────────────

/// Resultado de pista de tipo para una expresión.
#[derive(Clone, Copy, Debug, PartialEq)]
enum ExprTyHint { Str, Num, Bool, Unknown }

/// Determinar el tipo probable en tiempo de ejecución de una expresión usando la estructura del AST
/// y metadatos de atributos de clase.
fn expr_type_hint(ctx: &Ctx, expr: &Expr) -> ExprTyHint {
    match expr {
        // Literales
        Expr::String(_) => ExprTyHint::Str,
        Expr::Number(_) | Expr::PI | Expr::E | Expr::Rand => ExprTyHint::Num,
        Expr::Boolean(_) => ExprTyHint::Bool,

        // Operaciones binarias
        Expr::Binary(_, Op::Concat, _) | Expr::Binary(_, Op::ConcatSpace, _) => ExprTyHint::Str,
        Expr::Binary(_, op, _) if matches!(op, Op::Add | Op::Sub | Op::Mul | Op::Div | Op::Mod | Op::Pow) => ExprTyHint::Num,
        Expr::Binary(_, op, _) if matches!(op, Op::Eq | Op::Neq | Op::Lt | Op::Gt | Op::Le | Op::Ge | Op::And | Op::Or) => ExprTyHint::Bool,

        // Unario
        Expr::Unary(UnOp::Neg, _) => ExprTyHint::Num,
        Expr::Unary(UnOp::Not, _) => ExprTyHint::Bool,

        // Funciones matemáticas integradas
        Expr::Sqrt(_) | Expr::Sin(_) | Expr::Cos(_) | Expr::Exp(_) | Expr::Log(_, _) => ExprTyHint::Num,

        // Verificaciones de tipo
        Expr::Is(_, _) => ExprTyHint::Bool,

        // AccesoAtributo: buscar el tipo del atributo en la clase resuelta primero
        Expr::AttributeAccess { obj, attribute, .. } => {
            let obj_class = resolve_obj_class_from_expr(ctx, &obj.node);
            if let Some(ref cls) = obj_class {
                if let Some(layout) = ctx.classes.get(cls.as_str()) {
                    if let Some(type_name) = layout.attr_types.get(attribute.as_str()) {
                        return match type_name.as_str() {
                            "String" => ExprTyHint::Str,
                            "Number" => ExprTyHint::Num,
                            "Boolean" => ExprTyHint::Bool,
                            _ => ExprTyHint::Unknown,
                        };
                    }
                }
            }
            // Respaldo: buscar en todas las clases
            for (_, layout) in &ctx.classes {
                if let Some(type_name) = layout.attr_types.get(attribute.as_str()) {
                    return match type_name.as_str() {
                        "String" => ExprTyHint::Str,
                        "Number" => ExprTyHint::Num,
                        "Boolean" => ExprTyHint::Bool,
                        _ => ExprTyHint::Unknown,
                    };
                }
            }
            ExprTyHint::Unknown
        }

        // Llamadas a métodos: verificar method_ret_types desde la clase resuelta o cualquier clase
        Expr::MethodCall { obj, method, .. } => {
            // Primero intentar con la clase resuelta del objeto
            let obj_class = resolve_obj_class_from_expr(ctx, &obj.node);
            if let Some(ref cls) = obj_class {
                if let Some(layout) = ctx.classes.get(cls.as_str()) {
                    if let Some(ret_type) = layout.method_ret_types.get(method.as_str()) {
                        return match ret_type.as_str() {
                            "String" => ExprTyHint::Str,
                            "Number" => ExprTyHint::Num,
                            "Boolean" => ExprTyHint::Bool,
                            _ => ExprTyHint::Unknown,
                        };
                    }
                }
            }
            // Respaldo: buscar en todas las clases
            for (_, layout) in &ctx.classes {
                if let Some(ret_type) = layout.method_ret_types.get(method.as_str()) {
                    return match ret_type.as_str() {
                        "String" => ExprTyHint::Str,
                        "Number" => ExprTyHint::Num,
                        "Boolean" => ExprTyHint::Bool,
                        _ => ExprTyHint::Unknown,
                    };
                }
            }
            ExprTyHint::Unknown
        }

        // Flujo de control
        Expr::If { then_expr, else_expr, .. } => {
            let t = expr_type_hint(ctx, &then_expr.node);
            let e = expr_type_hint(ctx, &else_expr.node);
            if t == e { t } else if t != ExprTyHint::Unknown { t } else { e }
        }
        Expr::Block(stmts) => {
            stmts.last().map_or(ExprTyHint::Unknown, |s| expr_type_hint(ctx, &s.node))
        }
        Expr::Let { body, .. } => expr_type_hint(ctx, &body.node),

        // Identificadores: verificar los val_types rastreados
        Expr::Identifier(name) => {
            match ctx.get_var(name).map(|(_, vt)| vt) {
                Some(ValTy::Str) => ExprTyHint::Str,
                Some(ValTy::Bool) => ExprTyHint::Bool,
                Some(ValTy::Num) => ExprTyHint::Num,
                _ => ExprTyHint::Unknown,
            }
        }

        // Llamadas a funciones: verificar tipo de retorno desde funciones o val_types (para lambdas)
        Expr::Call { func, .. } => {
            // Primero verificar funciones con nombre con anotaciones de tipo de retorno
            if let Some(ret_vty) = ctx.func_ret_types.get(func.as_str()) {
                return match ret_vty {
                    ValTy::Str => ExprTyHint::Str,
                    ValTy::Bool => ExprTyHint::Bool,
                    ValTy::Num => ExprTyHint::Num,
                    _ => ExprTyHint::Unknown,
                };
            }
            // Luego verificar si es una lambda almacenada en val_types
            match ctx.get_var(func).map(|(_, vt)| vt) {
                Some(ValTy::Fn(ret_ty)) => match *ret_ty {
                    ValTy::Str => ExprTyHint::Str,
                    ValTy::Bool => ExprTyHint::Bool,
                    ValTy::Num => ExprTyHint::Num,
                    _ => ExprTyHint::Unknown,
                },
                Some(ValTy::Str) => ExprTyHint::Str,
                Some(ValTy::Bool) => ExprTyHint::Bool,
                Some(ValTy::Num) => ExprTyHint::Num,
                _ => ExprTyHint::Unknown,
            }
        }

        _ => ExprTyHint::Unknown,
    }
}

/// Dado un valor double compilado y su nodo AST, retorna un puntero `i8*` de cadena.
/// Si la expresión ya es una cadena, bitcast el double a un puntero.
/// Si es un número, llamar a @__hulk_num_to_str. Si es bool, llamar a @__hulk_bool_to_str.
/// Si el tipo es desconocido, llamar a @__hulk_to_str (auto-detección en runtime).
fn gen_to_str_ptr(ctx: &mut Ctx, expr: &Expr, val: &str) -> String {
    let hint = expr_type_hint(ctx, expr);
    match hint {
        ExprTyHint::Str => {
            ctx.decode_ptr(val, "i8*")
        }
        ExprTyHint::Bool => {
            let p = ctx.tmp();
            ctx.emit(&format!("{} = call i8* @__hulk_bool_to_str(double {})", p, val));
            p
        }
        ExprTyHint::Num => {
            let p = ctx.tmp();
            ctx.emit(&format!("{} = call i8* @__hulk_num_to_str(double {})", p, val));
            p
        }
        ExprTyHint::Unknown => {
            // Tipo desconocido en tiempo de compilación — auto-detección en runtime (puntero de cadena vs número)
            let p = ctx.tmp();
            ctx.emit(&format!("{} = call i8* @__hulk_to_str(double {})", p, val));
            p
        }
    }
}

fn gen_print(ctx: &mut Ctx, args: &[Spanned<Expr>]) {
    if let Some(arg) = args.first() {
        let hint = expr_type_hint(ctx, &arg.node);
        let val = gen_expr(ctx, arg);

        match hint {
            ExprTyHint::Str => {
                let p = ctx.decode_ptr(&val, "i8*");
                ctx.emit(&format!("call i32 @puts(i8* {})", p));
            }
            ExprTyHint::Bool => {
                let cb = ctx.tmp(); ctx.emit(&format!("{} = fcmp one double {}, 0.0", cb, val));
                let ts = ctx.tmp();
                ctx.emit(&format!("{} = select i1 {}, i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.true_s, i64 0, i64 0), i8* getelementptr inbounds ([6 x i8], [6 x i8]* @.false_s, i64 0, i64 0)", ts, cb));
                ctx.emit(&format!("call i32 @puts(i8* {})", ts));
            }
            ExprTyHint::Num => {
                ctx.emit(&format!("call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.fmt_num, i64 0, i64 0), double {})", val));
                ctx.emit("call i32 @puts(i8* getelementptr inbounds ([1 x i8], [1 x i8]* @.empty_s, i64 0, i64 0))");
            }
            ExprTyHint::Unknown => {
                ctx.emit(&format!("call void @__hulk_print_val(double {})", val));
            }
        }
    }
}

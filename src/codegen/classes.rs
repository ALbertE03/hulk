use std::collections::HashMap;
use crate::ast::nodes::{TypeDecl, TypeAnnotation, Expr};
use crate::utils::Spanned;
use super::context::Ctx;
use super::utils::{val_ty_from_annotation, ValTy};

// ─────────────────────────────────────────────────────────────────────────────
// Ordenamiento topológico de clases para emitir padres antes que hijos
// ─────────────────────────────────────────────────────────────────────────────

pub fn topo_sort_classes(
    decl_map: &HashMap<String, &TypeDecl>,
    parent_map: &HashMap<String, String>,
) -> Vec<String> {
    let mut visited: HashMap<String, bool> = HashMap::new();
    let mut order: Vec<String> = Vec::new();

    fn visit(
        name: &str,
        parent_map: &HashMap<String, String>,
        decl_map: &HashMap<String, &TypeDecl>,
        visited: &mut HashMap<String, bool>,
        order: &mut Vec<String>,
    ) {
        if visited.contains_key(name) { return; }
        visited.insert(name.to_string(), true);
        if let Some(parent) = parent_map.get(name) {
            if decl_map.contains_key(parent.as_str()) {
                visit(parent, parent_map, decl_map, visited, order);
            }
        }
        order.push(name.to_string());
    }

    for name in decl_map.keys() {
        visit(name, parent_map, decl_map, &mut visited, &mut order);
    }
    order
}

// ─────────────────────────────────────────────────────────────────────────────
// Emisión de Clases / Tipos  (con soporte de herencia profunda)
// ─────────────────────────────────────────────────────────────────────────────

pub fn emit_class(ctx: &mut Ctx, td: &TypeDecl, gen_expr: &dyn Fn(&mut Ctx, &Spanned<Expr>) -> String, infer_return_type_from_body: &dyn Fn(&Expr, &HashMap<String, String>) -> Option<String>) {
    // ── 1. Recopilar atributos + métodos heredados del padre ────────────
    let mut attr_indices: HashMap<String, u32> = HashMap::new();
    let mut attr_order: Vec<String> = Vec::new();
    let mut method_names: HashMap<String, String> = HashMap::new();
    let mut attr_types: HashMap<String, String> = HashMap::new();
    let mut parent_name: Option<String> = None;
    let mut idx: u32 = 1; // slot 0 = type-id (i32, rellenado a 8 bytes vía store i64)

    if let Some(ref pi) = td.parent {
        parent_name = Some(pi.name.clone());
        // Copiar todo del layout del padre
        if let Some(parent_layout) = ctx.classes.get(&pi.name) {
            for attr_name in &parent_layout.attr_order {
                let parent_idx = parent_layout.attr_indices[attr_name];
                attr_indices.insert(attr_name.clone(), parent_idx);
                attr_order.push(attr_name.clone());
                if parent_idx >= idx { idx = parent_idx + 1; }
            }
            // Heredar métodos (el hijo puede sobreescribir después)
            for (mname, mfn) in &parent_layout.method_names {
                method_names.insert(mname.clone(), mfn.clone());
            }
            // Heredar tipos de atributos
            for (aname, atype) in &parent_layout.attr_types {
                attr_types.insert(aname.clone(), atype.clone());
            }
        }
    }

    // ── 2. Atributos propios ──────────────────────────────────────────────
    // Construir mapa de tipos de parámetros: nombre_param -> nombre_tipo  (ej., "name" -> "String")
    let param_types: HashMap<String, String> = td.params.iter().filter_map(|p| {
        if let Some(TypeAnnotation::Name(ref tn)) = p.type_annotation {
            Some((p.name.clone(), tn.clone()))
        } else {
            None
        }
    }).collect();

    for attr in &td.attributes {
        if !attr_indices.contains_key(&attr.name) {
            attr_indices.insert(attr.name.clone(), idx);
            attr_order.push(attr.name.clone());
            idx += 1;
        }
        // Inferir tipo de atributo desde:
        //  a) Anotación de tipo explícita del atributo
        //  b) Expresión de inicialización referenciando un parámetro tipado del constructor
        //  c) Tipo literal de la expresión de inicialización
        if !attr_types.contains_key(&attr.name) {
            if let Some(TypeAnnotation::Name(ref tn)) = attr.type_annotation {
                attr_types.insert(attr.name.clone(), tn.clone());
            } else if let Expr::Identifier(ref pname) = attr.init.node {
                if let Some(tn) = param_types.get(pname) {
                    attr_types.insert(attr.name.clone(), tn.clone());
                }
            } else if matches!(attr.init.node, Expr::String(_)) {
                attr_types.insert(attr.name.clone(), "String".to_string());
            } else if matches!(attr.init.node, Expr::Number(_)) {
                attr_types.insert(attr.name.clone(), "Number".to_string());
            } else if matches!(attr.init.node, Expr::Boolean(_)) {
                attr_types.insert(attr.name.clone(), "Boolean".to_string());
            }
        }
    }

    // ── 3. Tipo struct { i64 typeid, double, double, ... } ──────────────
    let struct_name = format!("%T.{}", td.name);
    let mut fields = "i64".to_string(); // slot 0 = id de tipo
    for _ in 1..idx { fields.push_str(", double"); }

    ctx.globals.push_str(&format!("{} = type {{ {} }}\n", struct_name, fields));

    // ── 4. Registrar métodos propios (sobreescribir heredados) ──────────────
    let mut method_ret_types: HashMap<String, String> = HashMap::new();
    // Heredar tipos de retorno de métodos del padre
    if let Some(ref pi) = td.parent {
        if let Some(parent_layout) = ctx.classes.get(&pi.name) {
            for (mname, mret) in &parent_layout.method_ret_types {
                method_ret_types.insert(mname.clone(), mret.clone());
            }
        }
    }
    for m in &td.methods {
        method_names.insert(m.name.clone(), format!("@{}_{}", td.name, m.name));
        // Inferir tipo de retorno del método desde anotación explícita o heurística del cuerpo
        if let Some(TypeAnnotation::Name(ref tn)) = m.return_type {
            method_ret_types.insert(m.name.clone(), tn.clone());
        } else {
            // Intentar inferir desde la expresión del cuerpo del método
            if let Some(rt) = infer_return_type_from_body(&m.body.node, &attr_types) {
                method_ret_types.insert(m.name.clone(), rt);
            }
        }
    }

    let type_id = ctx.alloc_type_id();

    use super::context::ClassLayout;
    ctx.classes.insert(td.name.clone(), ClassLayout {
        struct_name: struct_name.clone(),
        type_id,
        attr_indices: attr_indices.clone(),
        attr_order: attr_order.clone(),
        method_names: method_names.clone(),
        parent_name: parent_name.clone(),
        total_fields: idx,
        attr_types: attr_types.clone(),
        method_ret_types,
    });

    // ── 5. Emitir constructor: @Tipo_new(args...) -> i8* ──────────────────
    {
        let mut sig = String::new();
        for (i, p) in td.params.iter().enumerate() {
            if i > 0 { sig.push_str(", "); }
            sig.push_str(&format!("double %{}", p.name));
        }
        ctx.functions.push_str(&format!("define i8* @{}_new({}) {{\nentry:\n", td.name, sig));

        let sz = idx as u64 * 8;
        let raw = ctx.tmp();
        ctx.emit(&format!("{} = call i8* @malloc(i64 {})", raw, sz));

        let typed = ctx.tmp();
        ctx.emit(&format!("{} = bitcast i8* {} to {}*", typed, raw, struct_name));

        // Almacenar type-id en slot 0
        let gep_tid = ctx.tmp();
        ctx.emit(&format!("{} = getelementptr inbounds {}, {}* {}, i32 0, i32 0",
            gep_tid, struct_name, struct_name, typed));
        ctx.emit(&format!("store i64 {}, i64* {}", type_id, gep_tid));

        // Parámetros del constructor en el ámbito
        ctx.enter_scope();
        for p in &td.params {
            let ptr = ctx.tmp();
            ctx.emit(&format!("{} = alloca double", ptr));
            ctx.emit(&format!("store double %{}, double* {}", p.name, ptr));
            ctx.def_var(&p.name, &ptr, val_ty_from_annotation(&p.type_annotation));
        }

        // Si el padre tiene constructor y se dan args de inicialización del padre, llamar
        // al constructor del padre para inicializar atributos heredados.
        if let Some(ref pi) = td.parent {
            if ctx.classes.contains_key(&pi.name) {
                // Evaluar args del constructor padre
                let mut parent_vals = Vec::new();
                for a in &pi.args {
                    parent_vals.push(gen_expr(ctx, a));
                }
                // Llamar al constructor padre para obtener un obj padre temporal, luego copiar sus atributos
                let mut parent_arg_s = String::new();
                for (i, v) in parent_vals.iter().enumerate() {
                    if i > 0 { parent_arg_s.push_str(", "); }
                    parent_arg_s.push_str("double "); parent_arg_s.push_str(v);
                }
                let parent_raw = ctx.tmp();
                ctx.emit(&format!("{} = call i8* @{}_new({})", parent_raw, pi.name, parent_arg_s));

                // Copiar atributos del padre desde parent_raw a nuestro objeto
                let parent_sn = ctx.classes.get(&pi.name).map(|l| l.struct_name.clone())
                    .unwrap_or_default();
                let parent_typed = ctx.tmp();
                ctx.emit(&format!("{} = bitcast i8* {} to {}*", parent_typed, parent_raw, parent_sn));

                if let Some(parent_layout) = ctx.classes.get(&pi.name) {
                    let parent_attr_order = parent_layout.attr_order.clone();
                    let parent_attr_indices = parent_layout.attr_indices.clone();
                    let parent_sn2 = parent_layout.struct_name.clone();
                    for attr_name in &parent_attr_order {
                        let pidx = parent_attr_indices[attr_name];
                        let gep_src = ctx.tmp();
                        ctx.emit(&format!("{} = getelementptr inbounds {}, {}* {}, i32 0, i32 {}",
                            gep_src, parent_sn2, parent_sn2, parent_typed, pidx));
                        let val = ctx.tmp();
                        ctx.emit(&format!("{} = load double, double* {}", val, gep_src));
                        // Almacenar en el hijo con el mismo índice
                        let child_idx = attr_indices[attr_name];
                        let gep_dst = ctx.tmp();
                        ctx.emit(&format!("{} = getelementptr inbounds {}, {}* {}, i32 0, i32 {}",
                            gep_dst, struct_name, struct_name, typed, child_idx));
                        ctx.emit(&format!("store double {}, double* {}", val, gep_dst));
                    }
                }
            }
        }

        // Inicializar atributos propios
        for attr in &td.attributes {
            let val = gen_expr(ctx, &attr.init);
            let i = attr_indices[&attr.name];
            let gep = ctx.tmp();
            ctx.emit(&format!("{} = getelementptr inbounds {}, {}* {}, i32 0, i32 {}",
                gep, struct_name, struct_name, typed, i));
            ctx.emit(&format!("store double {}, double* {}", val, gep));
        }

        ctx.exit_scope();
        ctx.emit(&format!("ret i8* {}", raw));
        ctx.functions.push_str("}\n\n");
    }

    // ── 6. Emitir métodos ─────────────────────────────────────────────────
    ctx.current_class = Some(td.name.clone());
    for m in &td.methods {
        let fname = format!("{}_{}", td.name, m.name);
        let mut sig = String::from("i8* %self");
        for p in &m.params {
            sig.push_str(&format!(", double %{}", p.name));
        }
        ctx.functions.push_str(&format!("define double @{}({}) {{\nentry:\n", fname, sig));
        ctx.enter_scope();

        for p in &m.params {
            let ptr = ctx.tmp();
            ctx.emit(&format!("{} = alloca double", ptr));
            ctx.emit(&format!("store double %{}, double* {}", p.name, ptr));
            ctx.def_var(&p.name, &ptr, val_ty_from_annotation(&p.type_annotation));
        }

        let res = gen_expr(ctx, &m.body);
        ctx.emit(&format!("ret double {}", res));
        ctx.functions.push_str("}\n\n");
        ctx.exit_scope();
    }
    ctx.current_class = None;
}

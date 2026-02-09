use crate::ast::nodes::{FunctionDecl, MacroDecl, TypeAnnotation, MacroParam, Expr};
use crate::utils::Spanned;
use super::context::Ctx;
use super::utils::{ValTy, val_ty_from_annotation};

// ─────────────────────────────────────────────────────────────────────────────
// Emisión de funciones helper (conversión de tipos, impresión)
// ─────────────────────────────────────────────────────────────────────────────

pub fn emit_helper_functions(ctx: &mut Ctx) {
    // ── @__hulk_num_to_str(double) -> i8*  ─  convertir un número a cadena en el heap
    //    Si el número es entero (floor(x) == x y |x| < 1e15), imprime sin decimales.
    //    Si tiene parte decimal, usa %g con 15 dígitos significativos.
    ctx.functions.push_str("\
define i8* @__hulk_num_to_str(double %val) {
entry:
  %fl = call double @llvm.floor.f64(double %val)
  %diff = fsub double %val, %fl
  %is_int = fcmp oeq double %diff, 0.0
  %abs_val = call double @llvm.fabs.f64(double %val)
  %small_enough = fcmp olt double %abs_val, 1.0e15
  %use_int = and i1 %is_int, %small_enough
  br i1 %use_int, label %fmt_as_int, label %fmt_as_dbl
fmt_as_int:
  %ilen = call i32 (i8*, i64, i8*, ...) @snprintf(i8* null, i64 0, i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.fmt_int, i64 0, i64 0), double %val)
  %ilen64 = sext i32 %ilen to i64
  %ibufsz = add i64 %ilen64, 1
  %ibuf = call i8* @malloc(i64 %ibufsz)
  call i32 (i8*, i64, i8*, ...) @snprintf(i8* %ibuf, i64 %ibufsz, i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.fmt_int, i64 0, i64 0), double %val)
  ret i8* %ibuf
fmt_as_dbl:
  %dlen = call i32 (i8*, i64, i8*, ...) @snprintf(i8* null, i64 0, i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.fmt_num, i64 0, i64 0), double %val)
  %dlen64 = sext i32 %dlen to i64
  %dbufsz = add i64 %dlen64, 1
  %dbuf = call i8* @malloc(i64 %dbufsz)
  call i32 (i8*, i64, i8*, ...) @snprintf(i8* %dbuf, i64 %dbufsz, i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.fmt_num, i64 0, i64 0), double %val)
  ret i8* %dbuf
}

");

    // ── @__hulk_bool_to_str(double) -> i8*  ─  retorna puntero a \"true\" o \"false\"
    ctx.functions.push_str("\
define i8* @__hulk_bool_to_str(double %val) {
entry:
  %cond = fcmp one double %val, 0.0
  %res = select i1 %cond, i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.true_s, i64 0, i64 0), i8* getelementptr inbounds ([6 x i8], [6 x i8]* @.false_s, i64 0, i64 0)
  ret i8* %res
}

");

    // ── @__hulk_print_val(double)  ─  despachador de impresión en tiempo de ejecución
    //    Usa una etiqueta pasada como segundo argumento:
    //      0 = número, 1 = cadena (puntero), 2 = booleano
    //    Respaldo: imprimir como número (por defecto seguro).
    ctx.functions.push_str("\
define void @__hulk_print_val(double %val) {
entry:
  ; Respaldo: imprimir como número (por defecto seguro para tipo Unknown)
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.fmt_num, i64 0, i64 0), double %val)
  call i32 @puts(i8* getelementptr inbounds ([1 x i8], [1 x i8]* @.empty_s, i64 0, i64 0))
  ret void
}

");

    // ── @__hulk_to_str(double) -> i8*  ─  convertir a cadena en tiempo de ejecución
    //    Respaldo seguro: siempre tratar como número. Las cadenas nunca deberían llegar aquí
    //    porque expr_type_hint debería detectarlas en tiempo de compilación.
    ctx.functions.push_str("\
define i8* @__hulk_to_str(double %val) {
entry:
  %numstr = call i8* @__hulk_num_to_str(double %val)
  ret i8* %numstr
}

");
}

// ─────────────────────────────────────────────────────────────────────────────
// Emisión de funciones de nivel superior
// ─────────────────────────────────────────────────────────────────────────────

/// Mangling del nombre de función HULK para que no colisione con el entry point de C @main.
pub fn mangle_fn(name: &str) -> String {
    if name == "main" { "__hulk_main".to_string() } else { name.to_string() }
}

pub fn emit_function(ctx: &mut Ctx, fd: &FunctionDecl, gen_expr: &dyn Fn(&mut Ctx, &Spanned<Expr>) -> String) {
    // Registrar tipo de retorno de la función
    let ret_vty = match &fd.return_type {
        Some(TypeAnnotation::Name(n)) => match n.as_str() {
            "String" => ValTy::Str,
            "Number" => ValTy::Num,
            "Boolean" => ValTy::Bool,
            other => ValTy::Obj(other.to_string()),
        },
        _ => ValTy::Num,
    };
    ctx.func_ret_types.insert(fd.name.clone(), ret_vty);

    let ir_name = mangle_fn(&fd.name);
    let mut sig = String::new();
    for (i, p) in fd.params.iter().enumerate() {
        if i > 0 { sig.push_str(", "); }
        sig.push_str(&format!("double %{}", p.name));
    }
    ctx.functions.push_str(&format!("define double @{}({}) {{\nentry:\n", ir_name, sig));
    ctx.enter_scope();

    for p in &fd.params {
        let ptr = ctx.tmp();
        ctx.emit(&format!("{} = alloca double", ptr));
        ctx.emit(&format!("store double %{}, double* {}", p.name, ptr));
        ctx.def_var(&p.name, &ptr, val_ty_from_annotation(&p.type_annotation));
    }

    let res = gen_expr(ctx, &fd.body);
    ctx.emit(&format!("ret double {}", res));
    ctx.functions.push_str("}\n\n");
    ctx.exit_scope();
}

/// Emitir un macro `def` como una función LLVM regular.
/// Solo parámetros Normal son soportados a nivel de codegen; los Simbólicos (@), Placeholder ($)
/// y Body (*) se tratan como paso por valor normal por ahora.
pub fn emit_macro(ctx: &mut Ctx, md: &MacroDecl, gen_expr: &dyn Fn(&mut Ctx, &Spanned<Expr>) -> String) {
    // Registrar tipo de retorno
    let ret_vty = match &md.return_type {
        Some(TypeAnnotation::Name(n)) => match n.as_str() {
            "String" => ValTy::Str,
            "Number" => ValTy::Num,
            "Boolean" => ValTy::Bool,
            other => ValTy::Obj(other.to_string()),
        },
        _ => ValTy::Num,
    };
    ctx.func_ret_types.insert(md.name.clone(), ret_vty);

    let ir_name = mangle_fn(&md.name);
    let mut sig = String::new();
    for (i, p) in md.params.iter().enumerate() {
        if i > 0 { sig.push_str(", "); }
        let pname = match p {
            MacroParam::Normal { name, .. } => name,
            MacroParam::Symbolic { name, .. } => name,
            MacroParam::Placeholder { name, .. } => name,
            MacroParam::Body { name, .. } => name,
        };
        sig.push_str(&format!("double %{}", pname));
    }
    ctx.functions.push_str(&format!("define double @{}({}) {{\nentry:\n", ir_name, sig));
    ctx.enter_scope();

    for p in &md.params {
        let (pname, ann) = match p {
            MacroParam::Normal { name, type_annotation } => (name, Some(TypeAnnotation::Name(match type_annotation { TypeAnnotation::Name(n) => n.clone(), _ => "Number".to_string() }))),
            MacroParam::Symbolic { name, type_annotation } => (name, Some(TypeAnnotation::Name(match type_annotation { TypeAnnotation::Name(n) => n.clone(), _ => "Number".to_string() }))),
            MacroParam::Placeholder { name, type_annotation } => (name, Some(TypeAnnotation::Name(match type_annotation { TypeAnnotation::Name(n) => n.clone(), _ => "Number".to_string() }))),
            MacroParam::Body { name, type_annotation } => (name, Some(TypeAnnotation::Name(match type_annotation { TypeAnnotation::Name(n) => n.clone(), _ => "Number".to_string() }))),
        };
        let ptr = ctx.tmp();
        ctx.emit(&format!("{} = alloca double", ptr));
        ctx.emit(&format!("store double %{}, double* {}", pname, ptr));
        ctx.def_var(pname, &ptr, val_ty_from_annotation(&ann));
    }

    let res = gen_expr(ctx, &md.body);
    ctx.emit(&format!("ret double {}", res));
    ctx.functions.push_str("}\n\n");
    ctx.exit_scope();
}

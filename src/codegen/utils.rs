use crate::ast::nodes::*;

/// Escapar caracteres especiales para LLVM IR.
pub fn escape_llvm(s: &str) -> String {
    s.replace('\\', "\\5C")
     .replace('\n', "\\0A")
     .replace('\t', "\\09")
     .replace('\"', "\\22")
}

/// Formatear un f64 como literal double de LLVM IR.
/// LLVM requiere un punto decimal en constantes flotantes (ej. `1.0e1` no `1e1`).
pub fn fmt_double(v: f64) -> String {
    let s = format!("{:e}", v);
    // Si la parte de la mantisa no tiene '.', insertar ".0" antes de la 'e'
    if let Some(pos) = s.find('e') {
        let mantissa = &s[..pos];
        if !mantissa.contains('.') {
            return format!("{}.0e{}", mantissa, &s[pos+1..]);
        }
    }
    // respaldo: si no hay 'e', asegurar un punto
    if !s.contains('.') && !s.contains('e') {
        return format!("{}.0", s);
    }
    s
}

/// Etiqueta ligera para saber cómo cargar / imprimir un valor.
#[derive(Clone, Debug, PartialEq)]
#[allow(dead_code)]
pub enum ValTy { Num, Bool, Str, Ptr, Obj(String), Fn(Box<ValTy>) }

/// Convertir anotación de tipo a ValTy.
pub fn val_ty_from_annotation(ann: &Option<TypeAnnotation>) -> ValTy {
    match ann {
        Some(TypeAnnotation::Name(n)) => match n.as_str() {
            "String" => ValTy::Str,
            "Number" => ValTy::Num,
            "Boolean" => ValTy::Bool,
            other => ValTy::Obj(other.to_string()),
        },
        _ => ValTy::Num,
    }
}

/// Mangling del nombre de función HULK para que no colisione con el entry point de C @main.
pub fn mangle_fn(name: &str) -> String {
    if name == "main" { "__hulk_main".to_string() } else { name.to_string() }
}

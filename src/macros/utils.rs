use std::sync::atomic::{AtomicUsize, Ordering};

/// Contador global para generar nombres únicos
static GENSYM_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// Genera un nombre único para variables sanitizadas
/// Si el prefijo es válido (no empieza con _), genera nombre sin _ al inicio
/// Si es inválido, retorna el prefijo original sin modificar
pub fn gensym(prefix: &str) -> String {
    // Si el prefijo empieza con _, es inválido - retornar sin generar
    if prefix.starts_with('_') {
        return prefix.to_string();
    }
    
    // Para prefijos válidos, generar con $$ y contador
    let count = GENSYM_COUNTER.fetch_add(1, Ordering::SeqCst);
    format!("{}$${}", prefix, count)
}

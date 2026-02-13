use super::context::Ctx;
use std::collections::HashMap;

/// Emite el tipo implícito `__Vector` que envuelve un array de doubles.
/// Estructura: [ type_id (i64), data_ptr (double*), length (double), index (double) ]
///
/// Métodos:
/// - size(): Number - retorna el número de elementos
/// - next(): Boolean - avanza el iterador, retorna true si hay más elementos
/// - current(): Number - retorna el elemento actual (get_current en el protocolo)
pub fn emit_vector_type(ctx: &mut Ctx) {
    // Asignar type_id único para Vector
    let type_id = ctx.next_type_id;
    ctx.next_type_id += 1;
    
    // Estructura: %__Vector = type { i64, double*, double, double }
    //   slot 0: type_id
    //   slot 1: puntero al array de datos (double*)
    //   slot 2: longitud (double)
    //   slot 3: índice de iteración actual (double)
    ctx.preamble.push_str("%__Vector = type { i64, double*, double, double }\n");
    
    // Constructor: __Vector_new(double* data, double length)
    ctx.functions.push_str("\
define i8* @__Vector_new(double* %data, double %length) {
entry:
  %obj = call i8* @malloc(i64 32)
  %ptr = bitcast i8* %obj to %__Vector*
  
  ; Almacenar type_id
  %tid_ptr = getelementptr inbounds %__Vector, %__Vector* %ptr, i32 0, i32 0
  store i64 ");
    ctx.functions.push_str(&type_id.to_string());
    ctx.functions.push_str(", i64* %tid_ptr
  
  ; Almacenar data pointer
  %data_ptr = getelementptr inbounds %__Vector, %__Vector* %ptr, i32 0, i32 1
  store double* %data, double** %data_ptr
  
  ; Almacenar length
  %len_ptr = getelementptr inbounds %__Vector, %__Vector* %ptr, i32 0, i32 2
  store double %length, double* %len_ptr
  
  ; Inicializar index en -1
  %idx_ptr = getelementptr inbounds %__Vector, %__Vector* %ptr, i32 0, i32 3
  store double -1.0, double* %idx_ptr
  
  ret i8* %obj
}

");

    // Método size(): Number
    ctx.functions.push_str("\
define double @__Vector_size(i8* %self) {
entry:
  %ptr = bitcast i8* %self to %__Vector*
  %len_ptr = getelementptr inbounds %__Vector, %__Vector* %ptr, i32 0, i32 2
  %len = load double, double* %len_ptr
  ret double %len
}

");

    // Método next(): Boolean
    ctx.functions.push_str("\
define double @__Vector_next(i8* %self) {
entry:
  %ptr = bitcast i8* %self to %__Vector*
  
  ; Cargar índice actual
  %idx_ptr = getelementptr inbounds %__Vector, %__Vector* %ptr, i32 0, i32 3
  %idx = load double, double* %idx_ptr
  
  ; Incrementar índice
  %next_idx = fadd double %idx, 1.0
  store double %next_idx, double* %idx_ptr
  
  ; Cargar length
  %len_ptr = getelementptr inbounds %__Vector, %__Vector* %ptr, i32 0, i32 2
  %len = load double, double* %len_ptr
  
  ; Comparar: next_idx < len
  %cond = fcmp olt double %next_idx, %len
  %result = select i1 %cond, double 1.0, double 0.0
  ret double %result
}

");

    // Método get_current(): Number (actual value at current index)
    ctx.functions.push_str("\
define double @__Vector_get_current(i8* %self) {
entry:
  %ptr = bitcast i8* %self to %__Vector*
  
  ; Cargar índice actual
  %idx_ptr = getelementptr inbounds %__Vector, %__Vector* %ptr, i32 0, i32 3
  %idx_d = load double, double* %idx_ptr
  %idx = fptosi double %idx_d to i64
  
  ; Cargar data pointer
  %data_ptr = getelementptr inbounds %__Vector, %__Vector* %ptr, i32 0, i32 1
  %data = load double*, double** %data_ptr
  
  ; Acceder al elemento en data[idx]
  %elem_ptr = getelementptr double, double* %data, i64 %idx
  %value = load double, double* %elem_ptr
  ret double %value
}

");

    // Registrar en ctx.classes para que sea conocido por el sistema de tipos
    let mut attr_indices = HashMap::new();
    let mut attr_order = Vec::new();
    let mut method_names = HashMap::new();
    let mut method_ret_types = HashMap::new();
    let mut attr_types = HashMap::new();
    
    // No tiene atributos accesibles directamente (son internos)
    
    // Registrar métodos
    method_names.insert("size".to_string(), "@__Vector_size".to_string());
    method_names.insert("next".to_string(), "@__Vector_next".to_string());
    method_names.insert("get_current".to_string(), "@__Vector_get_current".to_string());
    
    method_ret_types.insert("size".to_string(), "Number".to_string());
    method_ret_types.insert("next".to_string(), "Boolean".to_string());
    method_ret_types.insert("get_current".to_string(), "Number".to_string());
    
    use super::context::ClassLayout;
    ctx.classes.insert("__Vector".to_string(), ClassLayout {
        struct_name: "%__Vector".to_string(),
        type_id,
        attr_indices,
        attr_order,
        method_names,
        method_ret_types,
        attr_types,
        parent_name: None,
        total_fields: 4, // type_id + data_ptr + length + index
    });
}

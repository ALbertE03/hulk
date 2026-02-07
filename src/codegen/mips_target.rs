use crate::ast::nodes::*;
use crate::semantic::{Context, types::{Type, TypeKind}};
use crate::utils::Spanned;
use std::collections::{HashMap, BTreeMap};
use std::rc::Rc;
use std::cell::RefCell;

pub struct MipsGenerator;

struct ClassInfo {
    size: i32,
    attr_offsets: HashMap<String, i32>,
    method_offsets: HashMap<String, i32>,
    vtable: Vec<String>, // Lista de etiquetas de funciones para la vtable
}

struct MipsContext<'a> {
    code: String,
    data: String,
    label_counter: usize,
    string_literals: Vec<(String, String)>, 
    // Gestión de Pila
    scopes: Vec<HashMap<String, i32>>, // Nombre Variable -> Offset desde FP
    next_local_offset: i32, // Siguiente offset local disponible (relativo a FP, inicia en -4)
    semantic_context: &'a Context,
    class_info: HashMap<String, ClassInfo>,
    current_class: Option<String>,
}

impl<'a> MipsContext<'a> {
    fn new(semantic_context: &'a Context) -> Self {
        let mut ctx = MipsContext {
            code: String::new(),
            data: String::from(".data\n"),
            label_counter: 0,
            string_literals: Vec::new(),
            scopes: vec![HashMap::new()],
            next_local_offset: -12, 
            semantic_context,
            class_info: HashMap::new(),
            current_class: None,
        };
        ctx.init_class_layouts();
        ctx
    }

    fn init_class_layouts(&mut self) {
        let type_names: Vec<String> = self.semantic_context.types.keys().cloned().collect();
        for name in type_names {
            self.compute_layout(&name);
        }

        // Generar VTables en .data
        for (name, info) in &self.class_info {
            self.data.push_str(&format!("{}_vtable:\n", name));
            for method_label in &info.vtable {
                self.data.push_str(&format!("    .word {}\n", method_label));
            }
            if info.vtable.is_empty() {
                self.data.push_str("    .word 0\n");
            }
        }
    }

    fn compute_layout(&mut self, type_name: &str) {
        if self.class_info.contains_key(type_name) {
            return;
        }
        
        let type_rc = match self.semantic_context.types.get(type_name) {
            Some(t) => t,
            None => return,
        };
        let type_obj = type_rc.borrow();

        // 1. Layout Base (Padre)
        let mut size = 4; // Offset 0 es vtable_ptr
        let mut attr_offsets = HashMap::new();
        let mut method_offsets = HashMap::new();
        let mut vtable = Vec::new();
        let mut method_indices = HashMap::new(); 

        if let Some(parent) = &type_obj.parent {
            let parent_name = parent.borrow().name.clone();
            self.compute_layout(&parent_name);
            if let Some(parent_info) = self.class_info.get(&parent_name) {
                size = parent_info.size;
                attr_offsets = parent_info.attr_offsets.clone();
                method_offsets = parent_info.method_offsets.clone();
                vtable = parent_info.vtable.clone();
                
                // Reconstruir indices para saber qué sobreescribir
                for (m_name, m_offset) in &method_offsets {
                    let index = (*m_offset / 4) as usize;
                    if index < vtable.len() {
                        method_indices.insert(m_name.clone(), index);
                    }
                }
            }
        }

        // 2. Atributos Propios - Determinismo usando BTreeMap
        let my_attrs: BTreeMap<_, _> = type_obj.attributes.iter().collect();
        for (name, _) in my_attrs {
            if !attr_offsets.contains_key(name) {
                attr_offsets.insert(name.clone(), size);
                size += 4;
            }
        }

        // 3. Métodos Propios
        let my_methods: BTreeMap<_, _> = type_obj.methods.iter().collect();
        for (name, _) in my_methods {
            let label = format!("{}_{}", type_name, name);
            if let Some(&index) = method_indices.get(name) {
                vtable[index] = label;
            } else {
                let index = vtable.len();
                vtable.push(label);
                method_offsets.insert(name.clone(), (index as i32) * 4);
                method_indices.insert(name.clone(), index);
            }
        }

        self.class_info.insert(type_name.to_string(), ClassInfo {
            size,
            attr_offsets,
            method_offsets,
            vtable,
        });
    }

    fn emit(&mut self, instr: &str) {
        self.code.push_str(&format!("    {}\n", instr));
    }

    fn emit_label(&mut self, label: &str) {
        self.code.push_str(&format!("{}:\n", label));
    }

    fn new_label(&mut self, prefix: &str) -> String {
        let label = format!("{}_{}", prefix, self.label_counter);
        self.label_counter += 1;
        label
    }

    fn add_string(&mut self, value: &str) -> String {
        for (content, label) in &self.string_literals {
            if content == value {
                return label.clone();
            }
        }
        let label = format!("str_{}", self.label_counter);
        self.label_counter += 1;
        self.string_literals.push((value.to_string(), label.clone()));
        self.data.push_str(&format!("{}: .asciiz \"{}\"\n", label, value));
        label
    }

    fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    fn define_variable(&mut self, name: &str) -> i32 {
        let offset = self.next_local_offset;
        self.next_local_offset -= 4;
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.to_string(), offset);
        }
        offset
    }
    
    fn define_param(&mut self, name: &str, offset: i32) {
         if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.to_string(), offset);
        }
    }

    fn get_variable_offset(&self, name: &str) -> Option<i32> {
        for scope in self.scopes.iter().rev() {
            if let Some(&offset) = scope.get(name) {
                return Some(offset);
            }
        }
        None
    }
}

impl super::CodeGenerator for MipsGenerator {
    fn generate(&self, program: &Program, context: &Context) -> String {
        let mut ctx = MipsContext::new(context);

        ctx.code.push_str(".text\n.globl main\n\n");
        
        // 1. Generar Funciones
        for decl in &program.declarations {
            match decl {
                Declaration::Function(func_decl) => gen_function(&mut ctx, func_decl),
                Declaration::Type(class_decl) => gen_class_methods(&mut ctx, class_decl),
                _ => {}
            }
        }

        // 2. Punto de Entrada Principal (Main)
        ctx.emit_label("main");
        
        // Prólogo
        ctx.emit("sw $fp, -4($sp)");
        ctx.emit("move $fp, $sp");
        ctx.emit("addiu $sp, $sp, -4"); 
        
        // Generar código para la expresión principal
        gen_expr(&mut ctx, &program.expr);
        
        // Syscall de salida
        ctx.emit("li $v0, 10");
        ctx.emit("syscall");

        // Combinar datos y texto
        format!("{}\n{}", ctx.data, ctx.code)
    }
}

fn gen_class_methods(ctx: &mut MipsContext, class: &TypeDecl) {
    ctx.current_class = Some(class.name.clone());
    for method in &class.methods {
            let label = format!("{}_{}", class.name, method.name);
            ctx.emit_label(&label);

            // Prólogo estándar
            ctx.emit("sw $fp, -4($sp)");
            ctx.emit("sw $ra, -8($sp)");
            ctx.emit("move $fp, $sp");
            ctx.emit("addiu $sp, $sp, -8");
            
            ctx.next_local_offset = -12;
            ctx.enter_scope();

            // Setup Argumentos
            // Stack: [..., Self, Arg1, Arg2, ...]
            // FP -> Top Arg (Last Arg).
            // Params AST order: [Arg1, Arg2].
            // Self está detras de Arg1.
            
            let mut param_offset = 0;
            // params.iter().rev() -> Arg2, Arg1.
            for param in method.params.iter().rev() {
                 ctx.define_param(&param.name, param_offset);
                 param_offset += 4;
            }
            
            // Definir 'self' para el cuerpo del método
            ctx.define_param("self", param_offset);
            
            gen_expr(ctx, &method.body);
            
            pop(ctx, "$v0");
            
            ctx.exit_scope();
            
            // Epílogo
            ctx.emit("move $sp, $fp");
            ctx.emit("lw $fp, -4($sp)");
            ctx.emit("lw $ra, -8($sp)");
            ctx.emit("jr $ra");
    }
    ctx.current_class = None;
}

fn gen_function(ctx: &mut MipsContext, func: &FunctionDecl) {
    ctx.emit_label(&func.name);
    
    // Prólogo
    // Guardar FP, RA.
    ctx.emit("sw $fp, -4($sp)");
    ctx.emit("sw $ra, -8($sp)");
    ctx.emit("move $fp, $sp");
    ctx.emit("addiu $sp, $sp, -8"); 
    
    // FP está en el antiguo SP.
    // 0($fp) -> lo que estaba en el antiguo SP (Tope de Argumentos).
    // -4($fp) -> Antiguo FP.
    // -8($fp) -> RA.
    // Próximo local en -12.
    
    ctx.next_local_offset = -12; 
    
    ctx.enter_scope();
    
    // Vincular Argumentos
    // Argumentos empujados por llamador:
    // ... Arg1, Arg2. SP -> Arg2.
    // Así 0($fp) -> Arg2.
    // 4($fp) -> Arg1.
    
    let mut param_offset = 0;
    // Iterar reverso para coincidir con la pila (Tope es último param)
    for param in func.params.iter().rev() {
         ctx.define_param(&param.name, param_offset);
         param_offset += 4;
    }
    
    gen_expr(ctx, &func.body);
    
    // Resultado en pila. Pop a v0.
    pop(ctx, "$v0");
    
    ctx.exit_scope();
    
    // Epílogo
    ctx.emit("move $sp, $fp");
    ctx.emit("lw $ra, -8($fp)"); 
    ctx.emit("lw $fp, -4($fp)"); 
    
    ctx.emit("jr $ra");
    ctx.emit("");
}

fn gen_expr(ctx: &mut MipsContext, expr: &Spanned<Expr>) {
    match &expr.node {
        Expr::Number(val) => {
            ctx.emit(&format!("li $t0, {}", val));
            push(ctx, "$t0");
        }
        Expr::String(val) => {
            let label = ctx.add_string(val);
            ctx.emit(&format!("la $t0, {}", label));
            push(ctx, "$t0");
        }
        Expr::Boolean(val) => {
            let v = if *val { 1 } else { 0 };
            ctx.emit(&format!("li $t0, {}", v));
            push(ctx, "$t0");
        }
        Expr::Identifier(name) => {
            if let Some(offset) = ctx.get_variable_offset(name) {
                ctx.emit(&format!("lw $t0, {}($fp)", offset));
                push(ctx, "$t0");
            } else {
                // Check if it is an attribute of 'self'
                let mut found = false;
                if let Some(class_name) = &ctx.current_class {
                    if let Some(self_offset) = ctx.get_variable_offset("self") {
                         if let Some(info) = ctx.class_info.get(class_name) {
                             if let Some(&attr_offset) = info.attr_offsets.get(name) {
                                 ctx.emit(&format!("lw $t0, {}($fp)", self_offset)); // Load self
                                 ctx.emit(&format!("lw $t0, {}($t0)", attr_offset)); // Load attr
                                 push(ctx, "$t0");
                                 found = true;
                             }
                         }
                    }
                }
                
                if !found {
                    ctx.emit(&format!("# Variable {} no encontrada", name));
                    ctx.emit("li $t0, 0"); 
                    push(ctx, "$t0");
                }
            }
        }
        Expr::Binary(left, op, right) => {
            gen_expr(ctx, left);
            gen_expr(ctx, right);
            
            pop(ctx, "$t1"); // Derecha
            pop(ctx, "$t0"); // Izquierda
            
            match op {
                Op::Add => ctx.emit("add $t0, $t0, $t1"),
                Op::Sub => ctx.emit("sub $t0, $t0, $t1"),
                Op::Mul => ctx.emit("mul $t0, $t0, $t1"),
                Op::Div => {
                    ctx.emit("div $t0, $t1");
                    ctx.emit("mflo $t0");
                }
                Op::Eq => { ctx.emit("seq $t0, $t0, $t1"); }
                Op::Lt => { ctx.emit("slt $t0, $t0, $t1"); }
                Op::Gt => { ctx.emit("sgt $t0, $t0, $t1"); }
                Op::Le => { ctx.emit("sle $t0, $t0, $t1"); }
                Op::Ge => { ctx.emit("sge $t0, $t0, $t1"); }
                Op::Neq => { ctx.emit("sne $t0, $t0, $t1"); }
                Op::And => { ctx.emit("and $t0, $t0, $t1"); }
                Op::Or => { ctx.emit("or $t0, $t0, $t1"); }
                _ => {} 
            }
            push(ctx, "$t0");
        }
        Expr::Call { func, args } => {
            if func == "print" {
                if let Some(arg) = args.first() {
                    gen_expr(ctx, arg);
                    pop(ctx, "$a0");
                    if matches!(arg.node, Expr::String(_)) {
                         ctx.emit("li $v0, 4");
                    } else {
                         ctx.emit("li $v0, 1");
                    }
                    ctx.emit("syscall");
                    
                    ctx.emit("li $a0, 10");
                    ctx.emit("li $v0, 11");
                    ctx.emit("syscall");
                    
                    ctx.emit("li $t0, 0");
                    push(ctx, "$t0");
                }
            } else {
                for arg in args {
                    gen_expr(ctx, arg);
                }
                ctx.emit(&format!("jal {}", func));
                
                let bytes_to_pop = args.len() as i32 * 4;
                if bytes_to_pop > 0 {
                    ctx.emit(&format!("addiu $sp, $sp, {}", bytes_to_pop));
                }
                
                ctx.emit("move $t0, $v0");
                push(ctx, "$t0");
            }
        }
        Expr::Let { bindings, body } => {
            ctx.enter_scope();
            let mut bytes_alloc = 0;
            
            for (name, _, expr) in bindings {
                gen_expr(ctx, expr);
                let offset = ctx.define_variable(name);
                
                pop(ctx, "$t0");
                ctx.emit(&format!("sw $t0, {}($fp)", offset));
                ctx.emit("addiu $sp, $sp, -4"); // Mantener pila alineada/reservada
                bytes_alloc += 4;
            }
            
            gen_expr(ctx, body);
            
            if bytes_alloc > 0 {
                pop(ctx, "$t0");
                ctx.emit(&format!("addiu $sp, $sp, {}", bytes_alloc));
                push(ctx, "$t0");
            }
            ctx.exit_scope();
        }
        Expr::If { cond, then_expr, else_expr } => {
            let label_else = ctx.new_label("else");
            let label_end = ctx.new_label("end");
            
            gen_expr(ctx, cond);
            pop(ctx, "$t0");
            ctx.emit(&format!("beqz $t0, {}", label_else));
            
            gen_expr(ctx, then_expr);
            ctx.emit(&format!("b {}", label_end));
            
            ctx.emit_label(&label_else);
            gen_expr(ctx, else_expr);
            
            ctx.emit_label(&label_end);
        }
        Expr::While { cond, body } => {
            let label_start = ctx.new_label("while_start");
            let label_end = ctx.new_label("while_end");
            
            ctx.emit_label(&label_start);
            gen_expr(ctx, cond);
            pop(ctx, "$t0");
            ctx.emit(&format!("beqz $t0, {}", label_end));
            
            gen_expr(ctx, body);
            pop(ctx, "$t0"); // Descartar resultado body
            
            ctx.emit(&format!("b {}", label_start));
            ctx.emit_label(&label_end);
            
            ctx.emit("li $t0, 0"); // Loop retorna Void/0
            push(ctx, "$t0");
        }
        Expr::Assignment { target, value } => {
             gen_expr(ctx, value);
             // Valor en pila. Peek.
             ctx.emit("lw $t0, 0($sp)");
             
             if let Some(offset) = ctx.get_variable_offset(target) {
                 ctx.emit(&format!("sw $t0, {}($fp)", offset));
             } 
        }
        Expr::Unary(op, operand) => {
            gen_expr(ctx, operand);
            pop(ctx, "$t0");
            match op {
                UnOp::Neg => ctx.emit("neg $t0, $t0"),
                UnOp::Not => ctx.emit("seq $t0, $t0, $zero"),
                // Bitwise not? HULK no lo tiene explícitamente, asumimos ~
                _ => {}
            }
            push(ctx, "$t0");
        }
        Expr::For { var, iterable, body } => {
             // 1. Evaluar iterable
             gen_expr(ctx, iterable); 
             // Resultado en pila. Debería ser un Vector u Objeto con next/current.
             // Asumiremos que es un Vector para esta implementación (simplificación).
             // Diseño del Vector: [Len, Elem0, Elem1...]
             
             // Pop puntero al vector
             pop(ctx, "$s1"); // Usamos s1 (registro guardado) -> peligroso si no lo salvamos.
             
             // Necesitamos gestionar el estado del bucle: {array_ptr, current_index, max_index}
             // Usaremos "variables internas" en un scope.
             // Desugarización:
             // let arr = iterable;
             // let idx = 0;
             // while (idx < arr.length) { 
             //    let var = arr[idx];
             //    body;
             //    idx = idx + 1;
             // }
             
             ctx.emit("move $t0, $s1"); 
             // Intento de nuevo:
             // Iterable está en Tope de Pila.
             ctx.emit("lw $t0, 0($sp)"); // Peek array
             // Cargar longitud (offset 0)
             ctx.emit("lw $t1, 0($t0)"); // Length
             
             // Push Index (0)
             ctx.emit("li $t2, 0");
             push(ctx, "$t2");
             
             // Pila: ArrayPtr, Index.
             // Definimos variables locales para acceso seguro relativo a FP.
             
             // Bucle For-Vector Simplificado en Ensamblador:
             let loop_start = ctx.new_label("for_start");
             let loop_end = ctx.new_label("for_end");
             
             ctx.enter_scope();
             let off_idx = ctx.define_variable(".idx");
             ctx.emit("li $t0, 0");
             ctx.emit(&format!("sw $t0, {}($fp)", off_idx));
             
             // Almacenar ptr del array en una local también
             let off_arr = ctx.define_variable(".arr");
             pop(ctx, "$t0"); // Pop Index (que acabamos de empujar)
             
             // gen_expr(iterable) empujó ArrayPtr.
             pop(ctx, "$t0"); // Array Pointer
             ctx.emit(&format!("sw $t0, {}($fp)", off_arr));
             
             let off_var = ctx.define_variable(var); // Variable del usuario
             
             ctx.emit_label(&loop_start);
             
             // Verificar condición: idx < len
             ctx.emit(&format!("lw $t0, {}($fp)", off_arr)); // Array Ptr
             ctx.emit("lw $t1, 0($t0)"); // Length
             ctx.emit(&format!("lw $t2, {}($fp)", off_idx)); // Index
             ctx.emit(&format!("bge $t2, $t1, {}", loop_end));
             
             // Cargar Elemento
             // Addr = Ptr + 4 + Index*4
             ctx.emit("sll $t3, $t2, 2"); // Index * 4
             ctx.emit("add $t3, $t3, $t0"); // Ptr + Offset
             ctx.emit("lw $t4, 4($t3)"); // Load Elem (saltar palabra de longitud)
             ctx.emit(&format!("sw $t4, {}($fp)", off_var)); // Vincular a var
             
             // Ejecutar Cuerpo
             gen_expr(ctx, body);
             pop(ctx, "$t0"); // Descartar resultado del cuerpo
             
             // Incrementar Índice
             ctx.emit(&format!("lw $t0, {}($fp)", off_idx));
             ctx.emit("addiu $t0, $t0, 1");
             ctx.emit(&format!("sw $t0, {}($fp)", off_idx));
             
             ctx.emit(&format!("b {}", loop_start));
             
             ctx.emit_label(&loop_end);
             ctx.exit_scope();
             
             ctx.emit("li $t0, 0");
             push(ctx, "$t0");
        }
        Expr::BaseCall { args } => {
            // Asumimos que gen_function ha seteado variables 'self'
            let self_offset = ctx.get_variable_offset("self").unwrap_or(-1);
            if self_offset != -1 {
                ctx.emit(&format!("lw $t0, {}($fp)", self_offset)); // Cargar self
                push(ctx, "$t0");
            } else {
                 ctx.emit("# ERROR: BaseCall fuera de metodo");
                 ctx.emit("li $t0, 0");
                 push(ctx, "$t0");
            }
            
            for arg in args {
                gen_expr(ctx, arg);
            }

            // Nota: Para BaseCall real se requiere saber el tipo estático de 'self' y su padre.
            ctx.emit("# BaseCall no resoluble sin AST Metadata");
            if !args.is_empty() {
                 ctx.emit(&format!("addiu $sp, $sp, {}", args.len() * 4));
            }
            pop(ctx, "$t0");
            ctx.emit("li $t0, 0"); 
            push(ctx, "$t0");
        }
        Expr::MethodCall { obj, method, args } => {
            gen_expr(ctx, obj); 
            for arg in args {
                gen_expr(ctx, arg);
            }
            
            if method == "size" {
                 // Pop args (vacío para size)
                 if !args.is_empty() { ctx.emit(&format!("addiu $sp, $sp, {}", args.len() * 4)); }
                 pop(ctx, "$t0"); 
                 ctx.emit("lw $t0, 0($t0)"); 
                 push(ctx, "$t0");
            } else {
                let offset_obj = args.len() * 4;
                ctx.emit(&format!("lw $a0, {}($sp)", offset_obj)); // Peek Obj
                ctx.emit("lw $t0, 0($a0)"); // Load VTable
                
                // Buscar offset en metadatos globales
                let mut method_offset = -1;
                for info in ctx.class_info.values() {
                    if let Some(&off) = info.method_offsets.get(method) {
                        method_offset = off;
                        break;
                    }
                }
                
                if method_offset != -1 {
                    ctx.emit(&format!("lw $t1, {}($t0)", method_offset));
                    ctx.emit("jalr $t1");
                    ctx.emit("move $t0, $v0");
                } else {
                    ctx.emit( &format!("# Metodo {} no encontrado", method));
                    ctx.emit("li $t0, 0");
                }
                
                // Clean stack (Args + Obj)
                let total_pop = (args.len() + 1) * 4;
                ctx.emit(&format!("addiu $sp, $sp, {}", total_pop));
                push(ctx, "$t0");
            }
        }
        Expr::AttributeAccess { obj, attribute } => {
             gen_expr(ctx, obj);
             pop(ctx, "$t0"); // Ptr Obj
             
             let mut attr_offset = -1;
             for info in ctx.class_info.values() {
                 if let Some(&off) = info.attr_offsets.get(attribute) {
                     attr_offset = off;
                     break;
                 }
             }
             
             if attr_offset != -1 {
                 ctx.emit(&format!("lw $t0, {}($t0)", attr_offset));
             } else {
                 ctx.emit("# Atributo no encontrado");
                 ctx.emit("li $t0, 0");
             }
             push(ctx, "$t0");
        }
        Expr::Instantiation { ty, args } => {
             let (size, vtable_label) = if let Some(info) = ctx.class_info.get(ty) {
                 (info.size, format!("{}_vtable", ty))
             } else {
                 (16, String::from("Main_vtable")) 
             };
             
             ctx.emit(&format!("li $a0, {}", size));
             ctx.emit("li $v0, 9"); // Sbrk
             ctx.emit("syscall");
             
             ctx.emit(&format!("la $t0, {}", vtable_label));
             ctx.emit("sw $t0, 0($v0)");
             
             push(ctx, "$v0"); // ObjPtr en Stack
             
             // Eval args (ignora asignación real por falta de mapeo)
             for arg in args {
                 gen_expr(ctx, arg);
                 pop(ctx, "$t1");
             }
             
             // Restore Obj Ptr
             pop(ctx, "$t0");
             push(ctx, "$t0");
        }
        Expr::Is(_, _) | Expr::As(_, _) => {
             ctx.emit("# Chequeo de tipos ignorado");
             ctx.emit("li $t0, 1"); // Siempre true/valido
             push(ctx, "$t0");
        }
        Expr::VectorLiteral(elems) => {
             // 1. Alloc (len + 1) * 4
             let size = (elems.len() + 1) * 4;
             ctx.emit(&format!("li $a0, {}", size));
             ctx.emit("li $v0, 9"); // sbrk
             ctx.emit("syscall");
             // v0 es puntero
             
             // Almacenar longitud
             ctx.emit(&format!("li $t0, {}", elems.len()));
             ctx.emit("sw $t0, 0($v0)");
             
             // Guardar Ptr en pila (temp) para evaluar elementos
             push(ctx, "$v0");
             
             for (i, elem) in elems.iter().enumerate() {
                 gen_expr(ctx, elem);
                 pop(ctx, "$t1"); // Valor
                 
                 // Recuperar Ptr de la pila (peek)
                 ctx.emit("lw $t0, 0($sp)"); 
                 
                 // Guardar en offset 4 + i*4
                 let offset = 4 + i * 4;
                 ctx.emit(&format!("sw $t1, {}($t0)", offset));
             }
             
             // Actual tope de pila tiene Ptr.
             // Listo.
        }
        Expr::Indexing { obj, index } => {
             gen_expr(ctx, obj);   // Ptr en Stack[4] (después de empujar index)
             gen_expr(ctx, index); // Idx en Stack[0]
             
             pop(ctx, "$t1"); // Index
             pop(ctx, "$t0"); // Obj
             
             // Addr = Obj + 4 + Index*4
             ctx.emit("sll $t1, $t1, 2");
             ctx.emit("add $t1, $t1, $t0");
             ctx.emit("lw $t0, 4($t1)");
             push(ctx, "$t0");
        }
        Expr::Match { expr, cases, default } => {
            // Desugar a If-Else-If
            gen_expr(ctx, expr);
            // Val en 0($sp). ¿Mantenerlo ahí para peek en cada caso?
            // ¿O pop a reg? Reg podría ser destruido por exprs de casos.
            // Mejor: Guardar en var temporal o stack y Peek.
            // Pop a variable local.
            ctx.enter_scope();
            let match_var = ctx.define_variable(".match_val");
            pop(ctx, "$t0");
            ctx.emit(&format!("sw $t0, {}($fp)", match_var));
            
            let label_end = ctx.new_label("match_end");
            
            for case in cases {
                let label_next = ctx.new_label("case_next");
                // Verificar Patrón
                match &case.pattern {
                     Pattern::Literal(lit) => {
                         // Generar valor literal en pila
                         match lit {
                            Expr::Number(val) => {
                                // HULK usa f64, pero MIPS integer instructions para enteros simples
                                // Asumimos entero para 'case' por simplicidad o convertimos bits
                                let int_val = *val as i32;
                                ctx.emit(&format!("li $t1, {}", int_val));
                            },
                            Expr::Boolean(val) => {
                                let v = if *val { 1 } else { 0 };
                                ctx.emit(&format!("li $t1, {}", v));
                            },
                            Expr::String(s) => {
                                let label = ctx.add_string(s);
                                ctx.emit(&format!("la $t1, {}", label));
                            },
                            _ => {
                                 ctx.emit("li $t1, 0");
                            }
                         }

                         // Comparar con match_var (está en stack frame)
                         ctx.emit(&format!("lw $t0, {}($fp)", match_var));
                         // $t0 = variable, $t1 = literal
                         // bne $t0, $t1, label_next
                         ctx.emit(&format!("bne $t0, $t1, {}", label_next));
                     }
                     Pattern::Wildcard => {
                         // Siempre match, no generamos salto
                     }
                     _ => {
                         // Patrones complejos no soportados, saltamos
                         ctx.emit(&format!("b {}", label_next));
                     }
                }
                
                // Si match:
                gen_expr(ctx, &case.expr);
                ctx.emit(&format!("b {}", label_end));
                
                ctx.emit_label(&label_next);
            }
            
            if let Some(def) = default {
                gen_expr(ctx, def);
            } else {
                ctx.emit("li $t0, 0");
                push(ctx, "$t0");
            }
            
            ctx.emit_label(&label_end);
            ctx.exit_scope();
        }
        Expr::VectorGenerator { .. } => {
            ctx.emit("# Vector Gen no implementado");
            ctx.emit("li $t0, 0");
            push(ctx, "$t0");
        }
        Expr::Lambda { .. } => {
            ctx.emit("# Lambda no implementado");
            ctx.emit("li $t0, 0");
            push(ctx, "$t0");
        }
        Expr::Block(exprs) => {
            if exprs.is_empty() {
                ctx.emit("li $t0, 0");
                push(ctx, "$t0");
            } else {
                 for (i, e) in exprs.iter().enumerate() {
                     gen_expr(ctx, e);
                     if i < exprs.len() - 1 {
                         pop(ctx, "$t0");
                     }
                 }
            }
        }
        _ => {
            ctx.emit("# Expr Desconocida");
            ctx.emit("li $t0, 0");
            push(ctx, "$t0");
        }
    }
}

fn push(ctx: &mut MipsContext, reg: &str) {
    ctx.emit("addiu $sp, $sp, -4");
    ctx.emit(&format!("sw {}, 0($sp)", reg));
}

fn pop(ctx: &mut MipsContext, reg: &str) {
    ctx.emit(&format!("lw {}, 0($sp)", reg));
    ctx.emit("addiu $sp, $sp, 4");
}


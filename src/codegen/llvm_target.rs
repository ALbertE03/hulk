use crate::ast::nodes::*;
use crate::semantic::Context;
use std::collections::HashMap;

use super::context::Ctx;
use super::classes::{topo_sort_classes, emit_class};
use super::functions::{emit_function, emit_macro, emit_helper_functions};
use super::expressions::{gen_expr, infer_return_type_from_body};
use super::builtins::emit_vector_type;

/// Genera una función dispatch para un protocolo functor
fn emit_protocol_dispatch(ctx: &mut Ctx, proto_name: &str, implementations: &[(u32, String)]) {
    // solo soportamos protocolos con un método invoke que toma un argumento
    // Genera: double @{ProtoName}_dispatch(i8* obj, double arg) { switch... }
    
    ctx.functions.push_str(&format!(
        "define double @{}_dispatch(i8* %obj, double %arg) {{\nentry:\n",
        proto_name
    ));
    
    // Cargar type_id del objeto (primer campo del struct)
    let obj_ptr = ctx.tmp();
    ctx.functions.push_str(&format!("  {} = bitcast i8* %obj to i64*\n", obj_ptr));
    let type_id_val = ctx.tmp();
    ctx.functions.push_str(&format!("  {} = load i64, i64* {}\n", type_id_val, obj_ptr));
    let type_id = ctx.tmp();
    ctx.functions.push_str(&format!("  {} = trunc i64 {} to i32\n", type_id, type_id_val));
    
    // Switch basado en type_id
    ctx.functions.push_str(&format!("  switch i32 {}, label %default [\n", type_id));
    for (tid, class_name) in implementations {
        let label = format!("case_{}", class_name);
        ctx.functions.push_str(&format!("    i32 {}, label %{}\n", tid, label));
    }
    ctx.functions.push_str("  ]\n\n");
    
    // Casos para cada implementación
    for (_tid, class_name) in implementations {
        let label = format!("case_{}", class_name);
        let result = ctx.tmp();
        ctx.functions.push_str(&format!("{}:\n", label));
        ctx.functions.push_str(&format!("  {} = call double @{}_invoke(i8* %obj, double %arg)\n", result, class_name));
        ctx.functions.push_str(&format!("  ret double {}\n\n", result));
    }
    
    // Caso por defecto: error
    ctx.functions.push_str("default:\n");
    ctx.functions.push_str("  call void @abort()\n");
    ctx.functions.push_str("  unreachable\n");
    ctx.functions.push_str("}\n\n");
}

pub struct LlvmGenerator;

impl super::CodeGenerator for LlvmGenerator {
    fn generate(&self, program: &Program, context: &Context) -> String {
        let mut ctx = Ctx::new(context);

        // Primero, registrar todos los protocolos
        let mut protocols: HashMap<String, &ProtocolDecl> = HashMap::new();
        for decl in &program.declarations {
            if let Declaration::Protocol(pd) = decl {
                protocols.insert(pd.name.clone(), pd);
            }
        }

        let mut parent_map: HashMap<String, String> = HashMap::new();
        let mut decl_map: HashMap<String, &TypeDecl> = HashMap::new();
        for decl in &program.declarations {
            if let Declaration::Type(td) = decl {
                decl_map.insert(td.name.clone(), td);
                if let Some(ref pi) = td.parent {
                    parent_map.insert(td.name.clone(), pi.name.clone());
                }
            }
        }

        let ordered = topo_sort_classes(&decl_map, &parent_map);
        
        // Emitir tipo Vector implícito antes de las clases del usuario
        emit_vector_type(&mut ctx);
        
        for name in &ordered {
            if let Some(td) = decl_map.get(name.as_str()) {
                emit_class(&mut ctx, td, &gen_expr, &infer_return_type_from_body);
                
                // Detectar si este tipo implementa algún protocolo (tiene método invoke)
                for method in &td.methods {
                    if method.name == "invoke" {
                        // Este tipo es un functor, registrarlo para todos los protocolos con invoke
                        for (proto_name, proto) in &protocols {
                            if proto.methods.iter().any(|m| m.name == "invoke") {
                                if let Some(layout) = ctx.classes.get(name.as_str()) {
                                    ctx.protocol_implementations
                                        .entry(proto_name.clone())
                                        .or_insert_with(Vec::new)
                                        .push((layout.type_id, name.clone()));
                                }
                            }
                        }
                        break;
                    }
                }
            }
        }

        for decl in &program.declarations {
            match decl {
                Declaration::Function(fd) => emit_function(&mut ctx, fd, &gen_expr),
                Declaration::Macro(md) => emit_macro(&mut ctx, md, &gen_expr),
                _ => {}
            }
        }

        // Generar funciones dispatch para protocolos con invoke
        let protocol_impls = ctx.protocol_implementations.clone();
        for (proto_name, implementations) in &protocol_impls {
            if !implementations.is_empty() {
                emit_protocol_dispatch(&mut ctx, proto_name, implementations);
            }
        }

        emit_helper_functions(&mut ctx);

        ctx.functions.push_str("define i32 @main() {\nentry:\n");
        ctx.functions.push_str("  call double @__hulk_main()\n");
        ctx.functions.push_str("  ret i32 0\n}\n");

        format!("{}\n{}\n{}\n{}", ctx.preamble, ctx.globals, ctx.functions, ctx.lambda_defs)
    }
}

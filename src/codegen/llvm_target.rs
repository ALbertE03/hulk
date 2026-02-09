use crate::ast::nodes::*;
use crate::semantic::Context;
use crate::utils::Spanned;
use std::collections::HashMap;

use super::context::Ctx;
use super::classes::{topo_sort_classes, emit_class};
use super::functions::{emit_function, emit_macro, emit_helper_functions};
use super::expressions::{gen_expr, infer_return_type_from_body};

pub struct LlvmGenerator;

impl super::CodeGenerator for LlvmGenerator {
    fn generate(&self, program: &Program, context: &Context) -> String {
        let mut ctx = Ctx::new(context);

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
        for name in &ordered {
            if let Some(td) = decl_map.get(name.as_str()) {
                emit_class(&mut ctx, td, &gen_expr, &infer_return_type_from_body);
            }
        }

        for decl in &program.declarations {
            match decl {
                Declaration::Function(fd) => emit_function(&mut ctx, fd, &gen_expr),
                Declaration::Macro(md) => emit_macro(&mut ctx, md, &gen_expr),
                _ => {}
            }
        }

        emit_helper_functions(&mut ctx);

        ctx.functions.push_str("define i32 @main() {\nentry:\n");
        let _val = gen_expr(&mut ctx, &program.expr);
        ctx.functions.push_str("  ret i32 0\n}\n");

        format!("{}\n{}\n{}\n{}", ctx.preamble, ctx.globals, ctx.functions, ctx.lambda_defs)
    }
}

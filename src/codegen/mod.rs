pub mod llvm_target;
mod utils;
mod context;
mod classes;
mod functions;
mod expressions;
mod builtins;

#[cfg(test)]
mod tests;

use crate::ast::nodes::Program;
use crate::semantic::Context;

pub trait CodeGenerator {
    fn generate(&self, program: &Program, context: &Context) -> String;
}

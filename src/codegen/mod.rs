pub mod llvm_target;
mod utils;
mod context;
mod classes;
mod functions;
mod expressions;

#[cfg(test)]
mod tests;

use crate::ast::nodes::Program;
use crate::semantic::Context;

pub trait CodeGenerator {
    fn generate(&self, program: &Program, context: &Context) -> String;
}

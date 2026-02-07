pub mod llvm_target;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod extra_tests;

use crate::ast::nodes::Program;
use crate::semantic::Context;

pub trait CodeGenerator {
    fn generate(&self, program: &Program, context: &Context) -> String;
}

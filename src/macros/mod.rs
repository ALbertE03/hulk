mod utils;
mod visitors;
mod context;

#[cfg(test)]
mod tests;

use crate::ast::nodes::Program;
use context::MacroExpansionContext;

/// Expande macros en un programa
pub fn expand_macros(program: Program) -> Program {
    let mut ctx = MacroExpansionContext::new();
    ctx.expand_program(program)
}


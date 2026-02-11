use hulk_compiler::parser::Parser;
use hulk_compiler::macros::expand_macros;
use hulk_compiler::ast::optimize::optimize_program;
use hulk_compiler::ast::transform::transform_implicit_functors;
use hulk_compiler::codegen::{CodeGenerator, llvm_target::LlvmGenerator};
use std::io::{self, Read};
use std::env;

fn main() {
    // Leer input de stdin o argumentos
    let input = if env::args().len() > 1 {
        let arg = env::args().nth(1).unwrap();
        // Intentar leer como archivo, si falla usar como string literal
        std::fs::read_to_string(&arg).unwrap_or(arg)
    } else {
        // Si no, leer de stdin
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).expect("Failed to read from stdin");
        buffer
    };

    let mut parser = Parser::new(&input);
    match parser.parse_program() {

        Ok(program) => {
            let mut expanded = expand_macros(program);
            
            // Transformar implicit functors ANTES del semantic check
            let temp_ctx = hulk_compiler::semantic::Context::new();
            transform_implicit_functors(&mut expanded, &temp_ctx);
    
            match hulk_compiler::semantic::check_program(&expanded) {
                 Ok(context) => {
                     eprintln!("Semantic check passed!");
                     eprintln!("Defined types: {:?}", context.types.keys());
                     
                     // Optimizar
                     let optimized = optimize_program(expanded);
                     
        
                     let generator = LlvmGenerator;
                     let llvm_code = generator.generate(&optimized, &context);
                     
                    std::fs::write("output.ll", &llvm_code).expect("Unable to write file");
                 },
                 Err(errors) => {
                     eprintln!("Semantic errors found:");
                     for err in errors {
                         eprintln!("{:?}", err);
                     }
                     std::process::exit(1);
                 }
            }
        }
        Err(e) => {
            eprintln!("Parsing failed: {}", e);
            std::process::exit(1);
        }
    }
}

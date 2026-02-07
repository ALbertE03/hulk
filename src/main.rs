use hulk_compiler::parser::Parser;
use hulk_compiler::macros::expand_macros;
use hulk_compiler::ast::optimize::optimize_program;
use hulk_compiler::codegen::{CodeGenerator, mips_target::MipsGenerator};
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
            println!("Parsed successfully!\n");
            
            println!("=== DESPUÉS DE PARSING ===\n");
            println!("AST structure (using Display):\n");
            println!("{}", program);
            
            //println!("\nDebug Representation (Internal structure):\n");
            //println!("{:#?}", program);
            
            // Expandir macros
            let expanded = expand_macros(program);
            
            println!("\n=== DESPUÉS DE EXPANDIR MACROS ===\n");
            println!("AST structure (using Display):\n");
            println!("{}", expanded);
            
           // println!("\nDebug Representation (Internal structure):\n");
            //println!("{:#?}", expanded);
            
            // Análisis Semántico
            println!("\n=== ANÁLISIS SEMÁNTICO ===\n");
            match hulk_compiler::semantic::check_program(&expanded) {
                 Ok(context) => {
                     println!("Semantic check passed!");
                     println!("Defined types: {:?}", context.types.keys());
                     
                     // Optimizar
                     let optimized = optimize_program(expanded);
                     
                     println!("\n=== DESPUÉS DE OPTIMIZAR ===\n");
                     println!("{}", optimized);
                     
                     // Generación de Código
                     println!("\n=== GENERACIÓN DE CÓDIGO (MIPS) ===\n");
                     let generator = MipsGenerator;
                     let mips_code = generator.generate(&optimized, &context);
                     println!("{}", mips_code);
                     
                     // Opcional: Escribir a archivo
                     // std::fs::write("output.mips", mips_code).expect("Unable to write file");
                 },
                 Err(errors) => {
                     println!("Semantic errors found:");
                     for err in errors {
                         println!("{:?}", err);
                     }
                 }
            }
        }
        Err(e) => {
            eprintln!("Parsing failed: {}", e);
            std::process::exit(1);
        }
    }
}

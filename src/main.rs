use hulk_compiler::parser::Parser;
use hulk_compiler::macros::expand_macros;
use hulk_compiler::ast::optimize::optimize_program;
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
            
            // Optimizar
            let optimized = optimize_program(expanded);
            
            println!("\n=== DESPUÉS DE OPTIMIZAR ===\n");
            //println!("AST structure (using Display):\n");
            println!("{}", optimized);
            
            //println!("\nDebug Representation (Internal structure):\n");
            //println!("{:#?}", optimized);
        }
        Err(e) => {
            eprintln!("Parsing failed: {}", e);
            std::process::exit(1);
        }
    }
}

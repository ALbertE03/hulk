use hulk_compiler::parser::Parser;
use hulk_compiler::ast::optimize::optimize_program;
use std::io::{self, Read};
use std::env;

fn main() {
    // Leer input de stdin o argumentos
    let input = if env::args().len() > 1 {
        // Si hay argumentos, usar el primer argumento como código
        env::args().nth(1).unwrap()
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
            
            println!("=== ANTES DE OPTIMIZAR ===\n");
            println!("AST structure (using Display):\n");
            println!("{}", program);
            
            println!("\nDebug Representation (Internal structure):\n");
            println!("{:#?}", program);
            
            let optimized = optimize_program(program);
            
            println!("\n=== DESPUÉS DE OPTIMIZAR ===\n");
            println!("AST structure (using Display):\n");
            println!("{}", optimized);
            
            println!("\nDebug Representation (Internal structure):\n");
            println!("{:#?}", optimized);
        }
        Err(e) => {
            eprintln!("Parsing failed: {}", e);
            std::process::exit(1);
        }
    }
}

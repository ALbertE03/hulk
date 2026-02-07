use crate::parser::Parser;
use crate::semantic::check_program;
use super::mips_target::MipsGenerator;
use super::CodeGenerator;

fn generate_code(input: &str) -> String {
    let mut parser = Parser::new(input);
    let program = parser.parse_program().expect("Failed to parse");
    let context = check_program(&program).expect("Semantic check failed");
    let generator = MipsGenerator;
    generator.generate(&program, &context)
}

#[test]
fn test_generate_print_number() {
    let code = generate_code("print(42);");
    println!("{}", code);
    assert!(code.contains("li $v0, 1")); // Print integer syscall
    assert!(code.contains("li $t0, 42")); // Load immediate 42
    assert!(code.contains("syscall"));
}

#[test]
fn test_generate_print_string() {
    let code = generate_code("print(\"Hello\");");
    println!("{}", code);
    assert!(code.contains(".asciiz \"Hello\"")); // String data
    assert!(code.contains("li $v0, 4")); // Print string syscall
}

#[test]
fn test_generate_arithmetic() {
    let code = generate_code("print(2 + 3);");
    assert!(code.contains("add $t0, $t0, $t1")); // Addition instruction
}

#[test]
fn test_generate_nested_arithmetic() {
    let code = generate_code("print((2 + 3) * 4);");
    assert!(code.contains("add $t0, $t0, $t1")); 
    assert!(code.contains("mul $t0, $t0, $t1"));
}

#[test]
fn test_program_structure() {
    let code = generate_code("print(1);");
    assert!(code.contains(".text"));
    assert!(code.contains(".globl main"));
    assert!(code.contains("main:"));
    assert!(code.contains("li $v0, 10")); // Exit syscall
}

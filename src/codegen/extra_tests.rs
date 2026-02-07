use crate::ast::nodes::Program;
use crate::codegen::CodeGenerator;
use crate::codegen::mips_target::MipsGenerator;
use crate::semantic::Context;
use crate::parser::Parser;

fn compile(input: &str) -> String {
    let mut parser = Parser::new(input);
    let program = parser.parse_program().unwrap();
    let context = Context::new(); // In future, we should run semantic analysis
    let generator = MipsGenerator;
    generator.generate(&program, &context)
}

#[test]
fn test_let_binding() {
    let code = "let x = 42 in print(x);";
    let asm = compile(code);
    assert!(asm.contains("li $t0, 42"));
    assert!(asm.contains("sw $t0, -12($fp)")); // Variable storage
    assert!(asm.contains("lw $t0, -12($fp)")); // Variable usage
}

#[test]
fn test_if_else() {
    let code = "if (true) print(1) else print(0);";
    let asm = compile(code);
    assert!(asm.contains("beqz $t0, else_")); // Check it branches to an else label
    assert!(asm.contains("else_")); // Check the label exists
    // We expect print(1) code (li $v0, 1; syscall)
    assert!(asm.contains("li $v0, 1")); 
    assert!(asm.contains("syscall"));
}

#[test]
fn test_while_loop() {
    let code = "while (x < 10) x := x + 1;";
    let asm = compile(code);
    assert!(asm.contains("while_start_"));
    assert!(asm.contains("beqz $t0, while_end_"));
    assert!(asm.contains("b while_start_"));
}

#[test]
fn test_function_definition() {
    let code = "function add(a, b) => a + b; print(add(1, 2));";
    let asm = compile(code);
    assert!(asm.contains("add:")); // Function label
    assert!(asm.contains("lw $t0, 4($fp)")); // Param access (approximate)
    assert!(asm.contains("jal add")); // Function call
}

use crate::parser::Parser;
use crate::semantic::check_program;
use super::llvm_target::LlvmGenerator;
use super::CodeGenerator;

fn generate_code(input: &str) -> String {
    let mut parser = Parser::new(input);
    let program = parser.parse_program().expect("Failed to parse");
    let context = check_program(&program).expect("Semantic check failed");
    let generator = LlvmGenerator;
    generator.generate(&program, &context)
}

fn generate_code_no_sem(input: &str) -> String {
    let mut parser = Parser::new(input);
    let program = parser.parse_program().expect("Failed to parse");
    let context = crate::semantic::Context::new();
    let generator = LlvmGenerator;
    generator.generate(&program, &context)
}

// ── Structure tests ─────────────────────────────────────────────────────

#[test]
fn test_program_structure() {
    let code = generate_code("print(1);");
    assert!(code.contains("define i32 @main()"));
    assert!(code.contains("entry:"));
    assert!(code.contains("ret i32 0"));
}

#[test]
fn test_preamble_declarations() {
    let code = generate_code("print(1);");
    assert!(code.contains("declare i32 @printf(i8*, ...)"));
    assert!(code.contains("declare i32 @puts(i8*)"));
    assert!(code.contains("declare i8* @malloc(i64)"));
    assert!(code.contains("declare double @llvm.pow.f64(double, double)"));
    assert!(code.contains("declare double @llvm.sin.f64(double)"));
    assert!(code.contains("declare double @llvm.sqrt.f64(double)"));
}

// ── Print tests ─────────────────────────────────────────────────────────

#[test]
fn test_generate_print_number() {
    let code = generate_code("print(42);");
    println!("{}", code);
    assert!(code.contains("@printf"));
    assert!(code.contains("@.fmt_num"));
    assert!(code.contains("4.2e1")); // 42 as float literal
}

#[test]
fn test_generate_print_string() {
    let code = generate_code("print(\"Hello\");");
    println!("{}", code);
    assert!(code.contains("Hello\\00"));   // Null-terminated string global
    assert!(code.contains("@puts"));       // Uses puts for strings
}

#[test]
fn test_generate_print_boolean() {
    let code = generate_code("print(true);");
    println!("{}", code);
    assert!(code.contains("@.true_s"));
    assert!(code.contains("@.false_s"));
    assert!(code.contains("select"));
}

// ── Arithmetic tests ────────────────────────────────────────────────────

#[test]
fn test_generate_arithmetic_add() {
    let code = generate_code("print(2 + 3);");
    assert!(code.contains("fadd double"));
}

#[test]
fn test_generate_arithmetic_sub() {
    let code = generate_code("print(10 - 4);");
    assert!(code.contains("fsub double"));
}

#[test]
fn test_generate_arithmetic_mul() {
    let code = generate_code("print(3 * 5);");
    assert!(code.contains("fmul double"));
}

#[test]
fn test_generate_arithmetic_div() {
    let code = generate_code("print(10 / 2);");
    assert!(code.contains("fdiv double"));
}

#[test]
fn test_generate_arithmetic_mod() {
    let code = generate_code("print(10 % 3);");
    assert!(code.contains("frem double"));
}

#[test]
fn test_generate_arithmetic_pow() {
    let code = generate_code("print(2 ^ 3);");
    assert!(code.contains("@llvm.pow.f64"));
}

#[test]
fn test_generate_nested_arithmetic() {
    let code = generate_code("print((2 + 3) * 4);");
    assert!(code.contains("fadd double"));
    assert!(code.contains("fmul double"));
}

// ── Comparison tests ────────────────────────────────────────────────────

#[test]
fn test_generate_comparison_eq() {
    let code = generate_code("print(1 == 1);");
    assert!(code.contains("fcmp oeq"));
    assert!(code.contains("select i1"));
}

#[test]
fn test_generate_comparison_neq() {
    let code = generate_code("print(1 != 2);");
    assert!(code.contains("fcmp one"));
}

#[test]
fn test_generate_comparison_lt() {
    let code = generate_code("print(1 < 2);");
    assert!(code.contains("fcmp olt"));
}

#[test]
fn test_generate_comparison_gt() {
    let code = generate_code("print(2 > 1);");
    assert!(code.contains("fcmp ogt"));
}

// ── Unary tests ─────────────────────────────────────────────────────────

#[test]
fn test_generate_negation() {
    let code = generate_code("print(-5);");
    assert!(code.contains("fneg double"));
}

#[test]
fn test_generate_not() {
    let code = generate_code("print(!true);");
    assert!(code.contains("fcmp oeq double"));
    assert!(code.contains("select"));
}

// ── String concatenation tests ──────────────────────────────────────────

#[test]
fn test_generate_string_concat() {
    let code = generate_code_no_sem("print(\"Hello\" @ \"World\");");
    assert!(code.contains("@strlen"));
    assert!(code.contains("@malloc"));
    assert!(code.contains("@strcpy"));
    assert!(code.contains("@strcat"));
}

#[test]
fn test_generate_string_concat_space() {
    let code = generate_code_no_sem("print(\"Hello\" @@ \"World\");");
    assert!(code.contains("@.space_s"));  // Space inserted
    assert!(code.contains("@strcat"));
}

// ── Logical operators ───────────────────────────────────────────────────

#[test]
fn test_generate_and() {
    let code = generate_code("print(true & true);");
    assert!(code.contains("and i1"));
}

#[test]
fn test_generate_or() {
    let code = generate_code("print(true | false);");
    assert!(code.contains("or i1"));
}

// ── Math builtins ───────────────────────────────────────────────────────

#[test]
fn test_generate_sqrt() {
    let code = generate_code("print(sqrt(4));");
    assert!(code.contains("@llvm.sqrt.f64"));
}

#[test]
fn test_generate_sin() {
    let code = generate_code("print(sin(0));");
    assert!(code.contains("@llvm.sin.f64"));
}

#[test]
fn test_generate_cos() {
    let code = generate_code("print(cos(0));");
    assert!(code.contains("@llvm.cos.f64"));
}

#[test]
fn test_generate_exp() {
    let code = generate_code("print(exp(1));");
    assert!(code.contains("@llvm.exp.f64"));
}

#[test]
fn test_generate_log() {
    let code = generate_code("print(log(10, 100));");
    assert!(code.contains("@llvm.log.f64")); // called twice (ln of val and base)
    assert!(code.contains("fdiv double"));   // division
}

#[test]
fn test_generate_pi() {
    let code = generate_code("print(PI);");
    assert!(code.contains("3.141592653589793e0"));
}

#[test]
fn test_generate_e() {
    let code = generate_code("print(E);");
    assert!(code.contains("2.718281828459045e0"));
}
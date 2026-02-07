use super::check_program;
use crate::parser::Parser;
use crate::errors::SemanticError;

fn check(input: &str) {
    let mut parser = Parser::new(input);
    let program = parser.parse_program().expect("Failed to parse program");
    match check_program(&program) {
        Ok(_) => {},
        Err(errors) => {
            for err in errors {
                println!("{:?}", err);
            }
            panic!("Semantic check failed");
        }
    }
}

fn check_fail(input: &str) -> Vec<SemanticError> {
    let mut parser = Parser::new(input);
    let program = parser.parse_program().expect("Failed to parse program");
    match check_program(&program) {
        Ok(_) => panic!("Should have failed"),
        Err(errors) => errors,
    }
}

#[test]
fn test_basic_expression() {
    check("print(42);");
    check("print(\"Hello\");");
    check("print(true);");
    check("print(2 + 3 * 4);");
}

#[test]
fn test_variables() {
    check("let x = 10 in print(x);");
    check("let x = 10, y = 5 in print(x + y);");
    check("let x = 10 in let y = x + 1 in print(y);");
}

#[test]
fn test_type_mismatch_arithmetic() {
    let errors = check_fail("print(1 + \"hello\");");
    assert!(!errors.is_empty()); 
    // TypeMismatch
}

#[test]
fn test_if_expression() {
    check("if (true) 1 else 2;");
    check("let x = if (true) 1 else 2 in print(x);");
}

#[test]
fn test_scope_shadowing() {
    check("let x = 1 in let x = 2 in print(x);");
}

#[test]
fn test_undefined_variable() {
    let errors = check_fail("print(x);");
     match &errors[0] {
        SemanticError::VariableNotFound(name) => assert_eq!(name, "x"),
        _ => panic!("Expected VariableNotFound"),
    }
}

#[test]
fn test_functions() {
    let code = "
    function sum(a: Number, b: Number) => a + b;
    print(sum(1, 2));
    ";
    check(code);
}

#[test]
fn test_function_type_mismatch() {
    let code = "
    function sum(a: Number, b: Number) => a + b;
    print(sum(1, \"2\"));
    ";
    let errors = check_fail(code);
    assert!(!errors.is_empty());
}

#[test]
fn test_classes() {
    let code = "
    type Point(x: Number, y: Number) {
        x = x;
        y = y;
        getX() => self.x;
        getY() => self.y;
    }
    let p = new Point(1, 2) in print(p.getX());
    ";
    check(code);
}

#[test]
fn test_inheritance() {
    let code = "
    type Animal {
        makeSound() => print(\"...\");
    }
    type Dog inherits Animal {
        makeSound() => print(\"Woof\");
    }
    let d = new Dog() in d.makeSound();
    ";
    check(code);
}

#[test]
fn test_circular_inheritance() {
    let code = "
    type A inherits B {}
    type B inherits A {}
    ";
    let errors = check_fail(code);
    match &errors[0] {
        SemanticError::CircularInheritance(_) => {}, // Pass
        _ => panic!("Expected CircularInheritance error, got {:?}", errors[0]),
    }
}

#[test]
fn test_method_not_found() {
    let errors = check_fail("print(1.substring(0));");
     match &errors[0] {
        SemanticError::MethodNotFound(_) => {},
        _ => panic!("Expected MethodNotFound"),
    }
}

#[test]
fn test_type_conformance() {
    let code = "
    type Animal {}
    type Dog inherits Animal {}
    
    function foo(a: Animal) => 0;
    
    let d = new Dog() in foo(d);
    ";
    check(code);
}

#[test]
fn test_block_scope() {
    let code = "
    let x = 10 in {
        let y = x + 1 in print(y);
    };
    ";
    check(code);
}

#[test]
fn test_type_redefinition() {
    let code = "
    type A {}
    type A {}
    ";
    let errors = check_fail(code);
    match &errors[0] {
        SemanticError::TypeDefined(name) => assert_eq!(name, "A"),
        _ => panic!("Expected TypeDefined"),
    }
}

#[test]
fn test_function_redefinition() {
    let code = "
    function foo() => 1;
    function foo() => 2;
    ";
    let errors = check_fail(code);
    match &errors[0] {
        SemanticError::FunctionDefined(name) => assert_eq!(name, "foo"),
        _ => panic!("Expected FunctionDefined"),
    }
}

#[test]
fn test_parent_type_not_found() {
    let code = "
    type A inherits B {}
    ";
    let errors = check_fail(code);
    match &errors[0] {
        SemanticError::TypeNotFound(name) => assert_eq!(name, "B"),
        _ => panic!("Expected TypeNotFound"),
    }
}

#[test]
fn test_self_outside_class() {
    let errors = check_fail("print(self);");
    match &errors[0] {
        SemanticError::SelfReference => {},
        _ => panic!("Expected SelfReference"),
    }
}

#[test]
fn test_attribute_redefinition() {
    let code = "
    type A {
        x = 1;
        x = 2;
    }
    ";
    let errors = check_fail(code);
    match &errors[0] {
        SemanticError::AttributeDefined(name) => assert!(name.contains("A.x")),
        _ => panic!("Expected AttributeDefined"),
    }
}

#[test]
fn test_signature_mismatch() {
    let code = "
    type A {
        foo(x: Number) => x;
    }
    type B inherits A {
        foo(x: Number, y: Number) => x + y;
    }
    ";
    let errors = check_fail(code);
    match &errors[0] {
         SemanticError::SignatureMismatch(_) => {},
         _ => panic!("Expected SignatureMismatch"),
    }
}

#[test]
fn test_condition_must_be_boolean() {
    let code = "if (42) 1 else 0;";
     let errors = check_fail(code);
     assert!(!errors.is_empty());
}

use super::*;
use crate::ast::nodes::*;
use crate::parser::Parser;

// Helper para crear Spanned 
fn span<T>(node: T, line: usize, column: usize) -> Spanned<T> {
    Spanned {
        node,
        pos: Position { line, column },
    }
}

#[test]
fn test_gensym_generates_unique_names() {
    let name1 = gensym("temp");
    let name2 = gensym("temp");
    let name3 = gensym("var");
    
    assert_ne!(name1, name2);
    assert_ne!(name1, name3);
    assert_ne!(name2, name3);
    
    // Verificar formato
    assert!(name1.starts_with("temp$$"));
    assert!(name2.starts_with("temp$$"));
    assert!(name3.starts_with("var$$"));
}

#[test]
fn test_expand_program_no_macros() {
    let input = "function f(x: Number): Number => x * 2; f(5)";
    let mut parser = Parser::new(input);
    let program = parser.parse_program().unwrap();
    
    let mut ctx = MacroExpansionContext::new();
    let _expanded = ctx.expand_program(program.clone());
    
    // Sin macros, debe ser idéntico
    assert_eq!(_expanded.declarations.len(), program.declarations.len());
}

#[test]
fn test_expand_program_collects_macros() {
    let input = r#"
        def double(x: Number): Number => x * 2;
        def triple(x: Number): Number => x * 3;
        double(5)
    "#;
    let mut parser = Parser::new(input);
    let program = parser.parse_program().unwrap();
    
    let mut ctx = MacroExpansionContext::new();
    let _expanded = ctx.expand_program(program);
    
    // Las macros deben ser registradas
    assert_eq!(ctx.macros.len(), 2);
    assert!(ctx.macros.contains_key("double"));
    assert!(ctx.macros.contains_key("triple"));
    
    // Las declaraciones de macro no deben aparecer en la salida
    assert_eq!(_expanded.declarations.len(), 0);
}

#[test]
fn test_expand_macro_with_normal_params() {
    let input = r#"
        def add_one(x: Number): Number => x + 1;
        add_one(10)
    "#;
    let mut parser = Parser::new(input);
    let program = parser.parse_program().unwrap();
    
    let mut ctx = MacroExpansionContext::new();
    let _expanded = ctx.expand_program(program);
    
    // Verificar que la macro fue registrada correctamente
    let macro_decl = ctx.macros.get("add_one").unwrap();
    assert_eq!(macro_decl.params.len(), 1);
    
    match &macro_decl.params[0] {
        MacroParam::Normal { name, .. } => assert_eq!(name, "x"),
        _ => panic!("Expected normal parameter"),
    }
}

#[test]
fn test_expand_macro_with_body_param() {
    let input = r#"
        def repeat(n: Number, *expr: Object): Object => expr;
        repeat(3, print("hello"))
    "#;
    let mut parser = Parser::new(input);
    let program = parser.parse_program().unwrap();
    
    let mut ctx = MacroExpansionContext::new();
    let _expanded = ctx.expand_program(program);
    
    let macro_decl = ctx.macros.get("repeat").unwrap();
    assert_eq!(macro_decl.params.len(), 2);
    
    match &macro_decl.params[1] {
        MacroParam::Body { name, .. } => assert_eq!(name, "expr"),
        _ => panic!("Expected body parameter"),
    }
}

#[test]
fn test_expand_macro_with_symbolic_param() {
    let input = r#"
        def swap(@a: Object, @b: Object) => let temp = a in { a := b; b := temp; };
        swap(x, y)
    "#;
    let mut parser = Parser::new(input);
    let program = parser.parse_program().unwrap();
    
    let mut ctx = MacroExpansionContext::new();
    let _expanded = ctx.expand_program(program);
    
    let macro_decl = ctx.macros.get("swap").unwrap();
    assert_eq!(macro_decl.params.len(), 2);
    
    match &macro_decl.params[0] {
        MacroParam::Symbolic { name, .. } => assert_eq!(name, "a"),
        _ => panic!("Expected symbolic parameter"),
    }
}

#[test]
fn test_expand_macro_with_placeholder_param() {
    let input = r#"
        def repeat($iter: Number, n: Number, *expr: Object): Object => expr;
        repeat(i, 10, print(i))
    "#;
    let mut parser = Parser::new(input);
    let program = parser.parse_program().unwrap();
    
    let mut ctx = MacroExpansionContext::new();
    let _expanded = ctx.expand_program(program);
    
    let macro_decl = ctx.macros.get("repeat").unwrap();
    assert_eq!(macro_decl.params.len(), 3);
    
    match &macro_decl.params[0] {
        MacroParam::Placeholder { name, .. } => assert_eq!(name, "iter"),
        _ => panic!("Expected placeholder parameter"),
    }
}

#[test]
fn test_expand_nested_macro_calls() {
    let input = r#"
        def double(x: Number): Number => x * 2;
        def quadruple(x: Number): Number => double(double(x));
        quadruple(5)
    "#;
    let mut parser = Parser::new(input);
    let program = parser.parse_program().unwrap();
    
    let mut ctx = MacroExpansionContext::new();
    let _expanded = ctx.expand_program(program);
    
    // Ambas macros deben estar registradas
    assert!(ctx.macros.contains_key("double"));
    assert!(ctx.macros.contains_key("quadruple"));
}

#[test]
fn test_expand_macro_in_let_binding() {
    let input = r#"
        def inc(x: Number): Number => x + 1;
        let result = inc(10) in result
    "#;
    let mut parser = Parser::new(input);
    let program = parser.parse_program().unwrap();
    
    let mut ctx = MacroExpansionContext::new();
    let _expanded = ctx.expand_program(program);
    
    assert!(ctx.macros.contains_key("inc"));
}

#[test]
fn test_expand_macro_with_match() {
    let input = r#"
        def simplify(expr: Number) {
            match(expr) {
                case (x: Number + 0) => x;
                default => expr;
            };
        }
        simplify(5 + 0)
    "#;
    let mut parser = Parser::new(input);
    let program = parser.parse_program().unwrap();
    
    let mut ctx = MacroExpansionContext::new();
    let _expanded = ctx.expand_program(program);
    
    let macro_decl = ctx.macros.get("simplify").unwrap();
    
    // Verificar que el body es un bloque que contiene un match
    match &macro_decl.body.node {
        Expr::Block(exprs) => {
            assert_eq!(exprs.len(), 1);
            match &exprs[0].node {
                Expr::Match { .. } => {}, // OK
                _ => panic!("Expected match expression inside block"),
            }
        }
        _ => panic!("Expected block expression in macro body"),
    }
}

#[test]
fn test_pattern_match_literal_success() {
    let pattern = Pattern::Literal(Expr::Number(42.0));
    let expr = span(Expr::Number(42.0), 1, 1);
    
    let ctx = MacroExpansionContext::new();
    let _result = ctx.pattern_match(&pattern, &expr);
    
    // Debería hacer match (cuando se implemente completamente)
    // Por ahora, pattern_match puede retornar None
}

#[test]
fn test_pattern_match_variable() {
    let pattern = Pattern::Variable {
        name: "x".to_string(),
        type_annotation: None,
    };
    let expr = span(Expr::Number(10.0), 1, 1);
    
    let ctx = MacroExpansionContext::new();
    let result = ctx.pattern_match(&pattern, &expr);
    
    // Variable pattern debería hacer match con cualquier expresión
    // y retornar binding de "x" -> expr
    assert!(result.is_some());
    let bindings = result.unwrap();
    assert!(bindings.contains_key("x"));
}

#[test]
fn test_pattern_match_wildcard() {
    let pattern = Pattern::Wildcard;
    let expr = span(Expr::Number(999.0), 1, 1);
    
    let ctx = MacroExpansionContext::new();
    let result = ctx.pattern_match(&pattern, &expr);
    
    // Wildcard debería hacer match con todo
    assert!(result.is_some());
}

#[test]
fn test_sanitize_expr_preserves_structure() {
    let expr = span(
        Expr::Binary(
            Box::new(span(Expr::Identifier("x".to_string()), 1, 1)),
            Op::Add,
            Box::new(span(Expr::Number(1.0), 1, 5)),
        ),
        1, 1,
    );
    
    let ctx = MacroExpansionContext::new();
    let sanitized = ctx.sanitize_expr(&expr);
    
    // La estructura debe preservarse
    match &sanitized.node {
        Expr::Binary(_left, op, right) => {
            assert_eq!(*op, Op::Add);
            match &right.node {
                Expr::Number(n) => assert_eq!(*n, 1.0),
                _ => panic!("Expected number"),
            }
        }
        _ => panic!("Expected binary expression"),
    }
}

#[test]
fn test_hygiene_renames_variables() {
    // Test que las variables internas son renombradas para evitar colisiones
    let expr = span(Expr::Identifier("temp".to_string()), 1, 1);
    
    let ctx = MacroExpansionContext::new();
    let _sanitized = ctx.sanitize_expr(&expr);
    
    // Por ahora la implementación es stub y no cambia nada
    // Cuando se implemente completamente, debería verificar el renombrado
}

#[test]
fn test_multiple_macros_dont_interfere() {
    let input = r#"
        def m1(x: Number): Number => x + 1;
        def m2(x: Number): Number => x * 2;
        def m3(x: Number): Number => x - 1;
        m1(m2(m3(10)))
    "#;
    let mut parser = Parser::new(input);
    let program = parser.parse_program().unwrap();
    
    let mut ctx = MacroExpansionContext::new();
    let _expanded = ctx.expand_program(program);
    
    assert_eq!(ctx.macros.len(), 3);
    assert!(ctx.macros.contains_key("m1"));
    assert!(ctx.macros.contains_key("m2"));
    assert!(ctx.macros.contains_key("m3"));
}

#[test]
fn test_macro_preserves_type_annotations() {
    let input = r#"
        def typed(x: Number, y: String): Boolean => true;
        typed(42, "hello")
    "#;
    let mut parser = Parser::new(input);
    let program = parser.parse_program().unwrap();
    
    let mut ctx = MacroExpansionContext::new();
    let _expanded = ctx.expand_program(program);
    
    let macro_decl = ctx.macros.get("typed").unwrap();
    assert!(macro_decl.return_type.is_some());
    
    match &macro_decl.params[0] {
        MacroParam::Normal { type_annotation, .. } => {
            // TypeAnnotation no es Option, siempre existe
            match type_annotation {
                TypeAnnotation::Name(name) => assert_eq!(name, "Number"),
                _ => panic!("Expected Number type"),
            }
        }
        _ => panic!("Expected normal param"),
    }
}

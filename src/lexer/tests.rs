use super::*;
use crate::lexer::tokens::Token::*;

#[test]
fn test_keywords_and_identifiers() {
    let input = "let x = function";
    let mut lexer = Lexer::new(input);
    assert_eq!(lexer.next().unwrap().unwrap().0, Let);
    assert_eq!(lexer.next().unwrap().unwrap().0, Identifier("x".to_string()));
    assert_eq!(lexer.next().unwrap().unwrap().0, Assign);
    assert_eq!(lexer.next().unwrap().unwrap().0, Function);
    assert_eq!(lexer.next(), None);
}

#[test]
fn test_numbers() {
    let input = "123 45.67";
    let mut lexer = Lexer::new(input);
    assert_eq!(lexer.next().unwrap().unwrap().0, Number(123.0));
    assert_eq!(lexer.next().unwrap().unwrap().0, Number(45.67));
}

#[test]
fn test_strings() {
    let input = r#" "hello" "world" "#;
    let mut lexer = Lexer::new(input);
    assert_eq!(lexer.next().unwrap().unwrap().0, StringLiteral("hello".to_string()));
    assert_eq!(lexer.next().unwrap().unwrap().0, StringLiteral("world".to_string()));
}

#[test]
fn test_operators() {
    let input = "+ - * / % ^";
    let mut lexer = Lexer::new(input);
    assert_eq!(lexer.next().unwrap().unwrap().0, Plus);
    assert_eq!(lexer.next().unwrap().unwrap().0, Minus);
    assert_eq!(lexer.next().unwrap().unwrap().0, Star);
    assert_eq!(lexer.next().unwrap().unwrap().0, Slash);
    assert_eq!(lexer.next().unwrap().unwrap().0, Percent);
    assert_eq!(lexer.next().unwrap().unwrap().0, Power);
}

#[test]
fn test_compound_operators() {
    let input = "== != <= >= => := @@ @";
    let mut lexer = Lexer::new(input);
    assert_eq!(lexer.next().unwrap().unwrap().0, Equal);
    assert_eq!(lexer.next().unwrap().unwrap().0, NotEqual);
    assert_eq!(lexer.next().unwrap().unwrap().0, LessThanEq);
    assert_eq!(lexer.next().unwrap().unwrap().0, GreaterThanEq);
    assert_eq!(lexer.next().unwrap().unwrap().0, Arrow); 
    assert_eq!(lexer.next().unwrap().unwrap().0, DestructAssign);
    assert_eq!(lexer.next().unwrap().unwrap().0, ConcatSpace);
    assert_eq!(lexer.next().unwrap().unwrap().0, Concat);
}

#[test]
fn test_comments() {
    let input = "let // comment \n x /* block */ = 1";
    let mut lexer = Lexer::new(input);
    assert_eq!(lexer.next().unwrap().unwrap().0, Let);
    assert_eq!(lexer.next().unwrap().unwrap().0, Identifier("x".to_string()));
    assert_eq!(lexer.next().unwrap().unwrap().0, Assign);
    assert_eq!(lexer.next().unwrap().unwrap().0, Number(1.0));
}



#[test]
fn test_edge_cases() {
    let input = "42.5 42.foo";
    let mut lexer = Lexer::new(input);
    
    assert_eq!(lexer.next().unwrap().unwrap().0, Number(42.5));
    assert_eq!(lexer.next().unwrap().unwrap().0, Number(42.0)); 
    assert_eq!(lexer.next().unwrap().unwrap().0, Dot);          
    assert_eq!(lexer.next().unwrap().unwrap().0, Identifier("foo".to_string()));
}

#[test]
fn test_ackermann_function() {
    let input = "function ackermann(m: Number, n: Number) => 
        if (m == 0) n + 1 
        else if (n == 0) ackermann(m - 1, 1) 
        else ackermann(m - 1, ackermann(m, n - 1));";
    
    let tokens: Vec<Token> = Lexer::new(input).map(|r| r.unwrap().0).collect();
    let expected = vec![
        Function, Identifier("ackermann".to_string()), LParen, Identifier("m".to_string()), Colon, Identifier("Number".to_string()), Comma, Identifier("n".to_string()), Colon, Identifier("Number".to_string()), RParen, Arrow,
        If, LParen, Identifier("m".to_string()), Equal, Number(0.0), RParen, Identifier("n".to_string()), Plus, Number(1.0),
        Else, If, LParen, Identifier("n".to_string()), Equal, Number(0.0), RParen, Identifier("ackermann".to_string()), LParen, Identifier("m".to_string()), Minus, Number(1.0), Comma, Number(1.0), RParen,
        Else, Identifier("ackermann".to_string()), LParen, Identifier("m".to_string()), Minus, Number(1.0), Comma, Identifier("ackermann".to_string()), LParen, Identifier("m".to_string()), Comma, Identifier("n".to_string()), Minus, Number(1.0), RParen, RParen, Semicolon
    ];
    assert_eq!(tokens, expected);
}

#[test]
fn test_type_definition() {
    let input = "type Point { 
    x = 0; 
    y = 0; 
    function distance(other: Point) => (self.x - other.x)^2 + (self.y - other.y)^2; 
}";
    let tokens: Vec<Token> = Lexer::new(input).map(|r| r.unwrap().0).collect();
    let expected = vec![
        Type, Identifier("Point".to_string()), LBrace,
        Identifier("x".to_string()), Assign, Number(0.0), Semicolon,
        Identifier("y".to_string()), Assign, Number(0.0), Semicolon,
        Function, Identifier("distance".to_string()), LParen, Identifier("other".to_string()), Colon, Identifier("Point".to_string()), RParen, Arrow,
        LParen, Identifier("self".to_string()), Dot, Identifier("x".to_string()), Minus, Identifier("other".to_string()), Dot, Identifier("x".to_string()), RParen, Power, Number(2.0),
        Plus,
        LParen, Identifier("self".to_string()), Dot, Identifier("y".to_string()), Minus, Identifier("other".to_string()), Dot, Identifier("y".to_string()), RParen, Power, Number(2.0), Semicolon,
        RBrace
    ];
    assert_eq!(tokens, expected);
}

#[test]
fn test_loops_and_arrays() {
    let input = "let primes = [2, 3, 5, 7] in for p in primes print(p);";
    let tokens: Vec<Token> = Lexer::new(input).map(|r| r.unwrap().0).collect();
    let expected = vec![
        Let, Identifier("primes".to_string()), Assign, LBracket, Number(2.0), Comma, Number(3.0), Comma, Number(5.0), Comma, Number(7.0), RBracket,
        In, For, Identifier("p".to_string()), In, Identifier("primes".to_string()),
        Print, LParen, Identifier("p".to_string()), RParen, Semicolon
    ];
    assert_eq!(tokens, expected);
}

#[test]
fn test_while_loop() {
    let input = "let a = 10 in while (a > 0) { print(a); a := a - 1; }";
    let tokens: Vec<Token> = Lexer::new(input).map(|r| r.unwrap().0).collect();
    let expected = vec![
        Let, Identifier("a".to_string()), Assign, Number(10.0), In,
        While, LParen, Identifier("a".to_string()), GreaterThan, Number(0.0), RParen, LBrace,
        Print, LParen, Identifier("a".to_string()), RParen, Semicolon,
        Identifier("a".to_string()), DestructAssign, Identifier("a".to_string()), Minus, Number(1.0), Semicolon,
        RBrace
    ];
    assert_eq!(tokens, expected);
}

#[test]
fn test_protocol_definition() {
    let input = "protocol Hashable { hash(): Number; }";
    let tokens: Vec<Token> = Lexer::new(input).map(|r| r.unwrap().0).collect();
    let expected = vec![
        Protocol, Identifier("Hashable".to_string()), LBrace,
        Identifier("hash".to_string()), LParen, RParen, Colon, Identifier("Number".to_string()), Semicolon,
        RBrace
    ];
    assert_eq!(tokens, expected);
}

#[test]
fn test_string_interpolation_complex() {
    let input = "print(\"Value: \" @ 42 @@ \" is the answer\");";
    let tokens: Vec<Token> = Lexer::new(input).map(|r| r.unwrap().0).collect();
    let expected = vec![
        Print, LParen, StringLiteral("Value: ".to_string()), Concat, Number(42.0), ConcatSpace, StringLiteral(" is the answer".to_string()), RParen, Semicolon
    ];
    assert_eq!(tokens, expected);
}

#[test]
fn test_positions() {
    let input = "let x =\n42";
    let mut lexer = Lexer::new(input);
    
    // let (1:1)
    let (t1, p1) = lexer.next().unwrap().unwrap();
    assert_eq!(t1, Let);
    assert_eq!(p1.line, 1);
    assert_eq!(p1.column, 1);
    
    // x (1:5)
    let (t2, p2) = lexer.next().unwrap().unwrap();
    assert_eq!(t2, Identifier("x".to_string()));
    assert_eq!(p2.line, 1);
    assert_eq!(p2.column, 5);
    
    // = (1:7)
    let (t3, p3) = lexer.next().unwrap().unwrap();
    assert_eq!(t3, Assign);
    assert_eq!(p3.line, 1);
    assert_eq!(p3.column, 7);
    
    // 42 (2:1)
    let (t4, p4) = lexer.next().unwrap().unwrap();
    assert_eq!(t4, Number(42.0));
    assert_eq!(p4.line, 2);
    assert_eq!(p4.column, 1);
}

#[test]
fn test_errors() {
    use crate::lexer::errors::LexError;
    
    // Unterminated string
    let input = "\"hello";
    let mut lexer = Lexer::new(input);
    assert!(matches!(lexer.next(), Some(Err(LexError::UnterminatedString(_)))));
    
    // Unterminated block comment
    let input = "/* test";
    let mut lexer = Lexer::new(input);
    assert!(matches!(lexer.next(), Some(Err(LexError::UnterminatedBlockComment(_)))));
    
    // Unexpected character
    let input = "#";
    let mut lexer = Lexer::new(input);
    assert!(matches!(lexer.next(), Some(Err(LexError::UnexpectedCharacter('#', _)))));
}


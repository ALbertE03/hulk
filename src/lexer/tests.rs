use super::*;
use crate::lexer::tokens::Token::*;

#[test]
fn test_keywords_and_identifiers() {
    let input = "let x = function";
    let mut lexer = Lexer::new(input);
    assert_eq!(lexer.next(), Some(Let));
    assert_eq!(lexer.next(), Some(Identifier("x".to_string())));
    assert_eq!(lexer.next(), Some(Assign));
    assert_eq!(lexer.next(), Some(Function));
    assert_eq!(lexer.next(), None);
    let tokens: Vec<Token> = Lexer::new(input).collect();
    
    println!("Generated Tokens:\n{:#?}", tokens);
    println!("input:\n{:#?}",input)

}

#[test]
fn test_numbers() {
    let input = "123 45.67";
    let mut lexer = Lexer::new(input);
    assert_eq!(lexer.next(), Some(Number(123.0)));
    assert_eq!(lexer.next(), Some(Number(45.67)));
    let tokens: Vec<Token> = Lexer::new(input).collect();
    
    println!("Generated Tokens:\n{:#?}", tokens);
    println!("input:\n{:#?}",input)

}

#[test]
fn test_strings() {
    let input = r#" "hello" "world" "#;
    let mut lexer = Lexer::new(input);
    assert_eq!(lexer.next(), Some(StringLiteral("hello".to_string())));
    assert_eq!(lexer.next(), Some(StringLiteral("world".to_string())));
    let tokens: Vec<Token> = Lexer::new(input).collect();
    
    println!("Generated Tokens:\n{:#?}", tokens);
    println!("input:\n{:#?}",input)

}

#[test]
fn test_operators() {
    let input = "+ - * / % ^";
    let mut lexer = Lexer::new(input);
    assert_eq!(lexer.next(), Some(Plus));
    assert_eq!(lexer.next(), Some(Minus));
    assert_eq!(lexer.next(), Some(Star));
    assert_eq!(lexer.next(), Some(Slash));
    assert_eq!(lexer.next(), Some(Percent));
    assert_eq!(lexer.next(), Some(Power));
    let tokens: Vec<Token> = Lexer::new(input).collect();
    
    println!("Generated Tokens:\n{:#?}", tokens);
    println!("input:\n{:#?}",input)

}

#[test]
fn test_compound_operators() {
    let input = "== != <= >= => := @@ @";
    let mut lexer = Lexer::new(input);
    assert_eq!(lexer.next(), Some(Equal));
    assert_eq!(lexer.next(), Some(NotEqual));
    assert_eq!(lexer.next(), Some(LessThanEq));
    assert_eq!(lexer.next(), Some(GreaterThanEq));
    assert_eq!(lexer.next(), Some(Arrow)); 
    assert_eq!(lexer.next(), Some(DestructAssign));
    assert_eq!(lexer.next(), Some(ConcatSpace));
    assert_eq!(lexer.next(), Some(Concat));
    let tokens: Vec<Token> = Lexer::new(input).collect();
    
    println!("Generated Tokens:\n{:#?}", tokens);
    println!("input:\n{:#?}",input)

}

#[test]
fn test_comments() {
    let input = "let // comment \n x /* block */ = 1";
    let mut lexer = Lexer::new(input);
    assert_eq!(lexer.next(), Some(Let));
    assert_eq!(lexer.next(), Some(Identifier("x".to_string())));
    assert_eq!(lexer.next(), Some(Assign));
    assert_eq!(lexer.next(), Some(Number(1.0)));
    let tokens: Vec<Token> = Lexer::new(input).collect();
    
    println!("Generated Tokens:\n{:#?}", tokens);
    println!("input:\n{:#?}",input)

}



#[test]
fn test_edge_cases() {
    // float vs method call
    let input = "42.5 42.foo";
    let mut lexer = Lexer::new(input);
    
    assert_eq!(lexer.next(), Some(Number(42.5)));
    assert_eq!(lexer.next(), Some(Number(42.0))); 
    assert_eq!(lexer.next(), Some(Dot));          
    assert_eq!(lexer.next(), Some(Identifier("foo".to_string())));
    let tokens: Vec<Token> = Lexer::new(input).collect();
    
    println!("Generated Tokens:\n{:#?}", tokens);
    println!("input:\n{:#?}",input)

}

#[test]
fn test_ackermann_function() {
    let input = "function ackermann(m: Number, n: Number) => 
        if (m == 0) n + 1 
        else if (n == 0) ackermann(m - 1, 1) 
        else ackermann(m - 1, ackermann(m, n - 1));";
    
    let tokens: Vec<Token> = Lexer::new(input).collect();
    println!("Ackermann input:\n{}", input);
    println!("Generated Tokens:\n{:#?}", tokens);

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
    let tokens: Vec<Token> = Lexer::new(input).collect();
    println!("Type Def input:\n{}", input);
    println!("Generated Tokens:\n{:#?}", tokens);

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
    let tokens: Vec<Token> = Lexer::new(input).collect();
    println!("Loops input:\n{}", input);
    println!("Generated Tokens:\n{:#?}", tokens);

    let expected = vec![
        Let, Identifier("primes".to_string()), Assign, LBracket, Number(2.0), Comma, Number(3.0), Comma, Number(5.0), Comma, Number(7.0), RBracket,
        In, For, Identifier("p".to_string()), In, Identifier("primes".to_string()),
        Print, LParen, Identifier("p".to_string()), RParen, Semicolon
    ];
    assert_eq!(tokens, expected);
}


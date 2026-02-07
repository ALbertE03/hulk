use super::*;
use crate::lexer::tokens::Token::*;
use crate::errors::LexError;

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
fn test_hulk_identifier_rules() {
    // Valid identifiers
    let valid = vec!["x", "x0", "x_0", "lowercase", "TitleCase", "snake_case", "camelCase"];
    for input in valid {
        let tokens: Vec<_> = Lexer::new(input).collect();
        assert_eq!(tokens.len(), 1, "Failed for {}", input);
        assert!(matches!(tokens[0].as_ref().unwrap().0, Token::Identifier(_)));
    }
    
    // _x -> Error(_) + Id(x)
    let input = "_x";
    let tokens: Vec<_> = Lexer::new(input).collect();
    assert!(matches!(tokens[0], Err(crate::errors::LexError::UnexpectedCharacter('_', _))));
    assert!(matches!(tokens[1].as_ref().unwrap().0, Token::Identifier(_)));

    // 8ball -> Num(8) + Id(ball)
    let input = "8ball";
    let tokens: Vec<_> = Lexer::new(input).collect();
    assert!(matches!(tokens[0].as_ref().unwrap().0, Token::Number(_)));
    assert!(matches!(tokens[1].as_ref().unwrap().0, Token::Identifier(_)));
}

#[test]
fn test_large_program() {
    let input = r#"
        function fib(n) => if (n <= 1) n else fib(n-1) + fib(n-2);
        
        type Person(name, age) {
            name = name;
            age = age;
            hello() => print("Hello, I am " @ self.name @@ "and I have " @ self.age @@ "years old");
        }

        let x = 10, y = 20 in {
            while (x > 0) {
                print(x);
                x := x - 1;
            };
            if (y % 2 == 0) {
                print("Even");
            } else {
                print("Odd");
            };
        }
    "#;
    let tokens_res: Vec<_> = Lexer::new(input).collect();
    let tokens: Vec<Token> = tokens_res.into_iter().map(|r| r.unwrap().0).collect();
  
    let expected = vec![
        Function, Identifier("fib".to_string()), LParen, Identifier("n".to_string()), RParen, Arrow,
        If, LParen, Identifier("n".to_string()), LessThanEq, Number(1.0), RParen, Identifier("n".to_string()),
        Else, Identifier("fib".to_string()), LParen, Identifier("n".to_string()), Minus, Number(1.0), RParen,
        Plus, Identifier("fib".to_string()), LParen, Identifier("n".to_string()), Minus, Number(2.0), RParen, Semicolon,

        Type, Identifier("Person".to_string()), LParen, Identifier("name".to_string()), Comma, Identifier("age".to_string()), RParen, LBrace,
        Identifier("name".to_string()), Assign, Identifier("name".to_string()), Semicolon,
        Identifier("age".to_string()), Assign, Identifier("age".to_string()), Semicolon,
        Identifier("hello".to_string()), LParen, RParen, Arrow, Print, LParen,
        StringLiteral("Hello, I am ".to_string()), Concat, Identifier("self".to_string()), Dot, Identifier("name".to_string()),
        ConcatSpace, StringLiteral("and I have ".to_string()), Concat, Identifier("self".to_string()), Dot, Identifier("age".to_string()),
        ConcatSpace, StringLiteral("years old".to_string()), RParen, Semicolon,
        RBrace,

        Let, Identifier("x".to_string()), Assign, Number(10.0), Comma, Identifier("y".to_string()), Assign, Number(20.0), In, LBrace,
        While, LParen, Identifier("x".to_string()), GreaterThan, Number(0.0), RParen, LBrace,
        Print, LParen, Identifier("x".to_string()), RParen, Semicolon,
        Identifier("x".to_string()), DestructAssign, Identifier("x".to_string()), Minus, Number(1.0), Semicolon,
        RBrace, Semicolon,
        If, LParen, Identifier("y".to_string()), Percent, Number(2.0), Equal, Number(0.0), RParen, LBrace,
        Print, LParen, StringLiteral("Even".to_string()), RParen, Semicolon,
        RBrace, Else, LBrace,
        Print, LParen, StringLiteral("Odd".to_string()), RParen, Semicolon,
        RBrace, Semicolon,
        RBrace
    ];

    assert_eq!(tokens, expected);
}

#[test]
fn test_multiple_errors() {
    // Separate errors that don't consume each other
    let input = "# $ \"hello";
    let tokens: Vec<_> = Lexer::new(input).collect();

    // 1. Unexpected char '#' at 1:1
    assert!(matches!(tokens[0], Err(LexError::UnexpectedCharacter('#', p)) if p.line == 1 && p.column == 1));
    
    // 2. Unexpected char '$' at 1:3
    assert!(matches!(tokens[1], Err(LexError::UnexpectedCharacter('$', p)) if p.line == 1 && p.column == 3));

    // 3. Unterminated string starts at 1:5
    assert!(matches!(tokens[2], Err(LexError::UnterminatedString(p)) if p.line == 1 && p.column == 5));
}

#[test]
fn test_complex_expressions() {
    let input = "let result = (a + b) * (c / d) ^ 2 % 3 in print(result);";
    let tokens_res: Vec<_> = Lexer::new(input).collect();
    let tokens: Vec<Token> = tokens_res.into_iter().map(|r| r.unwrap().0).collect();

    let expected = vec![
        Token::Let, Token::Identifier("result".to_string()), Token::Assign,
        Token::LParen, Token::Identifier("a".to_string()), Token::Plus, Token::Identifier("b".to_string()), Token::RParen,
        Token::Star,
        Token::LParen, Token::Identifier("c".to_string()), Token::Slash, Token::Identifier("d".to_string()), Token::RParen,
        Token::Power, Token::Number(2.0),
        Token::Percent, Token::Number(3.0),
        Token::In, Token::Print, Token::LParen, Token::Identifier("result".to_string()), Token::RParen, Token::Semicolon
    ];
    assert_eq!(tokens, expected);
}

#[test]
fn test_intensive_strings() {
    let input = r#"print("Mixed " @ "concatenation" @@ "with " @ 123 @@ "and " @ true);"#;
    let tokens_res: Vec<_> = Lexer::new(input).collect();
    let tokens: Vec<Token> = tokens_res.into_iter().map(|r| r.unwrap().0).collect();
    let expected = vec![
        Token::Print, Token::LParen,
        Token::StringLiteral("Mixed ".to_string()), Token::Concat, Token::StringLiteral("concatenation".to_string()),
        Token::ConcatSpace,
        Token::StringLiteral("with ".to_string()), Token::Concat, Token::Number(123.0),
        Token::ConcatSpace,
        Token::StringLiteral("and ".to_string()), Token::Concat, Token::True,
        Token::RParen, Token::Semicolon
    ];
    assert_eq!(tokens, expected);
}

#[test]
fn test_function_block() {
    let input = "function add(a, b) => { let result = a + b; result; }";
    let tokens_res: Vec<_> = Lexer::new(input).collect();
    let tokens: Vec<Token> = tokens_res.into_iter().map(|r| r.unwrap().0).collect();
    
    let expected = vec![
        Function, Identifier("add".to_string()), LParen, Identifier("a".to_string()), Comma, Identifier("b".to_string()), RParen, Arrow,
        LBrace, Let, Identifier("result".to_string()), Assign, Identifier("a".to_string()), Plus, Identifier("b".to_string()), Semicolon,
        Identifier("result".to_string()), Semicolon, RBrace
    ];
    assert_eq!(tokens, expected);
}

#[test]
fn test_nested_calls_and_access() {
    let input = "math.sin(math.pi * person.age().value);";
    let tokens_res: Vec<_> = Lexer::new(input).collect();
    let tokens: Vec<Token> = tokens_res.into_iter().map(|r| r.unwrap().0).collect();
    let expected = vec![
        Token::Identifier("math".to_string()), Token::Dot, Token::Identifier("sin".to_string()), Token::LParen,
        Token::Identifier("math".to_string()), Token::Dot, Token::Identifier("pi".to_string()), Token::Star,
        Token::Identifier("person".to_string()), Token::Dot, Token::Identifier("age".to_string()), Token::LParen, Token::RParen, Token::Dot, Token::Identifier("value".to_string()),
        Token::RParen, Token::Semicolon
    ];
    assert_eq!(tokens, expected);
}

#[test]
fn test_errors() {
    
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

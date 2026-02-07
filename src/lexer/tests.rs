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
    assert_eq!(lexer.next().unwrap().unwrap().0, FuncArrow); 
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
        Function, Identifier("ackermann".to_string()), LParen, Identifier("m".to_string()), Colon, Identifier("Number".to_string()), Comma, Identifier("n".to_string()), Colon, Identifier("Number".to_string()), RParen, FuncArrow,
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
        Function, Identifier("distance".to_string()), LParen, Identifier("other".to_string()), Colon, Identifier("Point".to_string()), RParen, FuncArrow,
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
        Function, Identifier("fib".to_string()), LParen, Identifier("n".to_string()), RParen, FuncArrow,
        If, LParen, Identifier("n".to_string()), LessThanEq, Number(1.0), RParen, Identifier("n".to_string()),
        Else, Identifier("fib".to_string()), LParen, Identifier("n".to_string()), Minus, Number(1.0), RParen,
        Plus, Identifier("fib".to_string()), LParen, Identifier("n".to_string()), Minus, Number(2.0), RParen, Semicolon,

        Type, Identifier("Person".to_string()), LParen, Identifier("name".to_string()), Comma, Identifier("age".to_string()), RParen, LBrace,
        Identifier("name".to_string()), Assign, Identifier("name".to_string()), Semicolon,
        Identifier("age".to_string()), Assign, Identifier("age".to_string()), Semicolon,
        Identifier("hello".to_string()), LParen, RParen, FuncArrow, Print, LParen,
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
    let input = "# ~ \"hello";  // ~ es inválido, $ ahora es válido para macros
    let tokens: Vec<_> = Lexer::new(input).collect();

    // 1. Unexpected char '#' at 1:1
    assert!(matches!(tokens[0], Err(LexError::UnexpectedCharacter('#', p)) if p.line == 1 && p.column == 1));
    
    // 2. Unexpected char '~' at 1:3
    assert!(matches!(tokens[1], Err(LexError::UnexpectedCharacter('~', p)) if p.line == 1 && p.column == 3));

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
        Function, Identifier("add".to_string()), LParen, Identifier("a".to_string()), Comma, Identifier("b".to_string()), RParen, FuncArrow,
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

#[test]
fn test_new_hulk_features() {
    let input = "elif extends is as base -> => Number*";
    let tokens: Vec<Token> = Lexer::new(input).map(|r| r.unwrap().0).collect();
    let expected = vec![
        Elif, Extends, Is, As, Base, TypeArrow, FuncArrow, Identifier("Number".to_string()), Star
    ];
    assert_eq!(tokens, expected);
}

#[test]
fn test_complex_lambda_and_types() {
    let input = "function count_when(numbers: Number*, filter: (Number) -> Boolean) => 0;
                 let numbers = range(0, 100) in print(count_when(numbers, (x: Number): Boolean => x % 2 == 0));";
    
    let tokens: Vec<Token> = Lexer::new(input).map(|r| r.unwrap().0).collect();
    // Just verify key tokens are present to ensure correct arrow distinction
    assert!(tokens.contains(&TypeArrow));
    assert!(tokens.contains(&FuncArrow));
    assert!(tokens.contains(&Base).is_falsy()); // wait, no base here
}

#[test]
fn test_nominal_typing_inheritance() {
    let input = "type B inherits A { } protocol P extends Q { }";
    let tokens: Vec<Token> = Lexer::new(input).map(|r| r.unwrap().0).collect();
    let expected = vec![
        Type, Identifier("B".to_string()), Inherits, Identifier("A".to_string()), LBrace, RBrace,
        Protocol, Identifier("P".to_string()), Extends, Identifier("Q".to_string()), LBrace, RBrace
    ];
    assert_eq!(tokens, expected);
}

#[test]
fn test_base_call_lexing() {
    let input = "base() + base.method()";
    let tokens: Vec<Token> = Lexer::new(input).map(|r| r.unwrap().0).collect();
    let expected = vec![
        Base, LParen, RParen, Plus, Base, Dot, Identifier("method".to_string()), LParen, RParen
    ];
    assert_eq!(tokens, expected);
}

#[test]
fn test_hulk_latest_spec_snippets() {
    // 8.1 Multiple let variables
    let input = "let number = 42, text = \"The meaning of life is\" in print(text @ number);";
    let tokens: Vec<Token> = Lexer::new(input).map(|r| r.unwrap().0).collect();
    assert!(tokens.contains(&Let));
    assert!(tokens.contains(&Comma));
    assert!(tokens.contains(&In));

    // 8.6 Destructive assignment
    let input = "a := 1;";
    let tokens: Vec<Token> = Lexer::new(input).map(|r| r.unwrap().0).collect();
    assert_eq!(tokens, vec![Identifier("a".to_string()), DestructAssign, Number(1.0), Semicolon]);

    // 9.2 Elif
    let input = "if (mod == 0) \"Magic\" elif (mod % 3 == 1) \"Woke\" else \"Dumb\"";
    let tokens: Vec<Token> = Lexer::new(input).map(|r| r.unwrap().0).collect();
    assert!(tokens.contains(&If));
    assert!(tokens.contains(&Elif));
    assert!(tokens.contains(&Else));

    // Power aliases
    let input = "x ^ 2 + y ** 2";
    let tokens: Vec<Token> = Lexer::new(input).map(|r| r.unwrap().0).collect();
    assert_eq!(tokens, vec![
        Identifier("x".to_string()), Power, Number(2.0), 
        Plus, 
        Identifier("y".to_string()), Power, Number(2.0)
    ]);
}

#[test]
fn test_exhaustive_hulk_constructs() {
    let input = r#"
        protocol P extends Q { 
            method(a: Number, b: String): Boolean; 
        }
        type T(a, b) inherits S(a + b) {
            attr: Number* = [x + 1 | x in b];
            func(x: (Number) -> Boolean): Boolean => x(self.attr[0]);
        }
        function main() => let x = new T(1, [2, 3]) in {
            if (x is T) {
                print(x as T);
            } elif (false) {
                base();
            } else {
                while (true) {
                    for (i in x.attr) print(i);
                    x.attr[0] := x.attr[0] * 2 ** 3;
                }
            };
            (x: Number): Number => x + 1;
            !true & false | (1 <= 2) != (3 >= 4) < 5 > 6;
            "escaped \" \n \t \\" @ "concat" @@ 42;
        }
    "#;
    
    let tokens_res: Vec<_> = Lexer::new(input).collect();
    for res in &tokens_res {
        if let Err(e) = res {
            panic!("Lexing error in exhaustive test: {:?}", e);
        }
    }
    
    let tokens: Vec<Token> = tokens_res.into_iter().map(|r| r.unwrap().0).collect();
    
    let expected = vec![
        Protocol, Identifier("P".to_string()), Extends, Identifier("Q".to_string()), LBrace,
        Identifier("method".to_string()), LParen, Identifier("a".to_string()), Colon, Identifier("Number".to_string()), Comma, Identifier("b".to_string()), Colon, Identifier("String".to_string()), RParen, Colon, Identifier("Boolean".to_string()), Semicolon,
        RBrace,
        Type, Identifier("T".to_string()), LParen, Identifier("a".to_string()), Comma, Identifier("b".to_string()), RParen, Inherits, Identifier("S".to_string()), LParen, Identifier("a".to_string()), Plus, Identifier("b".to_string()), RParen, LBrace,
        Identifier("attr".to_string()), Colon, Identifier("Number".to_string()), Star, Assign, LBracket, Identifier("x".to_string()), Plus, Number(1.0), Or, Identifier("x".to_string()), In, Identifier("b".to_string()), RBracket, Semicolon,
        Identifier("func".to_string()), LParen, Identifier("x".to_string()), Colon, LParen, Identifier("Number".to_string()), RParen, TypeArrow, Identifier("Boolean".to_string()), RParen, Colon, Identifier("Boolean".to_string()), FuncArrow, Identifier("x".to_string()), LParen, Identifier("self".to_string()), Dot, Identifier("attr".to_string()), LBracket, Number(0.0), RBracket, RParen, Semicolon,
        RBrace,
        Function, Identifier("main".to_string()), LParen, RParen, FuncArrow, Let, Identifier("x".to_string()), Assign, New, Identifier("T".to_string()), LParen, Number(1.0), Comma, LBracket, Number(2.0), Comma, Number(3.0), RBracket, RParen, In, LBrace,
        If, LParen, Identifier("x".to_string()), Is, Identifier("T".to_string()), RParen, LBrace,
        Print, LParen, Identifier("x".to_string()), As, Identifier("T".to_string()), RParen, Semicolon,
        RBrace, Elif, LParen, False, RParen, LBrace,
        Base, LParen, RParen, Semicolon,
        RBrace, Else, LBrace,
        While, LParen, True, RParen, LBrace,
        For, LParen, Identifier("i".to_string()), In, Identifier("x".to_string()), Dot, Identifier("attr".to_string()), RParen, Print, LParen, Identifier("i".to_string()), RParen, Semicolon,
        Identifier("x".to_string()), Dot, Identifier("attr".to_string()), LBracket, Number(0.0), RBracket, DestructAssign, Identifier("x".to_string()), Dot, Identifier("attr".to_string()), LBracket, Number(0.0), RBracket, Star, Number(2.0), Power, Number(3.0), Semicolon,
        RBrace,
        RBrace, Semicolon,
        LParen, Identifier("x".to_string()), Colon, Identifier("Number".to_string()), RParen, Colon, Identifier("Number".to_string()), FuncArrow, Identifier("x".to_string()), Plus, Number(1.0), Semicolon,
        Not, True, And, False, Or, LParen, Number(1.0), LessThanEq, Number(2.0), RParen, NotEqual, LParen, Number(3.0), GreaterThanEq, Number(4.0), RParen, LessThan, Number(5.0), GreaterThan, Number(6.0), Semicolon,
        StringLiteral("escaped \" \n \t \\".to_string()), Concat, StringLiteral("concat".to_string()), ConcatSpace, Number(42.0), Semicolon,
        RBrace
    ];

    assert_eq!(tokens.len(), expected.len(), "Token count mismatch. Found: {:?}\n\nExpected: {:?}", tokens, expected);
    for (i, (found, exp)) in tokens.iter().zip(expected.iter()).enumerate() {
        assert_eq!(found, exp, "Mismatch at index {}: found {:?} expected {:?}", i, found, exp);
    }
}

#[test]
fn test_instantiation() {
    let input = "new Point(0, 0) + new Complex(1, 2).magnitude()";
    let tokens: Vec<Token> = Lexer::new(input).map(|r| r.unwrap().0).collect();
    let expected = vec![
        New, Identifier("Point".to_string()), LParen, Number(0.0), Comma, Number(0.0), RParen,
        Plus,
        New, Identifier("Complex".to_string()), LParen, Number(1.0), Comma, Number(2.0), RParen,
        Dot, Identifier("magnitude".to_string()), LParen, RParen
    ];
    assert_eq!(tokens, expected);
}

#[test]
fn test_30_complete_hulk_program() {
    let input = r#"
        protocol Hashable {
            hash(): Number;
        }

        protocol Comparable extends Hashable {
            compare(other: Hashable): Number;
        }

        type Point(x, y) {
            x = x;
            y = y;
            hash() => self.x ^ 2 + self.y ** 2;
            distance(other: Point) => sqrt((self.x - other.x) ^ 2 + (self.y - other.y) ^ 2);
        }

        function solve_quadratic(a, b, c) {
            let disc = b ^ 2 - 4 * a * c in {
                if (disc < 0) print("No real roots")
                elif (disc == 0) print(-b / (2 * a))
                else {
                    print((-b + sqrt(disc)) / (2 * a));
                    print((-b - sqrt(disc)) / (2 * a));
                };
            };
        }

        let p1 = new Point(1, 2), 
            p2 = new Point(3, 4),
            transformer = (p: Point): Point => new Point(p.x * 2, p.y * 2) 
        in {
            print("Distance: " @ p1.distance(p2));
            let squares = [x ** 2 | x in [1, 2, 3, 4, 5]] in
                for (s in squares) if (s > 10) print(s);
            
            let a = 10 in while (a > 0) {
                a := a - 1;
                print(a);
            };
            
            print(if (p1 is Point) "Success" else "Fail");
            print(p1 as Point);
        }
    "#;

    let res: Result<Vec<_>, _> = Lexer::new(input).collect();
    let tokens: Vec<Token> = res.expect("Failed to lex integral program").into_iter().map(|(t, _)| t).collect();
    
    let expected = vec![
        // protocol Hashable { hash(): Number; }
        Protocol, Identifier("Hashable".into()), LBrace,
        Identifier("hash".into()), LParen, RParen, Colon, Identifier("Number".into()), Semicolon,
        RBrace,

        // protocol Comparable extends Hashable { compare(other: Hashable): Number; }
        Protocol, Identifier("Comparable".into()), Extends, Identifier("Hashable".into()), LBrace,
        Identifier("compare".into()), LParen, Identifier("other".into()), Colon, Identifier("Hashable".into()), RParen, Colon, Identifier("Number".into()), Semicolon,
        RBrace,

        // type Point(x, y) { x = x; y = y; ... }
        Type, Identifier("Point".into()), LParen, Identifier("x".into()), Comma, Identifier("y".into()), RParen, LBrace,
        Identifier("x".into()), Assign, Identifier("x".into()), Semicolon,
        Identifier("y".into()), Assign, Identifier("y".into()), Semicolon,
        Identifier("hash".into()), LParen, RParen, FuncArrow, Identifier("self".into()), Dot, Identifier("x".into()), Power, Number(2.0), Plus, Identifier("self".into()), Dot, Identifier("y".into()), Power, Number(2.0), Semicolon,
        Identifier("distance".into()), LParen, Identifier("other".into()), Colon, Identifier("Point".into()), RParen, FuncArrow, Identifier("sqrt".into()), LParen, 
        LParen, Identifier("self".into()), Dot, Identifier("x".into()), Minus, Identifier("other".into()), Dot, Identifier("x".into()), RParen, Power, Number(2.0), 
        Plus, 
        LParen, Identifier("self".into()), Dot, Identifier("y".into()), Minus, Identifier("other".into()), Dot, Identifier("y".into()), RParen, Power, Number(2.0), 
        RParen, Semicolon,
        RBrace,

        // function solve_quadratic(a, b, c) { ... }
        Function, Identifier("solve_quadratic".into()), LParen, Identifier("a".into()), Comma, Identifier("b".into()), Comma, Identifier("c".into()), RParen, LBrace,
        Let, Identifier("disc".into()), Assign, Identifier("b".into()), Power, Number(2.0), Minus, Number(4.0), Star, Identifier("a".into()), Star, Identifier("c".into()), In, LBrace,
        If, LParen, Identifier("disc".into()), LessThan, Number(0.0), RParen, Print, LParen, StringLiteral("No real roots".into()), RParen,
        Elif, LParen, Identifier("disc".into()), Equal, Number(0.0), RParen, Print, LParen, Minus, Identifier("b".into()), Slash, LParen, Number(2.0), Star, Identifier("a".into()), RParen, RParen,
        Else, LBrace,
        Print, LParen, LParen, Minus, Identifier("b".into()), Plus, Identifier("sqrt".into()), LParen, Identifier("disc".into()), RParen, RParen, Slash, LParen, Number(2.0), Star, Identifier("a".into()), RParen, RParen, Semicolon,
        Print, LParen, LParen, Minus, Identifier("b".into()), Minus, Identifier("sqrt".into()), LParen, Identifier("disc".into()), RParen, RParen, Slash, LParen, Number(2.0), Star, Identifier("a".into()), RParen, RParen, Semicolon,
        RBrace, Semicolon,
        RBrace, Semicolon,
        RBrace,

        // let p1 = new Point(1, 2), ... in { ... }
        Let, Identifier("p1".into()), Assign, New, Identifier("Point".into()), LParen, Number(1.0), Comma, Number(2.0), RParen, Comma,
        Identifier("p2".into()), Assign, New, Identifier("Point".into()), LParen, Number(3.0), Comma, Number(4.0), RParen, Comma,
        Identifier("transformer".into()), Assign, LParen, Identifier("p".into()), Colon, Identifier("Point".into()), RParen, Colon, Identifier("Point".into()), FuncArrow, New, Identifier("Point".into()), LParen, Identifier("p".into()), Dot, Identifier("x".into()), Star, Number(2.0), Comma, Identifier("p".into()), Dot, Identifier("y".into()), Star, Number(2.0), RParen,
        In, LBrace,
        Print, LParen, StringLiteral("Distance: ".into()), Concat, Identifier("p1".into()), Dot, Identifier("distance".into()), LParen, Identifier("p2".into()), RParen, RParen, Semicolon,
        Let, Identifier("squares".into()), Assign, LBracket, Identifier("x".into()), Power, Number(2.0), Or, Identifier("x".into()), In, LBracket, Number(1.0), Comma, Number(2.0), Comma, Number(3.0), Comma, Number(4.0), Comma, Number(5.0), RBracket, RBracket, In,
        For, LParen, Identifier("s".into()), In, Identifier("squares".into()), RParen, If, LParen, Identifier("s".into()), GreaterThan, Number(10.0), RParen, Print, LParen, Identifier("s".into()), RParen, Semicolon,
        
        Let, Identifier("a".into()), Assign, Number(10.0), In, While, LParen, Identifier("a".into()), GreaterThan, Number(0.0), RParen, LBrace,
        Identifier("a".into()), DestructAssign, Identifier("a".into()), Minus, Number(1.0), Semicolon,
        Print, LParen, Identifier("a".into()), RParen, Semicolon,
        RBrace, Semicolon,

        Print, LParen, If, LParen, Identifier("p1".into()), Is, Identifier("Point".into()), RParen, StringLiteral("Success".into()), Else, StringLiteral("Fail".into()), RParen, Semicolon,
        Print, LParen, Identifier("p1".into()), As, Identifier("Point".into()), RParen, Semicolon,
        RBrace
    ];

    assert_eq!(tokens.len(), expected.len(), "Token count mismatch. Found {}. Expected {}.", tokens.len(), expected.len());
    for (i, (found, exp)) in tokens.iter().zip(expected.iter()).enumerate() {
        assert_eq!(found, exp, "Mismatch at index {}: found {:?} expected {:?}", i, found, exp);
    }
}

trait BoolExt { fn is_falsy(&self) -> bool; }
impl BoolExt for bool { fn is_falsy(&self) -> bool { !*self } }

// ============ MACRO TESTS ============

#[test]
fn test_macro_keywords() {
    let input = "def match case default";
    let mut lexer = Lexer::new(input);
    assert_eq!(lexer.next().unwrap().unwrap().0, Def);
    assert_eq!(lexer.next().unwrap().unwrap().0, Match);
    assert_eq!(lexer.next().unwrap().unwrap().0, Case);
    assert_eq!(lexer.next().unwrap().unwrap().0, Default);
}

#[test]
fn test_macro_special_tokens() {
    let input = "$ @";
    let mut lexer = Lexer::new(input);
    assert_eq!(lexer.next().unwrap().unwrap().0, Dollar);
    assert_eq!(lexer.next().unwrap().unwrap().0, Concat); // @ se tokeniza como Concat
}

#[test]
fn test_simple_macro_definition() {
    let input = "def repeat(n: Number, *expr: Object): Object => expr;";
    let tokens: Vec<_> = Lexer::new(input).collect::<Result<Vec<_>, _>>().unwrap().into_iter().map(|(t, _)| t).collect();
    
    let expected = vec![
        Def,
        Identifier("repeat".into()),
        LParen,
        Identifier("n".into()),
        Colon,
        Identifier("Number".into()),
        Comma,
        Star,
        Identifier("expr".into()),
        Colon,
        Identifier("Object".into()),
        RParen,
        Colon,
        Identifier("Object".into()),
        FuncArrow,
        Identifier("expr".into()),
        Semicolon,
    ];
    
    assert_eq!(tokens, expected);
}

#[test]
fn test_macro_with_symbolic_args() {
    let input = "def swap(@a: Object, @b: Object)";
    let tokens: Vec<_> = Lexer::new(input).collect::<Result<Vec<_>, _>>().unwrap().into_iter().map(|(t, _)| t).collect();
    
    let expected = vec![
        Def,
        Identifier("swap".into()),
        LParen,
        Concat, // @ para symbolic arg
        Identifier("a".into()),
        Colon,
        Identifier("Object".into()),
        Comma,
        Concat,
        Identifier("b".into()),
        Colon,
        Identifier("Object".into()),
        RParen,
    ];
    
    assert_eq!(tokens, expected);
}

#[test]
fn test_macro_with_placeholder() {
    let input = "def repeat($iter: Number, n: Number)";
    let tokens: Vec<_> = Lexer::new(input).collect::<Result<Vec<_>, _>>().unwrap().into_iter().map(|(t, _)| t).collect();
    
    let expected = vec![
        Def,
        Identifier("repeat".into()),
        LParen,
        Dollar,
        Identifier("iter".into()),
        Colon,
        Identifier("Number".into()),
        Comma,
        Identifier("n".into()),
        Colon,
        Identifier("Number".into()),
        RParen,
    ];
    
    assert_eq!(tokens, expected);
}

#[test]
fn test_match_expression() {
    let input = "match(x) { case (y: Number + 0) => y; default => x; }";
    let tokens: Vec<_> = Lexer::new(input).collect::<Result<Vec<_>, _>>().unwrap().into_iter().map(|(t, _)| t).collect();
    
    assert!(tokens.contains(&Match));
    assert!(tokens.contains(&Case));
    assert!(tokens.contains(&Default));
    assert!(tokens.contains(&FuncArrow));
}

#[test]
fn test_complete_macro_with_match() {
    let input = r#"def simplify(expr: Number) {
    match(expr) {
        case (x: Number + 0) => x;
        default => expr;
    };
}"#;
    
    let tokens: Vec<_> = Lexer::new(input).collect::<Result<Vec<_>, _>>().unwrap().into_iter().map(|(t, _)| t).collect();
    
    // Verificar que todos los tokens clave están presentes
    assert!(tokens.contains(&Def));
    assert!(tokens.contains(&Match));
    assert!(tokens.contains(&Case));
    assert!(tokens.contains(&Default));
    assert_eq!(tokens.iter().filter(|t| **t == LParen).count() >= 3, true);
    assert_eq!(tokens.iter().filter(|t| **t == LBrace).count() >= 2, true);
}

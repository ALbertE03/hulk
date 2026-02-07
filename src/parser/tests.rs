use super::*;
use crate::utils::{Position, Spanned};

macro_rules! pos {
    ($line:expr, $col:expr) => {
        Position { line: $line, column: $col }
    };
}

macro_rules! span {
    ($node:expr, $line:expr, $col:expr) => {
        Spanned::new($node, pos!($line, $col))
    };
}

macro_rules! bspan {
    ($node:expr, $line:expr, $col:expr) => {
        Box::new(span!($node, $line, $col))
    };
}


#[test]
fn test_parse_basic_arithmetic() {
    let input = "1 + 2 * 3;";
    let mut parser = Parser::new(input);
    let program = parser.parse_program().unwrap();
    
    let expected = Program {
        declarations: vec![],
        expr: span!(
            Expr::Binary(
                Box::new(span!(Expr::Number(1.0), 1, 1)),
                Op::Add,
                Box::new(span!(
                    Expr::Binary(
                        Box::new(span!(Expr::Number(2.0), 1, 5)),
                        Op::Mul,
                        Box::new(span!(Expr::Number(3.0), 1, 9)),
                    ),
                    1, 5
                ))
            ),
            1, 1
        ),
    };

    assert_eq!(program, expected);
}

#[test]
fn test_parse_function_declaration() {
    let input = "function add(a, b) => a + b; add(1, 2);";
    let mut parser = Parser::new(input);
    let program = parser.parse_program().unwrap();

    let expected = Program {
        declarations: vec![
            Declaration::Function(FunctionDecl {
                name: "add".to_string(),
                params: vec![
                    Param { name: "a".to_string(), type_annotation: None },
                    Param { name: "b".to_string(), type_annotation: None },
                ],
                return_type: None,
                body: span!(
                    Expr::Binary(
                        Box::new(span!(Expr::Identifier("a".to_string()), 1, 23)),
                        Op::Add,
                        Box::new(span!(Expr::Identifier("b".to_string()), 1, 27)),
                    ),
                    1, 23
                ),
            })
        ],
        expr: span!(
            Expr::Call {
                func: "add".to_string(),
                args: vec![
                    span!(Expr::Number(1.0), 1, 34),
                    span!(Expr::Number(2.0), 1, 37),
                ],
            },
            1, 30
        ),
    };

    assert_eq!(program, expected);
}

#[test]
fn test_parse_if_else() {
    let input = "if (true) 1 else 0;";
    let mut parser = Parser::new(input);
    let program = parser.parse_program().unwrap();

    let expected = Program {
        declarations: vec![],
        expr: span!(
            Expr::If {
                cond: Box::new(span!(Expr::Boolean(true), 1, 5)),
                then_expr: Box::new(span!(Expr::Number(1.0), 1, 11)),
                else_expr: Box::new(span!(Expr::Number(0.0), 1, 18)),
            },
            1, 1
        ),
    };

    assert_eq!(program, expected);
}
#[test]
fn test_parse_nominal_typing_basics() {
    let input = "let pt = new Point(3, 4) in print(\"x: \" @ pt.getX() @ \"; y: \" @ pt.getY());";
    let mut parser = Parser::new(input);
    let program = parser.parse_program().unwrap();

    let expected = Program {
        declarations: vec![],
        expr: span!(
            Expr::Let {
                bindings: vec![
                    (
                        "pt".to_string(),
                        None,
                        span!(
                            Expr::Instantiation {
                                ty: "Point".to_string(),
                                args: vec![
                                    span!(Expr::Number(3.0), 1, 20),
                                    span!(Expr::Number(4.0), 1, 23),
                                ],
                            },
                            1, 10
                        )
                    )
                ],
                body: Box::new(span!(
                    Expr::Call {
                        func: "print".to_string(),
                        args: vec![
                            span!(
                                Expr::Binary(
                                    Box::new(span!(
                                        Expr::Binary(
                                            Box::new(span!(
                                                Expr::Binary(
                                                    Box::new(span!(Expr::String("x: ".to_string()), 1, 35)),
                                                    Op::Concat,
                                                    Box::new(span!(
                                                        Expr::MethodCall {
                                                            obj: Box::new(span!(Expr::Identifier("pt".to_string()), 1, 43)),
                                                            method: "getX".to_string(),
                                                            args: vec![],
                                                        },
                                                        1, 45
                                                    ))
                                                ),
                                                1, 35
                                            )),
                                            Op::Concat,
                                            Box::new(span!(Expr::String("; y: ".to_string()), 1, 55)),
                                        ),
                                        1, 35
                                    )),
                                    Op::Concat,
                                    Box::new(span!(
                                        Expr::MethodCall {
                                            obj: Box::new(span!(Expr::Identifier("pt".to_string()), 1, 65)),
                                            method: "getY".to_string(),
                                            args: vec![],
                                        },
                                        1, 67
                                    ))
                                ),
                                1, 35
                            )
                        ],
                    },
                    1, 29
                )),
            },
            1, 1
        ),
    };

    assert_eq!(program, expected);
}

#[test]
fn test_parse_type_with_inheritance() {
    let input = "type PolarPoint inherits Point { rho() => sqrt(self.getX() ^ 2 + self.getY() ^ 2); }";
    let mut parser = Parser::new(input);
    let program = parser.parse_program().expect("Failed to parse PolarPoint");

    let expected = Program {
        declarations: vec![
            Declaration::Type(TypeDecl {
                name: "PolarPoint".to_string(),
                params: vec![],
                parent: Some(TypeInit { name: "Point".to_string(), args: vec![] }),
                attributes: vec![],
                methods: vec![
                    FunctionDecl {
                        name: "rho".to_string(),
                        params: vec![],
                        return_type: None,
                        body: span!(
                            Expr::Sqrt(Box::new(span!(
                                Expr::Binary(
                                    Box::new(span!(
                                        Expr::Binary(
                                            Box::new(span!(
                                                Expr::MethodCall {
                                                    obj: Box::new(span!(Expr::Identifier("self".to_string()), 1, 48)),
                                                    method: "getX".to_string(),
                                                    args: vec![],
                                                },
                                                1, 52
                                            )),
                                            Op::Pow,
                                            Box::new(span!(Expr::Number(2.0), 1, 62)),
                                        ),
                                        1, 52
                                    )),
                                    Op::Add,
                                    Box::new(span!(
                                        Expr::Binary(
                                            Box::new(span!(
                                                Expr::MethodCall {
                                                    obj: Box::new(span!(Expr::Identifier("self".to_string()), 1, 66)),
                                                    method: "getY".to_string(),
                                                    args: vec![],
                                                },
                                                1, 70
                                            )),
                                            Op::Pow,
                                            Box::new(span!(Expr::Number(2.0), 1, 80)),
                                        ),
                                        1, 70
                                    )),
                                ),
                                1, 52
                            ))),
                            1, 43
                        ),
                    }
                ],
            })
        ],
        expr: span!(Expr::Block(vec![]), 1, 84),
    };

    assert_eq!(program, expected);

}
#[test]
fn test_parse_inheritance_with_args() {
    let input = "type PolarPoint(phi, rho) inherits Point(rho * sin(phi), rho * cos(phi)) { }";
    let mut parser = Parser::new(input);
    let program = parser.parse_program().expect("Failed to parse inheritance with args");

    let expected = Program {
        declarations: vec![
            Declaration::Type(TypeDecl {
                name: "PolarPoint".to_string(),
                params: vec![
                    Param { name: "phi".to_string(), type_annotation: None },
                    Param { name: "rho".to_string(), type_annotation: None },
                ],
                parent: Some(TypeInit {
                    name: "Point".to_string(),
                    args: vec![
                        span!(
                            Expr::Binary(
                                Box::new(span!(Expr::Identifier("rho".to_string()), 1, 42)),
                                Op::Mul,
                                Box::new(span!(
                                    Expr::Sin(Box::new(span!(Expr::Identifier("phi".to_string()), 1, 52))),
                                    1, 48
                                ))
                            ),
                            1, 42
                        ),
                        span!(
                            Expr::Binary(
                                Box::new(span!(Expr::Identifier("rho".to_string()), 1, 58)),
                                Op::Mul,
                                Box::new(span!(
                                    Expr::Cos(Box::new(span!(Expr::Identifier("phi".to_string()), 1, 68))),
                                    1, 64
                                ))
                            ),
                            1, 58
                        ),
                    ],
                }),
                attributes: vec![],
                methods: vec![],
            })
        ],
        expr: span!(Expr::Block(vec![]), 1, 76),
    };

    assert_eq!(program, expected);
}

#[test]
fn test_parse_base_call() {
    let input = "type Knight inherits Person { name() => \"Sir\" @@ base(); }";
    let mut parser = Parser::new(input);
    let program = parser.parse_program().expect("Failed to parse base call");

    let expected = Program {
        declarations: vec![
            Declaration::Type(TypeDecl {
                name: "Knight".to_string(),
                params: vec![],
                parent: Some(TypeInit {
                    name: "Person".to_string(),
                    args: vec![],
                }),
                attributes: vec![],
                methods: vec![
                    FunctionDecl {
                        name: "name".to_string(),
                        params: vec![],
                        return_type: None,
                        body: span!(
                            Expr::Binary(
                                Box::new(span!(Expr::String("Sir".to_string()), 1, 41)),
                                Op::ConcatSpace,
                                Box::new(span!(
                                    Expr::BaseCall { args: vec![] },
                                    1, 50
                                )),
                            ),
                            1, 41
                        ),
                    },
                ],
            }),
        ],
        expr: span!(Expr::Block(vec![]), 1, 58),
    };

    assert_eq!(program, expected);
}

#[test]
fn test_parse_type_with_attributes() {
    let input = "type Point { x = 0; y = 0; getX() => self.x; }";
    let mut parser = Parser::new(input);
    let program = parser.parse_program().expect("Failed to parse type with attributes");

    let expected = Program {
        declarations: vec![
            Declaration::Type(TypeDecl {
                name: "Point".to_string(),
                params: vec![],
                parent: None,
                attributes: vec![
                    Attribute {
                        name: "x".to_string(),
                        type_annotation: None,
                        init: span!(Expr::Number(0.0), 1, 18),
                    },
                    Attribute {
                        name: "y".to_string(),
                        type_annotation: None,
                        init: span!(Expr::Number(0.0), 1, 25),
                    },
                ],
                methods: vec![
                    FunctionDecl {
                        name: "getX".to_string(),
                        params: vec![],
                        return_type: None,
                        body: span!(
                            Expr::AttributeAccess {
                                obj: Box::new(span!(Expr::Identifier("self".to_string()), 1, 38)),
                                attribute: "x".to_string(),
                            },
                            1, 42
                        ),
                    },
                ],
            }),
        ],
        expr: span!(Expr::Block(vec![]), 1, 46),
    };

    assert_eq!(program, expected);
}
#[test]
fn test_parse_lambda_simple() {
    let input = "(x) => x + 1;";
    let mut parser = Parser::new(input);
    let program = parser.parse_program().expect("Failed to parse simple lambda");

    let expected = Program {
        declarations: vec![],
        expr: span!(
            Expr::Lambda {
                params: vec![
                    Param { name: "x".to_string(), type_annotation: None },
                ],
                return_type: None,
                body: Box::new(span!(
                    Expr::Binary(
                        Box::new(span!(Expr::Identifier("x".to_string()), 1, 8)),
                        Op::Add,
                        Box::new(span!(Expr::Number(1.0), 1, 12)),
                    ),
                    1, 8
                )),
            },
            1, 1
        ),
    };

    assert_eq!(program, expected);
}

#[test]
fn test_parse_lambda_with_types() {
    let input = "(x: Number): Boolean => x % 2 == 0;";
    let mut parser = Parser::new(input);
    let program = parser.parse_program().expect("Failed to parse lambda with types");

    let expected = Program {
        declarations: vec![],
        expr: span!(
            Expr::Lambda {
                params: vec![
                    Param {
                        name: "x".to_string(),
                        type_annotation: Some(TypeAnnotation::Name("Number".to_string())),
                    },
                ],
                return_type: Some(TypeAnnotation::Name("Boolean".to_string())),
                body: Box::new(span!(
                    Expr::Binary(
                        Box::new(span!(
                            Expr::Binary(
                                Box::new(span!(Expr::Identifier("x".to_string()), 1, 25)),
                                Op::Mod,
                                Box::new(span!(Expr::Number(2.0), 1, 29)),
                            ),
                            1, 25
                        )),
                        Op::Eq,
                        Box::new(span!(Expr::Number(0.0), 1, 34)),
                    ),
                    1, 25
                )),
            },
            1, 1
        ),
    };

    assert_eq!(program, expected);
}

#[test]
fn test_parse_function_type_annotation() {
    let input = "function count_when(numbers: Number*, filter: (Number) -> Boolean) => 0;";
    let mut parser = Parser::new(input);
    let program = parser.parse_program().expect("Failed to parse function with complex type annotations");

    let expected = Program {
        declarations: vec![
            Declaration::Function(FunctionDecl {
                name: "count_when".to_string(),
                params: vec![
                    Param {
                        name: "numbers".to_string(),
                        type_annotation: Some(TypeAnnotation::Iterable(Box::new(TypeAnnotation::Name("Number".to_string())))),
                    },
                    Param {
                        name: "filter".to_string(),
                        type_annotation: Some(TypeAnnotation::Function {
                            params: vec![TypeAnnotation::Name("Number".to_string())],
                            return_type: Box::new(TypeAnnotation::Name("Boolean".to_string())),
                        }),
                    },
                ],
                return_type: None,
                body: span!(Expr::Number(0.0), 1, 71),
            })
        ],
        expr: span!(Expr::Block(vec![]), 1, 72),
    };

    assert_eq!(program, expected);
}

#[test]
fn test_parse_nested_lambdas() {
    let input = "(x) => (y) => x + y;";
    let mut parser = Parser::new(input);
    let program = parser.parse_program().expect("Failed to parse nested lambdas");

    let expected = Program {
        declarations: vec![],
        expr: span!(
            Expr::Lambda {
                params: vec![
                    Param { name: "x".to_string(), type_annotation: None },
                ],
                return_type: None,
                body: Box::new(span!(
                    Expr::Lambda {
                        params: vec![
                            Param { name: "y".to_string(), type_annotation: None },
                        ],
                        return_type: None,
                        body: Box::new(span!(
                            Expr::Binary(
                                Box::new(span!(Expr::Identifier("x".to_string()), 1, 15)),
                                Op::Add,
                                Box::new(span!(Expr::Identifier("y".to_string()), 1, 19)),
                            ),
                            1, 15
                        )),
                    },
                    1, 8
                )),
            },
            1, 1
        ),
    };

    assert_eq!(program, expected);
}

#[test]
fn test_parse_math_builtins() {
    let input = "sqrt(sin(PI) + cos(E) * exp(1)) + log(2, 8) + rand();";
    let mut parser = Parser::new(input);
    let program = parser.parse_program().expect("Failed to parse math built-ins");

    let expected = Program {
        declarations: vec![],
        expr: span!(
            Expr::Binary(
                Box::new(span!(
                    Expr::Binary(
                        Box::new(span!(
                            Expr::Sqrt(Box::new(span!(
                                Expr::Binary(
                                    Box::new(span!(
                                        Expr::Sin(Box::new(span!(Expr::PI, 1, 10))),
                                        1, 6
                                    )),
                                    Op::Add,
                                    Box::new(span!(
                                        Expr::Binary(
                                            Box::new(span!(
                                                Expr::Cos(Box::new(span!(Expr::E, 1, 20))),
                                                1, 16
                                            )),
                                            Op::Mul,
                                            Box::new(span!(
                                                Expr::Exp(Box::new(span!(Expr::Number(1.0), 1, 29))),
                                                1, 25
                                            )),
                                        ),
                                        1, 16
                                    )),
                                ),
                                1, 6
                            ))),
                            1, 1
                        )),
                        Op::Add,
                        Box::new(span!(
                            Expr::Log(
                                Box::new(span!(Expr::Number(2.0), 1, 39)),
                                Box::new(span!(Expr::Number(8.0), 1, 42)),
                            ),
                            1, 35
                        )),
                    ),
                    1, 1
                )),
                Op::Add,
                Box::new(span!(Expr::Rand, 1, 47)),
            ),
            1, 1
        ),
    };

    assert_eq!(program, expected);
}

#[test]
fn test_parse_integral_program() {
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
                for (s in squares) if (s > 10) print(s) else {};
            
            let a = 10 in while (a > 0) {
                a := a - 1;
                print(a);
            };
            
            print(if (p1 is Point) "Success" else "Fail");
            print(p1 as Point);
        }
    "#;
    let mut parser = Parser::new(input);
    let expected = Program {
        declarations: vec![
            Declaration::Protocol(ProtocolDecl {
                name: "Hashable".to_string(),
                parent: None,
                methods: vec![
                    MethodSignature {
                        name: "hash".to_string(),
                        params: vec![],
                        return_type: TypeAnnotation::Name("Number".to_string()),
                    },
                ],
            }),
            Declaration::Protocol(ProtocolDecl {
                name: "Comparable".to_string(),
                parent: Some("Hashable".to_string()),
                methods: vec![
                    MethodSignature {
                        name: "compare".to_string(),
                        params: vec![
                            Param {
                                name: "other".to_string(),
                                type_annotation: Some(TypeAnnotation::Name("Hashable".to_string())),
                            },
                        ],
                        return_type: TypeAnnotation::Name("Number".to_string()),
                    },
                ],
            }),
            Declaration::Type(TypeDecl {
                name: "Point".to_string(),
                params: vec![
                    Param { name: "x".to_string(), type_annotation: None },
                    Param { name: "y".to_string(), type_annotation: None },
                ],
                parent: None,
                attributes: vec![
                    Attribute {
                        name: "x".to_string(),
                        type_annotation: None,
                        init: span!(Expr::Identifier("x".to_string()), 11, 17),
                    },
                    Attribute {
                        name: "y".to_string(),
                        type_annotation: None,
                        init: span!(Expr::Identifier("y".to_string()), 12, 17),
                    },
                ],
                methods: vec![
                    FunctionDecl {
                        name: "hash".to_string(),
                        params: vec![],
                        return_type: None,
                        body: span!(
                            Expr::Binary(
                                bspan!(
                                    Expr::Binary(
                                        bspan!(
                                            Expr::AttributeAccess {
                                                obj: bspan!(Expr::Identifier("self".to_string()), 13, 23),
                                                attribute: "x".to_string(),
                                            },
                                            13, 27
                                        ),
                                        Op::Pow,
                                        bspan!(Expr::Number(2.0), 13, 32),
                                    ),
                                    13, 27
                                ),
                                Op::Add,
                                bspan!(
                                    Expr::Binary(
                                        bspan!(
                                            Expr::AttributeAccess {
                                                obj: bspan!(Expr::Identifier("self".to_string()), 13, 36),
                                                attribute: "y".to_string(),
                                            },
                                            13, 40
                                        ),
                                        Op::Pow,
                                        bspan!(Expr::Number(2.0), 13, 46),
                                    ),
                                    13, 40
                                ),
                            ),
                            13, 27
                        ),
                    },
                    FunctionDecl {
                        name: "distance".to_string(),
                        params: vec![
                            Param {
                                name: "other".to_string(),
                                type_annotation: Some(TypeAnnotation::Name("Point".to_string())),
                            },
                        ],
                        return_type: None,
                        body: span!(
                            Expr::Sqrt(bspan!(
                                Expr::Binary(
                                    bspan!(
                                        Expr::Binary(
                                            bspan!(
                                                Expr::Binary(
                                                    bspan!(
                                                        Expr::AttributeAccess {
                                                            obj: bspan!(Expr::Identifier("self".to_string()), 14, 45),
                                                            attribute: "x".to_string(),
                                                        },
                                                        14, 49
                                                    ),
                                                    Op::Sub,
                                                    bspan!(
                                                        Expr::AttributeAccess {
                                                            obj: bspan!(Expr::Identifier("other".to_string()), 14, 54),
                                                            attribute: "x".to_string(),
                                                        },
                                                        14, 59
                                                    ),
                                                ),
                                                14, 49
                                            ),
                                            Op::Pow,
                                            bspan!(Expr::Number(2.0), 14, 65),
                                        ),
                                        14, 49
                                    ),
                                    Op::Add,
                                    bspan!(
                                        Expr::Binary(
                                            bspan!(
                                                Expr::Binary(
                                                    bspan!(
                                                        Expr::AttributeAccess {
                                                            obj: bspan!(Expr::Identifier("self".to_string()), 14, 70),
                                                            attribute: "y".to_string(),
                                                        },
                                                        14, 74
                                                    ),
                                                    Op::Sub,
                                                    bspan!(
                                                        Expr::AttributeAccess {
                                                            obj: bspan!(Expr::Identifier("other".to_string()), 14, 79),
                                                            attribute: "y".to_string(),
                                                        },
                                                        14, 84
                                                    ),
                                                ),
                                                14, 74
                                            ),
                                            Op::Pow,
                                            bspan!(Expr::Number(2.0), 14, 90),
                                        ),
                                        14, 74
                                    ),
                                ),
                                14, 49
                            )),
                            14, 39
                        ),
                    },
                ],
            }),
            Declaration::Function(FunctionDecl {
                name: "solve_quadratic".to_string(),
                params: vec![
                    Param { name: "a".to_string(), type_annotation: None },
                    Param { name: "b".to_string(), type_annotation: None },
                    Param { name: "c".to_string(), type_annotation: None },
                ],
                return_type: None,
                body: span!(
                    Expr::Block(vec![
                        span!(
                            Expr::Let {
                                bindings: vec![
                                    (
                                        "disc".to_string(),
                                        None,
                                        span!(
                                            Expr::Binary(
                                                bspan!(
                                                    Expr::Binary(
                                                        bspan!(Expr::Identifier("b".to_string()), 18, 24),
                                                        Op::Pow,
                                                        bspan!(Expr::Number(2.0), 18, 28),
                                                    ),
                                                    18, 24
                                                ),
                                                Op::Sub,
                                                bspan!(
                                                    Expr::Binary(
                                                        bspan!(
                                                            Expr::Binary(
                                                                bspan!(Expr::Number(4.0), 18, 32),
                                                                Op::Mul,
                                                                bspan!(Expr::Identifier("a".to_string()), 18, 36),
                                                            ),
                                                            18, 32
                                                        ),
                                                        Op::Mul,
                                                        bspan!(Expr::Identifier("c".to_string()), 18, 40),
                                                    ),
                                                    18, 32
                                                ),
                                            ),
                                            18, 24
                                        )
                                    ),
                                ],
                                body: bspan!(
                                    Expr::Block(vec![
                                        span!(
                                            Expr::If {
                                                cond: bspan!(
                                                    Expr::Binary(
                                                        bspan!(Expr::Identifier("disc".to_string()), 19, 21),
                                                        Op::Lt,
                                                        bspan!(Expr::Number(0.0), 19, 28),
                                                    ),
                                                    19, 21
                                                ),
                                                then_expr: bspan!(
                                                    Expr::Call {
                                                        func: "print".to_string(),
                                                        args: vec![span!(Expr::String("No real roots".to_string()), 19, 37)],
                                                    },
                                                    19, 31
                                                ),
                                                else_expr: bspan!(
                                                    Expr::If {
                                                        cond: bspan!(
                                                            Expr::Binary(
                                                                bspan!(Expr::Identifier("disc".to_string()), 20, 23),
                                                                Op::Eq,
                                                                bspan!(Expr::Number(0.0), 20, 31),
                                                            ),
                                                            20, 23
                                                        ),
                                                        then_expr: bspan!(
                                                            Expr::Call {
                                                                func: "print".to_string(),
                                                                args: vec![
                                                                    span!(
                                                                        Expr::Binary(
                                                                            bspan!(
                                                                                Expr::Unary(
                                                                                    UnOp::Neg,
                                                                                    bspan!(Expr::Identifier("b".to_string()), 20, 41),
                                                                                ),
                                                                                20, 40
                                                                            ),
                                                                            Op::Div,
                                                                            bspan!(
                                                                                Expr::Binary(
                                                                                    bspan!(Expr::Number(2.0), 20, 46),
                                                                                    Op::Mul,
                                                                                    bspan!(Expr::Identifier("a".to_string()), 20, 50),
                                                                                ),
                                                                                20, 46
                                                                            ),
                                                                        ),
                                                                        20, 40
                                                                    )
                                                                ],
                                                            },
                                                            20, 34
                                                        ),
                                                        else_expr: bspan!(
                                                            Expr::Block(vec![
                                                                span!(
                                                                    Expr::Call {
                                                                        func: "print".to_string(),
                                                                        args: vec![
                                                                            span!(
                                                                                Expr::Binary(
                                                                                    bspan!(
                                                                                        Expr::Binary(
                                                                                            bspan!(
                                                                                                Expr::Unary(
                                                                                                    UnOp::Neg,
                                                                                                    bspan!(Expr::Identifier("b".to_string()), 22, 29),
                                                                                                ),
                                                                                                22, 28
                                                                                            ),
                                                                                            Op::Add,
                                                                                            bspan!(
                                                                                                Expr::Sqrt(bspan!(Expr::Identifier("disc".to_string()), 22, 38)),
                                                                                                22, 33
                                                                                            ),
                                                                                        ),
                                                                                        22, 28
                                                                                    ),
                                                                                    Op::Div,
                                                                                    bspan!(
                                                                                        Expr::Binary(
                                                                                            bspan!(Expr::Number(2.0), 22, 48),
                                                                                            Op::Mul,
                                                                                            bspan!(Expr::Identifier("a".to_string()), 22, 52),
                                                                                        ),
                                                                                        22, 48
                                                                                    ),
                                                                                ),
                                                                                22, 28
                                                                            )
                                                                        ],
                                                                    },
                                                                    22, 21
                                                                ),
                                                                span!(
                                                                    Expr::Call {
                                                                        func: "print".to_string(),
                                                                        args: vec![
                                                                            span!(
                                                                                Expr::Binary(
                                                                                    bspan!(
                                                                                        Expr::Binary(
                                                                                            bspan!(
                                                                                                Expr::Unary(
                                                                                                    UnOp::Neg,
                                                                                                    bspan!(Expr::Identifier("b".to_string()), 23, 29),
                                                                                                ),
                                                                                                23, 28
                                                                                            ),
                                                                                            Op::Sub,
                                                                                            bspan!(
                                                                                                Expr::Sqrt(bspan!(Expr::Identifier("disc".to_string()), 23, 38)),
                                                                                                23, 33
                                                                                            ),
                                                                                        ),
                                                                                        23, 28
                                                                                    ),
                                                                                    Op::Div,
                                                                                    bspan!(
                                                                                        Expr::Binary(
                                                                                            bspan!(Expr::Number(2.0), 23, 48),
                                                                                            Op::Mul,
                                                                                            bspan!(Expr::Identifier("a".to_string()), 23, 52),
                                                                                        ),
                                                                                        23, 48
                                                                                    ),
                                                                                ),
                                                                                23, 28
                                                                            )
                                                                        ],
                                                                    },
                                                                    23, 21
                                                                ),
                                                            ]),
                                                            21, 22
                                                        ),
                                                    },
                                                    20, 22
                                                ),
                                            },
                                            19, 17
                                        ),
                                    ]),
                                    18, 45
                                ),
                            },
                            18, 13
                        ),
                    ]),
                    17, 43
                ),
            }),
        ],
        expr: span!(
            Expr::Let {
                bindings: vec![
                    (
                        "p1".to_string(),
                        None,
                        span!(
                            Expr::Instantiation {
                                ty: "Point".to_string(),
                                args: vec![
                                    span!(Expr::Number(1.0), 28, 28),
                                    span!(Expr::Number(2.0), 28, 31),
                                ],
                            },
                            28, 18
                        )
                    ),
                    (
                        "p2".to_string(),
                        None,
                        span!(
                            Expr::Instantiation {
                                ty: "Point".to_string(),
                                args: vec![
                                    span!(Expr::Number(3.0), 29, 28),
                                    span!(Expr::Number(4.0), 29, 31),
                                ],
                            },
                            29, 18
                        )
                    ),
                    (
                        "transformer".to_string(),
                        None,
                        span!(
                            Expr::Lambda {
                                params: vec![
                                    Param {
                                        name: "p".to_string(),
                                        type_annotation: Some(TypeAnnotation::Name("Point".to_string())),
                                    },
                                ],
                                return_type: Some(TypeAnnotation::Name("Point".to_string())),
                                body: bspan!(
                                    Expr::Instantiation {
                                        ty: "Point".to_string(),
                                        args: vec![
                                            span!(
                                                Expr::Binary(
                                                    bspan!(
                                                        Expr::AttributeAccess {
                                                            obj: bspan!(Expr::Identifier("p".to_string()), 30, 58),
                                                            attribute: "x".to_string(),
                                                        },
                                                        30, 59
                                                    ),
                                                    Op::Mul,
                                                    bspan!(Expr::Number(2.0), 30, 64),
                                                ),
                                                30, 59
                                            ),
                                            span!(
                                                Expr::Binary(
                                                    bspan!(
                                                        Expr::AttributeAccess {
                                                            obj: bspan!(Expr::Identifier("p".to_string()), 30, 67),
                                                            attribute: "y".to_string(),
                                                        },
                                                        30, 68
                                                    ),
                                                    Op::Mul,
                                                    bspan!(Expr::Number(2.0), 30, 73),
                                                ),
                                                30, 68
                                            ),
                                        ],
                                    },
                                    30, 48
                                ),
                            },
                            30, 27
                        )
                    ),
                ],
                body: bspan!(
                    Expr::Block(vec![
                        span!(
                            Expr::Call {
                                func: "print".to_string(),
                                args: vec![
                                    span!(
                                        Expr::Binary(
                                            bspan!(Expr::String("Distance: ".to_string()), 32, 19),
                                            Op::Concat,
                                            bspan!(
                                                Expr::MethodCall {
                                                    obj: bspan!(Expr::Identifier("p1".to_string()), 32, 34),
                                                    method: "distance".to_string(),
                                                    args: vec![span!(Expr::Identifier("p2".to_string()), 32, 46)],
                                                },
                                                32, 36
                                            ),
                                        ),
                                        32, 19
                                    )
                                ],
                            },
                            32, 13
                        ),
                        span!(
                            Expr::Let {
                                bindings: vec![
                                    (
                                        "squares".to_string(),
                                        None,
                                        span!(
                                            Expr::VectorGenerator {
                                                expr: bspan!(
                                                    Expr::Binary(
                                                        bspan!(Expr::Identifier("x".to_string()), 33, 28),
                                                        Op::Pow,
                                                        bspan!(Expr::Number(2.0), 33, 33),
                                                    ),
                                                    33, 28
                                                ),
                                                var: "x".to_string(),
                                                iterable: bspan!(
                                                    Expr::VectorLiteral(vec![
                                                        span!(Expr::Number(1.0), 33, 43),
                                                        span!(Expr::Number(2.0), 33, 46),
                                                        span!(Expr::Number(3.0), 33, 49),
                                                        span!(Expr::Number(4.0), 33, 52),
                                                        span!(Expr::Number(5.0), 33, 55),
                                                    ]),
                                                    33, 42
                                                ),
                                            },
                                            33, 27
                                        )
                                    ),
                                ],
                                body: bspan!(
                                    Expr::For {
                                        var: "s".to_string(),
                                        iterable: bspan!(Expr::Identifier("squares".to_string()), 34, 27),
                                        body: bspan!(
                                            Expr::If {
                                                cond: bspan!(
                                                    Expr::Binary(
                                                        bspan!(Expr::Identifier("s".to_string()), 34, 40),
                                                        Op::Gt,
                                                        bspan!(Expr::Number(10.0), 34, 44),
                                                    ),
                                                    34, 40
                                                ),
                                                then_expr: bspan!(
                                                    Expr::Call {
                                                        func: "print".to_string(),
                                                        args: vec![span!(Expr::Identifier("s".to_string()), 34, 54)],
                                                    },
                                                    34, 48
                                                ),
                                                else_expr: bspan!(Expr::Block(vec![]), 34, 62),
                                            },
                                            34, 36
                                        ),
                                    },
                                    34, 17
                                ),
                            },
                            33, 13
                        ),
                        span!(
                            Expr::Let {
                                bindings: vec![
                                    (
                                        "a".to_string(),
                                        None,
                                        span!(Expr::Number(10.0), 36, 21)
                                    ),
                                ],
                                body: bspan!(
                                    Expr::While {
                                        cond: bspan!(
                                            Expr::Binary(
                                                bspan!(Expr::Identifier("a".to_string()), 36, 34),
                                                Op::Gt,
                                                bspan!(Expr::Number(0.0), 36, 38),
                                            ),
                                            36, 34
                                        ),
                                        body: bspan!(
                                            Expr::Block(vec![
                                                span!(
                                                    Expr::Assignment {
                                                        target: "a".to_string(),
                                                        value: bspan!(
                                                            Expr::Binary(
                                                                bspan!(Expr::Identifier("a".to_string()), 37, 22),
                                                                Op::Sub,
                                                                bspan!(Expr::Number(1.0), 37, 26),
                                                            ),
                                                            37, 22
                                                        ),
                                                    },
                                                    37, 17
                                                ),
                                                span!(
                                                    Expr::Call {
                                                        func: "print".to_string(),
                                                        args: vec![span!(Expr::Identifier("a".to_string()), 38, 23)],
                                                    },
                                                    38, 17
                                                ),
                                            ]),
                                            36, 41
                                        ),
                                    },
                                    36, 27
                                ),
                            },
                            36, 13
                        ),
                        span!(
                            Expr::Call {
                                func: "print".to_string(),
                                args: vec![
                                    span!(
                                        Expr::If {
                                            cond: bspan!(
                                                Expr::Is(
                                                    bspan!(Expr::Identifier("p1".to_string()), 41, 23),
                                                    "Point".to_string(),
                                                ),
                                                41, 26
                                            ),
                                            then_expr: bspan!(Expr::String("Success".to_string()), 41, 36),
                                            else_expr: bspan!(Expr::String("Fail".to_string()), 41, 51),
                                        },
                                        41, 19
                                    )
                                ],
                            },
                            41, 13
                        ),
                        span!(
                            Expr::Call {
                                func: "print".to_string(),
                                args: vec![
                                    span!(
                                        Expr::As(
                                            bspan!(Expr::Identifier("p1".to_string()), 42, 19),
                                            "Point".to_string(),
                                        ),
                                        42, 22
                                    )
                                ],
                            },
                            42, 13
                        ),
                    ]),
                    31, 12
                ),
            },
            28, 9
        ),
    };

    let program = parser.parse_program().expect("Failed to parse integral program");
    assert_eq!(program, expected);
}

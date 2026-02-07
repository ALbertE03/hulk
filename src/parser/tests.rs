use super::*;

#[test]
fn test_parse_basic_arithmetic() {
    let input = "1 + 2 * 3;";
    let mut parser = Parser::new(input);
    let program = parser.parse_program().unwrap();
    
    // Check that we have one expression (the program body)
    if let Expr::Binary(left, op, right) = &program.expr.node {
        assert_eq!(left.node, Expr::Number(1.0));
        assert_eq!(*op, Op::Add);
        // right should be 2 * 3
        if let Expr::Binary(rl, rop, rr) = &right.node {
            assert_eq!(rl.node, Expr::Number(2.0));
            assert_eq!(*rop, Op::Mul);
            assert_eq!(rr.node, Expr::Number(3.0));
        } else {
            panic!("Expected multiplication on the right");
        }
    } else {
        panic!("Expected binary expression");
    }
}

#[test]
fn test_parse_function_declaration() {
    let input = "function add(a, b) => a + b; add(1, 2);";
    let mut parser = Parser::new(input);
    let program = parser.parse_program().unwrap();

    assert_eq!(program.declarations.len(), 1);
    if let Declaration::Function(f) = &program.declarations[0] {
        assert_eq!(f.name, "add");
        assert_eq!(f.params.len(), 2);
    } else {
        panic!("Expected function declaration");
    }

    if let Expr::Call { func, args } = &program.expr.node {
        assert_eq!(func, "add");
        assert_eq!(args.len(), 2);
    } else {
        panic!("Expected function call");
    }
}

#[test]
fn test_parse_if_else() {
    let input = "if (true) 1 else 0;";
    let mut parser = Parser::new(input);
    let program = parser.parse_program().unwrap();

    if let Expr::If { cond, then_expr, else_expr } = &program.expr.node {
        assert!(matches!(cond.node, Expr::Boolean(true)));
        assert!(matches!(then_expr.node, Expr::Number(1.0)));
        assert!(matches!(else_expr.node, Expr::Number(0.0)));
    } else {
        panic!("Expected if expression");
    }
}
#[test]
fn test_parse_nominal_typing_basics() {
    let input = "let pt = new Point(3, 4) in print(\"x: \" @ pt.getX() @ \"; y: \" @ pt.getY());";
    let mut parser = Parser::new(input);
    let program = parser.parse_program().unwrap();

    // Verify 'let' and 'new'
    if let Expr::Let { bindings, body } = &program.expr.node {
        assert_eq!(bindings.len(), 1);
        assert_eq!(bindings[0].0, "pt");
        if let Expr::Instantiation { ty, args } = &bindings[0].2.node {
            assert_eq!(ty, "Point");
            assert_eq!(args.len(), 2);
        } else {
            panic!("Expected instantiation");
        }
        
        // Verify print with concatenation and method calls
        // body is print(...)
        if let Expr::Call { func, args: _ } = &body.node {
            assert_eq!(func, "print");
            // Check nested concat chain
        }
    } else {
        panic!("Expected let expression");
    }
}

#[test]
fn test_parse_type_with_inheritance() {
    let input = "type PolarPoint inherits Point { rho() => sqrt(self.getX() ^ 2 + self.getY() ^ 2); }";
    let mut parser = Parser::new(input);
    let program = parser.parse_program().expect("Failed to parse PolarPoint");

    assert_eq!(program.declarations.len(), 1);
    if let Declaration::Type(ty) = &program.declarations[0] {
        assert_eq!(ty.name, "PolarPoint");
        assert!(ty.parent.is_some());
        assert_eq!(ty.parent.as_ref().unwrap().name, "Point");
        assert_eq!(ty.methods.len(), 1);
        assert_eq!(ty.methods[0].name, "rho");
    } else {
        panic!("Expected type declaration");
    }
}

#[test]
fn test_parse_inheritance_with_args() {
    let input = "type PolarPoint(phi, rho) inherits Point(rho * sin(phi), rho * cos(phi)) { }";
    let mut parser = Parser::new(input);
    let program = parser.parse_program().expect("Failed to parse inheritance with args");

    if let Declaration::Type(ty) = &program.declarations[0] {
        assert_eq!(ty.params.len(), 2);
        let parent = ty.parent.as_ref().unwrap();
        assert_eq!(parent.name, "Point");
        assert_eq!(parent.args.len(), 2);
    } else {
        panic!("Expected type declaration");
    }
}

#[test]
fn test_parse_base_call() {
    let input = "type Knight inherits Person { name() => \"Sir\" @@ base(); }";
    let mut parser = Parser::new(input);
    let program = parser.parse_program().expect("Failed to parse base call");

    if let Declaration::Type(ty) = &program.declarations[0] {
        let method = &ty.methods[0];
        assert_eq!(method.name, "name");
        // body should be "Sir" @@ base()
        if let Expr::Binary(_, op, right) = &method.body.node {
            assert_eq!(*op, Op::ConcatSpace);
            assert!(matches!(right.node, Expr::BaseCall { .. }));
        }
    } else {
        panic!("Expected type declaration");
    }
}

#[test]
fn test_parse_type_with_attributes() {
    let input = "type Point { x = 0; y = 0; getX() => self.x; }";
    let mut parser = Parser::new(input);
    let program = parser.parse_program().expect("Failed to parse type with attributes");

    if let Declaration::Type(ty) = &program.declarations[0] {
        assert_eq!(ty.attributes.len(), 2);
        assert_eq!(ty.attributes[0].name, "x");
        assert_eq!(ty.methods.len(), 1);
        // getX() => self.x
        if let Expr::AttributeAccess { obj, attribute } = &ty.methods[0].body.node {
            assert!(matches!(obj.node, Expr::Identifier(ref n) if n == "self"));
            assert_eq!(attribute, "x");
        }
    }
}
#[test]
fn test_parse_lambda_simple() {
    let input = "(x) => x + 1;";
    let mut parser = Parser::new(input);
    let program = parser.parse_program().expect("Failed to parse simple lambda");

    if let Expr::Lambda { params, return_type, body } = &program.expr.node {
        assert_eq!(params.len(), 1);
        assert_eq!(params[0].name, "x");
        assert!(return_type.is_none());
        assert!(matches!(body.node, Expr::Binary(..)));
    } else {
        panic!("Expected lambda expression, found: {:?}", program.expr.node);
    }
}

#[test]
fn test_parse_lambda_with_types() {
    let input = "(x: Number): Boolean => x % 2 == 0;";
    let mut parser = Parser::new(input);
    let program = parser.parse_program().expect("Failed to parse lambda with types");

    if let Expr::Lambda { params, return_type, body: _ } = &program.expr.node {
        assert_eq!(params.len(), 1);
        assert_eq!(params[0].name, "x");
        assert!(matches!(&params[0].type_annotation, Some(TypeAnnotation::Name(n)) if n == "Number"));
        assert!(matches!(return_type, Some(TypeAnnotation::Name(n)) if n == "Boolean"));
    } else {
        panic!("Expected lambda expression");
    }
}

#[test]
fn test_parse_function_type_annotation() {
    let input = "function count_when(numbers: Number*, filter: (Number) -> Boolean) => 0;";
    let mut parser = Parser::new(input);
    let program = parser.parse_program().expect("Failed to parse function with complex type annotations");

    if let Declaration::Function(f) = &program.declarations[0] {
        assert_eq!(f.params.len(), 2);
        // numbers: Number*
        assert!(matches!(f.params[0].type_annotation, Some(TypeAnnotation::Iterable(_))));
        // filter: (Number) -> Boolean
        assert!(matches!(f.params[1].type_annotation, Some(TypeAnnotation::Function { .. })));
    } else {
        panic!("Expected function declaration");
    }
}

#[test]
fn test_parse_nested_lambdas() {
    let input = "(x) => (y) => x + y;";
    let mut parser = Parser::new(input);
    let program = parser.parse_program().expect("Failed to parse nested lambdas");

    if let Expr::Lambda { body, .. } = &program.expr.node {
        assert!(matches!(body.node, Expr::Lambda { .. }));
    } else {
        panic!("Expected nested lambda");
    }
}
#[test]
fn test_parse_math_builtins() {
    let input = "sqrt(sin(PI) + cos(E) * exp(1)) + log(2, 8) + rand();";
    let mut parser = Parser::new(input);
    let _program = parser.parse_program().expect("Failed to parse math built-ins");
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
    let _program = parser.parse_program().expect("Failed to parse integral program");
}

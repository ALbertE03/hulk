use hulk_compiler::parser::Parser;

fn main() {
    println!("--- HULK AST Visualization ---\n");

    let _input = r#"
        protocol Hashable {
            hash(): Number;
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
            p2 = new Point(3, 4)
        in {
            print("Distance: " @ p1.distance(p2));
            print("PI Value: " @ PI);
            print("Random: " @ rand());
        }
    "#;
    let a = "(-2)^2";
    let mut parser = Parser::new(a);
    match parser.parse_program() {
        Ok(program) => {
            println!("Parsed successfully!\n");
            println!("AST structure (using Display):\n");
            println!("{}", program);
            
            println!("\nDebug Representation (Internal structure):\n");
            println!("{:#?}", program);
        }
        Err(e) => {
            eprintln!("Parsing failed: {}", e);
        }
    }
}

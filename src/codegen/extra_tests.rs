use crate::codegen::CodeGenerator;
use crate::codegen::llvm_target::LlvmGenerator;
use crate::semantic::Context;
use crate::parser::Parser;
use crate::semantic::check_program;

/// Helper that runs full pipeline (parse → semantic → codegen)
fn compile(input: &str) -> String {
    let mut parser = Parser::new(input);
    let program = parser.parse_program().unwrap();
    let context = check_program(&program).expect("Semantic check failed");
    let generator = LlvmGenerator;
    generator.generate(&program, &context)
}

/// Helper that skips semantic analysis (for programs the checker may reject)
fn compile_no_sem(input: &str) -> String {
    let mut parser = Parser::new(input);
    let program = parser.parse_program().unwrap();
    let context = Context::new();
    let generator = LlvmGenerator;
    generator.generate(&program, &context)
}

// ── Let binding ─────────────────────────────────────────────────────────

#[test]
fn test_let_binding() {
    let code = compile("let x = 42 in print(x);");
    println!("{}", code);
    assert!(code.contains("alloca double"));       // Variable storage
    assert!(code.contains("store double"));         // Store value
    assert!(code.contains("load double, double*")); // Load value
}

#[test]
fn test_let_multiple_bindings() {
    let code = compile("let x = 10, y = 20 in print(x + y);");
    // Two alloca + store pairs
    let alloca_count = code.matches("alloca double").count();
    assert!(alloca_count >= 2, "Expected >=2 alloca, got {}", alloca_count);
    assert!(code.contains("fadd double"));
}

// ── If / Else ───────────────────────────────────────────────────────────

#[test]
fn test_if_else() {
    let code = compile("if (true) print(1) else print(0);");
    println!("{}", code);
    assert!(code.contains("fcmp one double"));      // Condition check
    assert!(code.contains("br i1"));                // Conditional branch
    assert!(code.contains("then_"));
    assert!(code.contains("else_"));
    assert!(code.contains("merge_"));
}

// ── While loop ──────────────────────────────────────────────────────────

#[test]
fn test_while_loop() {
    let code = compile_no_sem("while (true) print(1);");
    println!("{}", code);
    assert!(code.contains("wcond_"));
    assert!(code.contains("wbody_"));
    assert!(code.contains("wend_"));
    assert!(code.contains("br label %wcond_")); // Loop back
}

// ── Function definition & call ──────────────────────────────────────────

#[test]
fn test_function_definition() {
    let code = compile("function add(a: Number, b: Number): Number => a + b; print(add(1, 2));");
    println!("{}", code);
    assert!(code.contains("define double @add(double %a, double %b)"));
    assert!(code.contains("fadd double"));          // a + b
    assert!(code.contains("ret double"));            // Return from function
    assert!(code.contains("call double @add("));     // Call site
}

#[test]
fn test_function_params_alloca() {
    let code = compile("function f(x: Number): Number => x * x; print(f(5));");
    // Params should be stored in alloca'd slots
    assert!(code.contains("alloca double"));
    assert!(code.contains("store double %x, double*"));
    assert!(code.contains("fmul double"));
}

// ── Block expressions ───────────────────────────────────────────────────

#[test]
fn test_block_expression() {
    let code = compile("{ print(1); print(2); };");
    // Two printf calls
    let printf_count = code.matches("@printf").count();
    assert!(printf_count >= 2, "Expected >=2 printf calls, got {}", printf_count);
}

// ── Assignment ──────────────────────────────────────────────────────────

#[test]
fn test_assignment() {
    let code = compile("let x = 1 in { x := 2; print(x); };");
    // Should see store twice (init + assign)
    let store_count = code.matches("store double").count();
    assert!(store_count >= 2, "Expected >=2 stores, got {}", store_count);
}

// ── Vector literal ──────────────────────────────────────────────────────

#[test]
fn test_vector_literal() {
    let code = compile_no_sem("[1, 2, 3];");
    println!("{}", code);
    assert!(code.contains("@malloc"));              // Heap allocation
    assert!(code.contains("getelementptr double"));  // Element stores
}

// ── Indexing ────────────────────────────────────────────────────────────

#[test]
fn test_vector_indexing() {
    let code = compile_no_sem("let v = [10, 20, 30] in v[1];");
    println!("{}", code);
    assert!(code.contains("getelementptr double"));
    assert!(code.contains("load double, double*"));
}

// ── Type / Class ────────────────────────────────────────────────────────

#[test]
fn test_class_declaration() {
    let code = compile("type Point(x, y) { getX() => self.x; } print(1);");
    println!("{}", code);
    assert!(code.contains("%T.Point = type"));          // Struct decl
    assert!(code.contains("define i8* @Point_new("));   // Constructor
    assert!(code.contains("define double @Point_getX(i8* %self")); // Method
    assert!(code.contains("@malloc"));                  // Heap alloc in ctor
}

#[test]
fn test_instantiation() {
    let code = compile("type Point(x, y) {} let p = new Point(1, 2) in print(1);");
    println!("{}", code);
    assert!(code.contains("call i8* @Point_new("));
    assert!(code.contains("ptrtoint i8*"));  // Encode ptr as double
}

#[test]
fn test_method_call() {
    let code = compile("type Point(x, y) { getX() => self.x; } let p = new Point(3, 4) in print(p.getX());");
    println!("{}", code);
    assert!(code.contains("call double @Point_getX(i8*"));
}

// ── Concat operators ────────────────────────────────────────────────────

#[test]
fn test_concat_string_ops() {
    let code = compile_no_sem("let a = \"foo\" @ \"bar\" in print(a);");
    assert!(code.contains("@strlen"));
    assert!(code.contains("@strcpy"));
    assert!(code.contains("@strcat"));
}

// ── Match expression ────────────────────────────────────────────────────

#[test]
fn test_match_expression() {
    let code = compile("let x = 1 in match(x) { case 1 => print(10); case 2 => print(20); default => print(0); };");
    println!("{}", code);
    assert!(code.contains("fcmp oeq double"));  // Literal comparison
    assert!(code.contains("br i1"));            // Branches
    assert!(code.contains("mend_"));            // Merge at end
}

// ── Lambda ──────────────────────────────────────────────────────────────

#[test]
fn test_lambda_definition() {
    let code = compile_no_sem("let f = (x) => x + 1 in print(1);");
    println!("{}", code);
    assert!(code.contains("define double @__lambda_"));
    assert!(code.contains("fadd double"));
    assert!(code.contains("ret double"));
}

// ── Combined / integration ──────────────────────────────────────────────

#[test]
fn test_full_program_with_function_and_print() {
    let code = compile("function double(x: Number): Number => x * 2; print(double(21));");
    println!("{}", code);
    assert!(code.contains("define double @double(double %x)"));
    assert!(code.contains("define i32 @main()"));
    assert!(code.contains("call double @double("));
    assert!(code.contains("ret i32 0"));
}

#[test]
fn test_comparison_produces_uitofp() {
    let code = compile("print(5 <= 10);");
    assert!(code.contains("fcmp ole"));
    assert!(code.contains("uitofp i1"));
}

#[test]
fn test_comparison_ge() {
    let code = compile("print(10 >= 5);");
    assert!(code.contains("fcmp oge"));
}


// ── 1. Deep inheritance: parent attrs prepended to child struct ─────────

#[test]
fn test_deep_inheritance_parent_attrs() {
    let code = compile_no_sem(
        "type Animal(name: String) { name = name; }
         type Dog(name: String, breed: String) inherits Animal(name) { breed = breed; }
         let d = new Dog(\"Rex\", \"Lab\") in print(42);"
    );
    println!("{}", code);
    // Parent struct emitted
    assert!(code.contains("%T.Animal = type"));
    // Child struct should have more fields (inherits parent's)
    assert!(code.contains("%T.Dog = type"));
    // Constructor of Dog calls Animal_new to init parent attrs
    assert!(code.contains("call i8* @Animal_new("));
}

#[test]
fn test_three_level_inheritance() {
    let code = compile_no_sem(
        "type A(x: Number) { x = x; }
         type B(x: Number, y: Number) inherits A(x) { y = y; }
         type C(x: Number, y: Number, z: Number) inherits B(x, y) { z = z; }
         let c = new C(1, 2, 3) in print(42);"
    );
    println!("{}", code);
    // All three types should exist
    assert!(code.contains("%T.A = type"));
    assert!(code.contains("%T.B = type"));
    assert!(code.contains("%T.C = type"));
    // C's constructor calls B_new, B's calls A_new
    assert!(code.contains("call i8* @B_new("));
    assert!(code.contains("call i8* @A_new("));
}

// ── 2. base() calls ────────────────────────────────────────────────────

#[test]
fn test_base_call_emits_parent_constructor() {
    let code = compile_no_sem(
        "type Animal(sound: String) { sound = sound; }
         type Dog(sound: String) inherits Animal(sound) {
             bark() => base(sound);
         }
         let d = new Dog(\"woof\") in print(42);"
    );
    println!("{}", code);
    // base() in method should call parent constructor
    assert!(code.contains("@Animal_new("));
}

// ── 3. is / as runtime type checks ─────────────────────────────────────

#[test]
fn test_is_operator_type_id_check() {
    let code = compile_no_sem(
        "type Animal(x: Number) { x = x; }
         let a = new Animal(1) in print(a is Animal);"
    );
    println!("{}", code);
    // Should store a type-id at slot 0
    assert!(code.contains("store i64"));
    // Should load and compare type-id for `is`
    assert!(code.contains("icmp eq i64"));
}

#[test]
fn test_as_operator_emits_cast_check() {
    let code = compile_no_sem(
        "type Animal(x: Number) { x = x; }
         let a = new Animal(1) in (a as Animal);"
    );
    println!("{}", code);
    // Should emit abort path for invalid cast
    assert!(code.contains("call void @abort()"));
    assert!(code.contains("unreachable"));
}

#[test]
fn test_is_with_inheritance() {
    let code = compile_no_sem(
        "type Animal(x: Number) { x = x; }
         type Dog(x: Number) inherits Animal(x) { }
         let d = new Dog(1) in print(d is Animal);"
    );
    println!("{}", code);
    // Should check multiple type ids (Dog is also Animal)
    let icmp_count = code.matches("icmp eq i64").count();
    assert!(icmp_count >= 2, "Expected >=2 type-id comparisons for inheritance `is`, got {}", icmp_count);
}

// ── 4. rand() via libc ─────────────────────────────────────────────────

#[test]
fn test_rand_calls_libc() {
    let code = compile("print(rand());");
    println!("{}", code);
    // Should declare and call rand
    assert!(code.contains("declare i32 @rand()"));
    assert!(code.contains("call i32 @rand()"));
    // Should convert to double
    assert!(code.contains("sitofp i32"));
    // Should divide by RAND_MAX
    assert!(code.contains("fdiv double"));
    assert!(code.contains("2.147483647e9") || code.contains("2.147483647e+9") || code.contains("2147483647"));
}

#[test]
fn test_rand_seeds_once() {
    let code = compile("print(rand());");
    // Should have lazy seeding via @.rand_seeded global
    assert!(code.contains("@.rand_seeded"));
    assert!(code.contains("call void @srand("));
    assert!(code.contains("call i64 @time("));
}

// ── 5. Closures / lambda capture ────────────────────────────────────────

#[test]
fn test_lambda_captures_free_variable() {
    let code = compile_no_sem(
        "let x = 10 in
            let f = (y: Number) => x + y in
                print(42);"
    );
    println!("{}", code);
    // Lambda should be emitted as a separate function
    assert!(code.contains("define double @__lambda_"));
    // Closure should allocate environment
    assert!(code.contains("@malloc(i64 16)")); // 2 doubles for closure pair
}

#[test]
fn test_lambda_without_captures() {
    let code = compile_no_sem(
        "let f = (x: Number) => x * 2 in print(42);"
    );
    println!("{}", code);
    // Lambda still emitted as function
    assert!(code.contains("define double @__lambda_"));
    // Environment should be null (0.0) since no captures
    // The closure pair is still allocated but env is 0.0
    assert!(code.contains("store double 0.0"));
}

// ── 6. Bounds checking for vectors ──────────────────────────────────────

#[test]
fn test_vector_indexing_bounds_check() {
    let code = compile_no_sem(
        "let v = [1, 2, 3] in print(v[0]);"
    );
    println!("{}", code);
    // Should check lower bound (index < 0)
    assert!(code.contains("icmp slt i64"));
    // Should check upper bound (index >= len)
    assert!(code.contains("icmp sge i64"));
    // Should abort on out of bounds
    assert!(code.contains("call void @abort()"));
    assert!(code.contains("@.oob_msg"));
}

// ── 7. GC tracking ─────────────────────────────────────────────────────

#[test]
fn test_gc_track_called_on_malloc() {
    let code = compile_no_sem(
        "type Foo(x: Number) { x = x; }
         let f = new Foo(1) in print(42);"
    );
    println!("{}", code);
    // Every malloc should be tracked
    assert!(code.contains("call void @__hulk_gc_track(i8*"));
}

#[test]
fn test_gc_sweep_at_exit() {
    let code = compile("print(42);");
    println!("{}", code);
    // Should call sweep in main before ret
    assert!(code.contains("call void @__hulk_gc_sweep()"));
    // Should define the sweep function
    assert!(code.contains("define void @__hulk_gc_sweep()"));
}

#[test]
fn test_gc_sweep_frees_all() {
    let code = compile("print(42);");
    // The sweep function should iterate and call free
    assert!(code.contains("define void @__hulk_gc_sweep()"));
    assert!(code.contains("call void @free(i8*"));
}

#[test]
fn test_gc_global_tracking_buffer() {
    let code = compile("print(42);");
    // Growable GC buffer globals
    assert!(code.contains("@.gc_buf"));
    assert!(code.contains("@.gc_len"));
    assert!(code.contains("@.gc_cap"));
}

// ── Type-id in objects ──────────────────────────────────────────────────

#[test]
fn test_type_id_stored_in_slot_zero() {
    let code = compile_no_sem(
        "type Box(val: Number) { val = val; }
         let b = new Box(42) in print(42);"
    );
    println!("{}", code);
    // Struct should start with i64 (type-id)
    assert!(code.contains("%T.Box = type { i64"));
    // Constructor should store type-id at getelementptr ... i32 0, i32 0
    assert!(code.contains("store i64"));
}

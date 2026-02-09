# HULK Compiler

Compilador completo para el lenguaje de programación **HULK**, implementado en Rust. Genera **LLVM IR** que puede compilarse a código nativo para cualquier arquitectura soportada (x86, ARM, RISC-V, etc.). El proyecto cubre todas las fases clásicas de un compilador moderno: análisis léxico, análisis sintáctico, construcción del AST, expansión de macros, análisis semántico, optimización y generación de código LLVM IR.


---

## Fases del Compilador

### 1. **Árbol de Sintaxis Abstracta (AST)**
📂 [`src/ast/`](src/ast/) | 📖 [Documentación detallada](src/ast/README.md)

El AST es la representación intermedia del programa. Define todas las estructuras de datos (`Expr`, `Declaration`, `Program`) que representan el código de forma jerárquica y procesable. Incluye el optimizador multi-pasada en `optimize.rs`.

---

### 2. **Análisis Léxico (Lexer)**
📂 [`src/lexer/`](src/lexer/) | 📖 [Documentación detallada](src/lexer/README.md)

El lexer transforma el código fuente (texto plano) en una secuencia de **tokens** con significado. Implementado como iterador lazy con O(n) single-pass y lookahead de 1 carácter.

---

### 3. **Análisis Sintáctico (Parser)**
📂 [`src/parser/`](src/parser/) | 📖 [Documentación detallada](src/parser/README.md)

El parser recibe la secuencia de tokens del lexer y construye un **Árbol de Sintaxis Abstracta (AST)** que representa la estructura jerárquica del programa. Usa Pratt parsing para expresiones con precedencia.

---

### 4. **Expansión de Macros**
📂 [`src/macros/`](src/macros/) | 📖 [Documentación detallada](src/macros/README.md)

La fase de expansión de macros transpila construcciones de macros a código HULK estándar en tiempo de compilación. Las macros son **metaprogramación** que permite extender el lenguaje con nuevas construcciones sintácticas. Soporta parámetros simbólicos (`@`), placeholders (`$`), body arguments (`*`) y pattern matching sobre el AST.

---

### 5. **Análisis Semántico**
📂 [`src/semantic/`](src/semantic/) | 📖 [Documentación detallada](src/semantic/README.md)

El análisis semántico verifica que el programa sea correcto más allá de la sintaxis: resolución de tipos, validación de herencia (4 pasadas), verificación de conformidad de tipos y manejo de scopes.

---

### 6. **Optimización**
📂 [`src/ast/optimize.rs`](src/ast/optimize.rs)

La fase de optimización mejora el código sin cambiar su semántica, aplicando: constant folding, constant propagation (con detección de variables mutables vía `:=`), dead code elimination, simplificación algebraica, cortocircuito booleano y string interning.

---

### 7. **Generación de Código (Codegen) — LLVM IR**
📂 [`src/codegen/`](src/codegen/) | 📖 [Documentación detallada](src/codegen/README.md)

Backend completo que genera LLVM IR. Usa `double` como tipo universal, patrón alloca/store/load para variables, `malloc` para heap, y un GC por barrido. Soporta: clases con herencia profunda, closures con captura de variables libres, `is`/`as` en runtime, `base()`, `rand()`, vectores con bounds checking y más.

---

## Pipeline del Compilador

```
Código fuente (.hulk)
    ↓
1. Lexer → Tokens (src/lexer/)
    ↓
2. Parser → AST (src/parser/)
    ↓
3. Macro Expansion → AST expandido sin macros (src/macros/)
    ↓
4. Semantic Analyzer → Contexto de tipos verificado (src/semantic/)
    ↓
5. Optimizer → AST optimizado (src/ast/optimize.rs)
    ↓
6. LLVM IR Codegen → output.ll (src/codegen/llvm_target.rs)
    ↓
7. clang output.ll -o programa -lm → Ejecutable nativo
```

---

## Uso

### Compilar el proyecto

```bash
cargo build
```

### Ejecutar tests (169 tests)

```bash
cargo test
```

### 🌐 Playground Web Interactivo

Ejecuta el playground web con editor Monaco, compilación en tiempo real y documentación completa:

```bash
cargo run --bin hulk-playground
```

Abre tu navegador en [http://localhost:8080](http://localhost:8080)

**Características:**
- ✨ Editor Monaco con syntax highlighting para HULK
- 🚀 Compilación y ejecución en tiempo real
- 📚 Documentación completa integrada
- 🎯 9 ejemplos interactivos (funciones, tipos, iterables, etc.)
- 🔍 Visualización de LLVM IR generado
- ⌨️ Shortcuts: `⌘ Enter` para ejecutar

### Compilar un programa HULK a ejecutable nativo

```bash
# Desde archivo
cargo run -- mi_programa.hulk > output.ll
clang output.ll -o programa -lm
./programa

# Desde stdin
echo 'print(42);' | cargo run > output.ll
clang output.ll -o programa -lm
./programa
```

### Interpretar LLVM IR directamente (sin compilar a nativo)

```bash
echo 'print("Hello HULK!");' | cargo run > output.ll
lli output.ll
```

El punto de entrada está en [`src/main.rs`](src/main.rs), que ejecuta todo el pipeline y escribe el LLVM IR resultante en `output.ll`.

---

## Características del Lenguaje HULK

| Característica | Ejemplo | Soporte |
|---|---|---|
| Tipos numéricos (f64) | `let x = 42;` | ✅ |
| Strings y concatenación | `"hello" @ " world"` | ✅ |
| Booleanos | `true & false` | ✅ |
| If/Else | `if (x > 0) "pos" else "neg"` | ✅ |
| While loops | `while (x > 0) { ... }` | ✅ |
| For loops | `for (x in [1,2,3]) print(x);` | ✅ |
| Let bindings | `let x = 5 in x + 1` | ✅ |
| Funciones | `function f(x) => x * 2;` | ✅ |
| Lambdas / Closures | `let f = (x) => x + 1;` | ✅ |
| Clases (OOP) | `type Point(x, y) { ... }` | ✅ |
| Herencia profunda | `type C inherits B inherits A` | ✅ |
| `base()` calls | `base(args)` en constructores hijos | ✅ |
| `is` / `as` operators | `obj is Point`, `obj as Point` | ✅ |
| Protocolos | `protocol Printable { ... }` | ✅ |
| Vectores | `[1, 2, 3]`, `v[i]` | ✅ |
| Generadores de vectores | `[x*2 \|\| x in list]` | ✅ |
| Bounds checking | Error en runtime si índice fuera de rango | ✅ |
| Match expressions | `match(x) { case 1 => ... }` | ✅ |
| Macros | `def repeat(n, *body) => ...` | ✅ |
| Pattern matching (macros) | `match(expr) { case (x + 0) => x }` | ✅ |
| Math builtins | `sqrt`, `sin`, `cos`, `exp`, `log`, `PI`, `E` | ✅ |
| `rand()` | Número aleatorio en [0, 1) | ✅ |
| GC (garbage collection) | Barrido automático al final del programa | ✅ |

---

## 📁 Estructura del Proyecto

```
hulk-compiler/
├── src/
│   ├── main.rs              # Punto de entrada (CLI)
│   ├── lib.rs               # Biblioteca principal (8 módulos públicos)
│   ├── ast/                 # Fase 1: Definición del AST
│   │   ├── mod.rs           # Re-exports
│   │   ├── nodes.rs         # Todos los nodos: Expr, Declaration, Program...
│   │   ├── display.rs       # Pretty-printing del AST
│   │   ├── optimize.rs      # Optimizador multi-pasada 
│   │   └── README.md
│   ├── lexer/               # Fase 2: Análisis léxico
│   │   ├── mod.rs           # Lexer principal 
│   │   ├── tokens.rs        # Definición de Token y TokenKind
│   │   ├── tests.rs         # Tests del lexer
│   │   └── README.md
│   ├── parser/              # Fase 3: Análisis sintáctico
│   │   ├── mod.rs           # Parser Pratt 
│   │   ├── helpers.rs       # Funciones auxiliares para parsing 
│   │   ├── tests.rs         # Tests del parser
│   │   └── README.md
│   ├── macros/              # Fase 4: Expansión de macros
│   │   ├── mod.rs           # Orquestador principal 
│   │   ├── utils.rs         # Utilidades (gensym) 
│   │   ├── visitors.rs      # Visitors para expansión 
│   │   ├── context.rs       # Contexto de expansión 
│   │   ├── tests.rs         # Tests de macros
│   │   └── README.md
│   ├── semantic/            # Fase 5: Análisis semántico
│   │   ├── mod.rs           # Orquestador principal 
│   │   ├── context.rs       # Contexto semántico 
│   │   ├── visitor.rs       # Visitor de análisis semántico 
│   │   ├── types.rs         # Sistema de tipos
│   │   ├── tests.rs         # Tests semánticos
│   │   └── README.md
│   ├── codegen/             # Fase 6: Generación de código LLVM IR
│   │   ├── mod.rs           # Trait CodeGenerator + re-exports
│   │   ├── llvm_target.rs   # Backend LLVM IR 
│   │   ├── tests.rs         # Tests básicos 
│   │   ├── extra_tests.rs   # Tests avanzados 
│   │   └── README.md
│   ├── errors/              # Manejo de errores
│   │   ├── mod.rs
│   │   ├── lexer.rs
│   │   └── parser.rs
│   └── utils/               # Utilidades
│       └── mod.rs
├── Cargo.toml
└── README.md
```

---

## 🔧 Tecnologías

- **Lenguaje**: Rust 🦀
- **Backend**: LLVM IR (compilable con `clang` o `llc`)
- **Runtime**: Funciones C estándar (`printf`, `malloc`, `strlen`, `strcat`, etc.)
- **GC**: Recolector de basura por barrido al finalizar el programa
- **Paradigma**: Funcional + Orientado a Objetos
- **Algoritmos**:
  - Lexer basado en iteradores con lookahead
  - Parser descendente recursivo con Pratt parsing
  - Optimización multi-pasada sobre el AST (con detección de mutables)
  - Codegen tree-walking sobre AST → LLVM IR textual

---

## 🧑‍💻 Guía del Desarrollador: Cómo Agregar Nuevas Funcionalidades

Esta sección describe paso a paso cómo extender el compilador HULK con nuevas características del lenguaje.

### Flujo General

Para agregar cualquier funcionalidad nueva, se deben tocar (en orden):

1. **AST** (`src/ast/nodes.rs`) — Definir la nueva estructura en el árbol
2. **Display Visitor** (`src/ast/display.rs`) — Implementar `Display` para el nuevo nodo (visitor pattern)
3. **Lexer** (`src/lexer/`) — Si hay nueva sintaxis (tokens nuevos)
4. **Parser** (`src/parser/mod.rs`, `helpers.rs`) — Parsear la sintaxis al AST
5. **Semantic Visitor** (`src/semantic/visitor.rs`) — Agregar caso en `BodyChecker::check_expr()` o `check_type()`
6. **Types** (`src/semantic/types.rs`) — Si defines un nuevo tipo
7. **Macro Visitors** (`src/macros/visitors.rs`) — Si las macros necesitan procesar el nuevo nodo (SubstitutionVisitor, SanitizationVisitor, MacroExpansionVisitor)
8. **Optimizer Visitor** (`src/ast/optimize.rs`) — Agregar caso en `optimize_expr()` (si aplica)
9. **Codegen** (`src/codegen/llvm_target.rs`) — Agregar caso en `emit_expr()` o la función correspondiente
10. **Tests** — En cada módulo afectado

### Ejemplo 1: Agregar un Nuevo Operador Binario

Supongamos que quieres agregar el operador `<<` (shift left).

#### Paso 1: AST — Agregar variante al enum `Op`

En `src/ast/nodes.rs`:
```rust
pub enum Op {
    Add, Sub, Mul, Div, Mod, Pow,
    // ... existentes
    Shl, // ← NUEVO
}
```

#### Paso 2: Display Visitor — Mostrar el operador

En `src/ast/display.rs`, agregar el caso en el `Display` de `Op` (este archivo implementa el visitor pattern para pretty-printing):
```rust
Op::Shl => write!(f, "<<"),
```

#### Paso 3: Lexer — Reconocer `<<`

En `src/lexer/tokens.rs`, agregar el token:
```rust
pub enum TokenKind {
    // ...
    Shl, // <<
}
```

En `src/lexer/mod.rs`, en la función que lee operadores:
```rust
'<' if self.peek() == Some('<') => {
    self.advance();
    Token::new(TokenKind::Shl, ...)
}
```

#### Paso 4: Parser — Parsear con precedencia

En `src/parser/mod.rs`, en la tabla de precedencias (Pratt parsing):
```rust
TokenKind::Shl => Some((Precedence::Shift, Op::Shl)),
```

#### Paso 5: Semantic Visitor — Validar tipos

En `src/semantic/visitor.rs`, en el método `BodyChecker::check_expr()`, dentro del match de `Expr::Binary`, agregar el caso:
```rust
Op::Shl => {
    self.expect_type(&left_type, &Type::number());
    self.expect_type(&right_type, &Type::number());
    Ok(Type::number())
}
```

#### Paso 6: Optimizer Visitor (opcional)

En `src/ast/optimize.rs`, en la función `optimize_expr()` (que implementa visitor pattern), agregar constant folding:
```rust
(Expr::Number(a), Op::Shl, Expr::Number(b)) => {
    Expr::Number(((a as i64) << (b as i64)) as f64)
}
```

#### Paso 7: Codegen — Generar LLVM IR

En `src/codegen/llvm_target.rs`, en `emit_expr` → `Expr::Binary`:
```rust
Op::Shl => {
    let li = self.next_reg();
    let ri = self.next_reg();
    let res = self.next_reg();
    writeln!(out, "  {} = fptosi double {} to i64", li, lv);
    writeln!(out, "  {} = fptosi double {} to i64", ri, rv);
    writeln!(out, "  {} = shl i64 {}, {}", res, li, ri);
    let fd = self.next_reg();
    writeln!(out, "  {} = sitofp i64 {} to double", fd, res);
    fd
}
```

#### Paso 8: Tests

Agregar tests en `src/codegen/tests.rs` o `extra_tests.rs`:
```rust
#[test]
fn test_shl_operator() {
    let code = "print(2 << 3);"; // Debería imprimir 16
    let ir = compile(code);
    assert!(ir.contains("shl i64"));
}
```

---

### Ejemplo 2: Agregar una Nueva Función Builtin

Supongamos que quieres agregar `abs(x)` (valor absoluto).

#### Paso 1: AST

En `src/ast/nodes.rs`, agregar variante a `Expr`:
```rust
pub enum Expr {
    // ...
    Abs(Box<Spanned<Expr>>), // ← NUEVO
}
```

#### Paso 2: Display Visitor

En `src/ast/display.rs`, agregar caso al match de `Expr`:
```rust
Expr::Abs(x) => write!(f, "abs({})", x.node),
```

#### Paso 3: Parser

En `src/parser/mod.rs`, donde se parsean llamadas a funciones:
```rust
"abs" => {
    let arg = self.parse_expr()?;
    self.expect(TokenKind::RParen)?;
    Expr::Abs(Box::new(arg))
}
```

#### Paso 4: Semantic Visitor

En `src/semantic/visitor.rs`, en `BodyChecker::check_expr()`, agregar caso:
```rust
Expr::Abs(x) => {
    let t = self.check_expr(x)?;
    self.expect_type(&t, &Type::number());
    Ok(Type::number())
}
```

#### Paso 5: Optimizer Visitor

En `src/ast/optimize.rs`, en `optimize_expr()`, agregar caso:
```rust
Expr::Abs(x) => {
    let opt_x = optimize_expr(*x, interner, env);
    if let Expr::Number(n) = opt_x.node {
        Expr::Number(n.abs()) // Constant folding
    } else {
        Expr::Abs(Box::new(opt_x))
    }
}
```

#### Paso 6: Codegen Visitor

En `src/codegen/llvm_target.rs`, en `LlvmGenerator::emit_expr()`, agregar caso:
```rust
Expr::Abs(x) => {
    let xv = self.emit_expr(x, out, ctx);
    let r = self.next_reg();
    writeln!(out, "  {} = call double @llvm.fabs.f64(double {})", r, xv);
    r
}
```

No olvidar declarar el intrínseco al inicio del IR:
```rust
declarations.push("declare double @llvm.fabs.f64(double)");
```

---

### Ejemplo 3: Agregar un Nuevo Tipo de Declaración

Supongamos que quieres agregar `enum`.

1. **AST**: Crear `EnumDecl` en `src/ast/nodes.rs`, agregar `Declaration::Enum(EnumDecl)`
2. **Parser**: Parsear `enum Color { Red, Green, Blue }` en `src/parser/mod.rs` → función `parse_declaration()`
3. **Semántico**: 
   - En `src/semantic/mod.rs` → Pasada 1: Registrar el enum como tipo en el `Context`
   - En `src/semantic/types.rs` → Agregar variante `Enum` al `TypeKind` si es necesario
   - En `src/semantic/visitor.rs` → Validar uso de variantes del enum
4. **Codegen**: En `src/codegen/llvm_target.rs` → Asignar un `double` numérico a cada variante (0.0, 1.0, 2.0...)
5. **Tests**: Verificar declaración, uso en match, comparación

---

### Ejemplo 4: Agregar una Nueva Optimización

Para agregar una nueva pasada de optimización (ej: *strength reduction*):

1. En `src/ast/optimize.rs`, agregar un nuevo caso en `optimize_expr()`:
```rust
// Strength reduction: x * 2 → x + x
Expr::Binary(left, Op::Mul, right) => {
    if let Expr::Number(2.0) = opt_right.node {
        return Expr::Binary(
            Box::new(opt_left.clone()),
            Op::Add,
            Box::new(opt_left),
        );
    }
    // ... existing cases
}
```

2. **IMPORTANTE**: Si la optimización afecta variables que podrían ser mutadas con `:=`, usar `collect_assigned_vars()` (también en `optimize.rs`) para obtener el conjunto de variables mutables y excluirlas de la transformación:
```rust
let mut mutated = HashSet::new();
collect_assigned_vars(&body.node, &mut mutated);
// No optimizar variables en `mutated` - son mutables
```

---

### Ejemplo 5: Extender el Sistema de Macros

Supongamos que quieres agregar un nuevo tipo de parámetro de macro (ej: `&` para referencias).

1. **AST**: En `src/ast/nodes.rs`, agregar variante a `MacroParam`:
```rust
pub enum MacroParam {
    Normal { name: String, type_annotation: TypeAnnotation },
    Symbolic { name: String, type_annotation: TypeAnnotation },  // @
    Placeholder { name: String, type_annotation: TypeAnnotation }, // $
    Body { name: String, type_annotation: TypeAnnotation },       // *
    Reference { name: String, type_annotation: TypeAnnotation },  // & ← NUEVO
}
```

2. **Parser**: En `src/parser/mod.rs`, en `parse_macro_param()`, reconocer el prefijo `&`

3. **Expansión**: En `src/macros/context.rs`:
   - Modificar `expand_macro_call()` para manejar parámetros de tipo `Reference`
   - Actualizar la lógica de sustitución si es necesario

4. **Visitors**: En `src/macros/visitors.rs`, actualizar los visitors si necesitan comportamiento especial para referencias

5. **Tests**: En `src/macros/tests.rs`, agregar casos que usen el nuevo tipo de parámetro

---

### Checklist para Nuevas Funcionalidades

Al agregar una nueva característica al lenguaje, asegúrate de tocar todos los **visitors** necesarios:

- [ ] **AST**: Definir nodo en `ast/nodes.rs`
- [ ] **Display Visitor**: Implementar en `ast/display.rs` → agregar caso al match correspondiente
- [ ] **Lexer**: Agregar token(s) en `lexer/tokens.rs` (si hay nueva sintaxis)
- [ ] **Parser**: Parsear en `parser/mod.rs` (usar `helpers.rs` si necesitas utilidades)
- [ ] **Semantic Visitor**: Agregar caso en `semantic/visitor.rs` → `BodyChecker::check_expr()` o método correspondiente
- [ ] **Types**: Actualizar `semantic/types.rs` si defines un nuevo tipo
- [ ] **Macro Visitors**: Actualizar `macros/visitors.rs` si las macros deben procesar el nuevo nodo:
  - `SubstitutionVisitor`: Para sustituciones de variables/expresiones
  - `SanitizationVisitor`: Para renombrado higiénico
  - `MacroExpansionVisitor`: Para expansión recursiva
- [ ] **Optimizer Visitor**: Agregar caso en `ast/optimize.rs` → `optimize_expr()` (si aplica)
- [ ] **Codegen Visitor**: Agregar caso en `codegen/llvm_target.rs` → `emit_expr()` o función correspondiente
- [ ] **Tests**: Agregar tests en cada módulo afectado
- [ ] **Verificación**: Ejecutar `cargo test` para confirmar que todos los tests pasan

> **💡 Nota sobre Visitors**: El compilador usa el patrón **Visitor** extensivamente. Casi cada fase (display, semantic, macros, optimizer, codegen) implementa un visitor que recorre el AST. Cuando agregas un nuevo nodo al AST, debes actualizar **todos** los visitors relevantes para que sepan cómo procesarlo.

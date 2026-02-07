# HULK Compiler

Compilador completo para el lenguaje de programación HULK, implementado en Rust. Este proyecto cubre todas las fases clásicas de un compilador moderno: análisis léxico, análisis sintáctico, construcción del AST, expansión de macros, análisis semántico, optimización y generación de código.

## Fases del Compilador

### 1. **Árbol de Sintaxis Abstracta (AST)**
📂 [`src/ast/`](src/ast/) | 📖 [Documentación detallada](src/ast/README.md)

El AST es la representación intermedia del programa. Define todas las estructuras de datos que representan el código de forma jerárquica y procesable.

---

### 2. **Análisis Léxico (Lexer)**
📂 [`src/lexer/`](src/lexer/) | 📖 [Documentación detallada](src/lexer/README.md)

El lexer transforma el código fuente (texto plano) en una secuencia de **tokens** con significado.

---

### 3. **Análisis Sintáctico (Parser)**
📂 [`src/parser/`](src/parser/) | 📖 [Documentación detallada](src/parser/README.md)

El parser recibe la secuencia de tokens del lexer y construye un **Árbol de Sintaxis Abstracta (AST)** que representa la estructura jerárquica del programa.

---

### 4. **Expansión de Macros**
📂 [`src/macros/`](src/macros/) | 📖 [Documentación detallada](src/macros/README.md)

La fase de expansión de macros transpila construcciones de macros a código HULK estándar en tiempo de compilación. Las macros son **metaprogramación** que permite extender el lenguaje con nuevas construcciones sintácticas.

---

### 5. **Optimización**
📂 [`src/ast/optimize.rs`](src/ast/optimize.rs)

La fase de optimización mejora el código sin cambiar su semántica, aplicando transformaciones que reducen complejidad y mejoran rendimiento.

**Optimizaciones implementadas:**

#### 5.1. Constant Folding (Plegado de Constantes)
Evalúa operaciones con valores constantes en tiempo de compilación:
```hulk
2 + 3 * 4      →  14
sqrt(16)       →  4
5 > 3          →  true
"hola" @ " mundo"  →  "hola mundo"
```

#### 5.2. Constant Propagation (Propagación de Constantes)
Sustituye variables con valores constantes conocidos:
```hulk
let x = 5 in x + x        →  let x = 5 in 10
let x = 8, y = x + 2 in y →  let x = 8, y = 10 in 10
```

#### 5.3. Dead Code Elimination (Eliminación de Código Muerto)
Elimina código que nunca se ejecutará:
```hulk
if (true) 10 else 20   →  10
if (false) x else y    →  y
while (false) { ... }  →  { }
```

#### 5.4. Algebraic Simplification (Simplificación Algebraica)
Aplica identidades matemáticas:
```hulk
x + 0    →  x
x * 1    →  x
x * 0    →  0
x - 0    →  x
0 / x    →  0
```

#### 5.5. Boolean Short-Circuit (Cortocircuito Booleano)
Optimiza operaciones lógicas con constantes:
```hulk
true | x   →  true
false & x  →  false
!true      →  false
```

#### 5.6. String Interning (Internado de Cadenas)
Deduplica cadenas idénticas en memoria usando un `HashMap`, reduciendo el uso de memoria.

**Uso:**
```rust
use hulk_compiler::ast::optimize::optimize_program;

let optimized_ast = optimize_program(original_ast);
```

---

### 6. **Análisis Semántico**
📂 [`src/semantic/`](src/semantic/)

El análisis semántico verifica que el programa sea correcto más allá de la sintaxis:

**Funcionalidades:**
- **Scope (Alcance)**: Manejo de ámbitos de variables con `ScopeManager`
- **Tipos**: Sistema de tipos con verificación de compatibilidad
- Verificación de:
  - Variables declaradas antes de uso
  - Tipos compatibles en operaciones
  - Firmas de funciones correctas
  - Implementación correcta de protocolos

**Módulos:**
- `scope.rs`: Gestión de ámbitos anidados (stack de entornos)
- `types.rs`: Sistema de tipos, inferencia y verificación

---

### 7. **Generación de Código (Codegen)**
📂 [`src/codegen/`](src/codegen/)

---

## Pipeline del Compilador

El flujo completo de compilación es:

```
Código fuente (String)
    ↓
1. Lexer → Tokens
    ↓
2. Parser → AST (Program)
    ↓
3. Macro Expansion → AST expandido (sin macros)
    ↓
4. Semantic Analyzer → AST anotado con tipos
    ↓
5. Optimizer → AST optimizado
    ↓
6. Codegen → Código ejecutable
```

---

## Uso

```bash
# Compilar el proyecto
cargo build

# Ejecutar el compilador
cargo run

# Ejecutar tests
cargo test
```

El punto de entrada está en [`src/main.rs`](src/main.rs), que muestra el AST antes y después de optimización.

---

## 📁 Estructura del Proyecto

```
hulk-compiler/
├── src/
│   ├── main.rs           # Punto de entrada
│   ├── lib.rs            # Biblioteca principal
│   ├── ast/              # Fase 1: Definición del AST
│   │   ├── mod.rs
│   │   ├── nodes.rs
│   │   ├── display.rs
│   │   ├── optimize.rs   # Fase 4: Optimización
│   │   └── README.md
│   ├── lexer/            # Fase 2: Análisis léxico
│   │   ├── mod.rs
│   │   ├── tokens.rs
│   │   ├── tests.rs
│   │   └── README.md
│   ├── parser/           # Fase 3: Análisis sintáctico
│   │   ├── mod.rs
│   │   ├── tests.rs
│   │   └── README.md
│   ├── semantic/         # Fase 5: Análisis semántico
│   │   ├── mod.rs
│   │   ├── scope.rs
│   │   └── types.rs
│   ├── codegen/          # Fase 6: Generación de código
│   │   └── mod.rs
│   ├── errors/           # Manejo de errores
│   │   ├── mod.rs
│   │   ├── lexer.rs
│   │   └── parser.rs
│   └── utils/            # Utilidades
│       └── mod.rs
├── Cargo.toml
└── README.md
```

---

## 🔧 Tecnologías

- **Lenguaje**: Rust 🦀
- **Paradigma**: Funcional + Orientado a Objetos
- **Algoritmos**:
  - Lexer basado en iteradores con lookahead
  - Parser descendente recursivo con Pratt parsing
  - Optimización multi-pasada sobre el AST
  - Análisis de flujo de datos para constant propagation

---

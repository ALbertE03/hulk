# Documentación del Árbol de Sintaxis Abstracta (AST)

Este módulo define la estructura del Árbol de Sintaxis Abstracta (AST) para el compilador del lenguaje Hulk.

## Índice
- [Estructura del Programa](#estructura-del-programa)
- [Declaraciones](#declaraciones-declaration)
- [Expresiones](#expresiones-expr)
- [Sistema de Posicionamiento](#sistema-de-posicionamiento-spanned)
- [Módulo Display](#módulo-display)
- [Módulo Optimize](#módulo-optimize)
- [Ejemplos Completos](#ejemplos-completos)

## Visión General

El AST es la **representación intermedia** del compilador. Después de que el lexer produce tokens y el parser verifica la sintaxis, el resultado es un árbol de nodos que representa la estructura semántica del programa.

**Características:**
- ✅ **Inmutable**: El AST no se modifica después de ser creado (las optimizaciones crean nuevos nodos)
- ✅ **Tipado fuerte**: Cada nodo tiene un tipo específico en Rust
- ✅ **Posicionamiento**: Cada nodo conoce su ubicación en el código fuente
- ✅ **Recursivo**: Expresiones pueden contener otras expresiones (árboles anidados)

**Módulos:**
- `nodes.rs`: Definiciones de todas las estructuras del AST
- `display.rs`: Pretty-printing del AST para debugging
- `optimize.rs`: Optimizaciones que transforman el AST

## Estructura del Programa

La raíz de todo programa en Hulk está representada por la estructura `Program`.

```rust
pub struct Program {
    pub declarations: Vec<Declaration>, // Lista de declaraciones (funciones, tipos, protocolos)
    pub expr: Spanned<Expr>,            // Expresión principal del programa
}
```

## Declaraciones (`Declaration`)

Las declaraciones definen elementos que no son ejecutables por sí mismos en el flujo principal, pero que establecen estructuras para su uso posterior.

```rust
pub enum Declaration {
    Function(FunctionDecl),
    Type(TypeDecl),
    Protocol(ProtocolDecl),
    Macro(MacroDecl),
}
```

### `FunctionDecl` - Declaración de Funciones

Define una función global con nombre, parámetros, tipo de retorno opcional y cuerpo.

```rust
pub struct FunctionDecl {
    pub name: String,
    pub params: Vec<Param>,                    // Parámetros con tipo opcional
    pub return_type: Option<TypeAnnotation>,   // Tipo de retorno opcional
    pub body: Spanned<Expr>,                   // Cuerpo de la función
}
```

**Ejemplo:**
```hulk
function factorial(n: Number): Number =>
    if (n <= 1) 1
    else n * factorial(n - 1);
```

### `TypeDecl` - Definición de Tipos (Clases)

Define un tipo con constructor, herencia opcional, atributos y métodos.

```rust
pub struct TypeDecl {
    pub name: String,
    pub params: Vec<Param>,          // Parámetros del constructor
    pub parent: Option<TypeInit>,    // Herencia: inherits Parent(args)
    pub attributes: Vec<Attribute>,  // Atributos de la clase
    pub methods: Vec<FunctionDecl>,  // Métodos de la clase
}
```

**Ejemplo con herencia:**
```hulk
type Point(x, y) inherits Object() {
    x = x;
    y = y;
    distance(other: Point) => sqrt((self.x - other.x)^2 + (self.y - other.y)^2);
}
```

### `ProtocolDecl` - Definición de Protocolos

Define un contrato que los tipos pueden implementar.

```rust
pub struct ProtocolDecl {
    pub name: String,
    pub parent: Option<String>,           // Herencia de protocolos
    pub methods: Vec<MethodSignature>,    // Firmas de métodos requeridos
}
```

**Ejemplo:**
```hulk
protocol Hashable {
    hash(): Number;
}
```

### `MacroDecl` - Declaración de Macros

Define una macro en tiempo de compilación con parámetros especiales y body.

```rust
pub struct MacroDecl {
    pub name: String,
    pub params: Vec<MacroParam>,
    pub return_type: Option<TypeAnnotation>,
    pub body: Spanned<Expr>,
}
```

**Tipos de parámetros de macro:**

```rust
pub enum MacroParam {
    Normal { name: String, type_annotation: TypeAnnotation },
    Symbolic { name: String, type_annotation: TypeAnnotation },  // @param
    Placeholder { name: String, type_annotation: TypeAnnotation }, // $param
    Body { name: String, type_annotation: TypeAnnotation },       // *param
}
```

- **Normal**: Parámetro evaluado normalmente
- **Symbolic** (@): Recibe el nombre del argumento sin evaluar
- **Placeholder** ($): Se sustituye por un nombre generado
- **Body** (*): Captura código completo sin evaluar

**Ejemplo de macro con pattern matching:**
```hulk
def simplify(expr: Number) {
    match(expr) {
        case (x: Number + 0) => x;
        case (x: Number * 1) => x;
        default => expr;
    };
}
```

**Pattern Matching:**

```rust
pub enum Pattern {
    Literal(Expr),
    Variable { name: String, type_annotation: Option<TypeAnnotation> },
    Wildcard,
    Binary { left: Box<Pattern>, op: Op, right: Box<Pattern> },
    Unary { op: UnOp, operand: Box<Pattern> },
    Call { func: String, args: Vec<Pattern> },
}
```

### Estructuras Auxiliares para Declaraciones

**`Param` - Parámetro:**
```rust
pub struct Param {
    pub name: String,
    pub type_annotation: Option<TypeAnnotation>,
}
```

**`TypeInit` - Inicialización de tipo padre:**
```rust
pub struct TypeInit {
    pub name: String,
    pub args: Vec<Spanned<Expr>>,  // Argumentos para el constructor del padre
}
```

**`Attribute` - Atributo de clase:**
```rust
pub struct Attribute {
    pub name: String,
    pub type_annotation: Option<TypeAnnotation>,
    pub init: Spanned<Expr>,  // Expresión de inicialización
}
```

**`MethodSignature` - Firma de método (para protocolos):**
```rust
pub struct MethodSignature {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: TypeAnnotation,
}
```

**`TypeAnnotation` - Anotación de tipo:**
```rust
pub enum TypeAnnotation {
    Name(String),                      // Tipo simple: Number, String, etc.
    Function {                          // Tipo función
        params: Vec<TypeAnnotation>,
        return_type: Box<TypeAnnotation>,
    },
    Iterable(Box<TypeAnnotation>),     // Tipo iterable
}
```

### `MatchCase` - Caso de Pattern Matching

```rust
pub struct MatchCase {
    pub pattern: Pattern,
    pub expr: Spanned<Expr>,
}
```

## Expresiones (`Expr`)

El enum `Expr` es el núcleo del AST, representando cualquier construcción que pueda ser evaluada a un valor.

### Literales y Primitivos
- **`Number(f64)`**: Números de punto flotante
  ```hulk
  42, 3.14159, -5.0
  ```

- **`String(String)`**: Cadenas de texto
  ```hulk
  "Hola mundo", "HULK"
  ```

- **`Boolean(bool)`**: Valores booleanos
  ```hulk
  true, false
  ```

- **`Identifier(String)`**: Variables o nombres de referencia
  ```hulk
  x, nombre, contador
  ```

### Operaciones

**`Binary(Box<Spanned<Expr>>, Op, Box<Spanned<Expr>>)`** - Operaciones binarias:
- Aritméticas: `+`, `-`, `*`, `/`, `%` (módulo), `^` (potencia)
- Comparación: `==`, `!=`, `<`, `>`, `<=`, `>=`
- Lógicas: `&` (AND), `|` (OR)
- Concatenación: `@` (simple), `@@` (con espacio)

```hulk
2 + 3 * 4
5 > 3
true & false
"Hola" @ " " @ "mundo"
```

**`Unary(UnOp, Box<Spanned<Expr>>)`** - Operaciones unarias:
- Negación numérica: `-`
- NOT lógico: `!`

```hulk
-42
!true
```

### Control de Flujo

**`If`** - Condicional if-elif-else:
```rust
If {
    cond: Box<Spanned<Expr>>,
    then_expr: Box<Spanned<Expr>>,
    else_expr: Box<Spanned<Expr>>,  // elif se representa como If anidado
}
```
```hulk
if (x > 0) "positivo"
elif (x < 0) "negativo"
else "cero"
```

**`While`** - Bucle while:
```rust
While {
    cond: Box<Spanned<Expr>>,
    body: Box<Spanned<Expr>>,
}
```
```hulk
while (x < 10) {
    print(x);
    x := x + 1;
}
```

**`For`** - Bucle for sobre iterables:
```rust
For {
    var: String,
    iterable: Box<Spanned<Expr>>,
    body: Box<Spanned<Expr>>,
}
```
```hulk
for (i in range(10)) print(i);
```

**`Block(Vec<Spanned<Expr>>)`** - Secuencia de expresiones:
```hulk
{
    let x = 5;
    print(x);
    x * 2
}
```

### Funcionales

**`Let`** - Bindings locales con scope:
```rust
Let {
    bindings: Vec<(String, Option<TypeAnnotation>, Spanned<Expr>)>,
    body: Box<Spanned<Expr>>,
}
```
```hulk
let x = 5, y = 3 in x + y
```

**`Lambda`** - Funciones anónimas:
```rust
Lambda {
    params: Vec<Param>,
    body: Box<Spanned<Expr>>,
}
```
```hulk
|x, y| => x + y
```

**`Call(String, Vec<Spanned<Expr>>)`** - Llamada a función global:
```hulk
factorial(5)
range(10)
```

**`Print(Box<Spanned<Expr>>)`** - Función especial de impresión:
```hulk
print("Hola mundo")
```

### Orientado a Objetos

**`Instantiation`** - Creación de objetos:
```rust
Instantiation {
    type_name: String,
    args: Vec<Spanned<Expr>>,
}
```
```hulk
new Point(3, 4)
```

**`MethodCall`** - Llamada a método:
```rust
MethodCall {
    object: Box<Spanned<Expr>>,
    method: String,
    args: Vec<Spanned<Expr>>,
}
```
```hulk
p1.distance(p2)
obj.method()
```

**`BaseCall`** - Llamada a método del padre:
```rust
BaseCall {
    method: String,
    args: Vec<Spanned<Expr>>,
}
```
```hulk
base.init(x, y)
```

**`AttributeAccess`** - Acceso a atributo:
```rust
AttributeAccess {
    object: Box<Spanned<Expr>>,
    attribute: String,
}
```
```hulk
point.x
obj.name
```

**`Is(Box<Spanned<Expr>>, String)`** - Verificación de tipo:
```hulk
obj is Point
```

**`As(Box<Spanned<Expr>>, String)`** - Conversión de tipo:
```hulk
value as Number
```

**`Assignment(String, Box<Spanned<Expr>>)`** - Asignación destructiva:
```hulk
x := 10
```

### Vectores

**`VectorLiteral(Vec<Spanned<Expr>>)`** - Vector literal:
```hulk
[1, 2, 3, 4, 5]
```

**`VectorGenerator`** - Comprensión de vectores:
```rust
VectorGenerator {
    expr: Box<Spanned<Expr>>,
    var: String,
    iterable: Box<Spanned<Expr>>,
}
```
```hulk
[x^2 | x in range(10)]
```

**`Indexing`** - Acceso por índice:
```rust
Indexing {
    vector: Box<Spanned<Expr>>,
    index: Box<Spanned<Expr>>,
}
```
```hulk
arr[0]
vec[i + 1]
```

### Pattern Matching

**`Match`** - Expresión de pattern matching (usado en macros):
```rust
Match {
    expr: Box<Spanned<Expr>>,
    cases: Vec<MatchCase>,
    default: Option<Box<Spanned<Expr>>>,
}
```
```hulk
match(expr) {
    case (x: Number + 0) => x;
    case (x: Number * 1) => x;
    default => expr;
}
```

Los patrones pueden hacer matching estructural sobre expresiones, extrayendo variables en el proceso.
vector[0]
array[i + 1]
```

### Funciones Matemáticas

- **`Sqrt(Box<Spanned<Expr>>)`**: Raíz cuadrada
  ```hulk
  sqrt(16)  // → 4
  ```

- **`Sin(Box<Spanned<Expr>>)`**: Seno
- **`Cos(Box<Spanned<Expr>>)`**: Coseno
- **`Exp(Box<Spanned<Expr>>)`**: Exponencial (e^x)
- **`Log(Box<Spanned<Expr>>, Box<Spanned<Expr>>)`**: Logaritmo (base, valor)
  ```hulk
  log(10, 100)  // → 2
  ```

### Constantes y Valores Especiales

- **`PI`**: Constante π (pi)
- **`E`**: Constante e (número de Euler)
- **`Rand`**: Generador de números aleatorios

## Operadores (`Op` y `UnOp`)

Los operadores se definen en las enumeraciones `Op` (binarios) y `UnOp` (unarios).

### Operadores Binarios (`Op`)

```rust
pub enum Op {
    // Aritméticos
    Add,      // +
    Sub,      // -
    Mul,      // *
    Div,      // /
    Mod,      // %
    Pow,      // ^
    
    // Comparación
    Eq,       // ==
    Neq,      // !=
    Lt,       // <
    Gt,       // >
    Le,       // <=
    Ge,       // >=
    
    // Lógicos
    And,      // &
    Or,       // |
    
    // Cadenas
    Concat,       // @
    ConcatSpace,  // @@
}
```

**Precedencia (de menor a mayor):**
1. `|` (OR)
2. `&` (AND)
3. `==`, `!=`, `<`, `>`, `<=`, `>=` (Comparación)
4. `@`, `@@` (Concatenación)
5. `+`, `-` (Suma, Resta)
6. `*`, `/`, `%` (Multiplicación, División, Módulo)
7. `^` (Potencia)

### Operadores Unarios (`UnOp`)

```rust
pub enum UnOp {
    Neg,  // - (negación numérica)
    Not,  // ! (NOT lógico)
}
```

## Sistema de Posicionamiento (`Spanned`)

Cada nodo del AST está envuelto en `Spanned<T>` para mantener información de posición.

```rust
pub struct Spanned<T> {
    pub node: T,           // El nodo del AST
    pub pos: Position,     // Posición en el código fuente
}

pub struct Position {
    pub line: usize,       // Línea (base 1)
    pub column: usize,     // Columna (base 1)
}
```

**Ventajas:**
- **Errores precisos**: "Error en línea 5, columna 12"
- **Source maps**: Mapeo entre código fuente y AST
- **Debugging**: Saber exactamente dónde está cada nodo

**Ejemplo de uso:**
```rust
let expr = Spanned::new(
    Expr::Number(42.0),
    Position { line: 1, column: 9 }
);
```

## Módulo Display

El módulo `display.rs` implementa el trait `Display` para todas las estructuras del AST, permitiendo pretty-printing legible.

```rust
impl Display for Expr {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Expr::Number(n) => write!(f, "{}", n),
            Expr::String(s) => write!(f, "\"{}\"", s),
            Expr::Binary(left, op, right) => {
                write!(f, "({} {} {})", left.node, op, right.node)
            }
            // ... etc
        }
    }
}
```

**Uso:**
```rust
let program = parser.parse_program()?;
println!("{}", program.expr.node);  // Pretty print del AST
```

**Salida ejemplo:**
```
let x = (5 + 3) in (x * 2)
```

## Módulo Optimize

El módulo `optimize.rs` contiene todas las optimizaciones que se aplican sobre el AST.

### Arquitectura de Optimización

```rust
pub fn optimize_program(program: Program) -> Program

pub fn optimize_expr(
    expr: Spanned<Expr>,
    interner: &mut StringInterner,
    env: &ConstEnv
) -> Spanned<Expr>
```

**Componentes:**
- **`StringInterner`**: HashMap para deduplicar cadenas
- **`ConstEnv`**: Entorno de constantes conocidas (`HashMap<String, Expr>`)
- **`is_constant(expr)`**: Predicado que determina si una expresión es constante

### Optimizaciones Implementadas

#### 1. Constant Folding (Plegado de Constantes)

Evalúa operaciones con valores constantes en **tiempo de compilación**.

**Aritméticas:**
```hulk
2 + 3 * 4        →  14
10 / 2           →  5
5 % 3            →  2
2 ^ 3            →  8
```

**Booleanas:**
```hulk
5 > 3            →  true
10 == 10         →  true
true & false     →  false
```

**Cadenas:**
```hulk
"Hola" @ " mundo"     →  "Hola mundo"
"a" @@ "b"            →  "a b"
```

**Matemáticas:**
```hulk
sqrt(16)         →  4
sin(0)           →  0
log(10, 100)     →  2
```

**Implementación:**
```rust
Expr::Binary(left, Op::Add, right) => {
    let opt_left = optimize_expr(*left, interner, env);
    let opt_right = optimize_expr(*right, interner, env);
    
    match (&opt_left.node, &opt_right.node) {
        (Expr::Number(a), Expr::Number(b)) => {
            Expr::Number(a + b)  // Constante plegada
        }
        _ => Expr::Binary(Box::new(opt_left), Op::Add, Box::new(opt_right))
    }
}
```

---

#### 2. Constant Propagation (Propagación de Constantes)

Sustituye variables por sus valores constantes conocidos a través del programa.

**Ejemplo:**
```hulk
// Antes
let x = 5 in x + x

// Después
let x = 5 in 10
```

**Ejemplo con múltiples bindings:**
```hulk
// Antes
let x = 8, y = x + 2 in y * 2

// Después
let x = 8, y = 10 in 20
```

**Implementación:**
```rust
Expr::Let { bindings, body } => {
    let mut new_env = env.clone();
    let mut optimized_bindings = Vec::new();
    
    // Detectar variables mutadas con := en el body
    let mut mutated = HashSet::new();
    collect_assigned_vars(&body.node, &mut mutated);
    
    for (name, type_ann, init) in bindings {
        let opt_init = optimize_expr(init, interner, &new_env);
        
        // Solo propagar si es constante Y no se muta con :=
        if is_constant(&opt_init.node) && !mutated.contains(&name) {
            new_env.insert(name.clone(), opt_init.node.clone());
        }
        
        optimized_bindings.push((name, type_ann, opt_init));
    }
    
    // Optimizar cuerpo con nuevo entorno
    let opt_body = optimize_expr(*body, interner, &new_env);
    
    Expr::Let {
        bindings: optimized_bindings,
        body: Box::new(opt_body),
    }
}
```

> **⚠️ Nota sobre variables mutables**: La función `collect_assigned_vars()` escanea recursivamente el body buscando nodos `Expr::Assign(name, _)` (`:=`). Las variables encontradas se excluyen de la propagación de constantes para evitar que el optimizador sustituya valores que serán modificados en runtime (ej: variables usadas con `swap`).
```

**Uso de identificadores:**
```rust
Expr::Identifier(name) => {
    // Buscar en el entorno si es constante conocida
    if let Some(const_value) = env.get(name) {
        const_value.clone()  // Propagar la constante
    } else {
        Expr::Identifier(interner.intern(name))
    }
}
```

---

#### 3. Dead Code Elimination (Eliminación de Código Muerto)

Elimina código que nunca se ejecutará.

**Condicionales con condición constante:**
```hulk
// Antes
if (true) 10 else 20

// Después
10
```

```hulk
// Antes
if (false) expensive_computation() else 5

// Después
5
```

**Bucles while con condición falsa:**
```hulk
// Antes
while (false) {
    print("nunca");
    x := x + 1;
}

// Después
{ }  // Bloque vacío
```

**Implementación:**
```rust
Expr::If { cond, then_expr, else_expr } => {
    let opt_cond = optimize_expr(*cond, interner, env);
    
    // Si la condición es constante, elegir rama
    if let Expr::Boolean(b) = opt_cond.node {
        if b {
            return optimize_expr(*then_expr, interner, env);
        } else {
            return optimize_expr(*else_expr, interner, env);
        }
    }
    
    // Condición no constante, optimizar ambas ramas
    let opt_then = optimize_expr(*then_expr, interner, env);
    let opt_else = optimize_expr(*else_expr, interner, env);
    
    Expr::If {
        cond: Box::new(opt_cond),
        then_expr: Box::new(opt_then),
        else_expr: Box::new(opt_else),
    }
}
```

---

#### 4. Algebraic Simplification (Simplificación Algebraica)

Aplica identidades matemáticas para simplificar expresiones.

**Identidades de suma:**
```hulk
x + 0    →  x
0 + x    →  x
```

**Identidades de multiplicación:**
```hulk
x * 1    →  x
1 * x    →  x
x * 0    →  0
0 * x    →  0
```

**Identidades de resta:**
```hulk
x - 0    →  x
```

**Identidades de división:**
```hulk
0 / x    →  0   (si x != 0)
x / 1    →  x
```

**Implementación:**
```rust
Expr::Binary(left, Op::Mul, right) => {
    let opt_left = optimize_expr(*left, interner, env);
    let opt_right = optimize_expr(*right, interner, env);
    
    match (&opt_left.node, &opt_right.node) {
        // x * 0 → 0
        (_, Expr::Number(0.0)) | (Expr::Number(0.0), _) => {
            Expr::Number(0.0)
        }
        // x * 1 → x
        (expr, Expr::Number(1.0)) => expr.clone(),
        (Expr::Number(1.0), expr) => expr.clone(),
        // Constant folding
        (Expr::Number(a), Expr::Number(b)) => {
            Expr::Number(a * b)
        }
        _ => Expr::Binary(Box::new(opt_left), Op::Mul, Box::new(opt_right))
    }
}
```

---

#### 5. Boolean Short-Circuit (Cortocircuito Booleano)

Optimiza operaciones lógicas con valores constantes.

**AND:**
```hulk
false & x    →  false    (x nunca se evalúa)
true & x     →  x
x & false    →  false
x & true     →  x
```

**OR:**
```hulk
true | x     →  true     (x nunca se evalúa)
false | x    →  x
x | true     →  true
x | false    →  x
```

**NOT:**
```hulk
!true        →  false
!false       →  true
```

**Implementación:**
```rust
Expr::Binary(left, Op::And, right) => {
    let opt_left = optimize_expr(*left, interner, env);
    
    // Short-circuit: false & x → false
    if let Expr::Boolean(false) = opt_left.node {
        return Spanned::new(Expr::Boolean(false), opt_left.pos);
    }
    
    let opt_right = optimize_expr(*right, interner, env);
    
    match (&opt_left.node, &opt_right.node) {
        (Expr::Boolean(false), _) | (_, Expr::Boolean(false)) => {
            Expr::Boolean(false)
        }
        (Expr::Boolean(true), expr) | (expr, Expr::Boolean(true)) => {
            expr.clone()
        }
        _ => Expr::Binary(Box::new(opt_left), Op::And, Box::new(opt_right))
    }
}
```

---

#### 6. String Interning (Internado de Cadenas)

Deduplica cadenas idénticas en memoria usando un HashMap, reduciendo el uso de memoria.

**Estructura:**
```rust
pub struct StringInterner {
    cache: HashMap<String, String>,
}

impl StringInterner {
    pub fn new() -> Self {
        Self { cache: HashMap::new() }
    }
    
    pub fn intern(&mut self, s: String) -> String {
        if let Some(cached) = self.cache.get(&s) {
            cached.clone()  // Reusar cadena existente
        } else {
            self.cache.insert(s.clone(), s.clone());
            s
        }
    }
}
```

**Beneficio:**
```hulk
// Antes (en memoria):
"hola" (ptr1)
"hola" (ptr2)
"hola" (ptr3)

// Después:
"hola" (ptr1) ← todas las referencias apuntan aquí
```

**Uso en optimización:**
```rust
Expr::String(s) => Expr::String(interner.intern(s)),
Expr::Identifier(id) => {
    if let Some(const_val) = env.get(&id) {
        const_val.clone()
    } else {
        Expr::Identifier(interner.intern(id))
    }
}
```

---

### Predicado `is_constant`

Determina si una expresión es un valor constante evaluable.

```rust
fn is_constant(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Number(_)
            | Expr::Boolean(_)
            | Expr::String(_)
            | Expr::PI
            | Expr::E
    )
}
```

**Uso:**
- Constant propagation: Solo propagar valores constantes
- Dead code elimination: Detectar condiciones constantes
- Seguridad: No optimizar expresiones con efectos secundarios

---

### Detección de Variables Mutables (`collect_assigned_vars`)

Escanea recursivamente el AST buscando nodos `Expr::Assign(name, _)` (operador `:=`). Las variables encontradas se excluyen de la propagación de constantes.

```rust
fn collect_assigned_vars(expr: &Expr, set: &mut HashSet<String>) {
    match expr {
        Expr::Assign(name, val) => {
            set.insert(name.clone());
            collect_assigned_vars(&val.node, set);
        }
        Expr::Block(stmts) => {
            for s in stmts { collect_assigned_vars(&s.node, set); }
        }
        Expr::If { cond, then_expr, else_expr } => {
            collect_assigned_vars(&cond.node, set);
            collect_assigned_vars(&then_expr.node, set);
            collect_assigned_vars(&else_expr.node, set);
        }
        Expr::While { cond, body } => {
            collect_assigned_vars(&cond.node, set);
            collect_assigned_vars(&body.node, set);
        }
        // ... recorre todos los nodos recursivamente
        _ => {}
    }
}
```

**¿Por qué es necesario?** Sin esta función, el optimizador propagaría el valor inicial de una variable incluso si luego se modifica con `:=`, produciendo resultados incorrectos:

```hulk
// SIN collect_assigned_vars (BUG):
let a = 10 in {
    a := 20;   // se ignora
    print(a);  // imprimiría 10 (propagó el valor original)
}

// CON collect_assigned_vars (CORRECTO):
let a = 10 in {
    a := 20;
    print(a);  // imprime 20 (no propagó porque 'a' está en mutated)
}
```

---

### Pipeline de Optimización

```
AST original
    ↓
optimize_program()
    ↓
Crear StringInterner
Crear ConstEnv vacío
    ↓
optimize_expr() (recursivo)
    ↓
├─ Constant Folding
├─ Constant Propagation
├─ Dead Code Elimination
├─ Algebraic Simplification
├─ Boolean Short-Circuit
└─ String Interning
    ↓
AST optimizado
```

**Ejemplo completo:**
```hulk
// Input
let x = 5 + 3 in x * 1 + 0 - x * 0

// Paso 1: Constant Folding
let x = 8 in x * 1 + 0 - x * 0

// Paso 2: Constant Propagation (x = 8)
let x = 8 in 8 * 1 + 0 - 8 * 0

// Paso 3: Algebraic Simplification
let x = 8 in 8 + 0 - 0

// Paso 4: Más simplificación
let x = 8 in 8

// Output optimizado
let x = 8 in 8
```

---

### Limitaciones y Consideraciones

**No se optimiza:**
- ❌ Funciones con efectos secundarios (`print`, `rand`)
- ❌ Expresiones con variables no constantes
- ❌ Llamadas a funciones (podría haber side effects)
- ❌ Bucles (incluso con condición constante true, podría no terminar)
- ❌ Variables mutadas con `:=` (detectadas por `collect_assigned_vars`)

**Seguridad:**
- ✅ Las optimizaciones preservan semántica
- ✅ No optimiza si no está seguro
- ✅ Errores en runtime se mantienen (ej: división por cero)
- ✅ Variables reasignadas con `:=` no se propagan

**Performance:**
- Tiempo: O(n) donde n = nodos del AST
- Espacio: O(m) donde m = cadenas únicas

## Ejemplos Completos

## Ejemplos Completos

### Ejemplo 1: Programa Simple

**Código HULK:**
```hulk
function add(a, b) => a + b;

let x = 5, y = 10 in add(x, y);
```

**AST (simplificado):**
```rust
Program {
    declarations: [
        Declaration::Function(FunctionDecl {
            name: "add",
            params: [
                Param { name: "a", type_annotation: None },
                Param { name: "b", type_annotation: None },
            ],
            return_type: None,
            body: Expr::Binary(
                Expr::Identifier("a"),
                Op::Add,
                Expr::Identifier("b")
            ),
        })
    ],
    expr: Expr::Let {
        bindings: [
            ("x", None, Expr::Number(5.0)),
            ("y", None, Expr::Number(10.0)),
        ],
        body: Expr::Call("add", [
            Expr::Identifier("x"),
            Expr::Identifier("y"),
        ]),
    }
}
```

---

### Ejemplo 2: Tipo con Herencia

**Código HULK:**
```hulk
type Point(x, y) inherits Object() {
    x = x;
    y = y;
    distance(other: Point) => sqrt((self.x - other.x)^2 + (self.y - other.y)^2);
}

new Point(3, 4);
```

**AST:**
```rust
Program {
    declarations: [
        Declaration::Type(TypeDecl {
            name: "Point",
            params: [
                Param { name: "x", type_annotation: None },
                Param { name: "y", type_annotation: None },
            ],
            parent: Some(TypeInit {
                name: "Object",
                args: [],
            }),
            attributes: [
                Attribute {
                    name: "x",
                    type_annotation: None,
                    init: Expr::Identifier("x"),
                },
                Attribute {
                    name: "y",
                    type_annotation: None,
                    init: Expr::Identifier("y"),
                },
            ],
            methods: [
                FunctionDecl {
                    name: "distance",
                    params: [
                        Param {
                            name: "other",
                            type_annotation: Some(TypeAnnotation::Name("Point")),
                        }
                    ],
                    return_type: None,
                    body: Expr::Sqrt(
                        Expr::Binary(
                            Expr::Binary(
                                Expr::AttributeAccess(Expr::Identifier("self"), "x"),
                                Op::Sub,
                                Expr::AttributeAccess(Expr::Identifier("other"), "x"),
                            ),
                            Op::Pow,
                            Expr::Number(2.0),
                        ),
                        // ... + parte y
                    ),
                }
            ],
        })
    ],
    expr: Expr::Instantiation {
        type_name: "Point",
        args: [Expr::Number(3.0), Expr::Number(4.0)],
    }
}
```

---

### Ejemplo 3: Optimización Completa

**Input:**
```hulk
let x = 2 + 3 * 4,
    y = if (true) x else 100
in y + 0 - y * 0
```

**AST antes de optimizar:**
```rust
Expr::Let {
    bindings: [
        ("x", None, Expr::Binary(
            Expr::Number(2.0),
            Op::Add,
            Expr::Binary(Expr::Number(3.0), Op::Mul, Expr::Number(4.0))
        )),
        ("y", None, Expr::If {
            cond: Expr::Boolean(true),
            then_expr: Expr::Identifier("x"),
            else_expr: Expr::Number(100.0),
        }),
    ],
    body: Expr::Binary(
        Expr::Binary(
            Expr::Identifier("y"),
            Op::Add,
            Expr::Number(0.0)
        ),
        Op::Sub,
        Expr::Binary(
            Expr::Identifier("y"),
            Op::Mul,
            Expr::Number(0.0)
        )
    ),
}
```

**Optimizaciones aplicadas:**

1. **Constant Folding**: `3 * 4` → `12`, `2 + 12` → `14`
2. **Dead Code Elimination**: `if (true) x else 100` → `x`
3. **Constant Propagation**: `x` → `14`
4. **Constant Propagation**: `y` → `14` (porque `if (true) 14 else 100` → `14`)
5. **Constant Propagation en body**: `y` → `14`
6. **Algebraic Simplification**: `14 + 0` → `14`
7. **Algebraic Simplification**: `14 * 0` → `0`
8. **Constant Folding**: `14 - 0` → `14`

**AST después de optimizar:**
```rust
Expr::Let {
    bindings: [
        ("x", None, Expr::Number(14.0)),
        ("y", None, Expr::Number(14.0)),
    ],
    body: Expr::Number(14.0),
}
```

**Output:**
```hulk
let x = 14, y = 14 in 14
```

---

### Ejemplo 4: Programa Completo con Protocolo

**Código HULK:**
```hulk
protocol Comparable {
    compare(other: Self): Number;
}

type Rectangle(width, height) {
    width = width;
    height = height;
    
    area() => self.width * self.height;
    compare(other: Rectangle) => self.area() - other.area();
}

function max_area(r1: Rectangle, r2: Rectangle) =>
    if (r1.compare(r2) > 0) r1
    else r2;

let rect1 = new Rectangle(10, 5),
    rect2 = new Rectangle(8, 7)
in max_area(rect1, rect2).area();
```

**AST (estructura):**
```rust
Program {
    declarations: [
        // Protocol Comparable
        Declaration::Protocol(ProtocolDecl {
            name: "Comparable",
            parent: None,
            methods: [
                MethodSignature {
                    name: "compare",
                    params: [Param { name: "other", type_annotation: Some(...) }],
                    return_type: TypeAnnotation::Name("Number"),
                }
            ],
        }),
        
        // Type Rectangle
        Declaration::Type(TypeDecl {
            name: "Rectangle",
            params: [
                Param { name: "width", ... },
                Param { name: "height", ... },
            ],
            parent: None,
            attributes: [ /* width, height */ ],
            methods: [ /* area(), compare() */ ],
        }),
        
        // Function max_area
        Declaration::Function(FunctionDecl {
            name: "max_area",
            params: [ /* r1, r2 */ ],
            return_type: None,
            body: Expr::If { /* ... */ },
        }),
    ],
    
    expr: Expr::Let {
        bindings: [
            ("rect1", None, Expr::Instantiation { ... }),
            ("rect2", None, Expr::Instantiation { ... }),
        ],
        body: Expr::MethodCall {
            object: Expr::Call("max_area", [ /* ... */ ]),
            method: "area",
            args: [],
        },
    }
}
```

---

## Uso del AST en el Compilador

### Construcción

```rust
use hulk_compiler::parser::Parser;

let code = r#"
    function factorial(n) =>
        if (n <= 1) 1
        else n * factorial(n - 1);
"#;

let mut parser = Parser::new(code);
let program = parser.parse_program()?;
```

### Optimización

```rust
use hulk_compiler::ast::optimize::optimize_program;

let optimized = optimize_program(program);
```

### Pretty-Printing

```rust
println!("=== AST ===");
println!("{}", program.expr.node);

println!("\n=== Debug ===");
println!("{:#?}", program);
```

---

## Comparación: Antes vs Después de Optimización

| Input | Optimizado | Técnicas |
|-------|-----------|----------|
| `2 + 3` | `5` | Constant folding |
| `x * 1` | `x` | Algebraic simplification |
| `if (true) a else b` | `a` | Dead code elimination |
| `let x = 5 in x + x` | `let x = 5 in 10` | Constant propagation |
| `false & expensive()` | `false` | Short-circuit |
| `"a" @ "a" @ "a"` | `"aaa"` | Folding + interning |
| `sqrt(16) + 0` | `4` | Math folding + algebraic |
| `!(5 > 3)` | `false` | Folding + unary |

---

- Muchos nodos utilizan `Spanned<T>`, una estructura auxiliar que incluye la información de ubicación en el código fuente (línea, columna, offset) junto con el nodo, para reportar errores con precisión.

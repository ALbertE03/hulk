# Parser de HULK

Este módulo implementa un **Analizador Sintáctico (Parser)** descendente recursivo con **Pratt Parsing** para el lenguaje de programación HULK.

## Índice
- [Visión General](#visión-general)
- [Arquitectura](#arquitectura)
- [Pratt Parsing](#pratt-parsing-precedencia-de-operadores)
- [Métodos Principales](#métodos-principales)
- [Gramática Completa](#gramática-completa-de-hulk)
- [Parsing de Declaraciones](#parsing-de-declaraciones)
- [Parsing de Expresiones](#parsing-de-expresiones)
- [Manejo de Errores](#manejo-de-errores)
- [Ejemplos Completos](#ejemplos-completos)
- [Testing](#testing)

## Visión General

El parser es la **segunda fase del compilador**. Recibe un flujo de tokens del lexer y construye un Árbol de Sintaxis Abstracta (AST) que representa la estructura jerárquica del programa.

**Responsabilidades:**
- Verificar que la secuencia de tokens cumple con la gramática de HULK
- Construir nodos del AST correctamente anidados
- Rastrear posiciones precisas de cada construcción
- Detectar errores sintácticos con mensajes descriptivos
- Manejar precedencia de operadores correctamente

**Características clave:**
- ✅ **Descendente Recursivo**: Análisis top-down siguiendo la gramática
- ✅ **Pratt Parsing**: Manejo elegante de precedencia de operadores
- ✅ **Posicionamiento**: Cada nodo wrapped en `Spanned<T>`
- ✅ **Errores descriptivos**: Mensajes con ubicación exacta
- ✅ **Look-ahead limitado**: Peek de 1 token
- ✅ **Recuperación de errores**: Propaga errores sin panic

## Arquitectura

### Estructura Principal

```rust
pub struct Parser {
    tokens: Vec<Result<(Token, Position), LexError>>,  // Todos los tokens del lexer
    current: usize,                                     // Índice del token actual
}
```

El parser almacena:
- **`tokens`**: Vector completo de tokens (eager loading)
- **`current`**: Puntero al token que se está procesando

**Ventajas:**
- Fácil implementación de peek/lookahead
- Backtracking simple si fuera necesario
- Posiciones siempre disponibles

**Desventajas:**
- Memoria O(n) para los tokens
- No es streaming (todo el archivo debe tokenizarse primero)

### Flujo de Parsing

```
Código fuente
    ↓
Lexer → Tokens
    ↓
Parser::new()
    ↓
parse_program()
    ↓
├─ parse_declaration() (0..n veces)
│  ├─ parse_function_decl()
│  ├─ parse_type_decl()
│  └─ parse_protocol_decl()
│
└─ parse_expr() (expresión principal)
   ├─ parse_prefix()
   ├─ parse_infix()
   └─ parse_postfix()
    ↓
Program (AST)
```

## Pratt Parsing (Precedencia de Operadores)

HULK usa **Pratt Parsing** (también llamado "Precedence Climbing") para manejar operadores binarios con precedencia correcta.

### ¿Qué es Pratt Parsing?

Un algoritmo que parsea expresiones con operadores binarios/unarios respetando:
1. **Precedencia**: `*` se ejecuta antes que `+`
2. **Asociatividad**: `a - b - c` es `(a - b) - c` (izquierda)
3. **Prefijos y Postfijos**: `-x`, `x.method()`

### Niveles de Precedencia

```rust
enum Precedence {
    Lowest,      // Menor precedencia
    Assignment,  // :=
    Or,          // |
    And,         // &
    Comparison,  // ==, !=, <, >, <=, >=
    Concat,      // @, @@
    Sum,         // +, -
    Product,     // *, /, %
    Unary,       // -, !
    Power,       // ^
    Call,        // f(), obj.method(), obj[i]
}               // Mayor precedencia
```

**Tabla de precedencia (menor a mayor):**

| Nivel | Operadores | Asociatividad | Ejemplo |
|-------|-----------|---------------|---------|
| 1 | `:=` | Derecha | `x := y := 5` |
| 2 | `\|` | Izquierda | `a \| b \| c` |
| 3 | `&` | Izquierda | `a & b & c` |
| 4 | `==`, `!=`, `<`, `>`, `<=`, `>=` | Izquierda | `a < b` |
| 5 | `@`, `@@` | Izquierda | `"a" @ "b"` |
| 6 | `+`, `-` | Izquierda | `a + b - c` |
| 7 | `*`, `/`, `%` | Izquierda | `a * b / c` |
| 8 | `-`, `!` (unarios) | Derecha | `-(-x)` |
| 9 | `^` | Derecha | `2 ^ 3 ^ 4` |
| 10 | `()`, `.`, `[]` | Izquierda | `f(x).attr[0]` |

### Algoritmo de Pratt

```rust
fn parse_expr(min_precedence) -> Expr {
    // 1. Parse prefijo (literal, unario, paréntesis)
    let mut left = parse_prefix();
    
    // 2. Mientras el operador siguiente tenga precedencia >= min_precedence
    while current_precedence() >= min_precedence {
        let operator = current_token();
        
        // 3. Parse lado derecho con precedencia mayor
        let right = parse_expr(operator.precedence + 1);
        
        // 4. Combinar: left op right
        left = Binary(left, operator, right);
    }
    
    return left;
}
```

**Ejemplo:** `2 + 3 * 4`

```
parse_expr(Lowest)
  ├─ parse_prefix() → 2
  ├─ ver '+' (precedencia Sum)
  │  ├─ parse_expr(Sum + 1 = Product)
  │  │  ├─ parse_prefix() → 3
  │  │  ├─ ver '*' (precedencia Product)
  │  │  │  ├─ parse_expr(Product + 1 = Unary)
  │  │  │  │  └─ parse_prefix() → 4
  │  │  │  └─ Binary(3, *, 4)
  │  │  └─ return 3 * 4
  │  └─ Binary(2, +, 3 * 4)
  └─ return 2 + (3 * 4)
```

## Métodos Principales

### Métodos de Control de Tokens

#### `new(input: &str) -> Self`

Crea un nuevo parser a partir del código fuente.

```rust
let mut parser = Parser::new("let x = 5;");
```

**Proceso:**
1. Crea un `Lexer` para la entrada
2. Consume todos los tokens en un vector
3. Asegura que hay un token `EOF` al final
4. Inicializa `current = 0`

---

#### `peek() -> Option<&Result<(Token, Position), LexError>>`

Mira el token actual **sin consumirlo**.

```rust
if let Some(Ok((Token::Let, _))) = self.peek() {
    // Es un let binding
}
```

---

#### `advance() -> Result<(Token, Position), ParseError>`

Consume y devuelve el token actual, avanzando el puntero.

```rust
let (token, pos) = self.advance()?;
```

**Comportamiento:**
- Si encuentra un error léxico: lo convierte a `ParseError::Lex`
- Si llega al `EOF`: no avanza más (se queda en EOF)
- Incrementa `current` automáticamente

---

#### `check(token: &Token) -> bool`

Verifica si el token actual es `token` sin consumirlo.

```rust
if self.check(&Token::LParen) {
    // El siguiente token es '('
}
```

---

#### `match_token(token: &Token) -> bool`

Si el token actual es `token`, lo consume y devuelve `true`.

```rust
if self.match_token(&Token::Semicolon) {
    // Consumió el ';' opcional
}
```

---

#### `consume(token: &Token, message: &str) -> Result<(Token, Position), ParseError>`

Consume el token esperado o devuelve error.

```rust
self.consume(&Token::LParen, "Expected '(' after function name")?;
```

**Uso típico:**
- Verificar puntuación obligatoria
- Sintaxis fija (keywords, paréntesis, etc.)

---

### Métodos de Parsing Principal

#### `parse_program() -> Result<Program, ParseError>`

Punto de entrada principal. Parsea un programa completo.

```rust
pub fn parse_program(&mut self) -> Result<Program, ParseError> {
    let mut declarations = Vec::new();
    
    // Parse todas las declaraciones
    while !self.at_end() && !self.is_expr_start() {
        declarations.push(self.parse_declaration()?);
    }
    
    // Parse expresión principal
    let expr = if self.at_end() {
        Spanned::new(Expr::Block(Vec::new()), self.peek_pos())
    } else {
        self.parse_spanned_expr(Precedence::Lowest)?
    };
    
    Ok(Program { declarations, expr })
}
```

**Estructura de un programa:**
```hulk
// Declaraciones (0 o más)
function foo() => 42;
type Point(x, y) { ... }
protocol Hashable { ... }

// Expresión principal (1)
let x = 5 in x + 10
```

---

#### `parse_declaration() -> Result<Declaration, ParseError>`

Parsea una declaración (función, tipo o protocolo).

```rust
fn parse_declaration(&mut self) -> Result<Declaration, ParseError> {
    match self.peek() {
        Some(Ok((Token::Function, _))) => {
            Ok(Declaration::Function(self.parse_function_decl()?))
        }
        Some(Ok((Token::Type, _))) => {
            Ok(Declaration::Type(self.parse_type_decl()?))
        }
        Some(Ok((Token::Protocol, _))) => {
            Ok(Declaration::Protocol(self.parse_protocol_decl()?))
        }
        Some(Ok((Token::Def, _))) => {
            Ok(Declaration::Macro(self.parse_macro_decl()?))
        }
        _ => Err(ParseError::UnexpectedToken { ... })
    }
}
        Some(Ok((Token::Type, _))) => {
            Ok(Declaration::Type(self.parse_type_decl()?))
        }
        Some(Ok((Token::Protocol, _))) => {
            Ok(Declaration::Protocol(self.parse_protocol_decl()?))
        }
        _ => Err(ParseError::UnexpectedToken { /* ... */ })
    }
}
```

---

#### `parse_expr(precedence: Precedence) -> Result<Expr, ParseError>`

Parsea una expresión con Pratt parsing.

**Algoritmo:**
1. Parse prefijo (`parse_prefix`)
2. Mientras haya operador infijo con precedencia suficiente:
   - Parse lado derecho con mayor precedencia
   - Construir nodo binario
3. Return expresión completa

```rust
fn parse_expr(&mut self, min_prec: Precedence) -> Result<Expr, ParseError> {
    let start_pos = self.peek_pos();
    let mut left = self.parse_prefix()?;
    
    while let Some(prec) = self.infix_precedence() {
        if prec < min_prec {
            break;
        }
        left = self.parse_infix(left, prec)?;
    }
    
    Ok(left)
}
```

---

#### `parse_prefix() -> Result<Expr, ParseError>`

Parsea expresiones prefijas (literales, unarios, paréntesis, keywords).

```rust
fn parse_prefix(&mut self) -> Result<Expr, ParseError> {
    let (token, pos) = self.advance()?;
    
    match token {
        Token::Number(n) => Ok(Expr::Number(n)),
        Token::StringLiteral(s) => Ok(Expr::String(s)),
        Token::True => Ok(Expr::Boolean(true)),
        Token::False => Ok(Expr::Boolean(false)),
        Token::Identifier(id) => Ok(Expr::Identifier(id)),
        
        Token::Minus => {
            let expr = self.parse_expr(Precedence::Unary)?;
            Ok(Expr::Unary(UnOp::Neg, Box::new(expr)))
        }
        
        Token::Not => {
            let expr = self.parse_expr(Precedence::Unary)?;
            Ok(Expr::Unary(UnOp::Not, Box::new(expr)))
        }
        
        Token::LParen => {
            let expr = self.parse_expr(Precedence::Lowest)?;
            self.consume(&Token::RParen, "Expected ')'")?;
            Ok(expr)
        }
        
        Token::Let => self.parse_let_expr(),
        Token::If => self.parse_if_expr(),
        Token::While => self.parse_while_expr(),
        Token::For => self.parse_for_expr(),
        // ... más casos
    }
}
```

---

#### `parse_infix(left: Expr, prec: Precedence) -> Result<Expr, ParseError>`

Parsea operadores infijos (binarios, llamadas, acceso a miembros).

```rust
fn parse_infix(&mut self, left: Expr, prec: Precedence) -> Result<Expr, ParseError> {
    let (token, _) = self.advance()?;
    
    match token {
        Token::Plus => {
            let right = self.parse_expr(prec + 1)?;
            Ok(Expr::Binary(Box::new(left), Op::Add, Box::new(right)))
        }
        
        Token::Star => {
            let right = self.parse_expr(prec + 1)?;
            Ok(Expr::Binary(Box::new(left), Op::Mul, Box::new(right)))
        }
        
        Token::LParen => {
            // Llamada a función
            let args = self.parse_argument_list()?;
            self.consume(&Token::RParen, "Expected ')'")?;
            Ok(Expr::Call(left, args))
        }
        
        Token::Dot => {
            // Acceso a miembro
            let member = self.parse_identifier()?;
            if self.check(&Token::LParen) {
                // Llamada a método
                let args = self.parse_argument_list()?;
                Ok(Expr::MethodCall(Box::new(left), member, args))
            } else {
                // Acceso a atributo
                Ok(Expr::AttributeAccess(Box::new(left), member))
            }
        }
        
        Token::LBracket => {
            // Indexación
            let index = self.parse_expr(Precedence::Lowest)?;
            self.consume(&Token::RBracket, "Expected ']'")?;
            Ok(Expr::Indexing(Box::new(left), Box::new(index)))
        }
        
        // ... más operadores
    }
}
```

## Gramática Completa de HULK

### Notación
- `<no_terminal>` - Reglas de la gramática
- `TOKEN` - Tokens del lexer
- `|` - Alternativa (OR)
- `*` - Cero o más
- `+` - Uno o más
- `?` - Opcional
- `[ ]` - Agrupación

### Programa

```
<program> ::= <declaration>* <expr>?

<declaration> ::= <function_decl>
                | <type_decl>
                | <protocol_decl>
                | <macro_decl>
```

### Declaraciones

**Función:**
```
<function_decl> ::= FUNCTION IDENTIFIER LPAREN <param_list>? RPAREN
                    (COLON <type_annotation>)?
                    FUNCARROW <expr>
                    SEMICOLON

<param_list> ::= <param> (COMMA <param>)*

<param> ::= IDENTIFIER (COLON <type_annotation>)?
```

**Macro:**
```
<macro_decl> ::= DEF IDENTIFIER LPAREN <macro_param_list>? RPAREN
                 (COLON <type_annotation>)?
                 (FUNCARROW <expr> | <block>)
                 SEMICOLON?

<macro_param_list> ::= <macro_param> (COMMA <macro_param>)*

<macro_param> ::= (CONCAT | DOLLAR | STAR)? IDENTIFIER COLON <type_annotation>

# Prefijos de parámetros de macro:
# @ (CONCAT) - Argumento simbólico (captura el nombre)
# $ (DOLLAR) - Placeholder (sustituido por nombre generado)
# * (STAR) - Body argument (captura código sin evaluar)
# (ninguno) - Parámetro normal
```

**Tipo:**
```
<type_decl> ::= TYPE IDENTIFIER LPAREN <param_list>? RPAREN
                (INHERITS <type_init>)?
                LBRACE
                  (<attribute> SEMICOLON)*
                  (<method_decl> SEMICOLON)*
                RBRACE

<type_init> ::= IDENTIFIER LPAREN <expr_list>? RPAREN

<attribute> ::= IDENTIFIER (COLON <type_annotation>)? ASSIGN <expr>

<method_decl> ::= IDENTIFIER LPAREN <param_list>? RPAREN
                  (COLON <type_annotation>)?
                  FUNCARROW <expr>
```

**Protocolo:**
```
<protocol_decl> ::= PROTOCOL IDENTIFIER
                    (EXTENDS IDENTIFIER)?
                    LBRACE
                      (<method_signature> SEMICOLON)*
                    RBRACE

<method_signature> ::= IDENTIFIER LPAREN <param_list>? RPAREN
                       COLON <type_annotation>
```

### Expresiones (con precedencia)

**Primarias:**
```
<primary_expr> ::= NUMBER
                 | STRING_LITERAL
                 | TRUE | FALSE
                 | IDENTIFIER
                 | PI | E
                 | LPAREN <expr> RPAREN
                 | LBRACKET <expr_list>? RBRACKET              // Vector literal
                 | LBRACKET <expr> PIPE IDENTIFIER IN <expr> RBRACKET  // Comprehension
```

**Prefijos:**
```
<prefix_expr> ::= MINUS <expr>                    // Negación
                | NOT <expr>                      // NOT lógico
                | LET <let_bindings> IN <expr>    // Let binding
                | IF <if_expr>                    // Condicional
                | WHILE <expr> <expr>             // While loop
                | FOR LPAREN IDENTIFIER IN <expr> RPAREN <expr>  // For loop
                | LBRACE <block> RBRACE           // Bloque
                | NEW IDENTIFIER LPAREN <expr_list>? RPAREN     // Instanciación
                | PRINT LPAREN <expr> RPAREN      // Print
                | SQRT LPAREN <expr> RPAREN       // Funciones matemáticas
                | SIN | COS | EXP | LOG ...
                | PIPE <param_list>? PIPE FUNCARROW <expr>      // Lambda
```

**Infijos (por precedencia):**
```
// Precedencia más baja
<assign_expr> ::= <identifier> DESTRUCTASSIGN <expr>

<or_expr> ::= <expr> OR <expr>

<and_expr> ::= <expr> AND <expr>

<comparison_expr> ::= <expr> (EQUAL | NOTEQUAL | LT | GT | LE | GE) <expr>

<concat_expr> ::= <expr> (CONCAT | CONCATSPACE) <expr>

<sum_expr> ::= <expr> (PLUS | MINUS) <expr>

<product_expr> ::= <expr> (STAR | SLASH | PERCENT) <expr>

<power_expr> ::= <expr> POWER <expr>  // Asociatividad derecha

// Precedencia más alta
<postfix_expr> ::= <expr> LPAREN <expr_list>? RPAREN         // Llamada
                 | <expr> DOT IDENTIFIER                     // Atributo
                 | <expr> DOT IDENTIFIER LPAREN <expr_list>? RPAREN  // Método
                 | <expr> LBRACKET <expr> RBRACKET           // Indexación
                 | <expr> IS IDENTIFIER                      // Type check
                 | <expr> AS IDENTIFIER                      // Type cast
```

**Let:**
```
<let_bindings> ::= <let_binding> (COMMA <let_binding>)*

<let_binding> ::= IDENTIFIER (COLON <type_annotation>)? ASSIGN <expr>
```

**If:**
```
<if_expr> ::= IF LPAREN <expr> RPAREN <expr>
              (ELIF LPAREN <expr> RPAREN <expr>)*
              (ELSE <expr>)?
```

**Bloque:**
```
<block> ::= (<expr> SEMICOLON)* <expr>?
```

### Anotaciones de Tipo

```
<type_annotation> ::= IDENTIFIER
                    | LPAREN <type_list>? RPAREN TYPEARROW <type_annotation>
                    | LBRACKET <type_annotation> RBRACKET  // Iterable
```

## Parsing de Declaraciones

### Funciones

**Sintaxis:**
```hulk
function nombre(param1: Tipo, param2) => cuerpo;
```

**Algoritmo:**
1. Consume `function`
2. Parse nombre (identificador)
3. Consume `(`
4. Parse lista de parámetros
5. Consume `)`
6. Si hay `:`, parse tipo de retorno
7. Consume `=>`
8. Parse cuerpo (expresión)
9. Consume `;`

**Código:**
```rust
fn parse_function_decl(&mut self) -> Result<FunctionDecl, ParseError> {
    self.consume(&Token::Function, "Expected 'function'")?;
    let name = self.parse_identifier()?;
    self.consume(&Token::LParen, "Expected '('")?;
    
    let params = if !self.check(&Token::RParen) {
        self.parse_param_list()?
    } else {
        Vec::new()
    };
    
    self.consume(&Token::RParen, "Expected ')'")?;
    
    let return_type = if self.match_token(&Token::Colon) {
        Some(self.parse_type_annotation()?)
    } else {
        None
    };
    
    self.consume(&Token::FuncArrow, "Expected '=>'")?;
    let body = self.parse_spanned_expr(Precedence::Lowest)?;
    self.consume(&Token::Semicolon, "Expected ';'")?;
    
    Ok(FunctionDecl { name, params, return_type, body })
}
```

### Tipos (Clases)

**Sintaxis:**
```hulk
type NombreTipo(param1, param2) inherits Padre(args) {
    atributo1 = expr;
    atributo2: Tipo = expr;
    
    metodo1(params) => body;
    metodo2(): Tipo => body;
}
```

**Características:**
- Constructor implícito con parámetros
- Herencia opcional con argumentos para constructor padre
- Atributos con inicialización
- Métodos con sintaxis de función

**Parsing:**
1. `type` keyword
2. Nombre del tipo
3. Parámetros del constructor
4. Opcional: `inherits ParentType(args)`
5. Bloque con atributos y métodos

### Macros

**Sintaxis:**
```hulk
def nombre(x, @symbolic, $placeholder, *body: Type) => expr;

def with_block(@name) {
    match @name {
        case Number => "número",
        case String => "cadena",
        default => "otro"
    }
};
```

**Tipos de Parámetros:**
- **Normal (`x`)**: Argumento evaluado normalmente
- **Simbólico (`@name`)**: Captura el nombre del argumento sin evaluarlo
- **Placeholder (`$var`)**: Sustituido por un identificador único generado
- **Body (`*code`)**: Captura código sin evaluar (como un bloque de código)

**Algoritmo:**
1. Consume `def`
2. Parse nombre (identificador)
3. Consume `(`
4. Parse lista de parámetros de macro
5. Consume `)`
6. Si hay `:`, parse tipo de retorno
7. Si hay `=>`, parse expresión; si hay `{`, parse bloque
8. Consume `;` opcional

**Código:**
```rust
fn parse_macro_decl(&mut self) -> Result<MacroDecl, ParseError> {
    self.consume(&Token::Def, "Expected 'def'")?;
    let name = self.parse_identifier()?;
    self.consume(&Token::LParen, "Expected '('")?;
    
    let params = if !self.check(&Token::RParen) {
        self.parse_macro_param_list()?
    } else {
        Vec::new()
    };
    
    self.consume(&Token::RParen, "Expected ')'")?;
    
    let return_type = if self.match_token(&Token::Colon) {
        Some(self.parse_type_annotation()?)
    } else {
        None
    };
    
    let body = if self.match_token(&Token::FuncArrow) {
        self.parse_spanned_expr(Precedence::Lowest)?
    } else {
        self.match_token(&Token::LBrace);
        self.parse_block_expr()?
    };
    
    self.match_token(&Token::Semicolon);
    
    Ok(MacroDecl { name, params, return_type, body })
}
```

### Protocolos

**Sintaxis:**
```hulk
protocol NombreProtocolo extends OtroProtocolo {
    metodo1(params): TipoRetorno;
    metodo2(): Tipo;
}
```

**Diferencias con tipos:**
- Solo firmas de métodos (sin cuerpo)
- Tipo de retorno obligatorio
- No hay atributos

## Parsing de Expresiones

### Let Bindings

**Sintaxis:**
```hulk
let x = 5, y = x + 3, z: Number = 10 in x + y + z
```

**Características:**
- Múltiples bindings separados por coma
- Tipo opcional
- Scope limitado al cuerpo (expresión después de `in`)
- Bindings pueden depender de anteriores

**Algoritmo:**
```rust
fn parse_let_expr(&mut self) -> Result<Expr, ParseError> {
    self.consume(&Token::Let, "Expected 'let'")?;
    
    let mut bindings = Vec::new();
    loop {
        let name = self.parse_identifier()?;
        
        let type_ann = if self.match_token(&Token::Colon) {
            Some(self.parse_type_annotation()?)
        } else {
            None
        };
        
        self.consume(&Token::Assign, "Expected '='")?;
        let init = self.parse_spanned_expr(Precedence::Lowest)?;
        
        bindings.push((name, type_ann, init));
        
        if !self.match_token(&Token::Comma) {
            break;
        }
    }
    
    self.consume(&Token::In, "Expected 'in'")?;
    let body = Box::new(self.parse_spanned_expr(Precedence::Lowest)?);
    
    Ok(Expr::Let { bindings, body })
}
```

### If-Elif-Else

**Sintaxis:**
```hulk
if (condicion1) expr1
elif (condicion2) expr2
elif (condicion3) expr3
else expr4
```

**Representación en AST:**
- `elif` se desazucara a `If` anidados en la rama `else`

```rust
If {
    cond: condicion1,
    then_expr: expr1,
    else_expr: If {
        cond: condicion2,
        then_expr: expr2,
        else_expr: If {
            cond: condicion3,
            then_expr: expr3,
            else_expr: expr4
        }
    }
}
```

**Algoritmo:**
```rust
fn parse_if_expr(&mut self) -> Result<Expr, ParseError> {
    self.consume(&Token::If, "Expected 'if'")?;
    self.consume(&Token::LParen, "Expected '('")?;
    let cond = Box::new(self.parse_spanned_expr(Precedence::Lowest)?);
    self.consume(&Token::RParen, "Expected ')'")?;
    
    let then_expr = Box::new(self.parse_spanned_expr(Precedence::Lowest)?);
    
    let else_expr = if self.match_token(&Token::Elif) {
        // Recursión: elif es otro if
        Box::new(Spanned::new(
            self.parse_if_expr()?,
            self.peek_pos()
        ))
    } else if self.match_token(&Token::Else) {
        Box::new(self.parse_spanned_expr(Precedence::Lowest)?)
    } else {
        // Sin else, default a bloque vacío
        Box::new(Spanned::new(
            Expr::Block(Vec::new()),
            self.peek_pos()
        ))
    };
    
    Ok(Expr::If { cond, then_expr, else_expr })
}
```

### Bloques

**Sintaxis:**
```hulk
{
    expr1;
    expr2;
    expr3
}
```

**Características:**
- Última expresión sin `;` es el valor retornado
- Expresiones intermedias deben terminar con `;`
- Bloque vacío `{}` retorna "nada" (unit)

**Parsing:**
```rust
fn parse_block(&mut self) -> Result<Expr, ParseError> {
    self.consume(&Token::LBrace, "Expected '{'")?;
    
    let mut exprs = Vec::new();
    
    while !self.check(&Token::RBrace) && !self.at_end() {
        let expr = self.parse_spanned_expr(Precedence::Lowest)?;
        exprs.push(expr);
        
        if !self.match_token(&Token::Semicolon) {
            // Última expresión sin ';'
            break;
        }
    }
    
    self.consume(&Token::RBrace, "Expected '}'")?;
    
    Ok(Expr::Block(exprs))
}
```

### Lambdas

**Sintaxis:**
```hulk
|x, y| => x + y
|a: Number, b: Number| => a * b
|| => 42
```

**Parsing:**
```rust
fn parse_lambda(&mut self) -> Result<Expr, ParseError> {
    self.consume(&Token::Pipe, "Expected '|'")?;
    
    let params = if !self.check(&Token::Pipe) {
        self.parse_param_list()?
    } else {
        Vec::new()
    };
    
    self.consume(&Token::Pipe, "Expected '|'")?;
    self.consume(&Token::FuncArrow, "Expected '=>'")?;
    
    let body = Box::new(self.parse_spanned_expr(Precedence::Lowest)?);
    
    Ok(Expr::Lambda { params, body })
}
```

### Vectores

**Literal:**
```hulk
[1, 2, 3, 4, 5]
```

**Comprehension:**
```hulk
[x^2 | x in range(10)]
[x + y | x in [1, 2], y in [3, 4]]  // No soportado en versión actual
```

**Indexación:**
```hulk
vector[0]
matrix[i][j]
```

## Manejo de Errores

### Tipos de ParseError

```rust
pub enum ParseError {
    // Error léxico propagado
    Lex(LexError),
    
    // Token inesperado
    UnexpectedToken {
        expected: String,
        found: String,
        pos: Position,
    },
    
    // Fin de archivo inesperado
    UnexpectedEOF(Position),
}
```

### Ejemplos de Errores

**Token inesperado:**
```hulk
let x = ;
//      ^
// Error: UnexpectedToken { expected: "expression", found: "Semicolon", pos: ... }
```

**EOF inesperado:**
```hulk
function foo(a, b
//               ^
// Error: UnexpectedEOF(Position { line: 1, col: 17 })
```

**Paréntesis sin cerrar:**
```hulk
let x = (5 + 3;
//            ^
// Error: UnexpectedToken { expected: "')'", found: "Semicolon", ... }
```

### Mensajes de Error

Los errores incluyen:
- ✅ **Posición exacta**: Línea y columna
- ✅ **Contexto**: Qué se esperaba vs qué se encontró
- ✅ **Descriptivos**: Mensajes legibles

**Ejemplo:**
```
Parse Error at line 5, column 12:
  Expected ')' after function parameters
  Found: '=>'
```

## Ejemplos Completos

### Ejemplo 1: Expresión Simple

**Input:**
```hulk
2 + 3 * 4
```

**Tokens:**
```
Number(2), Plus, Number(3), Star, Number(4), EOF
```

**Parsing (Pratt):**
```
parse_expr(Lowest)
  parse_prefix() → Number(2)
  current = Plus (prec = Sum)
    parse_expr(Sum+1 = Product)
      parse_prefix() → Number(3)
      current = Star (prec = Product)
        parse_expr(Product+1 = Unary)
          parse_prefix() → Number(4)
          no more operators
        → Number(4)
      → Binary(3, *, 4)
    → Binary(3, *, 4)
  → Binary(2, +, Binary(3, *, 4))
```

**AST:**
```rust
Expr::Binary(
    Box::new(Spanned::new(Expr::Number(2.0), pos1)),
    Op::Add,
    Box::new(Spanned::new(
        Expr::Binary(
            Box::new(Spanned::new(Expr::Number(3.0), pos2)),
            Op::Mul,
            Box::new(Spanned::new(Expr::Number(4.0), pos3))
        ),
        pos2
    ))
)
```

---

### Ejemplo 2: Función con Control de Flujo

**Input:**
```hulk
function factorial(n: Number): Number =>
    if (n <= 1) 1
    else n * factorial(n - 1);
```

**AST (simplificado):**
```rust
Declaration::Function(FunctionDecl {
    name: "factorial",
    params: [
        Param {
            name: "n",
            type_annotation: Some(TypeAnnotation::Name("Number"))
        }
    ],
    return_type: Some(TypeAnnotation::Name("Number")),
    body: Spanned::new(
        Expr::If {
            cond: Box::new(
                Expr::Binary(
                    Expr::Identifier("n"),
                    Op::Le,
                    Expr::Number(1.0)
                )
            ),
            then_expr: Box::new(Expr::Number(1.0)),
            else_expr: Box::new(
                Expr::Binary(
                    Expr::Identifier("n"),
                    Op::Mul,
                    Expr::Call(
                        "factorial",
                        [Expr::Binary(
                            Expr::Identifier("n"),
                            Op::Sub,
                            Expr::Number(1.0)
                        )]
                    )
                )
            )
        },
        pos
    )
})
```

---

### Ejemplo 3: Tipo con Herencia

**Input:**
```hulk
type Point(x, y) inherits Object() {
    x = x;
    y = y;
    
    distance(other: Point): Number =>
        sqrt((self.x - other.x)^2 + (self.y - other.y)^2);
}
```

**Parsing paso a paso:**

1. **Parse `type` keyword** → Token::Type
2. **Parse nombre** → "Point"
3. **Parse parámetros** → `(x, y)`
4. **Parse herencia** → `inherits Object()`
5. **Parse bloque `{`**
6. **Parse atributos:**
   - `x = x;`
   - `y = y;`
7. **Parse métodos:**
   - `distance(other: Point): Number => ...`
8. **Parse cierre `}`**

**AST:**
```rust
Declaration::Type(TypeDecl {
    name: "Point",
    params: [
        Param { name: "x", type_annotation: None },
        Param { name: "y", type_annotation: None },
    ],
    parent: Some(TypeInit {
        name: "Object",
        args: []
    }),
    attributes: [
        Attribute {
            name: "x",
            type_annotation: None,
            init: Spanned::new(Expr::Identifier("x"), pos)
        },
        Attribute {
            name: "y",
            type_annotation: None,
            init: Spanned::new(Expr::Identifier("y"), pos)
        },
    ],
    methods: [
        FunctionDecl {
            name: "distance",
            params: [
                Param {
                    name: "other",
                    type_annotation: Some(TypeAnnotation::Name("Point"))
                }
            ],
            return_type: Some(TypeAnnotation::Name("Number")),
            body: Spanned::new(
                Expr::Sqrt(
                    Box::new(
                        Expr::Binary(
                            // (self.x - other.x)^2
                            Expr::Binary(
                                Expr::AttributeAccess(
                                    Box::new(Expr::Identifier("self")),
                                    "x"
                                ),
                                Op::Sub,
                                Expr::AttributeAccess(
                                    Box::new(Expr::Identifier("other")),
                                    "x"
                                )
                            ),
                            Op::Pow,
                            Expr::Number(2.0)
                        ),
                        Op::Add,
                        // + (self.y - other.y)^2
                        // ...
                    )
                ),
                pos
            )
        }
    ]
})
```

---

### Ejemplo 4: Macro con Pattern Matching

**Input:**
```hulk
def assert_type(@value, expected_type) {
    match @value {
        case x: Number => if (expected_type == "Number") x else error(),
        case s: String => if (expected_type == "String") s else error(),
        default => error()
    }
};

let x = assert_type(42, "Number");
```

**Parsing:**

1. **`parse_macro_decl()`**
2. **Parse nombre** → "assert_type"
3. **Parse parámetros macro:**
   - `@value` (simbólico - captura nombre)
   - `expected_type` (normal)
4. **Parse body** → Bloque con `match`
5. **`parse_match_expr()`**
6. **Parse patrón** → `@value`
7. **Parse casos:**
   - `case x: Number => ...`
   - `case s: String => ...`
   - `default => ...`

**AST:**
```rust
Declaration::Macro(MacroDecl {
    name: "assert_type",
    params: [
        MacroParam::Symbolic("value"),  // @value
        MacroParam::Normal(Param { name: "expected_type", type_annotation: None }),
    ],
    return_type: None,
    body: Spanned::new(
        Expr::Block([
            Spanned::new(
                Expr::Match {
                    pattern: Box::new(Spanned::new(
                        Expr::Identifier("value"),  // @value expandido
                        pos
                    )),
                    cases: [
                        MatchCase {
                            pattern: Pattern::Typed {
                                name: "x",
                                type_annotation: TypeAnnotation::Name("Number")
                            },
                            body: Spanned::new(
                                Expr::If {
                                    cond: Box::new(Expr::Binary(
                                        Expr::Identifier("expected_type"),
                                        Op::Eq,
                                        Expr::String("Number")
                                    )),
                                    then_expr: Box::new(Expr::Identifier("x")),
                                    else_expr: Box::new(Expr::Call("error", []))
                                },
                                pos
                            )
                        },
                        MatchCase {
                            pattern: Pattern::Typed {
                                name: "s",
                                type_annotation: TypeAnnotation::Name("String")
                            },
                            body: Spanned::new(
                                Expr::If { /* ... */ },
                                pos
                            )
                        },
                        MatchCase {
                            pattern: Pattern::Default,
                            body: Spanned::new(
                                Expr::Call("error", []),
                                pos
                            )
                        }
                    ]
                },
                pos
            )
        ]),
        pos
    )
})
```

**Expansión de la macro:**
Cuando se llama `assert_type(42, "Number")`:
- `@value` captura el nombre "42" (como símbolo)
- El `match` compara el valor 42 contra los patrones
- Se ejecuta el caso `x: Number`
- Retorna `42`

---

### Ejemplo 5: Let con Expresiones Complejas

**Input:**
```hulk
let x = 5,
    y = x * 2,
    z = if (y > 5) y else 0
in {
    print(x);
    print(y);
    x + y + z
}
```

**Parsing:**

1. **`parse_let_expr()`**
2. **Parse bindings:**
   - `x = 5`
   - `y = x * 2` (depende de x)
   - `z = if (y > 5) y else 0` (depende de y)
3. **`in` keyword**
4. **Parse body** → Bloque `{ ... }`

**AST:**
```rust
Expr::Let {
    bindings: [
        ("x", None, Expr::Number(5.0)),
        ("y", None, Expr::Binary(
            Expr::Identifier("x"),
            Op::Mul,
            Expr::Number(2.0)
        )),
        ("z", None, Expr::If {
            cond: Expr::Binary(
                Expr::Identifier("y"),
                Op::Gt,
                Expr::Number(5.0)
            ),
            then_expr: Expr::Identifier("y"),
            else_expr: Expr::Number(0.0)
        })
    ],
    body: Box::new(Expr::Block([
        Expr::Print(Box::new(Expr::Identifier("x"))),
        Expr::Print(Box::new(Expr::Identifier("y"))),
        Expr::Binary(
            Expr::Binary(
                Expr::Identifier("x"),
                Op::Add,
                Expr::Identifier("y")
            ),
            Op::Add,
            Expr::Identifier("z")
        )
    ]))
}
```

---

### Ejemplo 5: Programa Completo

**Input:**
```hulk
protocol Comparable {
    compare(other: Self): Number;
}

type Rectangle(width, height) {
    width = width;
    height = height;
    
    area(): Number => self.width * self.height;
    
    compare(other: Rectangle): Number =>
        self.area() - other.area();
}

function max_area(r1: Rectangle, r2: Rectangle): Rectangle =>
    if (r1.compare(r2) > 0) r1
    else r2;

let rect1 = new Rectangle(10, 5),
    rect2 = new Rectangle(8, 7)
in max_area(rect1, rect2).area();
```

**AST estructura:**
```rust
Program {
    declarations: [
        // 1. Protocol
        Declaration::Protocol(ProtocolDecl {
            name: "Comparable",
            parent: None,
            methods: [
                MethodSignature {
                    name: "compare",
                    params: [Param { name: "other", type_annotation: Some(...) }],
                    return_type: TypeAnnotation::Name("Number")
                }
            ]
        }),
        
        // 2. Type
        Declaration::Type(TypeDecl {
            name: "Rectangle",
            params: [
                Param { name: "width", ... },
                Param { name: "height", ... }
            ],
            parent: None,
            attributes: [
                Attribute { name: "width", ... },
                Attribute { name: "height", ... }
            ],
            methods: [
                FunctionDecl { name: "area", ... },
                FunctionDecl { name: "compare", ... }
            ]
        }),
        
        // 3. Function
        Declaration::Function(FunctionDecl {
            name: "max_area",
            params: [
                Param { name: "r1", type_annotation: Some(...) },
                Param { name: "r2", type_annotation: Some(...) }
            ],
            return_type: Some(TypeAnnotation::Name("Rectangle")),
            body: Expr::If { ... }
        })
    ],
    
    // Expresión principal
    expr: Spanned::new(
        Expr::Let {
            bindings: [
                ("rect1", None, Expr::Instantiation { ... }),
                ("rect2", None, Expr::Instantiation { ... })
            ],
            body: Box::new(
                Expr::MethodCall(
                    Box::new(
                        Expr::Call("max_area", [
                            Expr::Identifier("rect1"),
                            Expr::Identifier("rect2")
                        ])
                    ),
                    "area",
                    []
                )
            )
        },
        pos
    )
}
```

---

## Testing

### Estructura de Tests

Los tests están en `src/parser/tests.rs`:

```rust
#[test]
fn test_simple_arithmetic() {
    let input = "2 + 3 * 4";
    let mut parser = Parser::new(input);
    let program = parser.parse_program().unwrap();
    
    // Verificar estructura del AST
    match &program.expr.node {
        Expr::Binary(left, Op::Add, right) => {
            assert!(matches!(left.node, Expr::Number(2.0)));
            match &right.node {
                Expr::Binary(l, Op::Mul, r) => {
                    assert!(matches!(l.node, Expr::Number(3.0)));
                    assert!(matches!(r.node, Expr::Number(4.0)));
                }
                _ => panic!("Expected multiplication")
            }
        }
        _ => panic!("Expected addition at top level")
    }
}
```

### Casos de Test

**Básicos:**
- ✅ Literales (números, cadenas, booleanos)
- ✅ Operadores binarios con precedencia
- ✅ Operadores unarios
- ✅ Paréntesis y agrupación

**Declaraciones:**
- ✅ Funciones simples
- ✅ Funciones con tipos
- ✅ Tipos sin herencia
- ✅ Tipos con herencia
- ✅ Protocolos

**Control de flujo:**
- ✅ If-else simple
- ✅ If-elif-else anidado
- ✅ While loops
- ✅ For loops

**Let y scope:**
- ✅ Let simple
- ✅ Let con múltiples bindings
- ✅ Let con dependencias entre bindings

**Objetos:**
- ✅ Instanciación
- ✅ Acceso a atributos
- ✅ Llamadas a métodos
- ✅ Llamadas a base

**Vectores:**
- ✅ Literales de vector
- ✅ Comprehensions
- ✅ Indexación

**Errores:**
- ✅ Token inesperado
- ✅ EOF inesperado
- ✅ Paréntesis desbalanceados

### Ejecutar Tests

```bash
# Todos los tests del parser
cargo test parser

# Test específico
cargo test parser::tests::test_function_declaration

# Con output detallado
cargo test parser -- --nocapture

# Solo tests que fallan
cargo test parser -- --test-threads=1
```

---

## Uso en el Compilador

### Pipeline Completo

```rust
use hulk_compiler::parser::Parser;
use hulk_compiler::ast::optimize::optimize_program;

// 1. Código fuente
let code = r#"
    function factorial(n) =>
        if (n <= 1) 1
        else n * factorial(n - 1);
    
    factorial(5)
"#;

// 2. Parser
let mut parser = Parser::new(code);
let program = parser.parse_program()?;

// 3. Optimizar (opcional)
let optimized = optimize_program(program);

// 4. Pretty print
println!("AST: {}", optimized.expr.node);
```

### Integración con Otras Fases

```
Código fuente (String)
    ↓
Lexer → Tokens
    ↓
Parser → AST (Program)
    ↓
Semantic Analyzer → AST anotado
    ↓
Optimizer → AST optimizado
    ↓
Codegen → Código ejecutable
```

---

## Casos Especiales y Consideraciones

### Precedencia y Asociatividad

**Potencia (asociatividad derecha):**
```hulk
2 ^ 3 ^ 4  →  2 ^ (3 ^ 4) = 2 ^ 81 = 2417851639229258349412352
```

**Suma (asociatividad izquierda):**
```hulk
a - b - c  →  (a - b) - c
```

### Ambigüedades Resueltas

**Punto decimal vs acceso a miembro:**
- `42.5` → Número (lexer lo resuelve)
- `42.toString` → Número + acceso a método (parser)

**Llamada vs lambda:**
- `f(x)` → Llamada a función
- `|x| => x + 1` → Lambda

**Paréntesis en múltiples contextos:**
- Agrupación: `(2 + 3)`
- Llamada: `func()`
- Parámetros de función: `function f(x)`
- Tipo función: `(Number, Number) -> Number`

### Limitaciones Actuales

- ⚠️ No soporta operadores definidos por el usuario
- ⚠️ No soporta sobrecarga de operadores
- ⚠️ Comprensiones de vectores limitadas (un solo iterador)
- ⚠️ No hay macros o metaprogramación
- ⚠️ Sin azúcar sintáctico para tuplas

---


## Resumen Técnico

| Aspecto | Detalle |
|---------|---------|
| **Tipo de parser** | Descendente recursivo con Pratt parsing |
| **Complejidad** | O(n) tiempo, O(n) espacio (tokens + AST) |
| **Lookahead** | 1 token (peek) |
| **Precedencia** | 11 niveles |
| **Asociatividad** | Izquierda (mayoría), derecha (^, :=) |
| **Gramática** | LL(1) con extensiones |
| **Recuperación de errores** | Ninguna (fail-fast) |
| **Posicionamiento** | Línea y columna en cada nodo |
| **Construcciones** | 40+ tipos de expresiones, 3 tipos de declaraciones |
| **Ambigüedades** | Resueltas por precedencia y lookahead |

---
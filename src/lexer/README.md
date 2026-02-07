# Hulk Lexer (Analizador Léxico)

Este módulo implementa el **Analizador Léxico (Lexer)** para el lenguaje de programación Hulk. Se encarga de transformar el texto del código fuente en una secuencia de tokens con significado.

## Índice
- [Visión General](#visión-general)
- [Arquitectura](#arquitectura)
- [Conjunto Completo de Tokens](#conjunto-completo-de-tokens)
- [Funcionamiento Interno Detallado](#funcionamiento-interno-detallado)
- [Métodos Principales](#métodos-principales)
- [Casos Especiales y Ambigüedades](#casos-especiales-y-ambigüedades)
- [Manejo de Errores](#manejo-de-errores)
- [Ejemplos de Tokenización](#ejemplos-de-tokenización)
- [Pruebas](#pruebas)

## Visión General

El lexer es la **primera fase del compilador**. Toma código fuente HULK en formato de texto y lo convierte en una secuencia de tokens (unidades léxicas) que el parser puede procesar.

## Visión General

El lexer es la **primera fase del compilador**. Toma código fuente HULK en formato de texto y lo convierte en una secuencia de tokens (unidades léxicas) que el parser puede procesar.

**Responsabilidades:**
- Ignorar espacios en blanco y comentarios
- Reconocer palabras clave, identificadores, literales y operadores
- Rastrear la posición exacta (línea, columna) de cada token
- Detectar errores léxicos (cadenas no terminadas, caracteres inválidos, etc.)
- Resolver ambigüedades (ej: distinguir entre `42.5` y `42.method()`)

**Características clave:**
- ✅ **Zero-copy**: Trabaja directamente sobre la cadena de entrada sin copias innecesarias
- ✅ **Lazy**: Produce tokens bajo demanda mediante el trait `Iterator`
- ✅ **Lookahead**: Puede ver el siguiente carácter sin consumirlo (`peek`)
- ✅ **Posicionamiento preciso**: Cada token sabe exactamente dónde está en el código
- ✅ **Manejo de errores**: Errores descriptivos con posición exacta

## Arquitectura

### Estructura Principal

```rust
pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,  // Iterador de caracteres con lookahead
    line: usize,                  // Línea actual (base 1)
    column: usize,                // Columna actual (base 1)
}
```

El lexer utiliza:
- **`Peekable<Chars<'a>>`**: Iterador de caracteres que permite "mirar hacia adelante" sin consumir
- **`line` y `column`**: Rastreo de posición para mensajes de error precisos
- **Lifetime `'a`**: El lexer no posee la cadena de entrada, solo la referencia

### Trait Iterator

```rust
impl<'a> Iterator for Lexer<'a> {
    type Item = Result<(Token, Position), LexError>;
    
    fn next(&mut self) -> Option<Self::Item>;
}
```

El lexer implementa `Iterator`, lo que permite:
```rust
let lexer = Lexer::new("let x = 5;");
for token_result in lexer {
    match token_result {
        Ok((token, pos)) => println!("{:?} at {:?}", token, pos),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
```

## Conjunto Completo de Tokens

El enum `Token` define todos los tipos de tokens que el lexer puede producir:

### Palabras Clave (Keywords)

| Token | Sintaxis | Descripción |
|-------|----------|-------------|
| `Function` | `function` | Declaración de función |
| `Let` | `let` | Binding local |
| `If` | `if` | Condicional |
| `Else` | `else` | Rama else |
| `Elif` | `elif` | Else-if |
| `While` | `while` | Bucle while |
| `For` | `for` | Bucle for |
| `In` | `in` | Iteración (for x in...) |
| `Type` | `type` | Declaración de clase |
| `New` | `new` | Instanciación |
| `Inherits` | `inherits` | Herencia de clase |
| `Protocol` | `protocol` | Declaración de protocolo |
| `Extends` | `extends` | Extensión de protocolo |
| `Is` | `is` | Verificación de tipo |
| `As` | `as` | Conversión de tipo |
| `Base` | `base` | Referencia a clase padre |
| `Print` | `print` | Función de impresión |
| `True` | `true` | Booleano verdadero |
| `False` | `false` | Booleano falso |

### Palabras Clave de Macros

| Token | Sintaxis | Descripción |
|-------|----------|-------------|
| `Def` | `def` | Declaración de macro |
| `Match` | `match` | Expresión de pattern matching |
| `Case` | `case` | Caso en match |
| `Default` | `default` | Caso por defecto en match |

### Identificadores y Literales

| Token | Ejemplo | Descripción |
|-------|---------|-------------|
| `Identifier(String)` | `x`, `nombre`, `var2` | Variables, funciones, tipos |
| `Number(f64)` | `42`, `3.14159`, `-5.0` | Números de punto flotante |
| `StringLiteral(String)` | `"hola"`, `"mundo\n"` | Cadenas de texto |

### Operadores Aritméticos

| Token | Sintaxis | Operación |
|-------|----------|-----------|
| `Plus` | `+` | Suma |
| `Minus` | `-` | Resta / Negación |
| `Star` | `*` | Multiplicación |
| `Slash` | `/` | División |
| `Percent` | `%` | Módulo |
| `Power` | `^` | Potencia |

### Operadores de Comparación

| Token | Sintaxis | Operación |
|-------|----------|-----------|
| `Equal` | `==` | Igualdad |
| `NotEqual` | `!=` | Desigualdad |
| `LessThan` | `<` | Menor que |
| `GreaterThan` | `>` | Mayor que |
| `LessThanEq` | `<=` | Menor o igual |
| `GreaterThanEq` | `>=` | Mayor o igual |

### Operadores Lógicos

| Token | Sintaxis | Operación |
|-------|----------|-----------|
| `And` | `&` | AND lógico |
| `Or` | `\|` | OR lógico |
| `Not` | `!` | NOT lógico |

### Operadores de Cadenas

| Token | Sintaxis | Operación |
|-------|----------|-----------|
| `Concat` | `@` | Concatenación simple / Argumento simbólico en macros |
| `ConcatSpace` | `@@` | Concatenación con espacio |

### Tokens de Macros

| Token | Sintaxis | Descripción |
|-------|----------|-------------|
| `Dollar` | `$` | Placeholder en parámetros de macro |

### Asignación y Funciones

| Token | Sintaxis | Descripción |
|-------|----------|-------------|
| `Assign` | `=` | Asignación / Definición |
| `DestructAssign` | `:=` | Asignación destructiva |
| `FuncArrow` | `=>` | Función lambda / método |
| `TypeArrow` | `->` | Anotación de tipo función |

### Puntuación y Delimitadores

| Token | Sintaxis | Uso |
|-------|----------|-----|
| `LParen` | `(` | Paréntesis izquierdo |
| `RParen` | `)` | Paréntesis derecho |
| `LBrace` | `{` | Llave izquierda |
| `RBrace` | `}` | Llave derecha |
| `LBracket` | `[` | Corchete izquierdo |
| `RBracket` | `]` | Corchete derecho |
| `Comma` | `,` | Separador |
| `Colon` | `:` | Anotación de tipo |
| `Dot` | `.` | Acceso a miembros |
| `Semicolon` | `;` | Separador de expresiones |

### Tokens Especiales

| Token | Descripción |
|-------|-------------|
| `EOF` | Fin de archivo (no se emite actualmente) |
| `Unknown(char)` | Carácter no reconocido |

## Funcionamiento Interno Detallado

### Flujo de Tokenización

El proceso de tokenización sigue estos pasos:

```
Input: "let x = 42;"
  ↓
1. skip_whitespace()   → Ignorar espacios
  ↓
2. next_char()         → Consumir 'l'
  ↓
3. Identificar tipo    → Es letra → lex_identifier_or_keyword()
  ↓
4. Leer completo       → "let"
  ↓
5. Match keyword       → Token::Let
  ↓
6. Retornar            → Ok((Token::Let, Position{line:1, col:1}))
  ↓
7. Repetir para cada token...
```

### Algoritmo Principal (Iterator::next)

```rust
fn next(&mut self) -> Option<Self::Item> {
    self.skip_whitespace();          // 1. Saltar espacios
    let pos = self.current_pos();     // 2. Guardar posición
    let c = self.next_char()?;        // 3. Consumir primer carácter
    
    // 4. Decidir qué hacer según el carácter
    match c {
        '/' => // Comentarios o división
        '"' => // String literal
        '0'..='9' => // Número
        'a'..='z' | 'A'..='Z' | '_' => // Identificador/keyword
        '+' | '-' | '*' => // Operadores
        // ... etc
    }
}
```

## Métodos Principales

### `new(input: &'a str) -> Self`

Crea un nuevo lexer para la entrada dada.

```rust
let lexer = Lexer::new("let x = 5;");
```

**Inicializa:**
- `chars`: Iterador peekable sobre los caracteres de input
- `line`: 1 (líneas base 1)
- `column`: 1 (columnas base 1)

---

### `current_pos(&self) -> Position`

Devuelve la posición actual del lexer.

```rust
Position { line: 1, column: 5 }
```

Usado para asociar cada token con su ubicación exacta en el código fuente.

---

### `next_char(&mut self) -> Option<char>`

Consume y devuelve el siguiente carácter, **actualizando automáticamente** `line` y `column`.

**Comportamiento:**
- Si encuentra `\n`: incrementa `line`, resetea `column` a 1
- Otro carácter: incrementa `column`
- Fin de entrada: devuelve `None`

```rust
let c = self.next_char()?; // Consume y trackea posición
```

---

### `peek_char(&mut self) -> Option<&char>`

Devuelve una referencia al **siguiente carácter SIN consumirlo**.

**Crucial para:**
- Distinguir `=` de `==`
- Distinguir `@` de `@@`
- Distinguir `:` de `:=`
- Distinguir `42.5` (número) de `42.method()` (llamada)
- Detectar comentarios (`//`, `/*`)

```rust
if let Some(&next) = self.peek_char() {
    if next == '=' {
        self.next_char(); // Consumir el '='
        return Token::Equal;
    }
}
return Token::Assign;
```

---

### `skip_whitespace(&mut self)`

Salta todos los espacios en blanco consecutivos (espacio, tab, newline, etc).

**Comportamiento:**
- Usa `peek_char()` para ver sin consumir
- Si es whitespace: `next_char()` para consumir
- Repite hasta encontrar no-whitespace

```rust
fn skip_whitespace(&mut self) {
    while let Some(&c) = self.peek_char() {
        if c.is_whitespace() {
            self.next_char();
        } else {
            break;
        }
    }
}
```

---

### `lex_number(&mut self, first: char) -> Token`

Analiza un número (entero o flotante) empezando por `first`.

**Algoritmo:**
1. Crear string con `first`
2. Consumir todos los dígitos consecutivos (parte entera)
3. Si encuentra `.`:
   - Clonar iterador para lookahead
   - Ver si después del `.` hay dígito
   - Si sí: consumir `.` y parte decimal
   - Si no: es acceso a método (`42.foo`), no consumir
4. Parsear string a `f64`

**Ejemplo:**
```rust
Input: "42.5 + 3"
       ^
lex_number('4') →
  - Lee '4', '2'
  - Encuentra '.'
  - Peek: siguiente es '5' (dígito)
  - Lee '.', '5'
  - Resultado: Token::Number(42.5)

Input: "42.foo"
       ^
lex_number('4') →
  - Lee '4', '2'
  - Encuentra '.'
  - Peek: siguiente es 'f' (NO dígito)
  - NO consume el '.'
  - Resultado: Token::Number(42.0)
```

---

### `lex_string(&mut self, start_pos: Position) -> Result<Token, LexError>`

Analiza una cadena literal, manejando secuencias de escape.

**Secuencias de escape soportadas:**
- `\n` → newline
- `\t` → tab
- `\r` → carriage return
- `\\` → backslash
- `\"` → comillas dobles
- `\x` (otro) → carácter literal

**Algoritmo:**
1. Consumir caracteres hasta encontrar `"` de cierre
2. Si encuentra `\`: modo escape activado
3. En modo escape: siguiente carácter se procesa según tabla
4. Si llega al final sin `"`: error `UnterminatedString`

**Ejemplo:**
```rust
Input: "\"Hola\\nmundo\""
        ^
lex_string() →
  Consume: H o l a \ n m u n d o "
  Procesa: H o l a \n m u n d o
  Resultado: Token::StringLiteral("Hola\nmundo")
```

---

### `lex_identifier_or_keyword(&mut self, first: char) -> Token`

Analiza un identificador o palabra clave empezando por `first`.

**Reglas de identificador:**
- Primer carácter: letra (a-z, A-Z) solamente
- Caracteres siguientes: letras, dígitos o `_`
- Válidos: `x`, `nombre`, `var2`, `mi_variable`
- Inválidos: `_temp` (empieza con `_`), `2var` (empieza con dígito)

**Algoritmo:**
1. Crear string con `first`
2. Consumir caracteres mientras sean alphanumeric o `_`
3. Match contra tabla de palabras clave
4. Si match: retornar token keyword
5. Si no: retornar `Token::Identifier(string)`

**Tabla de keywords:**
```rust
match ident.as_str() {
    "function" => Token::Function,
    "let" => Token::Let,
    "if" => Token::If,
    "else" => Token::Else,
    "while" => Token::While,
    "for" => Token::For,
    "type" => Token::Type,
    "new" => Token::New,
    "inherits" => Token::Inherits,
    "protocol" => Token::Protocol,
    "is" => Token::Is,
    "as" => Token::As,
    "print" => Token::Print,
    "true" => Token::True,
    "false" => Token::False,
    "in" => Token::In,
    "base" => Token::Base,
    "elif" => Token::Elif,
    "extends" => Token::Extends,
    _ => Token::Identifier(ident),
}
```

## Casos Especiales y Ambigüedades

### 1. Números vs Acceso a Métodos

**Problema:** Distinguir entre `42.5` (número flotante) y `42.toString()` (método).

**Solución:**
```rust
if let Some(&'.') = self.peek_char() {
    let mut clone = self.chars.clone();  // Clonar para lookahead
    clone.next(); // Skip '.'
    if let Some(&next_c) = clone.peek() {
        if next_c.is_ascii_digit() {
            // Es parte decimal: 42.5
            self.next_char(); // Consumir '.'
            // ... leer decimales
        }
    }
    // No es dígito: 42.toString(), NO consumir '.'
}
```

**Ejemplos:**
- `42.5` → `Token::Number(42.5)`
- `42.toString` → `Token::Number(42.0)`, `Token::Dot`, `Token::Identifier("toString")`

---

### 2. Operadores Multicarácter

**Problema:** Distinguir `=` de `==`, `<` de `<=`, `@` de `@@`, etc.

**Solución:** Usar `peek_char()` para lookahead de 1 carácter.

**Ejemplos:**
```rust
'=' => {
    if let Some(&'=') = self.peek_char() {
        self.next_char();  // Consumir segundo '='
        Token::Equal       // ==
    } else if let Some(&'>') = self.peek_char() {
        self.next_char();
        Token::FuncArrow   // =>
    } else {
        Token::Assign      // =
    }
}

'<' => {
    if let Some(&'=') = self.peek_char() {
        self.next_char();
        Token::LessThanEq  // <=
    } else {
        Token::LessThan    // <
    }
}

'@' => {
    if let Some(&'@') = self.peek_char() {
        self.next_char();
        Token::ConcatSpace  // @@
    } else {
        Token::Concat       // @
    }
}

':' => {
    if let Some(&'=') = self.peek_char() {
        self.next_char();
        Token::DestructAssign  // :=
    } else {
        Token::Colon           // :
    }
}
```

---

### 3. Comentarios vs División

**Problema:** `/` puede ser división o inicio de comentario.

**Solución:**
```rust
'/' => {
    if let Some(&'/') = self.peek_char() {
        // Comentario de línea: //
        self.next_char(); // Consumir segundo /
        while let Some(c) = self.next_char() {
            if c == '\n' { break; }  // Fin de comentario
        }
        return self.next(); // Recursión para siguiente token
    } else if let Some(&'*') = self.peek_char() {
        // Comentario de bloque: /* ... */
        self.next_char(); // Consumir *
        let mut terminated = false;
        while let Some(c) = self.next_char() {
            if c == '*' && self.peek_char() == Some(&'/') {
                self.next_char(); // Consumir /
                terminated = true;
                break;
            }
        }
        if !terminated {
            return Some(Err(LexError::UnterminatedBlockComment(pos)));
        }
        return self.next(); // Recursión
    } else {
        // División simple: /
        Token::Slash
    }
}
```

**Ejemplos:**
- `10 / 2` → `Token::Number(10)`, `Token::Slash`, `Token::Number(2)`
- `// comentario` → (ignorado), siguiente token
- `/* bloque */` → (ignorado), siguiente token

---

### 4. Identificadores vs Palabras Clave

**Problema:** `let` es keyword, `letter` es identificador.

**Solución:** Leer identificador completo, luego match exacto:

```rust
fn lex_identifier_or_keyword(&mut self, first: char) -> Token {
    let mut ident = String::from(first);
    // Leer hasta encontrar no-alphanumeric
    while let Some(&c) = self.peek_char() {
        if c.is_alphanumeric() || c == '_' {
            ident.push(c);
            self.next_char();
        } else {
            break;
        }
    }
    
    // Match completo
    match ident.as_str() {
        "let" => Token::Let,
        _ => Token::Identifier(ident),
    }
}
```

**Ejemplos:**
- `let` → `Token::Let`
- `letter` → `Token::Identifier("letter")`
- `let_x` → `Token::Identifier("let_x")`

## Manejo de Errores

El lexer puede producir los siguientes errores (enum `LexError`):

### `UnterminatedString(Position)`

Se produce cuando una cadena literal no tiene comilla de cierre.

```hulk
let x = "hola mundo
        ^--- Error: UnterminatedString at line 1, col 9
```

---

### `UnterminatedBlockComment(Position)`

Se produce cuando un comentario de bloque `/* ... */` no se cierra.

```hulk
/* Este comentario no termina
   function foo() => 42;
^--- Error: UnterminatedBlockComment at line 1, col 1
```

---

### `UnexpectedCharacter(char, Position)`

Se produce cuando se encuentra un carácter que no es válido en HULK.

```hulk
let x = 5 $ 3;
          ^--- Error: UnexpectedCharacter('$') at line 1, col 11
```

**Manejo:**
```rust
match lexer.next() {
    Some(Ok((token, pos))) => { /* procesar token */ }
    Some(Err(e)) => {
        eprintln!("Lexer error: {:?}", e);
        // Puede intentar recuperarse o abortar
    }
    None => { /* fin de archivo */ }
}
```

## Ejemplos de Tokenización

### Ejemplo 1: Let Simple

**Input:**
```hulk
let x = 42;
```

**Tokens producidos:**
```
Token::Let              at Position { line: 1, column: 1 }
Token::Identifier("x")  at Position { line: 1, column: 5 }
Token::Assign           at Position { line: 1, column: 7 }
Token::Number(42.0)     at Position { line: 1, column: 9 }
Token::Semicolon        at Position { line: 1, column: 11 }
```

---

### Ejemplo 2: Función con Operadores

**Input:**
```hulk
function add(a, b) => a + b;
```

**Tokens:**
```
Token::Function
Token::Identifier("add")
Token::LParen
Token::Identifier("a")
Token::Comma
Token::Identifier("b")
Token::RParen
Token::FuncArrow
Token::Identifier("a")
Token::Plus
Token::Identifier("b")
Token::Semicolon
```

---

### Ejemplo 3: Tipo con Herencia

**Input:**
```hulk
type Point(x, y) inherits Object() {
    x = x;
}
```

**Tokens:**
```
Token::Type
Token::Identifier("Point")
Token::LParen
Token::Identifier("x")
Token::Comma
Token::Identifier("y")
Token::RParen
Token::Inherits
Token::Identifier("Object")
Token::LParen
Token::RParen
Token::LBrace
Token::Identifier("x")
Token::Assign
Token::Identifier("x")
Token::Semicolon
Token::RBrace
```

---

### Ejemplo 4: Expresión con Comentarios

**Input:**
```hulk
// Calcular área
let r = 5.0;
let area = PI * r ^ 2; /* A = πr² */
```

**Tokens:**
```
(// Calcular área → ignorado)

Token::Let
Token::Identifier("r")
Token::Assign
Token::Number(5.0)
Token::Semicolon
Token::Let
Token::Identifier("area")
Token::Assign
Token::Identifier("PI")
Token::Star
Token::Identifier("r")
Token::Power
Token::Number(2.0)
Token::Semicolon

(/* A = πr² */ → ignorado)
```

---

### Ejemplo 5: Concatenación de Cadenas

**Input:**
```hulk
"Hola" @ " " @@ "mundo"
```

**Tokens:**
```
Token::StringLiteral("Hola")
Token::Concat
Token::StringLiteral(" ")
Token::ConcatSpace
Token::StringLiteral("mundo")
```

---

### Ejemplo 6: Ambigüedad de Punto

**Input:**
```hulk
let x = 42.5;
let y = 42.toString();
```

**Tokens:**
```
Token::Let
Token::Identifier("x")
Token::Assign
Token::Number(42.5)      // Punto consumido como parte decimal
Token::Semicolon
Token::Let
Token::Identifier("y")
Token::Assign
Token::Number(42.0)      // Punto NO consumido
Token::Dot               // Punto como operador
Token::Identifier("toString")
Token::LParen
Token::RParen
Token::Semicolon
```

---

## Pruebas

El lexer incluye un conjunto completo de pruebas unitarias que cubren:

### Casos Básicos
- ✅ Tokenización de keywords
- ✅ Números enteros y flotantes
- ✅ Cadenas con escapes
- ✅ Identificadores válidos
- ✅ Todos los operadores

### Casos Complejos
- ✅ Función Ackermann completa
- ✅ Definiciones de tipos con herencia
- ✅ Protocolos y métodos
- ✅ Expresiones anidadas

### Casos Edge
- ✅ Comentarios dentro de expresiones
- ✅ Ambigüedad número/método (`42.5` vs `42.toString()`)
- ✅ Operadores multicarácter (`==`, `@@`, `:=`)
- ✅ Cadenas no terminadas (error)
- ✅ Comentarios de bloque no terminados (error)

### Ejecutar Tests

```bash
# Ejecutar todos los tests del lexer
cargo test lexer

# Ver output detallado de tokens
cargo test lexer -- --nocapture

# Test específico
cargo test lexer::tests::test_ackermann_function
```

### Ejemplo de Test

```rust
#[test]
fn test_simple_let() {
    let input = "let x = 42;";
    let lexer = Lexer::new(input);
    let tokens: Vec<_> = lexer.map(|r| r.unwrap().0).collect();
    
    assert_eq!(tokens, vec![
        Token::Let,
        Token::Identifier("x".to_string()),
        Token::Assign,
        Token::Number(42.0),
        Token::Semicolon,
    ]);
}
```

## Uso en el Compilador

El lexer se integra con el parser de la siguiente manera:

```rust
use crate::lexer::Lexer;
use crate::parser::Parser;

// Código HULK
let code = r#"
    function factorial(n) =>
        if (n <= 1) 1
        else n * factorial(n - 1);
"#;

// Crear lexer
let lexer = Lexer::new(code);

// Pasar al parser
let mut parser = Parser::new_from_lexer(lexer);

// Parsear programa
match parser.parse_program() {
    Ok(ast) => println!("Parsed: {:?}", ast),
    Err(e) => eprintln!("Error: {:?}", e),
}
```

## Optimizaciones y Consideraciones

### Rendimiento
- ✅ **Zero-copy**: No copia la entrada, trabaja sobre referencias
- ✅ **Lazy**: Solo produce tokens cuando se necesitan
- ✅ **O(n)**: Un solo paso sobre la entrada
- ✅ **Sin backtracking**: Lookahead máximo de 1 carácter

### Limitaciones Actuales
- ⚠️ Solo soporta números en base 10 (no hexadecimal, binario, etc.)
- ⚠️ No soporta notación científica (ej: `1e10`)
- ⚠️ Unicode en identificadores limitado (solo ASCII alphanumeric + `_`)

### Posibles Mejoras Futuras
- [ ] Soporte para literales de caracteres (`'a'`)
- [ ] Números en diferentes bases (0x, 0b, 0o)
- [ ] Notación científica (1.5e10)
- [ ] Interpolación de cadenas (`"x = \(x)"`)
- [ ] Raw strings (sin escapes)
- [ ] Comentarios de documentación (///)

---

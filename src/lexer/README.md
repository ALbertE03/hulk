# Hulk Lexer (Analizador Léxico)

Este módulo implementa el **Analizador Léxico (Lexer)** para el lenguaje de programación Hulk. Se encarga de transformar el texto del código fuente en una secuencia de tokens con significado.

## Detalles de Implementación

- **Implementación**: El lexer está construido desde cero utilizando `std::iter::Peekable` sobre flujos de caracteres.
- **Patrón Iterador**: La estructura `Lexer` implementa el trait `Iterator<Item = Result<(Token, Position), LexError>>`.
- **Rastreo de Posición**: Cada token incluye su línea y columna exactas.
- **Manejo de Errores**: Reporta errores específicos para:
  - Cadenas no terminadas.
  - Comentarios de bloque no terminados.
  - Caracteres inesperados.
- **Robustez**: Maneja características del lenguaje, incluyendo:
  - Expresiones anidadas.
  - Concatenación de cadenas (`@`, `@@`).
  - Asignación destructiva (`:=`).
  - Funciones de flecha (`=>`).
  - Comentarios (Línea `//` y Bloque `/* ... */`).

## Cómo Funciona Internamente

El lexer opera mediante un paso único sobre la cadena de entrada, utilizando una lógica de "mirada hacia adelante" (lookahead).

### 1. El Motor: Chars y Peekable
Utilizamos el iterador de caracteres nativo de Rust (`Chars`), envuelto en un `Peekable`. Esto nos permite:
- **`next()`**: Consumir el carácter actual y avanzar.
- **`peek()`**: Ver el siguiente carácter sin consumirlo. Esto es vital para distinguir entre `=` y `==`, o `42` y `42.5`.

### 2. Ciclo de Tokenización
El método principal es `next()`, que sigue estos pasos:
1. **Ignorar espacios**: Salta espacios, tabulaciones y saltos de línea.
2. **Identificar Comentarios**: Si encuentra `//` o `/*`, entra en un modo de salto hasta encontrar el fin del comentario y reanuda la búsqueda de tokens de forma recursiva.
3. **Coincidencia de Patrones**:
   - **Dígitos**: Si empieza con un número, llama a `lex_number()`. Este tiene lógica para no confundir el punto decimal con una llamada a método (ej: `42.foo`).
   - **Letras/_**: Si empieza con letra o guion bajo, lee el identificador completo y luego verifica si es una **Palabra Reservada** mediante un `match`.
   - **Símbolos Especiales**: Maneja operadores multicarácter como `=>` o `@@` mirando el siguiente carácter antes de decidir qué token retornar.


## Conjunto de Tokens

El enum `Token` distingue entre:
- **Palabras Clave (Keywords)**: `function`, `let`, `if`, `else`, `while`, `for`, `type`, `protocol`, etc.
- **Identificadores**: Nombres de variables y funciones.
- **Literales**: Números (punto flotante) y Cadenas de texto.
- **Operadores**: Aritméticos (`+`, `-`, `*`, `/`, `%`, `^`), Lógicos (`&`, `|`, `!`), Comparación (`==`, `!=`, `<`, `>`, `<=`, `>=`).
- **Puntuación**: Paréntesis, llaves, corchetes, punto y coma, coma, punto, dos puntos.
- **Especiales**: Flecha (`=>`), Asignación Destructiva (`:=`).

## Uso

```rust
use crate::lexer::Lexer;
use crate::lexer::tokens::Token;

let code = "let x = 42;";
let lexer = Lexer::new(code);

for token in lexer {
    println!("{:?}", token);
}
```

## Pruebas (Testing)

El lexer está completamente probado con pruebas unitarias que cubren casos extremos, programas complejos (función Ackermann, definiciones de tipos) y resolución de ambigüedades.

Para ejecutar las pruebas y ver los tokens generados:
```bash
cargo test lexer -- --nocapture
```

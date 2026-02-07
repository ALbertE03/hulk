# Parser de HULK

Este módulo implementa un Analizador Sintáctico (Parser) descendente recursivo para el lenguaje de programación HULK.

## Descripción General

El parser transforma un flujo de tokens (producidos por el `Lexer`) en un Árbol de Sintaxis Abstracta (AST). Está diseñado para ser:

- **Descendente Recursivo**: Utiliza una estrategia de análisis de arriba hacia abajo (top-down).
- **Resiliente a Errores**:Proporcionando mensajes de error descriptivos con información de posición.
- **Mapeo de Fuente Completo**: Rastrea la posición (`Position`) (línea/columna) de cada nodo del AST mediante el envoltorio `Spanned<T>`.

## Cómo Funciona

El parser opera siguiendo la gramática del lenguaje de forma recursiva. A continuación se detalla el proceso:

1. **Inicialización**: Se crea con una cadena de texto, la cual pasa primero por el `Lexer` para obtener una lista de tokens.
2. **Análisis de Arriba hacia Abajo**: Comienza desde la regla más general (`parse_program`) y va descendiendo a reglas más específicas (`parse_declaration`, `parse_expr`, etc.).
3. **Consumo de Tokens**:
    - `peek()`: Observa el token actual sin avanzar.
    - `advance()`: Consume el token actual y avanza al siguiente.
    - `consume(token, error_msg)`: Verifica que el token actual sea el esperado; si lo es, avanza, de lo contrario lanza un error sintáctico.
4. **Construcción del AST**: Cada función de análisis devuelve un nodo del AST (como `Expr`, `Declaration`, etc.) envuelto en un `Spanned`, capturando así su ubicación exacta en el código fuente.
5. **Manejo de Errores**: Utiliza el tipo `Result<T, ParseError>` para propagar errores hacia arriba, permitiendo una salida limpia en caso de sintaxis inválida.

## Arquitectura

El núcleo del parser es la estructura `Parser` ubicada en `mod.rs`.

### Punto de Entrada Principal

```rust
let mut parser = Parser::new(input);
let program = parser.parse_program()?;
```


## Soporte de Gramática

El parser soporta la totalidad de las construcciones del lenguaje HULK, incluyendo:

- **Declaraciones**: Funciones (`function`), Tipos (`type`) con herencia y constructores, y Protocolos (`protocol`).
- **Literales**: Números (punto flotante), Cadenas de texto (Strings), Booleanos (`true`, `false`) e Identificadores.
- **Operaciones Binarias**:
    - **Aritméticas**: `+`, `-`, `*`, `/`, `%` (módulo), `^` (potencia).
    - **Lógicas**: `&` (And), `|` (Or).
    - **Comparación**: `==`, `!=`, `<`, `<=`, `>`, `>=`.
    - **Concatenación**: `@` (simple), `@@` (con espacio).
- **Operaciones Unarias**: `-` (negación numérica), `!` (not lógico).
- **Control de Flujo**: Expresiones `if-else` condicionales, bucles `while` y bucles `for` sobre iterables.
- **Bloques y Ámbito**:
    - Expresiones `let`-`in` para declaración de variables locales.
    - Asignación destructiva con el operador `:=`.
    - Bloques de código `{ ... }` que devuelven el valor de la última expresión.
- **Funciones y Objetos**:
    - Llamadas a funciones globales.
    - Llamadas a métodos (`obj.metodo(...)`).
    - Acceso a atributos (`obj.atributo`).
    - Instanciación de tipos con la palabra clave `new`.
    - Llamadas a la clase base mediante `base()`.
- **Programación Funcional**: Lambdas de estilo flecha `(params) => cuerpo`.
- **Tipado Dinámico**: Chequeo de tipos con `is` y casteo de tipos con `as`.
- **Vectores**:
    - Literales de vector: `[e1, e2, ...]`.
    - Generadores de vectores (comprensión de listas): `[exp || x in iterable]`.
    - Indexación de vectores: `vector[indice]`.
- **Funciones Matemáticas Intrínsecas**: `sqrt(x)`, `sin(x)`, `cos(x)`, `exp(x)`, `log(base, x)`, `rand()`.
- **Constantes Matemáticas**: `PI`, `E`.


### Ejecución de Pruebas

```bash
cargo test parser::tests
```
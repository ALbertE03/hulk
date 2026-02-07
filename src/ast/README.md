# Documentación del Árbol de Sintaxis Abstracta (AST)

Este módulo define la estructura del Árbol de Sintaxis Abstracta (AST) para el compilador del lenguaje Hulk.

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

- **`Function`**: Declaración de funciones globales.
- **`Type`**: Definición de clases o tipos, incluyendo herencia y polimorfismo.
- **`Protocol`**: Definición de contratos que los tipos pueden implementar.

### Detalles de las Declaraciones

- **`FunctionDecl`**: Define una función con nombre, parámetros, tipo de retorno opcional y un cuerpo (expresión).
- **`TypeDecl`**: Define un tipo (clase) con constructor, herencia (`parent`), atributos y métodos.
- **`ProtocolDecl`**: Define un protocolo con una lista de firmas de métodos (`MethodSignature`).

## Expresiones (`Expr`)

El enum `Expr` es el núcleo del AST, representando cualquier construcción que pueda ser evaluada a un valor.

### Primitivos
- `Number(f64)`: Números de punto flotante.
- `String(String)`: Cadenas de texto.
- `Boolean(bool)`: Valores `true` o `false`.
- `Identifier(String)`: Variables o nombres de referencia.

### Operaciones
- **Binarias** (`Binary`): Operaciones como suma, resta, comparaciones, lógicas, etc.
- **Unarias** (`Unary`): Negación numérica (`-`) o lógica (`!`).

### Control de Flujo
- **`If`**: Estructura condicional `if-else`.
- **`While`**: Bucle `while`.
- **`For`**: Bucle `for`, que itera sobre una expresión iterable.

### Bloques y Alcance
- **`Block`**: Una secuencia de expresiones encerradas entre llaves `{ ... }`.
- **`Let`**: Declaración de variables locales (`let x = ... in ...`).
- **`Assignment`**: Asignación a una variable existente (`x := ...`).

### Funciones y Llamadas
- **`Call`**: Llamada a una función global (`print("hola")`).
- **`MethodCall`**: Llamada a un método de un objeto (`obj.metodo()`).
- **`Instantiation`**: Creación de una nueva instancia de un tipo (`new Point(1, 2)`).

### Verificación de Tipos
- **`Is`**: Comprueba si una expresión es de un tipo determinado.
- **`As`**: Realiza una conversión de tipo (casting) segura o insegura.

### Vectores
- **`VectorLiteral`**: Definición literal de un vector (`[1, 2, 3]`).
- **`VectorGenerator`**: Definición de vector por comprensión (`[x * 2 || x in range(10)]`).
- **`Indexing`**: Acceso a elementos de un vector (`vec[0]`).

## Operadores (`Op` y `UnOp`)

Los operadores se definen en las enumeraciones `Op` (binarios) y `UnOp` (unarios).

- **Aritméticos**: `Add` (+), `Sub` (-), `Mul` (*), `Div` (/), `Mod` (%), `Pow` (^).
- **Comparación**: `Eq` (==), `Neq` (!=), `Lt` (<), `Gt` (>), `Le` (<=), `Ge` (>=).
- **Lógicos**: `And` (&), `Or` (|).
- **Cadenas**: `Concat` (@), `ConcatSpace` (@@).

## Notas Adicionales

- Muchos nodos utilizan `Spanned<T>`, una estructura auxiliar que incluye la información de ubicación en el código fuente (línea, columna, offset) junto con el nodo, para reportar errores con precisión.

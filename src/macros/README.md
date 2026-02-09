# Módulo de Expansión de Macros

## Índice
- [Visión General](#visión-general)
- [¿Qué son las Macros?](#qué-son-las-macros)
- [Sintaxis de Macros](#sintaxis-de-macros)
- [Tipos de Parámetros](#tipos-de-parámetros)
- [Variable Hygiene](#variable-hygiene)
- [Pattern Matching](#pattern-matching)
- [Arquitectura](#arquitectura)
- [Ejemplos Completos](#ejemplos-completos)
- [Limitaciones](#limitaciones)

---

## Visión General

Las **macros** en HULK son una forma de metaprogramación que permite extender el lenguaje con construcciones sintácticas personalizadas. A diferencia de las funciones normales, las macros:

1. ✅ **Se expanden en tiempo de compilación** (no en runtime)
2. ✅ **Trabajan con el AST** (no con valores evaluados)
3. ✅ **Permiten sintaxis personalizada** (como `repeat(10) { ... }`)
4. ✅ **Hacen pattern matching estructural** sobre código
5. ✅ **Son más rápidas** (no hay overhead de llamada a función)

---

## ¿Qué son las Macros?

### Problema: Funciones Limitadas

Supón que quieres algo como:

```hulk
repeat(10) {
    print("Hello");
}
```

Con funciones regulares:

```hulk
function repeat(times: Number, expr: () -> Object): Object {
    let total = times in
        while (total > 0) {
            total := total - 1;
            expr();
        };
}

// Usar:
repeat(10, () => {  // ❌ Sintaxis más pesada
    print("Hello");
});
```

**Problemas:**
- ❌ Sintaxis incómoda (necesitas lambda explícito)
- ❌ El bloque es una "caja negra" (la función no puede inspeccionarlo)
- ❌ Overhead de llamada a función en runtime

### Solución: Macros

```hulk
def repeat(n: Number, *expr: Object): Object =>
    let total = n in
        while (total > 0) {
            total := total - 1;
            expr;  // ← Interpolación directa del código
        };

// Usar:
repeat(10) {
    print("Hello");
}
```

**Expansión en tiempo de compilación:**

```hulk
// El código anterior se transpila a:
let total$$1 = 10 in
    while (total$$1 > 0) {
        total$$1 := total$$1 - 1;
        {
            print("Hello");
        };
    }
```

---

## Sintaxis de Macros

### Definición Básica

```hulk
def nombre_macro(parametros): TipoRetorno =>
    cuerpo_de_la_macro;
```

**Diferencias con funciones:**
- `def` en lugar de `function`
- Parámetros pueden tener prefijos especiales: `@`, `$`, `*`
- El cuerpo se **transpila**, no se ejecuta

---

## Tipos de Parámetros

### 1. **Parámetro Normal** (sin prefijo)

Parámetro estándar que recibe una expresión:

```hulk
def double(x: Number): Number => x * 2;

double(5 + 3)  →  (5 + 3) * 2
```

**Expansión:**
El argumento se sustituye textualmente en el cuerpo.

---

### 2. **Parámetro Simbólico** (`@symbol`)

Permite pasar **el nombre de una variable** en lugar de su valor, habilitando asignación:

```hulk
def swap(@a: Object, @b: Object) {
    let temp: Object = a in {
        a := b;
        b := temp;
    }
}
```

**Uso:**

```hulk
let x: Object = 5, y: Object = "Hello" in {
    swap(@x, @y);
    print(x);  // "Hello"
    print(y);  // 5
};
```

**Expansión:**

```hulk
let x: Object = 5, y: Object = "Hello" in {
    let temp$$1 = x in {
        x := y;
        y := temp$$1;
    };
    print(x);
    print(y);
};
```

**¿Por qué `@`?**
Sin `@`, solo recibirías el **valor** de `x`, no su **nombre**. El `@` indica que la macro debe recibir el **símbolo** (identificador) para poder reasignarlo.

---

### 3. **Variable Placeholder** (`$symbol`)

Introduce una **nueva variable** en el scope donde se expande la macro:

```hulk
def repeat($iter: Number, n: Number, *expr: Object) {
    let iter: Number = 0, total: Number = n in {
        while (total > 0) {
            total := total - 1;
            expr;
            iter := iter + 1;
        };
    }
}
```

**Uso:**

```hulk
repeat(current, 10) {
    print(current);  // 0, 1, 2, ..., 9
};
```

**Expansión:**

```hulk
let current: Number = 0, total$$1: Number = 10 in {
    while (total$$1 > 0) {
        total$$1 := total$$1 - 1;
        {
            print(current);
        };
        current := current + 1;
    };
};
```

**¿Qué hace `$`?**
`$iter` en la definición se **renombra** a `current` cuando usas la macro. El cuerpo (`*expr`) puede usar esa variable.

---

### 4. **Body Argument** (`*expr`)

Captura un **bloque completo de código** (típicamente entre `{}`):

```hulk
def repeat(n: Number, *expr: Object): Object =>
    let total = n in
        while (total > 0) {
            total := total - 1;
            expr;
        };
```

**Uso:**

```hulk
repeat(3) {
    print("Hi");
    print("Bye");
}
```

**Expansión:**

```hulk
let total$$1 = 3 in
    while (total$$1 > 0) {
        total$$1 := total$$1 - 1;
        {
            print("Hi");
            print("Bye");
        };
    }
```

---

## Variable Hygiene

Las macros en HULK implementan **hygiene automática**: las variables internas se renombran para evitar conflictos de nombres.

### Problema sin Hygiene

```hulk
let total = 10 in repeat(total) {
    print(total);
};
```

Si `repeat` usa internamente una variable `total`, **sin hygiene** imprimiría `9, 8, 7, ...` (¡violando encapsulación!).

### Solución: Renombrado Automático

El compilador renombra variables internas:

```hulk
let total = 10 in {
    let total$$1 = total in  // ← Renombrado
        while (total$$1 > 0) {
            total$$1 := total$$1 - 1;
            {
                print(total);  // ← El 'total' externo no se ve afectado
            };
        }
}
```

**Resultado:** Imprime `10` tres veces.

---

## Pattern Matching

Las macros pueden hacer **pattern matching estructural** sobre el AST de los argumentos.

### Ejemplo: Macro `simplify`

```hulk
def simplify(expr: Number) {
    match(expr) {
        case (x: Number + x2: Number) => simplify(x) + simplify(x2);
        case (x: Number + 0) => simplify(x);
        case (x: Number - x2: Number) => simplify(x) - simplify(x2);
        case (x: Number - 0) => simplify(x);
        case (x: Number * x2: Number) => simplify(x) * simplify(x2);
        case (x: Number * 1) => simplify(x);
        case (x: Number * 0) => 0;
        case (x: Number / 1) => simplify(x);
        default => expr;
    };
}
```

**Uso:**

```hulk
print(simplify((42 + 0) * 1));
```

**Expansión:**

```hulk
print(42);
```

### ¿Cómo Funciona?

1. **Análisis sintáctico:** `(42 + 0) * 1` se parsea como:
   ```
   Binary(
       Binary(Number(42), Add, Number(0)),
       Mul,
       Number(1)
   )
   ```

2. **Pattern matching:**
   - Matchea `case (x: Number * 1)` con `x = Binary(42, Add, 0)`
   - Llama recursivamente: `simplify(x)`
   - Matchea `case (x: Number + 0)` con `x = Number(42)`
   - Devuelve `Number(42)`

3. **Código generado:** `42`

---

## Arquitectura

### Estructura del Módulo

El módulo `macros` está organizado en los siguientes archivos:

- **`mod.rs`**: Orquestador principal que expone la función pública `expand_macros()`
- **`utils.rs`** : Funciones utilitarias como `gensym()` para generación de nombres únicos
- **`visitors.rs`** : Implementaciones de visitors para la expansión de macros:
  - `SubstitutionVisitor`: Aplica sustituciones de variables y expresiones
  - `SanitizationVisitor`: Renombra variables para evitar captura (hygiene)
  - `MacroExpansionVisitor`: Expande llamadas a macros recursivamente
- **`context.rs`**: Contexto de expansión y lógica de pattern matching
- **`tests.rs`**: Suite de tests de macros

### Estructuras AST

#### `MacroDecl`
```rust
pub struct MacroDecl {
    pub name: String,
    pub params: Vec<MacroParam>,
    pub return_type: Option<TypeAnnotation>,
    pub body: Spanned<Expr>,
}
```

#### `MacroParam`
```rust
pub enum MacroParam {
    Normal { name: String, type_annotation: TypeAnnotation },
    Symbolic { name: String, type_annotation: TypeAnnotation },  // @
    Placeholder { name: String, type_annotation: TypeAnnotation }, // $
    Body { name: String, type_annotation: TypeAnnotation },       // *
}
```

#### `Pattern`
```rust
pub enum Pattern {
    Literal(Expr),
    Variable { name: String, type_annotation: Option<TypeAnnotation> },
    Binary { left: Box<Pattern>, op: Op, right: Box<Pattern> },
    Unary { op: UnOp, operand: Box<Pattern> },
    Call { func: String, args: Vec<Pattern> },
    Wildcard, // _
}
```

---

### Proceso de Expansión

```
┌─────────────────────┐
│  Parser → AST       │
│  (con macros)       │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│  Macro Expansion    │
│  1. Registrar       │
│  2. Sustituir       │
│  3. Higienizar      │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│  AST expandido      │
│  (sin macros)       │
└─────────────────────┘
```

### `MacroExpansionContext` (en `context.rs`)

```rust
pub struct MacroExpansionContext {
    macros: HashMap<String, MacroDecl>,
    substitutions: HashMap<String, String>,
}
```

**Métodos principales:**
- `register_macro()`: Registra una definición de macro
- `expand_program()`: Expande todas las macros en un programa (3 fases)
- `expand_macro_call()`: Expande una llamada específica
- `sanitize_expr()`: Aplica hygiene a expresiones
- `pattern_match()`: Hace pattern matching recursivo

### Utilidades (`utils.rs`)

- **`gensym(prefix: &str) -> String`**: Genera nombres únicos con formato `prefix$$N` usando un contador atómico

---

## Ejemplos Completos

### Ejemplo 1: `unless` (condicional invertido)

**Definición:**
```hulk
def unless(cond: Boolean, *then_expr: Object, *else_expr: Object): Object =>
    if (!cond) then_expr else else_expr;
```

**Uso:**
```hulk
unless(x < 0) {
    print("Positivo");
} else {
    print("Negativo");
}
```

**Expansión:**
```hulk
if (!(x < 0)) {
    print("Positivo");
} else {
    print("Negativo");
}
```

---

### Ejemplo 2: `with_resource` (gestión de recursos)

**Definición:**
```hulk
def with_resource($res: Object, init: Object, *body: Object): Object =>
    let res = init in {
        body;
        // Aquí iría cleanup de res
    };
```

**Uso:**
```hulk
with_resource(file, open("data.txt")) {
    print(file.read());
}
```

---

### Ejemplo 3: `assert` (verificación con mensaje)

**Definición:**
```hulk
def assert(cond: Boolean, msg: String): Object =>
    if (!cond) {
        print("Assertion failed: " @ msg);
        error();
    } else {
        {}
    };
```

**Uso:**
```hulk
assert(x > 0, "x debe ser positivo");
```

---

## Limitaciones

### Actuales
- ⚠️ **Pattern matching parcial**: No todos los patrones están implementados
- ⚠️ **Sin pattern guards**: No hay `if` condicionales en patterns
- ⚠️ **Hygiene básica**: Renombrado simple, no completo
- ⚠️ **Sin recursividad**: Una macro no puede llamarse a sí misma de forma segura
- ⚠️ **Sin expansión incremental**: Se expanden todas o ninguna

### Futuras Mejoras
- 🔮 Pattern matching exhaustivo con guards
- 🔮 Hygiene completa con captura de variables
- 🔮 Macros recursivas con límite de profundidad
- 🔮 Debugging: mostrar AST pre y post-expansión
- 🔮 Macro hygiene verification

---

## Comparación: Funciones vs Macros

| Característica | Funciones | Macros |
|---------------|-----------|--------|
| **Ejecución** | Runtime | Compile-time |
| **Trabaja con** | Valores evaluados | AST (sintaxis) |
| **Sintaxis** | Rígida (`func(args)`) | Flexible (`macro {...}`) |
| **Pattern matching** | ❌ | ✅ |
| **Performance** | Overhead de llamada | Zero overhead |
| **Recursión** | ✅ Segura | ⚠️ Limitada |
| **Debugging** | Fácil (stack traces) | Difícil (expansión) |
| **Composición** | ✅ Natural | ⚠️ Compleja |

---

## Resumen Técnico

| Aspecto | Detalle |
|---------|---------|
| **Fase** | Entre parser y semantic analyzer |
| **Entrada** | AST con `Declaration::Macro` |
| **Salida** | AST sin macros (solo `Function`, `Type`, `Protocol`) |
| **Algoritmo** | Tree walking + substitution |
| **Hygiene** | Renombrado con contador global atómico |
| **Pattern matching** | Recursivo sobre estructura AST |
| **Complejidad** | O(n × m) donde n = nodos AST, m = llamadas a macro |

---

**Sistema de macros completo para el compilador HULK** 🚀

# Generador de Código LLVM IR

Este módulo traduce el AST de HULK a **LLVM IR** (Intermediate Representation), un lenguaje intermedio tipado que puede ser compilado a código nativo para cualquier arquitectura soportada por LLVM (x86, ARM, RISC-V, etc.) usando `llc` o `clang`.

## Arquitectura del Backend

### Estrategia de Tipos
| HULK Type  | LLVM IR Type | Notas                                         |
|------------|-------------|-----------------------------------------------|
| `Number`   | `double`    | Tipo numérico universal (IEEE 754)            |
| `Boolean`  | `double`    | `1.0` = true, `0.0` = false                  |
| `String`   | `double`    | Puntero `i8*` codificado como `double` vía bitcast |
| Objetos    | `double`    | Puntero `i8*` a struct en heap, codificado como `double` |
| Vectores   | `double`    | Puntero `double*` a buffer en heap, codificado como `double` |

> **Nota**: Usar `double` como tipo universal simplifica enormemente el pipeline SSA. Los punteros se codifican/decodifican via `ptrtoint`/`inttoptr`/`bitcast`. LLVM optimiza estos patrones eficientemente.

### Modelo de Variables
- Se usa el patrón **alloca + store + load** para todas las variables.
- LLVM's `mem2reg` pass promueve automáticamente a registros SSA durante la optimización.

### Modelo de Memoria
- **Stack**: Variables locales via `alloca`.
- **Heap**: Objetos, strings concatenados y vectores via `@malloc`.
- **GC**: Recolección de basura por barrido (*sweep*) al finalizar el programa. Toda asignación dinámica se registra en un buffer global (`@.gc_buf`) y se libera al salir de `@main` con `@__hulk_gc_sweep()`.

## Características Implementadas

### 1. Expresiones Aritméticas Completas
- Operaciones: `fadd`, `fsub`, `fmul`, `fdiv`, `frem` (módulo)
- Potencia: `@llvm.pow.f64` (intrínseco LLVM)
- Negación: `fneg`

### 2. Comparaciones y Lógica
- Comparaciones: `fcmp oeq/one/olt/ogt/ole/oge` → `uitofp i1 to double`
- AND / OR: Conversión a `i1`, operación `and`/`or`, reconversión a `double`
- NOT: `fcmp oeq` contra `0.0`

### 3. Strings
- Literales: Almacenados como constantes globales `@.slit_N`
- Concatenación (`@`): `strlen` → `malloc` → `strcpy` → `strcat`
- Concatenación con espacio (`@@`): Inserta espacio entre operandos
- Print: Usa `@puts` para strings, `@printf` con formato `%.6g` para números

### 4. Control de Flujo
- **If/Else**: `alloca` + branches `br i1` + merge label (evita phi nodes)
- **While**: Loop con `br label %wcond` → condición → `br i1` → body/end
- **For**: Decodifica vector, itera con `icmp slt` + `getelementptr`
- **Block**: Evalúa secuencialmente, retorna último valor

### 5. Funciones
- Declaradas como `define double @nombre(double %param, ...)`
- Parámetros almacenados en `alloca` slots
- Llamadas via `call double @nombre(...)`
- `print()` tratado como builtin especial

### 6. Lambdas / Closures
- Emitidas como funciones anónimas `@__lambda_N(double* %__env, double %p0, ...)`
- **Captura de variables libres**: Se identifican variables del scope envolvente usadas en el cuerpo. Se crea un *environment* en heap (`malloc`) con los valores capturados.
- **Par closure**: Se empaquetan `[fn_ptr, env_ptr]` como un buffer de 2 doubles en heap.
- Las variables capturadas se leen del environment al inicio de la función lambda.

### 7. Sistema de Tipos y OOP
- **Clases**: Structs LLVM `%T.ClassName = type { i64, double, double, ... }`
  - Campo 0: `i64` tipo-id único (para `is`/`as` en runtime)
  - Campos 1+: atributos de instancia (padres primero, luego propios)
- **Constructores**: `@ClassName_new(double %param, ...) -> i8*`
  - `malloc` + registro en GC + almacenamiento de tipo-id + inicialización de atributos
  - Si hay padre, llama al constructor padre y copia atributos heredados
- **Métodos**: `@ClassName_method(i8* %self, double %arg, ...) -> double`
  - Acceso a `self` via `bitcast` + `getelementptr`
  - Métodos heredados se incluyen automáticamente; los hijos pueden sobrescribirlos
- **Instanciación**: `call i8* @ClassName_new(...)` → encode como `double`
- **Method calls**: Decode objeto → `call double @ClassName_method(i8* %self, ...)`
- **Attribute access**: Decode objeto → `bitcast` a struct → `getelementptr` → `load`
- **Herencia profunda**: Los atributos del padre se prependen al struct del hijo (orden topológico). Soporta cadenas de herencia de 3+ niveles.
- **`base()` calls**: Llama al constructor del tipo padre con los argumentos dados
- **`is` operator**: Lee el `type_id` del slot 0 del objeto y compara con el id del tipo objetivo + todos sus descendientes
- **`as` operator**: Verifica el `type_id` en runtime; si no coincide, imprime error y llama a `@abort()`

### 8. Vectores
- **Literales**: `[1, 2, 3]` → `malloc` buffer `[len, e0, e1, e2]`
- **Generadores**: `[expr || var in iterable]` → loop que mapea elementos
- **Indexing**: `v[i]` → `getelementptr double, double* %ptr, i64 (i+1)`
- **Bounds checking**: Antes de acceder, verifica `0 <= i < len`; si falla, imprime error y llama a `@abort()`

### 9. Match Expressions
- Desendulzado a cadena de `fcmp oeq` + `br i1` por cada case
- Soporte para patrones: Literal, Variable, Wildcard
- Resultado acumulado en `alloca double` + merge label

### 10. Funciones Matemáticas (Intrínsecos LLVM)
| HULK       | LLVM Intrinsic              |
|------------|-----------------------------|
| `sqrt(x)`  | `@llvm.sqrt.f64`           |
| `sin(x)`   | `@llvm.sin.f64`            |
| `cos(x)`   | `@llvm.cos.f64`            |
| `exp(x)`   | `@llvm.exp.f64`            |
| `log(b,x)` | `@llvm.log.f64` × 2 + `fdiv` |
| `PI`       | Constante `3.141592653589793e0` |
| `E`        | Constante `2.718281828459045e0` |
| `rand()`   | `@rand()` de libc → `sitofp` → `fdiv` por `RAND_MAX` |

## Funciones C Declaradas

```llvm
declare i32 @printf(i8*, ...)
declare i32 @puts(i8*)
declare i8* @malloc(i64)
declare i8* @realloc(i8*, i64)
declare void @free(i8*)
declare i64 @strlen(i8*)
declare i8* @strcpy(i8*, i8*)
declare i8* @strcat(i8*, i8*)
declare i32 @rand()
declare void @srand(i32)
declare i64 @time(i64*)
declare void @abort()
```

## Funciones Internas (GC)

| Función | Descripción |
|---------|-------------|
| `@__hulk_gc_track(i8*)` | Registra un puntero en el buffer global de GC |
| `@__hulk_gc_sweep()` | Libera todos los punteros registrados al finalizar el programa |

## Funciones Auxiliares del Generador

| Función Rust | Descripción |
|-------------|-------------|
| `mangle_fn(name)` | Renombra `main` → `__hulk_main` para evitar colisión con `@main` de C |
| `fmt_double(f64)` | Formatea doubles válidos para LLVM IR (siempre con punto decimal, ej: `1.0e1` en vez de `1e1`) |
| `collect_free_vars()` | Detecta variables libres en lambdas para captura en closures |
| `class_inherits_from()` | Verifica cadena de herencia para operadores `is`/`as` |

## Tests

El módulo incluye **55 tests** en dos archivos:

- **`tests.rs`** (26 tests): Estructura del programa, print, aritmética, comparaciones, unarios, strings, lógica, math builtins
- **`extra_tests.rs`** (29 tests): Let bindings, if/else, while, funciones, bloques, assignments, vectores, clases, match, lambdas, integración, **herencia profunda, base(), is/as, rand(), closures, bounds checking, GC**

## Ejecución del IR Generado

```bash
# Compilar HULK a LLVM IR desde archivo
cargo run -- mi_programa.hulk > output.ll

# O desde stdin
echo 'print(42);' | cargo run > output.ll

# Compilar IR a ejecutable nativo
clang output.ll -o programa -lm
./programa

# O interpretar directamente con lli
lli output.ll
```
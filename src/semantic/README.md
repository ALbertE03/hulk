# Análisis Semántico (Semantic Analysis)

Este módulo implementa la fase de análisis semántico del compilador HULK. Su objetivo es validar que el programa cumple con las reglas del lenguaje, incluyendo verificación de tipos, resolución de nombres (scope) y comprobaciones de herencia.

## Estructura del Módulo

El módulo `semantic` está organizado en los siguientes archivos:

- **`mod.rs`** : Orquestador principal con la función pública `check_program()` que ejecuta las 4 pasadas de análisis
- **`context.rs`** : Define la estructura `Context` que contiene toda la información semántica (tipos, jerarquía, métodos)
- **`visitor.rs`** : Implementa `BodyChecker`, el visitor que recorre expresiones validando tipos
- **`types.rs`**: Sistema de tipos completo (`Type`, `TypeKind`, `MethodInfo`, verificación de conformidad)
- **`tests.rs`**: Suite de pruebas unitarias que verifica diversos escenarios semánticos

## Proceso de Análisis

El análisis se realiza en **4 pasadas** sobre el Árbol de Sintaxis Abstracta (AST) para manejar correctamente las dependencias hacia adelante (forward references) y la recursividad:

### Pasada 1: Recolección de Tipos
- Se registran todos los nombres de **Tipos** (Clases) y **Protocolos** definidos por el usuario.
- Se detectan errores de redefinición de tipos.

### Pasada 2: Jerarquía de Herencia
- Se resuelven las relaciones de herencia (`inherits`).
- Se valida que no existan ciclos de herencia (Ej: A hereda de B, B hereda de A).
- Se verifica que las clases no hereden de protocolos y viceversa (donde no está permitido).
- Se asigna `Object` como padre por defecto si no se especifica ninguno.

### Pasada 3: Definición de Miembros (Features)
- Se procesan los **Atributos** y **Métodos** de cada tipo.
- Se verifican las firmas de las **Funciones Globales**.
- Se valida la sobreescritura (override) de métodos (misma cantidad de argumentos).
- Se detectan duplicados de métodos o atributos.

### Pasada 4: Verificación de Cuerpos (Body Check)
- Se visitan las expresiones dentro de las funciones globales y los miembros de las clases.
- Se utiliza `BodyChecker` (en `visitor.rs`) para recorrer el AST de expresiones.
- Se resuelven los tipos de las expresiones y se comparan con los esperados.
- Se maneja el **Scope** (ámbito de variables), entrando y saliendo de nuevos ámbitos en expresiones `let`, funciones, etc.

## Sistema de Tipos

El sistema de tipos (en `types.rs`) soporta:
- **Tipos Básicos**: `Number`, `Boolean`, `String`, `Object`.
- **Clases de Usuario**: Definidas con `type`.
- **Protocolos**: Definidos con `protocol` (verificación estructural).
- **Herencia Simple**: Cada clase tiene un único padre.
- **Polimorfismo**: Verificado mediante `conforms_to`.

## Contexto Semántico (`Context`)

La función `check_program` (en `mod.rs`) retorna un `Context` (definido en `context.rs`) que contiene la información de tipos resuelta, utilizada por el codegen:

```rust
pub struct Context {
    pub types: HashMap<String, TypeInfo>,
    pub functions: HashMap<String, FunctionInfo>,
    pub protocols: HashMap<String, ProtocolInfo>,
}
```

Este contexto es pasado directamente al generador de código LLVM IR.

## Errores

Los errores semánticos se definen en `src/errors/` y cubren casos como:
- `TypeMismatch`: Incompatibilidad de tipos.
- `MethodNotFound`: Llamada a método inexistente.
- `CircularInheritance`: Ciclos en la jerarquía de clases.
- `VariableNotFound`: Uso de variables no declaradas.
- `DuplicateDefinition`: Redefinición de tipos, métodos o atributos.

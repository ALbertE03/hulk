# Análisis Semántico (Semantic Analysis)

Este módulo implementa la fase de análisis semántico del compilador HULK. Su objetivo es validar que el programa cumple con las reglas del lenguaje, incluyendo verificación de tipos, resolución de nombres (scope) y comprobaciones de herencia.

## Estructura del Módulo

- **`mod.rs`**: Contiene la lógica principal del chequeo semántico (`check_program`) y el visitante de expresiones (`BodyChecker`).
- **`types.rs`**: Define el sistema de tipos (`Type`, `TypeKind`, `MethodInfo`). Incluye lógica crucial como la verificación de conformidad de tipos (`conforms_to`) y el cálculo del ancestro común más bajo (`lowest_common_ancestor`).
- **`scope.rs`**: Implementa la tabla de símbolos para el manejo de variables y su visibilidad. Soporta anidamiento de ámbitos (scopes).
- **`tests.rs`**: Suite de pruebas unitarias que verifica diversos escenarios semánticos.

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
- Se utiliza `BodyChecker` para recorrer el AST de expresiones.
- Se resuelven los tipos de las expresiones y se comparan con los esperados.
- Se maneja el **Scope** (ámbito de variables), entrando y saliendo de nuevos ámbitos en expresiones `let`, funciones, etc.

## Sistema de Tipos

El sistema de tipos soporta:
- **Tipos Básicos**: `Number`, `Boolean`, `String`, `Object`.
- **Clases de Usuario**: Definidas con `type`.
- **Protocolos**: Definidos con `protocol` (verificación estructural).
- **Herencia Simple**: Cada clase tiene un único padre.
- **Polimorfismo**: Verificado mediante `conforms_to`.

## Errores

Los errores semánticos se definen en `src/errors/semantic.rs` y cubren casos como:
- `TypeMismatch`: Incompatibilidad de tipos.
- `methodNotFound`: Llamada a método inexistente.
- `CircularInheritance`: Ciclos en la jerarquía de clases.
- `VariableNotFound`: Uso de variables no declaradas.

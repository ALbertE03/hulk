# Generador de Código MIPS

Este módulo implementa la fase de generación de código del compilador Hulk, traduciendo el AST verificado a ensamblador MIPS de 32 bits.

## Arquitectura

El generador utiliza una arquitectura basada en **Máquina de Pila (Stack Machine)**. Esto simplifica la generación de código para expresiones complejas al evitar la necesidad de una asignación de registros compleja (Register Allocation).

### Convención de Llamadas y Memoria

*   **Registros**:
    *   `$sp` (Stack Pointer): Apunta al tope de la pila.
    *   `$fp` (Frame Pointer): Apunta a la base del marco de activación actual (Activation Record).
    *   `$ra` (Return Address): Dirección de retorno.
    *   `$v0`: Resultado de funciones y valores de syscall.
    *   `$a0-$a3`: Argumentos de syscall.
    *   `$t0-$t9`: Registros temporales generales.

*   **Pila (Stack)**:
    *   Se utiliza para evaluar todas las expresiones.
    *   Los resultados intermedios se empujan (`push`) y se sacan (`pop`).
    *   Gestión de marcos de activación para funciones y variables locales.

*   **Montículo (Heap)**:
    *   Utiliza `syscall 9` (`sbrk`) para la asignación dinámica de memoria.
    *   Soporte para vectores (`VectorLiteral`) e instancias de tipos (`Instantiation`).
    *   Vectores: Almacenan su longitud en la dirección base y elementos en direcciones subsiguientes.

## Características Implementadas

1.  **Expresiones Básicas**: Literales (Enteros, Booleanos, Strings), Operaciones Binarias (Aritmética, Lógica, Comparación), Operaciones Unarias.
2.  **Control de Flujo**:
    *   `If-Else`: Ramificación condicional estándar.
    *   `While`: Bucles con etiquetas de salto.
    *   `For`: Desendulzado (desugared) a bucles `while` usando el protocolo de iteración (next/current).
    *   `Match`: Desendulzado a cadenas de `if-else-if`.
3.  **Funciones y Variables**:
    *   Definición y uso de variables locales (`Let`).
    *   Llamadas a funciones globales.
    *   Recursividad soportada gracias al manejo de marcos de pila (`$fp`).
4.  **Tipos Compuestos**:
    *   **Vectores**: Inicialización (`[1, 2, 3]`), Indexado (`v[0]`), y método `size()`.
    *   **Objetos**: Instanciación (`new Type()`), acceso a atributos y llamadas a métodos (dispatch dinámico básico).

## Pasos de Generación

1.  **Pre-escaneo**: (Opcional) Recolección de cadenas literales para la sección `.data`.
2.  **Sección .text**:
    *   Generación del punto de entrada `main`.
    *   Iteración sobre todas las funciones definidas en el AST.
3.  **Gen_Expr**:
    *   Función recursiva que emite instrucciones MIPS para cada nodo del AST.
    *   Invariante: Cada expresión evalúa su resultado y lo deja en el tope de la pila (`0($sp)`).

## Ejecución

El código generado puede ser ejecutado en simuladores MIPS como SPIM o Mars.

```bash
spim -file output.mips
```

## Estado Actual

La implementación cubre la mayoría de las características de Hulk requeridas para funcionalidad compleja, incluyendo recursión, manejo de memoria dinámica básica y estructuras de control avanzadas.

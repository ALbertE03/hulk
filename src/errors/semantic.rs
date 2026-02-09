use crate::utils::Position;

#[derive(Debug, Clone)]
pub enum SemanticError {
    TypeDefined(String),
    FunctionDefined(String),
    TypeNotFound(String),
    FunctionNotFound(String, usize), // Nombre, Cantidad de argumentos
    UndefinedFunction(String, Position),
    VariableNotFound(String),
    UndefinedVariable(String, Position),
    OperationNotDefined(String, String), // Operador, Tipo
    TypeMismatch { expected: String, found: String, pos: Position },
    ArgumentMismatch { expected: usize, found: usize, pos: Position },
    AttributeDefined(String),
    MethodDefined(String),
    MethodNotFound(String),
    SignatureMismatch(String),
    CircularInheritance(String),
    ArgumentCountMismatch(String, usize, usize), // Metodo/Func, Esperado, Encontrado
    AccessingPrivateMember(String),
    NotAProtocol(String),
    ProtocolMismatch(String, String), // Tipo, Protocolo
    SelfReference, // 'self' usado fuera de un método
    GenericError(String),
}

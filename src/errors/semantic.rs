#[derive(Debug, Clone)]
pub enum SemanticError {
    TypeDefined(String),
    FunctionDefined(String),
    TypeNotFound(String),
    FunctionNotFound(String, usize), // Name, Args count
    VariableNotFound(String),
    OperationNotDefined(String, String), // Op, Type
    TypeMismatch { expected: String, found: String },
    AttributeDefined(String),
    MethodDefined(String),
    MethodNotFound(String),
    SignatureMismatch(String),
    CircularInheritance(String),
    ArgumentCountMismatch(String, usize, usize), // Method/Func, Expected, Found
    AccessingPrivateMember(String),
    NotAProtocol(String),
    ProtocolMismatch(String, String), // Type, Protocol
    SelfReference, // 'self' used outside of method
    GenericError(String),
}

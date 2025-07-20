use thiserror::Error;

#[derive(Error, Debug)]
pub enum QuasarError {
    #[error("Variable not found: {0}")]
    VariableNotFound(String),
    
    #[error("Type error: {0}")]
    TypeError(String),
    
    #[error("External function error: {0}")]
    ExternalFunctionError(String),
    
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
    
    #[error("Runtime error: {0}")]
    RuntimeError(String),
}

pub type Result<T> = std::result::Result<T, QuasarError>;
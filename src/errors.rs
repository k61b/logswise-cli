use std::fmt;

/// Custom error types for the Logswise CLI
#[derive(Debug)]
#[allow(dead_code)] // These will be used in future improvements
pub enum LogswiseError {
    /// Configuration file not found or invalid
    ConfigError(String),
    /// Network/API communication errors
    NetworkError(String),
    /// Input validation errors
    ValidationError(String),
    /// File system errors
    FileSystemError(String),
    /// User cancelled operation
    UserCancelled,
}

impl fmt::Display for LogswiseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogswiseError::ConfigError(msg) => write!(f, "Configuration error: {msg}"),
            LogswiseError::NetworkError(msg) => write!(f, "Network error: {msg}"),
            LogswiseError::ValidationError(msg) => write!(f, "Validation error: {msg}"),
            LogswiseError::FileSystemError(msg) => write!(f, "File system error: {msg}"),
            LogswiseError::UserCancelled => write!(f, "Operation cancelled by user"),
        }
    }
}

impl std::error::Error for LogswiseError {}

/// Result type alias for Logswise operations
#[allow(dead_code)] // Will be used in future improvements
pub type LogswiseResult<T> = Result<T, LogswiseError>;

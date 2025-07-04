//! Common error types for the application.

use std::fmt;

/// Represents the different kinds of errors in the application.
#[derive(Debug)]
pub enum AppError {
    /// Configuration-related errors
    Config(String),
    /// Network connectivity errors
    Network(String),
    /// LLM model errors
    #[allow(dead_code)]
    Model(String),
    /// File I/O errors
    Io(String),
    /// Invalid user input
    InvalidInput(String),
    /// Database/Supabase errors
    Database(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Config(msg) => write!(f, "Configuration error: {}", msg),
            AppError::Network(msg) => write!(f, "Network error: {}", msg),
            AppError::Model(msg) => write!(f, "Model error: {}", msg),
            AppError::Io(msg) => write!(f, "IO error: {}", msg),
            AppError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            AppError::Database(msg) => write!(f, "Database error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Io(err.to_string())
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::Network(err.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::Config(format!("JSON parsing error: {}", err))
    }
}

/// Convenience type alias for Results with AppError.
pub type AppResult<T> = Result<T, AppError>;

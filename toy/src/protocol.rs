use thiserror::Error;

/// Protocol configuration
#[derive(Debug, Clone)]
pub struct ProtocolConfig {
    /// Number of servers
    pub num_servers: usize,
    /// Threshold for secret sharing
    pub threshold: usize,
    /// Field modulus
    pub field_modulus: u64,
}

impl Default for ProtocolConfig {
    fn default() -> Self {
        Self {
            num_servers: 3,
            threshold: 2,
            field_modulus: 0xFFFFFFFFFFFFFFC5,
        }
    }
}

/// Protocol errors
#[derive(Error, Debug)]
pub enum ProtocolError {
    #[error("Field operation failed")]
    FieldOperationFailed,

    #[error("Dimension mismatch")]
    DimensionMismatch,

    #[error("Empty input")]
    EmptyInput,

    #[error("Server not found")]
    ServerNotFound,

    #[error("Sharing failed")]
    SharingFailed,

    #[error("Invalid configuration: {message}")]
    InvalidConfiguration { message: String },

    #[error("Network error: {message}")]
    NetworkError { message: String },

    #[error("Internal error: {message}")]
    InternalError { message: String },
}

impl From<crate::finite_field::FieldError> for ProtocolError {
    fn from(err: crate::finite_field::FieldError) -> Self {
        match err {
            crate::finite_field::FieldError::DimensionMismatch => ProtocolError::DimensionMismatch,
            crate::finite_field::FieldError::EmptyInput => ProtocolError::EmptyInput,
            _ => ProtocolError::FieldOperationFailed,
        }
    }
}

impl ProtocolError {
    /// Create an invalid configuration error
    pub fn invalid_configuration(message: impl Into<String>) -> Self {
        Self::InvalidConfiguration {
            message: message.into(),
        }
    }

    /// Create a network error
    pub fn network_error(message: impl Into<String>) -> Self {
        Self::NetworkError {
            message: message.into(),
        }
    }

    /// Create an internal error
    pub fn internal_error(message: impl Into<String>) -> Self {
        Self::InternalError {
            message: message.into(),
        }
    }
} 
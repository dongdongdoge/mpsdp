use thiserror::Error;

/// Errors that can occur during shuffle operations
#[derive(Error, Debug)]
pub enum ShuffleError {
    /// Input data is empty
    #[error("Input data is empty")]
    EmptyInput,

    /// Invalid input data format or content
    #[error("Invalid input data: {message}")]
    InvalidInput { message: String },

    /// Privacy budget has been exceeded
    #[error("Privacy budget exceeded: epsilon={epsilon}, delta={delta}")]
    PrivacyBudgetExceeded { epsilon: f64, delta: f64 },

    /// Shuffle operation failed
    #[error("Shuffle operation failed: {message}")]
    ShuffleFailed { message: String },

    /// Schema validation failed
    #[error("Schema mismatch at data index {data_index}: {message}")]
    SchemaMismatch { data_index: usize, message: String },

    /// Invalid query parameters
    #[error("Invalid query: {0}")]
    InvalidQuery(String),

    /// Configuration error
    #[error("Configuration error: {message}")]
    ConfigError { message: String },

    /// Internal error during processing
    #[error("Internal error: {message}")]
    InternalError { message: String },

    /// Timeout during operation
    #[error("Operation timed out after {duration}ms")]
    Timeout { duration: u64 },

    /// Resource exhaustion
    #[error("Resource exhausted: {resource}")]
    ResourceExhausted { resource: String },
}

impl ShuffleError {
    /// Create an invalid input error
    pub fn invalid_input(message: impl Into<String>) -> Self {
        Self::InvalidInput {
            message: message.into(),
        }
    }

    /// Create a shuffle failed error
    pub fn shuffle_failed(message: impl Into<String>) -> Self {
        Self::ShuffleFailed {
            message: message.into(),
        }
    }

    /// Create a privacy budget exceeded error
    pub fn privacy_budget_exceeded(epsilon: f64, delta: f64) -> Self {
        Self::PrivacyBudgetExceeded { epsilon, delta }
    }

    /// Create a configuration error
    pub fn config_error(message: impl Into<String>) -> Self {
        Self::ConfigError {
            message: message.into(),
        }
    }

    /// Create an internal error
    pub fn internal_error(message: impl Into<String>) -> Self {
        Self::InternalError {
            message: message.into(),
        }
    }

    /// Create a timeout error
    pub fn timeout(duration: u64) -> Self {
        Self::Timeout { duration }
    }

    /// Create a resource exhausted error
    pub fn resource_exhausted(resource: impl Into<String>) -> Self {
        Self::ResourceExhausted {
            resource: resource.into(),
        }
    }

    /// Check if this is a recoverable error
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            ShuffleError::Timeout { .. } | ShuffleError::ResourceExhausted { .. }
        )
    }

    /// Check if this is a privacy-related error
    pub fn is_privacy_error(&self) -> bool {
        matches!(self, ShuffleError::PrivacyBudgetExceeded { .. })
    }

    /// Get a user-friendly error message
    pub fn user_message(&self) -> String {
        match self {
            ShuffleError::EmptyInput => "No data provided for shuffling".to_string(),
            ShuffleError::InvalidInput { message } => format!("Invalid data: {}", message),
            ShuffleError::PrivacyBudgetExceeded { epsilon, delta } => {
                format!("Privacy budget exceeded (ε={}, δ={})", epsilon, delta)
            }
            ShuffleError::ShuffleFailed { message } => format!("Shuffle failed: {}", message),
            ShuffleError::SchemaMismatch { data_index, message } => {
                format!("Data at index {} doesn't match schema: {}", data_index, message)
            }
            ShuffleError::InvalidQuery(message) => format!("Invalid query: {}", message),
            ShuffleError::ConfigError { message } => format!("Configuration error: {}", message),
            ShuffleError::InternalError { message } => format!("Internal error: {}", message),
            ShuffleError::Timeout { duration } => format!("Operation timed out after {}ms", duration),
            ShuffleError::ResourceExhausted { resource } => {
                format!("Resource '{}' exhausted", resource)
            }
        }
    }
}

/// Result type for shuffle operations
pub type ShuffleResult<T> = Result<T, ShuffleError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let error = ShuffleError::invalid_input("test message");
        assert!(matches!(error, ShuffleError::InvalidInput { .. }));

        let error = ShuffleError::shuffle_failed("test failure");
        assert!(matches!(error, ShuffleError::ShuffleFailed { .. }));

        let error = ShuffleError::privacy_budget_exceeded(1.0, 1e-5);
        assert!(matches!(error, ShuffleError::PrivacyBudgetExceeded { .. }));
    }

    #[test]
    fn test_error_properties() {
        let timeout_error = ShuffleError::timeout(5000);
        assert!(timeout_error.is_recoverable());

        let privacy_error = ShuffleError::privacy_budget_exceeded(1.0, 1e-5);
        assert!(privacy_error.is_privacy_error());

        let empty_error = ShuffleError::EmptyInput;
        assert!(!empty_error.is_recoverable());
        assert!(!empty_error.is_privacy_error());
    }

    #[test]
    fn test_user_messages() {
        let error = ShuffleError::EmptyInput;
        assert!(!error.user_message().is_empty());

        let error = ShuffleError::invalid_input("test");
        assert!(error.user_message().contains("test"));

        let error = ShuffleError::privacy_budget_exceeded(0.5, 1e-6);
        assert!(error.user_message().contains("0.5"));
        assert!(error.user_message().contains("1e-6"));
    }
} 
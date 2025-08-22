use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShuffleError {
    #[error("Input data is empty")]
    EmptyInput,

    #[error("Invalid input data: {message}")]
    InvalidInput { message: String },

    #[error("Privacy budget exceeded: epsilon={epsilon}, delta={delta}")]
    PrivacyBudgetExceeded { epsilon: f64, delta: f64 },

    #[error("Shuffle operation failed: {message}")]
    ShuffleFailed { message: String },

    #[error("Invalid query: {0}")]
    InvalidQuery(String),

    #[error("Configuration error: {message}")]
    ConfigError { message: String },
}

impl ShuffleError {
    pub fn invalid_input(message: impl Into<String>) -> Self {
        Self::InvalidInput {
            message: message.into(),
        }
    }

    pub fn shuffle_failed(message: impl Into<String>) -> Self {
        Self::ShuffleFailed {
            message: message.into(),
        }
    }

    pub fn privacy_budget_exceeded(epsilon: f64, delta: f64) -> Self {
        Self::PrivacyBudgetExceeded { epsilon, delta }
    }

    pub fn config_error(message: impl Into<String>) -> Self {
        Self::ConfigError {
            message: message.into(),
        }
    }
}

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
} 
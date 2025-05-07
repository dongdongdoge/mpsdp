mod mechanisms;

use crate::schema::{DataPoint, Query, QueryResult};
use crate::arith::PrivacyBudget;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DPError {
    #[error("Invalid input data")]
    InvalidInput,
    #[error("Privacy budget exceeded")]
    PrivacyBudgetExceeded,
    #[error("DP mechanism failed")]
    MechanismFailed,
}

pub struct DPConfig {
    pub privacy_budget: PrivacyBudget,
    pub mechanism_type: MechanismType,
}

#[derive(Clone, Copy, Debug)]
pub enum MechanismType {
    Laplace,
    Gaussian,
    Exponential,
}

impl Default for DPConfig {
    fn default() -> Self {
        Self {
            privacy_budget: PrivacyBudget::new(1.0, 1e-5),
            mechanism_type: MechanismType::Laplace,
        }
    }
}

pub struct DPMechanism {
    config: DPConfig,
    mechanism: mechanisms::DPMechanismImpl,
}

impl DPMechanism {
    pub fn new(config: DPConfig) -> Self {
        Self {
            mechanism: mechanisms::DPMechanismImpl::new(config.mechanism_type),
            config,
        }
    }

    pub fn apply_mechanism(&self, data: Vec<DataPoint>, query: Query) -> Result<QueryResult, DPError> {
        self.mechanism.apply(data, query, &self.config)
    }

    pub fn get_sensitivity(&self, query: &Query) -> f64 {
        self.mechanism.get_sensitivity(query)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dp_mechanism_laplace() {
        let config = DPConfig {
            privacy_budget: PrivacyBudget::new(1.0, 1e-5),
            mechanism_type: MechanismType::Laplace,
        };
        
        let mechanism = DPMechanism::new(config);
        let data = vec![
            DataPoint::new(vec![1.0, 2.0]),
            DataPoint::new(vec![3.0, 4.0]),
        ];

        let query = Query::new(
            QueryType::Mean,
            vec!["feature1".to_string()],
        );

        let result = mechanism.apply_mechanism(data, query).unwrap();
        assert!(result.has_noise());
    }

    #[test]
    fn test_dp_mechanism_gaussian() {
        let config = DPConfig {
            privacy_budget: PrivacyBudget::new(1.0, 1e-5),
            mechanism_type: MechanismType::Gaussian,
        };
        
        let mechanism = DPMechanism::new(config);
        let data = vec![
            DataPoint::new(vec![1.0, 2.0]),
            DataPoint::new(vec![3.0, 4.0]),
        ];

        let query = Query::new(
            QueryType::Mean,
            vec!["feature1".to_string()],
        );

        let result = mechanism.apply_mechanism(data, query).unwrap();
        assert!(result.has_noise());
    }
} 
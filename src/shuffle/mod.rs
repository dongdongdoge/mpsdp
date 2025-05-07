mod mechanism;

use crate::schema::{DataPoint, Query, QueryResult};
use crate::arith::PrivacyBudget;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShuffleError {
    #[error("Invalid input data")]
    InvalidInput,
    #[error("Privacy budget exceeded")]
    PrivacyBudgetExceeded,
    #[error("Shuffle operation failed")]
    ShuffleFailed,
}

pub struct ShuffleConfig {
    pub batch_size: usize,
    pub privacy_budget: PrivacyBudget,
    pub shuffle_rounds: usize,
}

impl Default for ShuffleConfig {
    fn default() -> Self {
        Self {
            batch_size: 1000,
            privacy_budget: PrivacyBudget::new(1.0, 1e-5),
            shuffle_rounds: 3,
        }
    }
}

pub struct Shuffler {
    config: ShuffleConfig,
    mechanism: mechanism::ShuffleMechanism,
}

impl Shuffler {
    pub fn new(config: ShuffleConfig) -> Self {
        Self {
            mechanism: mechanism::ShuffleMechanism::new(),
            config,
        }
    }

    pub fn shuffle_data(&mut self, data: Vec<DataPoint>) -> Result<Vec<DataPoint>, ShuffleError> {
        self.mechanism.shuffle(data, self.config.shuffle_rounds)
    }

    pub fn process_query(&self, query: Query, data: Vec<DataPoint>) -> Result<QueryResult, ShuffleError> {
        self.mechanism.process_query(query, data, &self.config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::QueryType;

    #[test]
    fn test_shuffle_basic() {
        let config = ShuffleConfig::default();
        let mut shuffler = Shuffler::new(config);
        
        let data = vec![
            DataPoint::new(vec![1.0, 2.0]),
            DataPoint::new(vec![3.0, 4.0]),
            DataPoint::new(vec![5.0, 6.0]),
        ];

        let shuffled = shuffler.shuffle_data(data).unwrap();
        assert_eq!(shuffled.len(), 3);
    }

    #[test]
    fn test_shuffle_query() {
        let config = ShuffleConfig::default();
        let shuffler = Shuffler::new(config);
        
        let data = vec![
            DataPoint::new(vec![1.0, 2.0]),
            DataPoint::new(vec![3.0, 4.0]),
            DataPoint::new(vec![5.0, 6.0]),
        ];

        let query = Query::new(
            QueryType::Mean,
            vec!["feature1".to_string()],
        );

        let result = shuffler.process_query(query, data).unwrap();
        assert!(result.has_noise());
    }
} 
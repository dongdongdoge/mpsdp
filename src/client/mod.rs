mod report;
mod query;

use crate::schema::{DataPoint, Query, QueryResult};
use crate::shuffle::{Shuffler, ShuffleConfig};
use crate::dp::{DPMechanism, DPConfig, MechanismType};
use crate::arith::PrivacyBudget;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("Invalid input data")]
    InvalidInput,
    #[error("Privacy budget exceeded")]
    PrivacyBudgetExceeded,
    #[error("Query execution failed")]
    QueryExecutionFailed,
}

pub struct Client {
    shuffler: Shuffler,
    dp_mechanism: DPMechanism,
}

impl Client {
    pub fn new() -> Self {
        let shuffle_config = ShuffleConfig::default();
        let dp_config = DPConfig::default();
        
        Self {
            shuffler: Shuffler::new(shuffle_config),
            dp_mechanism: DPMechanism::new(dp_config),
        }
    }

    pub fn submit_data(&mut self, data: DataPoint) -> Result<(), ClientError> {
        // Apply local privacy guarantees
        let mut data_vec = vec![data];
        self.shuffler.shuffle_data(data_vec)?;
        Ok(())
    }

    pub fn execute_query(&self, query: Query) -> Result<QueryResult, ClientError> {
        // Process query with DP guarantees
        self.dp_mechanism.apply_mechanism(vec![], query)
            .map_err(|_| ClientError::QueryExecutionFailed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::QueryType;

    #[test]
    fn test_client_submit_data() {
        let mut client = Client::new();
        let data = DataPoint::new(vec![1.0, 2.0]);
        assert!(client.submit_data(data).is_ok());
    }

    #[test]
    fn test_client_execute_query() {
        let client = Client::new();
        let query = Query::new(
            QueryType::Mean,
            vec!["feature1".to_string()],
        );
        let result = client.execute_query(query).unwrap();
        assert!(result.has_noise());
    }
} 
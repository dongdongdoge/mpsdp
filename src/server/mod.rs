// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT license.

mod histogram;
mod ot;
mod role;
mod server;

use crate::schema::{DataPoint, Query, QueryResult};
use crate::shuffle::{Shuffler, ShuffleConfig};
use crate::dp::{DPMechanism, DPConfig, MechanismType};
use crate::arith::PrivacyBudget;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("Invalid input data")]
    InvalidInput,
    #[error("Privacy budget exceeded")]
    PrivacyBudgetExceeded,
    #[error("Query processing failed")]
    QueryProcessingFailed,
}

pub struct Server {
    shuffler: Shuffler,
    dp_mechanism: DPMechanism,
}

impl Server {
    pub fn new() -> Self {
        let shuffle_config = ShuffleConfig::default();
        let dp_config = DPConfig::default();
        
        Self {
            shuffler: Shuffler::new(shuffle_config),
            dp_mechanism: DPMechanism::new(dp_config),
        }
    }

    pub async fn start(&self) {
        // Server initialization and startup logic
    }

    pub fn process_data(&mut self, data: Vec<DataPoint>) -> Result<Vec<DataPoint>, ServerError> {
        self.shuffler.shuffle_data(data)
            .map_err(|_| ServerError::QueryProcessingFailed)
    }

    pub fn process_query(&self, query: Query, data: Vec<DataPoint>) -> Result<QueryResult, ServerError> {
        self.dp_mechanism.apply_mechanism(data, query)
            .map_err(|_| ServerError::QueryProcessingFailed)
    }
}

pub use histogram::Histogram;
pub use role::Role;
pub use server::SummationModulus;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::QueryType;

    #[test]
    fn test_server_process_data() {
        let mut server = Server::new();
        let data = vec![
            DataPoint::new(vec![1.0, 2.0]),
            DataPoint::new(vec![3.0, 4.0]),
        ];
        let result = server.process_data(data).unwrap();
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_server_process_query() {
        let server = Server::new();
        let data = vec![
            DataPoint::new(vec![1.0, 2.0]),
            DataPoint::new(vec![3.0, 4.0]),
        ];
        let query = Query::new(
            QueryType::Mean,
            vec!["feature1".to_string()],
        );
        let result = server.process_query(query, data).unwrap();
        assert!(result.has_noise());
    }
}

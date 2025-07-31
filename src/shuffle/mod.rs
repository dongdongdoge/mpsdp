mod mechanism;
mod config;
mod types;
mod error;

pub use config::ShuffleConfig;
pub use types::{ShuffleData, ShuffleResult};
pub use error::ShuffleError;
pub use mechanism::ShuffleMechanism;

use crate::arith::PrivacyBudget;
use crate::schema::{DataPoint, Query, QueryResult};

/// Main shuffler that orchestrates the shuffle differential privacy process
pub struct Shuffler {
    config: ShuffleConfig,
    mechanism: ShuffleMechanism,
}

impl Shuffler {
    /// Create a new shuffler with the given configuration
    pub fn new(config: ShuffleConfig) -> Self {
        Self {
            mechanism: ShuffleMechanism::new(),
            config,
        }
    }

    /// Create a new shuffler with default configuration
    pub fn new_default() -> Self {
        Self::new(ShuffleConfig::default())
    }

    /// Shuffle data with privacy guarantees
    pub fn shuffle_data(&mut self, data: Vec<DataPoint>) -> Result<Vec<DataPoint>, ShuffleError> {
        if data.is_empty() {
            return Err(ShuffleError::EmptyInput);
        }

        // Validate data against schema if provided
        if let Some(schema) = &self.config.schema {
            self.validate_data_against_schema(&data, schema)?;
        }

        // Apply shuffle mechanism
        let shuffled_data = self.mechanism.shuffle(
            data, 
            self.config.shuffle_rounds,
            &self.config.privacy_budget
        )?;

        Ok(shuffled_data)
    }

    /// Process a query with shuffle differential privacy
    pub fn process_query(&self, query: Query, data: Vec<DataPoint>) -> Result<QueryResult, ShuffleError> {
        if data.is_empty() {
            return Err(ShuffleError::EmptyInput);
        }

        // Validate query
        self.validate_query(&query)?;

        // Process query with shuffle mechanism
        self.mechanism.process_query(query, data, &self.config)
    }

    /// Get the current configuration
    pub fn config(&self) -> &ShuffleConfig {
        &self.config
    }

    /// Update the configuration
    pub fn update_config(&mut self, config: ShuffleConfig) {
        self.config = config;
    }

    /// Validate data against schema
    fn validate_data_against_schema(&self, data: &[DataPoint], schema: &crate::schema::Schema) -> Result<(), ShuffleError> {
        for (i, point) in data.iter().enumerate() {
            if !schema.is_compatible_attr_array(&point.attributes()) {
                return Err(ShuffleError::SchemaMismatch {
                    data_index: i,
                    message: "Data point incompatible with schema".to_string(),
                });
            }
        }
        Ok(())
    }

    /// Validate query parameters
    fn validate_query(&self, query: &Query) -> Result<(), ShuffleError> {
        if query.features.is_empty() {
            return Err(ShuffleError::InvalidQuery("No features specified".to_string()));
        }

        // Check if features exist in schema if schema is provided
        if let Some(schema) = &self.config.schema {
            for feature in &query.features {
                if schema.get_attr_index(feature).is_none() {
                    return Err(ShuffleError::InvalidQuery(
                        format!("Feature '{}' not found in schema", feature)
                    ));
                }
            }
        }

        Ok(())
    }
}

impl Default for Shuffler {
    fn default() -> Self {
        Self::new_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::{QueryType, Schema, AttributeType};

    #[test]
    fn test_shuffler_creation() {
        let config = ShuffleConfig::default();
        let shuffler = Shuffler::new(config);
        assert!(shuffler.config().shuffle_rounds > 0);
    }

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
    fn test_shuffle_empty_data() {
        let config = ShuffleConfig::default();
        let mut shuffler = Shuffler::new(config);
        
        let data = vec![];
        let result = shuffler.shuffle_data(data);
        assert!(result.is_err());
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

    #[test]
    fn test_shuffle_with_schema() {
        let schema = Schema(vec![
            ("feature1".to_string(), AttributeType::C4),
            ("feature2".to_string(), AttributeType::N8(255)),
        ]);
        
        let config = ShuffleConfig::builder()
            .schema(schema)
            .shuffle_rounds(5)
            .build();
        
        let shuffler = Shuffler::new(config);
        assert_eq!(shuffler.config().shuffle_rounds, 5);
    }
} 
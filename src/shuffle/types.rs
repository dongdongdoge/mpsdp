use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a data point in the shuffle system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShuffleData {
    /// Unique identifier for the data point
    pub id: String,
    /// Feature values
    pub features: Vec<f64>,
    /// Metadata associated with the data point
    pub metadata: HashMap<String, String>,
    /// Timestamp when the data was created
    pub timestamp: u64,
}

impl ShuffleData {
    /// Create a new shuffle data point
    pub fn new(id: impl Into<String>, features: Vec<f64>) -> Self {
        Self {
            id: id.into(),
            features,
            metadata: HashMap::new(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }
    }

    /// Create a new shuffle data point with metadata
    pub fn with_metadata(
        id: impl Into<String>,
        features: Vec<f64>,
        metadata: HashMap<String, String>,
    ) -> Self {
        Self {
            id: id.into(),
            features,
            metadata,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }
    }

    /// Get the number of features
    pub fn feature_count(&self) -> usize {
        self.features.len()
    }

    /// Get a specific feature value
    pub fn get_feature(&self, index: usize) -> Option<f64> {
        self.features.get(index).copied()
    }

    /// Set a feature value
    pub fn set_feature(&mut self, index: usize, value: f64) -> Result<(), String> {
        if index >= self.features.len() {
            return Err(format!("Feature index {} out of bounds", index));
        }
        self.features[index] = value;
        Ok(())
    }

    /// Add metadata
    pub fn add_metadata(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.metadata.insert(key.into(), value.into());
    }

    /// Get metadata value
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }
}

/// Result of a shuffle operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShuffleResult {
    /// The shuffled data
    pub data: Vec<ShuffleData>,
    /// Statistics about the shuffle operation
    pub statistics: ShuffleStatistics,
    /// Privacy guarantees provided
    pub privacy_guarantees: PrivacyGuarantees,
}

impl ShuffleResult {
    /// Create a new shuffle result
    pub fn new(data: Vec<ShuffleData>) -> Self {
        let statistics = ShuffleStatistics::from_data(&data);
        let privacy_guarantees = PrivacyGuarantees::default();

        Self {
            data,
            statistics,
            privacy_guarantees,
        }
    }

    /// Get the number of data points
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if the result is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Get the shuffled data
    pub fn data(&self) -> &[ShuffleData] {
        &self.data
    }

    /// Get mutable access to the shuffled data
    pub fn data_mut(&mut self) -> &mut [ShuffleData] {
        &mut self.data
    }
}

/// Statistics about a shuffle operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShuffleStatistics {
    /// Number of data points processed
    pub data_count: usize,
    /// Number of features per data point
    pub feature_count: usize,
    /// Processing time in milliseconds
    pub processing_time_ms: u64,
    /// Memory usage in bytes
    pub memory_usage_bytes: usize,
    /// Number of shuffle rounds applied
    pub shuffle_rounds: usize,
}

impl ShuffleStatistics {
    /// Create statistics from data
    pub fn from_data(data: &[ShuffleData]) -> Self {
        let data_count = data.len();
        let feature_count = data.first().map(|d| d.feature_count()).unwrap_or(0);

        Self {
            data_count,
            feature_count,
            processing_time_ms: 0, // Will be set by the shuffler
            memory_usage_bytes: 0, // Will be calculated
            shuffle_rounds: 0,      // Will be set by the shuffler
        }
    }

    /// Update processing time
    pub fn set_processing_time(&mut self, time_ms: u64) {
        self.processing_time_ms = time_ms;
    }

    /// Update shuffle rounds
    pub fn set_shuffle_rounds(&mut self, rounds: usize) {
        self.shuffle_rounds = rounds;
    }

    /// Calculate memory usage
    pub fn calculate_memory_usage(&mut self) {
        // Rough estimation: each ShuffleData with features and metadata
        let per_data_point = std::mem::size_of::<ShuffleData>() + 
                           self.feature_count * std::mem::size_of::<f64>();
        self.memory_usage_bytes = self.data_count * per_data_point;
    }
}

/// Privacy guarantees provided by the shuffle operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyGuarantees {
    /// Epsilon value for differential privacy
    pub epsilon: f64,
    /// Delta value for differential privacy
    pub delta: f64,
    /// Whether the guarantees are proven
    pub is_proven: bool,
    /// Additional privacy parameters
    pub additional_params: HashMap<String, f64>,
}

impl PrivacyGuarantees {
    /// Create new privacy guarantees
    pub fn new(epsilon: f64, delta: f64) -> Self {
        Self {
            epsilon,
            delta,
            is_proven: true,
            additional_params: HashMap::new(),
        }
    }

    /// Add additional privacy parameter
    pub fn add_param(&mut self, key: impl Into<String>, value: f64) {
        self.additional_params.insert(key.into(), value);
    }

    /// Get additional parameter
    pub fn get_param(&self, key: &str) -> Option<f64> {
        self.additional_params.get(key).copied()
    }

    /// Check if privacy guarantees are satisfied
    pub fn is_satisfied(&self) -> bool {
        self.epsilon > 0.0 && self.delta >= 0.0 && self.is_proven
    }
}

impl Default for PrivacyGuarantees {
    fn default() -> Self {
        Self {
            epsilon: 1.0,
            delta: 1e-5,
            is_proven: true,
            additional_params: HashMap::new(),
        }
    }
}

/// Configuration for shuffle batch processing
#[derive(Debug, Clone)]
pub struct BatchConfig {
    /// Maximum batch size
    pub max_batch_size: usize,
    /// Whether to process batches in parallel
    pub parallel_processing: bool,
    /// Timeout for batch processing in milliseconds
    pub timeout_ms: Option<u64>,
}

impl Default for BatchConfig {
    fn default() -> Self {
        Self {
            max_batch_size: 1000,
            parallel_processing: true,
            timeout_ms: Some(30000), // 30 seconds
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shuffle_data_creation() {
        let data = ShuffleData::new("test_id", vec![1.0, 2.0, 3.0]);
        assert_eq!(data.id, "test_id");
        assert_eq!(data.features, vec![1.0, 2.0, 3.0]);
        assert_eq!(data.feature_count(), 3);
    }

    #[test]
    fn test_shuffle_data_metadata() {
        let mut data = ShuffleData::new("test_id", vec![1.0, 2.0]);
        data.add_metadata("source", "test");
        data.add_metadata("version", "1.0");

        assert_eq!(data.get_metadata("source"), Some(&"test".to_string()));
        assert_eq!(data.get_metadata("version"), Some(&"1.0".to_string()));
        assert_eq!(data.get_metadata("nonexistent"), None);
    }

    #[test]
    fn test_shuffle_data_feature_access() {
        let mut data = ShuffleData::new("test_id", vec![1.0, 2.0, 3.0]);
        
        assert_eq!(data.get_feature(0), Some(1.0));
        assert_eq!(data.get_feature(1), Some(2.0));
        assert_eq!(data.get_feature(2), Some(3.0));
        assert_eq!(data.get_feature(3), None);

        assert!(data.set_feature(1, 5.0).is_ok());
        assert_eq!(data.get_feature(1), Some(5.0));

        assert!(data.set_feature(3, 4.0).is_err());
    }

    #[test]
    fn test_shuffle_result() {
        let data = vec![
            ShuffleData::new("id1", vec![1.0, 2.0]),
            ShuffleData::new("id2", vec![3.0, 4.0]),
        ];

        let result = ShuffleResult::new(data);
        assert_eq!(result.len(), 2);
        assert!(!result.is_empty());
        assert_eq!(result.statistics.data_count, 2);
        assert_eq!(result.statistics.feature_count, 2);
    }

    #[test]
    fn test_privacy_guarantees() {
        let guarantees = PrivacyGuarantees::new(0.5, 1e-6);
        assert!(guarantees.is_satisfied());
        assert_eq!(guarantees.epsilon, 0.5);
        assert_eq!(guarantees.delta, 1e-6);

        guarantees.add_param("sensitivity", 1.0);
        assert_eq!(guarantees.get_param("sensitivity"), Some(1.0));
        assert_eq!(guarantees.get_param("nonexistent"), None);
    }

    #[test]
    fn test_batch_config() {
        let config = BatchConfig::default();
        assert_eq!(config.max_batch_size, 1000);
        assert!(config.parallel_processing);
        assert_eq!(config.timeout_ms, Some(30000));
    }
} 
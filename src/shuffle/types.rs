use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShuffleData {
    pub id: String,
    pub features: Vec<f64>,
    pub metadata: HashMap<String, String>,
    pub timestamp: u64,
}

impl ShuffleData {
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

    pub fn feature_count(&self) -> usize {
        self.features.len()
    }

    pub fn get_feature(&self, index: usize) -> Option<f64> {
        self.features.get(index).copied()
    }

    pub fn set_feature(&mut self, index: usize, value: f64) -> Result<(), String> {
        if index >= self.features.len() {
            return Err(format!("Feature index {} out of bounds", index));
        }
        self.features[index] = value;
        Ok(())
    }

    pub fn add_metadata(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.metadata.insert(key.into(), value.into());
    }

    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }
}

#[derive(Debug, Clone)]
pub struct ShuffleResult {
    pub data: Vec<ShuffleData>,
}

impl ShuffleResult {
    pub fn new(data: Vec<ShuffleData>) -> Self {
        Self { data }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn data(&self) -> &[ShuffleData] {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut [ShuffleData] {
        &mut self.data
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
    }
} 
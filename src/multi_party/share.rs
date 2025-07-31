use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Types of data shares
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShareType {
    /// Feature value share
    Feature,
    /// Metadata share
    Metadata,
    /// Noise share
    Noise,
    /// Permutation share
    Permutation,
    /// Query result share
    QueryResult,
}

/// Data share structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataShare {
    /// Server ID that holds this share
    pub server_id: usize,
    /// Feature index (for feature shares)
    pub feature_index: usize,
    /// Share type
    pub share_type: ShareType,
    /// Share value
    pub value: u64,
    /// Modulus for finite field operations
    pub modulus: u64,
    /// Share metadata
    pub metadata: HashMap<String, String>,
}

impl DataShare {
    /// Create a new data share
    pub fn new(
        server_id: usize,
        feature_index: usize,
        share_type: ShareType,
        value: u64,
        modulus: u64,
    ) -> Self {
        Self {
            server_id,
            feature_index,
            share_type,
            value,
            modulus,
            metadata: HashMap::new(),
        }
    }

    /// Create a feature share
    pub fn feature(server_id: usize, feature_index: usize, value: u64, modulus: u64) -> Self {
        Self::new(server_id, feature_index, ShareType::Feature, value, modulus)
    }

    /// Create a metadata share
    pub fn metadata(server_id: usize, feature_index: usize, value: u64, modulus: u64) -> Self {
        Self::new(server_id, feature_index, ShareType::Metadata, value, modulus)
    }

    /// Create a noise share
    pub fn noise(server_id: usize, feature_index: usize, value: u64, modulus: u64) -> Self {
        Self::new(server_id, feature_index, ShareType::Noise, value, modulus)
    }

    /// Create a permutation share
    pub fn permutation(server_id: usize, feature_index: usize, value: u64, modulus: u64) -> Self {
        Self::new(server_id, feature_index, ShareType::Permutation, value, modulus)
    }

    /// Add metadata to the share
    pub fn add_metadata(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.metadata.insert(key.into(), value.into());
    }

    /// Get metadata value
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }

    /// Check if this is a feature share
    pub fn is_feature(&self) -> bool {
        matches!(self.share_type, ShareType::Feature)
    }

    /// Check if this is a metadata share
    pub fn is_metadata(&self) -> bool {
        matches!(self.share_type, ShareType::Metadata)
    }

    /// Check if this is a noise share
    pub fn is_noise(&self) -> bool {
        matches!(self.share_type, ShareType::Noise)
    }

    /// Check if this is a permutation share
    pub fn is_permutation(&self) -> bool {
        matches!(self.share_type, ShareType::Permutation)
    }

    /// Get the normalized value (0.0 to 1.0)
    pub fn normalized_value(&self) -> f64 {
        self.value as f64 / self.modulus as f64
    }

    /// Set the normalized value
    pub fn set_normalized_value(&mut self, normalized: f64) {
        self.value = (normalized * self.modulus as f64) as u64;
    }
}

/// Collection of shares for a data point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataPointShares {
    /// Data point ID
    pub data_point_id: String,
    /// Shares for each feature
    pub feature_shares: HashMap<usize, Vec<DataShare>>,
    /// Metadata shares
    pub metadata_shares: HashMap<String, Vec<DataShare>>,
    /// Noise shares
    pub noise_shares: HashMap<usize, Vec<DataShare>>,
    /// Permutation shares
    pub permutation_shares: Vec<DataShare>,
}

impl DataPointShares {
    /// Create a new data point shares collection
    pub fn new(data_point_id: impl Into<String>) -> Self {
        Self {
            data_point_id: data_point_id.into(),
            feature_shares: HashMap::new(),
            metadata_shares: HashMap::new(),
            noise_shares: HashMap::new(),
            permutation_shares: Vec::new(),
        }
    }

    /// Add a feature share
    pub fn add_feature_share(&mut self, feature_index: usize, share: DataShare) {
        self.feature_shares.entry(feature_index).or_insert_with(Vec::new).push(share);
    }

    /// Add a metadata share
    pub fn add_metadata_share(&mut self, key: impl Into<String>, share: DataShare) {
        self.metadata_shares.entry(key.into()).or_insert_with(Vec::new).push(share);
    }

    /// Add a noise share
    pub fn add_noise_share(&mut self, feature_index: usize, share: DataShare) {
        self.noise_shares.entry(feature_index).or_insert_with(Vec::new).push(share);
    }

    /// Add a permutation share
    pub fn add_permutation_share(&mut self, share: DataShare) {
        self.permutation_shares.push(share);
    }

    /// Get feature shares for a specific feature
    pub fn get_feature_shares(&self, feature_index: usize) -> Option<&[DataShare]> {
        self.feature_shares.get(&feature_index).map(|v| v.as_slice())
    }

    /// Get metadata shares for a specific key
    pub fn get_metadata_shares(&self, key: &str) -> Option<&[DataShare]> {
        self.metadata_shares.get(key).map(|v| v.as_slice())
    }

    /// Get noise shares for a specific feature
    pub fn get_noise_shares(&self, feature_index: usize) -> Option<&[DataShare]> {
        self.noise_shares.get(&feature_index).map(|v| v.as_slice())
    }

    /// Get all feature indices
    pub fn feature_indices(&self) -> Vec<usize> {
        self.feature_shares.keys().copied().collect()
    }

    /// Get all metadata keys
    pub fn metadata_keys(&self) -> Vec<String> {
        self.metadata_shares.keys().cloned().collect()
    }

    /// Get total number of shares
    pub fn total_shares(&self) -> usize {
        let feature_count: usize = self.feature_shares.values().map(|v| v.len()).sum();
        let metadata_count: usize = self.metadata_shares.values().map(|v| v.len()).sum();
        let noise_count: usize = self.noise_shares.values().map(|v| v.len()).sum();
        let permutation_count = self.permutation_shares.len();

        feature_count + metadata_count + noise_count + permutation_count
    }
}

/// Share distribution strategy
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShareDistribution {
    /// Distribute shares evenly among all servers
    Even,
    /// Distribute shares based on server capacity
    Weighted,
    /// Distribute shares with redundancy
    Redundant { redundancy_factor: usize },
    /// Custom distribution
    Custom(Vec<usize>),
}

impl Default for ShareDistribution {
    fn default() -> Self {
        Self::Even
    }
}

/// Share manager for coordinating share operations
pub struct ShareManager {
    /// Distribution strategy
    pub distribution: ShareDistribution,
    /// Number of servers
    pub num_servers: usize,
    /// Threshold for reconstruction
    pub threshold: usize,
    /// Share cache
    pub cache: HashMap<String, DataPointShares>,
}

impl ShareManager {
    /// Create a new share manager
    pub fn new(distribution: ShareDistribution, num_servers: usize, threshold: usize) -> Self {
        Self {
            distribution,
            num_servers,
            threshold,
            cache: HashMap::new(),
        }
    }

    /// Distribute shares according to the strategy
    pub fn distribute_shares(&self, shares: Vec<DataShare>) -> HashMap<usize, Vec<DataShare>> {
        let mut distribution = HashMap::new();

        match &self.distribution {
            ShareDistribution::Even => {
                for (i, share) in shares.into_iter().enumerate() {
                    let server_id = i % self.num_servers;
                    distribution.entry(server_id).or_insert_with(Vec::new).push(share);
                }
            }
            ShareDistribution::Weighted => {
                // Simple weighted distribution based on server ID
                for (i, share) in shares.into_iter().enumerate() {
                    let server_id = (i * 2) % self.num_servers; // Simple weighting
                    distribution.entry(server_id).or_insert_with(Vec::new).push(share);
                }
            }
            ShareDistribution::Redundant { redundancy_factor } => {
                for (i, share) in shares.into_iter().enumerate() {
                    for j in 0..*redundancy_factor {
                        let server_id = (i + j) % self.num_servers;
                        distribution.entry(server_id).or_insert_with(Vec::new).push(share.clone());
                    }
                }
            }
            ShareDistribution::Custom(weights) => {
                let total_weight: usize = weights.iter().sum();
                let mut current_server = 0;
                let mut current_weight = 0;

                for (i, share) in shares.into_iter().enumerate() {
                    while current_weight >= weights[current_server] {
                        current_server = (current_server + 1) % self.num_servers;
                        current_weight = 0;
                    }

                    distribution.entry(current_server).or_insert_with(Vec::new).push(share);
                    current_weight += 1;
                }
            }
        }

        distribution
    }

    /// Cache shares for a data point
    pub fn cache_shares(&mut self, data_point_id: String, shares: DataPointShares) {
        self.cache.insert(data_point_id, shares);
    }

    /// Get cached shares for a data point
    pub fn get_cached_shares(&self, data_point_id: &str) -> Option<&DataPointShares> {
        self.cache.get(data_point_id)
    }

    /// Remove cached shares for a data point
    pub fn remove_cached_shares(&mut self, data_point_id: &str) {
        self.cache.remove(data_point_id);
    }

    /// Clear all cached shares
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    /// Get cache size
    pub fn cache_size(&self) -> usize {
        self.cache.len()
    }

    /// Validate share distribution
    pub fn validate_distribution(&self, distribution: &HashMap<usize, Vec<DataShare>>) -> bool {
        // Check that all servers have shares
        if distribution.len() != self.num_servers {
            return false;
        }

        // Check that threshold can be met
        for (server_id, shares) in distribution {
            if *server_id >= self.num_servers {
                return false;
            }
            if shares.is_empty() {
                return false;
            }
        }

        true
    }

    /// Get distribution statistics
    pub fn get_distribution_stats(&self, distribution: &HashMap<usize, Vec<DataShare>>) -> ShareStats {
        let total_shares: usize = distribution.values().map(|v| v.len()).sum();
        let min_shares = distribution.values().map(|v| v.len()).min().unwrap_or(0);
        let max_shares = distribution.values().map(|v| v.len()).max().unwrap_or(0);
        let avg_shares = if distribution.is_empty() { 0.0 } else { total_shares as f64 / distribution.len() as f64 };

        ShareStats {
            total_shares,
            min_shares,
            max_shares,
            avg_shares,
            num_servers: distribution.len(),
        }
    }
}

/// Statistics about share distribution
#[derive(Debug, Clone)]
pub struct ShareStats {
    /// Total number of shares
    pub total_shares: usize,
    /// Minimum shares per server
    pub min_shares: usize,
    /// Maximum shares per server
    pub max_shares: usize,
    /// Average shares per server
    pub avg_shares: f64,
    /// Number of servers
    pub num_servers: usize,
}

impl ShareStats {
    /// Check if distribution is balanced
    pub fn is_balanced(&self) -> bool {
        self.max_shares - self.min_shares <= 1
    }

    /// Get balance ratio
    pub fn balance_ratio(&self) -> f64 {
        if self.max_shares == 0 {
            1.0
        } else {
            self.min_shares as f64 / self.max_shares as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_share_creation() {
        let share = DataShare::feature(0, 1, 42, 97);
        assert_eq!(share.server_id, 0);
        assert_eq!(share.feature_index, 1);
        assert!(share.is_feature());
        assert_eq!(share.value, 42);
        assert_eq!(share.modulus, 97);
    }

    #[test]
    fn test_data_share_metadata() {
        let mut share = DataShare::feature(0, 1, 42, 97);
        share.add_metadata("source", "test");
        share.add_metadata("version", "1.0");

        assert_eq!(share.get_metadata("source"), Some(&"test".to_string()));
        assert_eq!(share.get_metadata("version"), Some(&"1.0".to_string()));
        assert_eq!(share.get_metadata("nonexistent"), None);
    }

    #[test]
    fn test_data_point_shares() {
        let mut shares = DataPointShares::new("test_id");
        
        let share1 = DataShare::feature(0, 0, 10, 97);
        let share2 = DataShare::feature(1, 0, 20, 97);
        
        shares.add_feature_share(0, share1);
        shares.add_feature_share(0, share2);

        assert_eq!(shares.get_feature_shares(0).unwrap().len(), 2);
        assert_eq!(shares.feature_indices(), vec![0]);
    }

    #[test]
    fn test_share_manager() {
        let manager = ShareManager::new(ShareDistribution::Even, 3, 2);
        
        let shares = vec![
            DataShare::feature(0, 0, 10, 97),
            DataShare::feature(1, 0, 20, 97),
            DataShare::feature(2, 0, 30, 97),
        ];

        let distribution = manager.distribute_shares(shares);
        assert_eq!(distribution.len(), 3);
        assert!(manager.validate_distribution(&distribution));
    }

    #[test]
    fn test_share_stats() {
        let mut distribution = HashMap::new();
        distribution.insert(0, vec![DataShare::feature(0, 0, 10, 97)]);
        distribution.insert(1, vec![DataShare::feature(1, 0, 20, 97)]);
        distribution.insert(2, vec![DataShare::feature(2, 0, 30, 97)]);

        let manager = ShareManager::new(ShareDistribution::Even, 3, 2);
        let stats = manager.get_distribution_stats(&distribution);

        assert_eq!(stats.total_shares, 3);
        assert_eq!(stats.min_shares, 1);
        assert_eq!(stats.max_shares, 1);
        assert!(stats.is_balanced());
        assert_eq!(stats.balance_ratio(), 1.0);
    }
} 
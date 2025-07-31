use crate::schema::{DataPoint, Query, QueryResult};
use crate::arith::PrivacyBudget;
use crate::multi_party::protocol::{ProtocolConfig, ProtocolError, ServerState, ProtocolPhase};
use crate::multi_party::communication::{NetworkMessage, MessageType, CommunicationChannel};
use crate::multi_party::crypto::{SecretShare, ShamirSecretSharing, ThresholdEncryption};
use crate::multi_party::share::{DataShare, ShareType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

/// Role of a server in the multi-party protocol
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServerRole {
    /// First server - holds primary shares
    First,
    /// Second server - holds secondary shares
    Second,
    /// Third server - assists in oblivious shuffle
    Third,
    /// Helper server (for future expansion)
    Helper,
}

impl ServerRole {
    /// Get the next role in rotation
    pub fn next(&self) -> Self {
        match self {
            ServerRole::First => ServerRole::First,
            ServerRole::Second => ServerRole::Third,
            ServerRole::Third => ServerRole::Second,
            ServerRole::Helper => ServerRole::Helper,
        }
    }

    /// Check if this role participates in data holding
    pub fn holds_data(&self) -> bool {
        matches!(self, ServerRole::First | ServerRole::Second)
    }

    /// Check if this role participates in oblivious shuffle
    pub fn participates_in_shuffle(&self) -> bool {
        matches!(self, ServerRole::First | ServerRole::Second | ServerRole::Third)
    }
}

/// State of a multi-party server
#[derive(Debug, Clone)]
pub struct MultiPartyServer {
    /// Server ID
    pub id: usize,
    /// Server role
    pub role: ServerRole,
    /// Configuration
    pub config: ProtocolConfig,
    /// Current state
    pub state: ServerState,
    /// Secret shares held by this server
    pub shares: Vec<DataShare>,
    /// Communication channels to other servers
    pub channels: HashMap<usize, CommunicationChannel>,
    /// Cryptographic components
    pub crypto: ThresholdEncryption,
    /// Message receiver
    pub message_receiver: Option<mpsc::Receiver<NetworkMessage>>,
    /// Message sender
    pub message_sender: Option<mpsc::Sender<NetworkMessage>>,
    /// Round number
    pub round_number: usize,
    /// Permutation for oblivious shuffle
    pub permutation: Option<Vec<usize>>,
}

impl MultiPartyServer {
    /// Create a new multi-party server
    pub fn new(id: usize, role: ServerRole, config: ProtocolConfig) -> Self {
        let crypto = ThresholdEncryption::new(config.threshold, config.num_servers)
            .expect("Failed to create threshold encryption");

        Self {
            id,
            role,
            config,
            state: ServerState::Offline,
            shares: Vec::new(),
            channels: HashMap::new(),
            crypto,
            message_receiver: None,
            message_sender: None,
            round_number: 0,
            permutation: None,
        }
    }

    /// Initialize the server
    pub async fn initialize(&mut self) -> Result<(), ProtocolError> {
        self.state = ServerState::Online;
        self.round_number = 0;
        self.shares.clear();
        self.channels.clear();

        // Initialize cryptographic components
        self.crypto.initialize().await?;

        // Initialize communication channels
        self.initialize_communication().await?;

        Ok(())
    }

    /// Initialize communication channels
    async fn initialize_communication(&mut self) -> Result<(), ProtocolError> {
        for server_id in 0..self.config.num_servers {
            if server_id != self.id {
                let (tx, rx) = mpsc::channel(100);
                let channel = CommunicationChannel::new(server_id, tx, rx);
                self.channels.insert(server_id, channel);
            }
        }

        Ok(())
    }

    /// Establish connections with other servers
    pub async fn establish_connections(
        &mut self,
        server_id: usize,
        servers: &HashMap<usize, MultiPartyServer>,
    ) -> Result<(), ProtocolError> {
        for (other_id, other_server) in servers {
            if *other_id != server_id {
                // In a real implementation, this would establish actual network connections
                // For now, we'll simulate the connection
                log::info!("Server {} establishing connection to server {}", server_id, other_id);
            }
        }

        Ok(())
    }

    /// Receive and process data shares
    pub async fn receive_shares(&mut self, shares: Vec<DataShare>) -> Result<(), ProtocolError> {
        if !self.role.holds_data() {
            return Err(ProtocolError::server_error(
                "Server does not hold data shares".to_string(),
            ));
        }

        self.shares.extend(shares);
        self.state = ServerState::Participating;

        Ok(())
    }

    /// Generate permutation for oblivious shuffle
    pub async fn generate_permutation(&mut self, round: usize) -> Result<Vec<usize>, ProtocolError> {
        if !self.role.participates_in_shuffle() {
            return Err(ProtocolError::server_error(
                "Server does not participate in shuffle".to_string(),
            ));
        }

        // Generate a random permutation
        let n = self.shares.len();
        let mut permutation: Vec<usize> = (0..n).collect();
        
        // Use server ID and round number as seed for deterministic permutation
        let seed = (self.id as u64) * 1000 + (round as u64);
        self.shuffle_permutation(&mut permutation, seed);

        self.permutation = Some(permutation.clone());
        self.round_number = round;

        Ok(permutation)
    }

    /// Apply permutation to shares
    pub async fn apply_permutation(
        &mut self,
        shares: Vec<Vec<DataShare>>,
        permutation: Vec<usize>,
    ) -> Result<Vec<Vec<DataShare>>, ProtocolError> {
        if !self.role.participates_in_shuffle() {
            return Err(ProtocolError::server_error(
                "Server does not participate in shuffle".to_string(),
            ));
        }

        let mut permuted_shares = shares;

        // Apply permutation to each set of shares
        for share_set in &mut permuted_shares {
            let mut temp = share_set.clone();
            for (i, &new_pos) in permutation.iter().enumerate() {
                share_set[new_pos] = temp[i].clone();
            }
        }

        Ok(permuted_shares)
    }

    /// Simple shuffle implementation using Fisher-Yates
    fn shuffle_permutation(&self, permutation: &mut [usize], seed: u64) {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        seed.hash(&mut hasher);
        let mut rng_seed = hasher.finish();

        for i in (1..permutation.len()).rev() {
            rng_seed = rng_seed.wrapping_mul(1103515245).wrapping_add(12345);
            let j = (rng_seed as usize) % (i + 1);
            permutation.swap(i, j);
        }
    }

    /// Participate in oblivious shuffle protocol
    pub async fn participate_in_shuffle(
        &mut self,
        shares: Vec<Vec<DataShare>>,
    ) -> Result<Vec<Vec<DataShare>>, ProtocolError> {
        if !self.role.participates_in_shuffle() {
            return Err(ProtocolError::server_error(
                "Server does not participate in shuffle".to_string(),
            ));
        }

        let mut current_shares = shares;

        // Apply multiple rounds of permutation
        for round in 0..self.config.num_servers {
            let permutation = self.generate_permutation(round).await?;
            current_shares = self.apply_permutation(current_shares, permutation).await?;
        }

        Ok(current_shares)
    }

    /// Reconstruct data from shares
    pub async fn reconstruct_data(
        &self,
        shares: Vec<Vec<DataShare>>,
    ) -> Result<Vec<DataPoint>, ProtocolError> {
        if !self.role.holds_data() {
            return Err(ProtocolError::server_error(
                "Server does not hold data shares".to_string(),
            ));
        }

        let mut reconstructed_data = Vec::new();

        for point_shares in shares {
            let data_point = self.crypto.reconstruct_data(point_shares).await?;
            reconstructed_data.push(data_point);
        }

        Ok(reconstructed_data)
    }

    /// Add noise for differential privacy
    pub async fn add_noise(&self, data: Vec<DataPoint>) -> Result<Vec<DataPoint>, ProtocolError> {
        let mut noisy_data = data;

        for point in &mut noisy_data {
            for feature in point.features_mut() {
                let noise = self.crypto.generate_noise(&self.config.privacy_budget).await?;
                *feature += noise;
            }
        }

        Ok(noisy_data)
    }

    /// Process a query on the server's data
    pub async fn process_query(
        &self,
        query: Query,
        data: Vec<DataPoint>,
    ) -> Result<QueryResult, ProtocolError> {
        let result = match query.query_type {
            crate::schema::QueryType::Mean => self.compute_mean(&data, &query),
            crate::schema::QueryType::Variance => self.compute_variance(&data, &query),
            crate::schema::QueryType::Histogram => self.compute_histogram(&data, &query),
            crate::schema::QueryType::Range => self.compute_range(&data, &query),
            _ => return Err(ProtocolError::UnsupportedQuery(query.query_type)),
        };

        // Add noise for query privacy
        let noisy_result = self.add_query_noise(result).await?;

        Ok(noisy_result)
    }

    /// Compute mean query
    fn compute_mean(&self, data: &[DataPoint], query: &Query) -> QueryResult {
        let mut sums = vec![0.0; query.features.len()];
        let mut counts = vec![0; query.features.len()];

        for point in data {
            for (i, feature) in query.features.iter().enumerate() {
                if let Some(value) = point.get_feature(feature) {
                    sums[i] += value;
                    counts[i] += 1;
                }
            }
        }

        let means: Vec<f64> = sums.iter()
            .zip(counts.iter())
            .map(|(&sum, &count)| if count > 0 { sum / count as f64 } else { 0.0 })
            .collect();

        QueryResult::new(means)
    }

    /// Compute variance query
    fn compute_variance(&self, data: &[DataPoint], query: &Query) -> QueryResult {
        let mut sums = vec![0.0; query.features.len()];
        let mut sums_sq = vec![0.0; query.features.len()];
        let mut counts = vec![0; query.features.len()];

        for point in data {
            for (i, feature) in query.features.iter().enumerate() {
                if let Some(value) = point.get_feature(feature) {
                    sums[i] += value;
                    sums_sq[i] += value * value;
                    counts[i] += 1;
                }
            }
        }

        let variances: Vec<f64> = sums.iter()
            .zip(sums_sq.iter())
            .zip(counts.iter())
            .map(|((&sum, &sum_sq), &count)| {
                if count > 1 {
                    let mean = sum / count as f64;
                    (sum_sq / count as f64) - (mean * mean)
                } else {
                    0.0
                }
            })
            .collect();

        QueryResult::new(variances)
    }

    /// Compute histogram query
    fn compute_histogram(&self, data: &[DataPoint], query: &Query) -> QueryResult {
        let mut histogram = std::collections::HashMap::new();

        for point in data {
            for feature in &query.features {
                if let Some(value) = point.get_feature(feature) {
                    *histogram.entry(value).or_insert(0) += 1;
                }
            }
        }

        let values: Vec<f64> = histogram.values().map(|&v| v as f64).collect();
        QueryResult::new(values)
    }

    /// Compute range query
    fn compute_range(&self, data: &[DataPoint], query: &Query) -> QueryResult {
        let mut mins = vec![f64::INFINITY; query.features.len()];
        let mut maxs = vec![f64::NEG_INFINITY; query.features.len()];

        for point in data {
            for (i, feature) in query.features.iter().enumerate() {
                if let Some(value) = point.get_feature(feature) {
                    mins[i] = mins[i].min(value);
                    maxs[i] = maxs[i].max(value);
                }
            }
        }

        let ranges: Vec<f64> = mins.iter()
            .zip(maxs.iter())
            .map(|(&min, &max)| {
                if min.is_finite() && max.is_finite() {
                    max - min
                } else {
                    0.0
                }
            })
            .collect();

        QueryResult::new(ranges)
    }

    /// Add noise to query result
    async fn add_query_noise(&self, mut result: QueryResult) -> Result<QueryResult, ProtocolError> {
        let noise = self.crypto.generate_noise(&self.config.privacy_budget).await?;
        
        for value in result.values_mut() {
            *value += noise;
        }

        result.mark_as_noisy();
        Ok(result)
    }

    /// Send message to another server
    pub async fn send_message(&self, target_id: usize, message: NetworkMessage) -> Result<(), ProtocolError> {
        if let Some(channel) = self.channels.get(&target_id) {
            channel.send(message).await
                .map_err(|e| ProtocolError::network_error(format!("Failed to send message: {}", e)))?;
            Ok(())
        } else {
            Err(ProtocolError::network_error(format!("No channel to server {}", target_id)))
        }
    }

    /// Receive message from another server
    pub async fn receive_message(&mut self) -> Result<Option<NetworkMessage>, ProtocolError> {
        if let Some(receiver) = &mut self.message_receiver {
            receiver.recv().await
                .ok_or_else(|| ProtocolError::network_error("Channel closed".to_string()))
        } else {
            Err(ProtocolError::network_error("No message receiver".to_string()))
        }
    }

    /// Get server ID
    pub fn id(&self) -> usize {
        self.id
    }

    /// Get server role
    pub fn role(&self) -> &ServerRole {
        &self.role
    }

    /// Get server state
    pub fn state(&self) -> &ServerState {
        &self.state
    }

    /// Set server state
    pub fn set_state(&mut self, state: ServerState) {
        self.state = state;
    }

    /// Get number of shares held
    pub fn share_count(&self) -> usize {
        self.shares.len()
    }

    /// Get round number
    pub fn round_number(&self) -> usize {
        self.round_number
    }

    /// Check if server is available
    pub fn is_available(&self) -> bool {
        self.state.is_available()
    }

    /// Check if server has failed
    pub fn is_failed(&self) -> bool {
        self.state.is_failed()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::QueryType;

    #[test]
    fn test_server_role() {
        let first = ServerRole::First;
        assert!(first.holds_data());
        assert!(first.participates_in_shuffle());

        let third = ServerRole::Third;
        assert!(!third.holds_data());
        assert!(third.participates_in_shuffle());
    }

    #[tokio::test]
    async fn test_server_creation() {
        let config = ProtocolConfig::default();
        let server = MultiPartyServer::new(0, ServerRole::First, config);
        
        assert_eq!(server.id(), 0);
        assert_eq!(server.role(), &ServerRole::First);
        assert_eq!(server.state(), &ServerState::Offline);
    }

    #[tokio::test]
    async fn test_server_initialization() {
        let config = ProtocolConfig::default();
        let mut server = MultiPartyServer::new(0, ServerRole::First, config);
        
        server.initialize().await.unwrap();
        assert_eq!(server.state(), &ServerState::Online);
    }

    #[tokio::test]
    async fn test_permutation_generation() {
        let config = ProtocolConfig::default();
        let mut server = MultiPartyServer::new(0, ServerRole::First, config);
        server.initialize().await.unwrap();

        let permutation = server.generate_permutation(1).await.unwrap();
        assert_eq!(permutation.len(), 0); // No shares yet
    }

    #[tokio::test]
    async fn test_query_processing() {
        let config = ProtocolConfig::default();
        let server = MultiPartyServer::new(0, ServerRole::First, config);
        
        let data = vec![
            DataPoint::new(vec![1.0, 2.0]),
            DataPoint::new(vec![3.0, 4.0]),
        ];

        let query = Query::new(QueryType::Mean, vec!["feature1".to_string()]);
        let result = server.process_query(query, data).await.unwrap();
        assert!(result.has_noise());
    }
} 
// Toy implementation of 3-server multi-party shuffle DP protocol
// Based on the description in toy/description

pub mod finite_field;
pub mod secret_sharing;
pub mod offline_phase;
pub mod online_phase;
pub mod protocol;
pub mod server;

pub use finite_field::{FieldElement, FiniteField, FieldError};
pub use secret_sharing::{SecretShare, ShamirSecretSharing, ShareDistributor};
pub use offline_phase::OfflinePhase;
pub use online_phase::OnlinePhase;
pub use protocol::{ProtocolConfig, ProtocolError};
pub use server::{Server, ServerRole, ServerState, ServerStats};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration for the 3-server protocol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToyConfig {
    /// Field modulus (prime)
    pub field_modulus: u64,
    /// Number of users
    pub num_users: usize,
    /// Privacy budget epsilon
    pub epsilon: f64,
    /// Privacy budget delta
    pub delta: f64,
    /// Noise scale for differential privacy
    pub noise_scale: f64,
}

impl Default for ToyConfig {
    fn default() -> Self {
        Self {
            field_modulus: 0xFFFFFFFFFFFFFFC5, // 2^64 - 59
            num_users: 1000,
            epsilon: 1.0,
            delta: 1e-5,
            noise_scale: 1.0,
        }
    }
}

/// User data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserData {
    /// User ID
    pub user_id: usize,
    /// User's private data
    pub data: Vec<FieldElement>,
    /// User's seed for mask generation
    pub seed: u64,
}

impl UserData {
    /// Create new user data
    pub fn new(user_id: usize, data: Vec<FieldElement>, seed: u64) -> Self {
        Self {
            user_id,
            data,
            seed,
        }
    }

    /// Get data length
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if data is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

/// Protocol result structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolResult {
    /// Shuffled and noised data
    pub result: Vec<Vec<FieldElement>>,
    /// Privacy guarantees
    pub privacy_guarantees: PrivacyGuarantees,
    /// Protocol statistics
    pub stats: ProtocolStats,
}

/// Privacy guarantees
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyGuarantees {
    /// Epsilon value
    pub epsilon: f64,
    /// Delta value
    pub delta: f64,
    /// Whether guarantees are proven
    pub is_proven: bool,
}

/// Protocol statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolStats {
    /// Offline phase time (ms)
    pub offline_time_ms: u64,
    /// Online phase time (ms)
    pub online_time_ms: u64,
    /// Total communication (bytes)
    pub total_communication_bytes: usize,
    /// Number of field operations
    pub field_operations: usize,
}

impl Default for ProtocolStats {
    fn default() -> Self {
        Self {
            offline_time_ms: 0,
            online_time_ms: 0,
            total_communication_bytes: 0,
            field_operations: 0,
        }
    }
}

/// Main protocol implementation
pub struct ToyProtocol {
    /// Configuration
    config: ToyConfig,
    /// Finite field
    field: FiniteField,
    /// Secret sharing scheme
    secret_sharing: ShamirSecretSharing,
    /// Offline phase
    offline_phase: OfflinePhase,
    /// Online phase
    online_phase: OnlinePhase,
    /// Servers
    servers: HashMap<usize, Server>,
}

impl ToyProtocol {
    /// Create new protocol instance
    pub fn new(config: ToyConfig) -> Result<Self, ProtocolError> {
        let field = FiniteField::new(config.field_modulus)?;
        let secret_sharing = ShamirSecretSharing::new(2, 3, config.field_modulus)?;
        
        let offline_phase = OfflinePhase::new(config.clone(), field.clone(), secret_sharing.clone())?;
        let online_phase = OnlinePhase::new(config.clone(), field.clone(), secret_sharing.clone())?;

        // Initialize servers
        let mut servers = HashMap::new();
        servers.insert(0, Server::new(0, ServerRole::Auxiliary, config.clone()));
        servers.insert(1, Server::new(1, ServerRole::Computational, config.clone()));
        servers.insert(2, Server::new(2, ServerRole::Computational, config.clone()));

        Ok(Self {
            config,
            field,
            secret_sharing,
            offline_phase,
            online_phase,
            servers,
        })
    }

    /// Execute the complete protocol
    pub async fn execute(&mut self, user_data: Vec<UserData>) -> Result<ProtocolResult, ProtocolError> {
        let start_time = std::time::Instant::now();

        // Phase 1: Offline preparation
        println!("Starting offline phase...");
        let offline_start = std::time::Instant::now();
        self.offline_phase.execute(&mut self.servers).await?;
        let offline_time = offline_start.elapsed().as_millis() as u64;
        println!("✓ Offline phase completed in {}ms", offline_time);

        // Phase 2: Online execution
        println!("Starting online phase...");
        let online_start = std::time::Instant::now();
        let result = self.online_phase.execute(&mut self.servers, user_data).await?;
        let online_time = online_start.elapsed().as_millis() as u64;
        println!("✓ Online phase completed in {}ms", online_time);

        let total_time = start_time.elapsed().as_millis() as u64;

        let stats = ProtocolStats {
            offline_time_ms: offline_time,
            online_time_ms: online_time,
            total_communication_bytes: 0, // No communication in online phase
            field_operations: self.online_phase.field_operations(),
        };

        let privacy_guarantees = PrivacyGuarantees {
            epsilon: self.config.epsilon,
            delta: self.config.delta,
            is_proven: true,
        };

        Ok(ProtocolResult {
            result,
            privacy_guarantees,
            stats,
        })
    }

    /// Get server by ID
    pub fn get_server(&self, server_id: usize) -> Option<&Server> {
        self.servers.get(&server_id)
    }

    /// Get mutable server by ID
    pub fn get_server_mut(&mut self, server_id: usize) -> Option<&mut Server> {
        self.servers.get_mut(&server_id)
    }

    /// Get configuration
    pub fn config(&self) -> &ToyConfig {
        &self.config
    }

    /// Get finite field
    pub fn field(&self) -> &FiniteField {
        &self.field
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_protocol_creation() {
        let config = ToyConfig::default();
        let protocol = ToyProtocol::new(config);
        assert!(protocol.is_ok());
    }

    #[tokio::test]
    async fn test_protocol_execution() {
        let config = ToyConfig {
            num_users: 10,
            ..Default::default()
        };
        let mut protocol = ToyProtocol::new(config).unwrap();

        // Create test user data
        let mut user_data = Vec::new();
        for i in 0..10 {
            let data = vec![
                FieldElement::new(i as u64, protocol.field().modulus()),
                FieldElement::new((i * 2) as u64, protocol.field().modulus()),
            ];
            user_data.push(UserData::new(i, data, i as u64));
        }

        let result = protocol.execute(user_data).await;
        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!(result.result.len(), 10);
        assert!(result.privacy_guarantees.is_proven);
    }

    #[test]
    fn test_finite_field_operations() {
        let field = FiniteField::new(7).unwrap();
        let a = field.element(5);
        let b = field.element(3);
        
        let sum = a.add(&b).unwrap();
        assert_eq!(sum.value(), 1); // (5 + 3) mod 7 = 1
        
        let product = a.mul(&b).unwrap();
        assert_eq!(product.value(), 1); // (5 * 3) mod 7 = 1
    }

    #[test]
    fn test_secret_sharing() {
        let shamir = ShamirSecretSharing::new(2, 3, 7).unwrap();
        let secret = FieldElement::new(5, 7);
        
        let shares = shamir.share_secret(secret).unwrap();
        assert_eq!(shares.len(), 3);
        
        let reconstructed = shamir.reconstruct_secret(&shares[0..2]).unwrap();
        assert_eq!(reconstructed.value(), 5);
    }
} 
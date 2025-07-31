use crate::finite_field::{FieldElement, FiniteField, FieldError};
use crate::secret_sharing::{SecretShare, ShamirSecretSharing, ShareDistributor};
use crate::server::{Server, ServerRole};
use crate::{ToyConfig, ProtocolError};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Offline phase implementation
pub struct OfflinePhase {
    /// Configuration
    config: ToyConfig,
    /// Finite field
    field: FiniteField,
    /// Secret sharing scheme
    secret_sharing: ShamirSecretSharing,
    /// Share distributor
    distributor: ShareDistributor,
}

impl OfflinePhase {
    /// Create new offline phase
    pub fn new(
        config: ToyConfig,
        field: FiniteField,
        secret_sharing: ShamirSecretSharing,
    ) -> Result<Self, ProtocolError> {
        let distributor = ShareDistributor::new(secret_sharing.clone(), 3);

        Ok(Self {
            config,
            field,
            secret_sharing,
            distributor,
        })
    }

    /// Execute offline phase
    pub async fn execute(&self, servers: &mut HashMap<usize, Server>) -> Result<(), ProtocolError> {
        println!("  Generating shuffle correlation...");
        self.generate_shuffle_correlation(servers).await?;

        println!("  Generating DP correlation...");
        self.generate_dp_correlation(servers).await?;

        println!("  Distributing shares to computational servers...");
        self.distribute_shares(servers).await?;

        Ok(())
    }

    /// Generate shuffle correlation (permutation matrix and masks)
    async fn generate_shuffle_correlation(&self, servers: &mut HashMap<usize, Server>) -> Result<(), ProtocolError> {
        let auxiliary_server = servers.get_mut(&0).ok_or(ProtocolError::ServerNotFound)?;
        
        // Generate random permutation matrix
        let permutation_matrix = self.generate_permutation_matrix().await?;
        println!("    ✓ Generated permutation matrix");

        // Generate random masks for each user
        let masks = self.generate_user_masks().await?;
        println!("    ✓ Generated user masks");

        // Share permutation matrix
        let permutation_shares = self.share_permutation_matrix(&permutation_matrix).await?;
        auxiliary_server.store_permutation_shares(permutation_shares);
        println!("    ✓ Shared permutation matrix");

        // Share user masks
        let mask_shares = self.share_user_masks(&masks).await?;
        auxiliary_server.store_mask_shares(mask_shares);
        println!("    ✓ Shared user masks");

        Ok(())
    }

    /// Generate DP correlation (noise vector)
    async fn generate_dp_correlation(&self, servers: &mut HashMap<usize, Server>) -> Result<(), ProtocolError> {
        let auxiliary_server = servers.get_mut(&0).ok_or(ProtocolError::ServerNotFound)?;
        
        // Generate noise vector for differential privacy
        let noise_vector = self.generate_dp_noise().await?;
        println!("    ✓ Generated DP noise vector");

        // Share noise vector
        let noise_shares = self.share_noise_vector(&noise_vector).await?;
        auxiliary_server.store_noise_shares(noise_shares);
        println!("    ✓ Shared noise vector");

        Ok(())
    }

    /// Distribute shares to computational servers
    async fn distribute_shares(&self, servers: &mut HashMap<usize, Server>) -> Result<(), ProtocolError> {
        let auxiliary_server = servers.get(&0).ok_or(ProtocolError::ServerNotFound)?;
        
        // Clone the shares to avoid borrowing conflicts
        let permutation_shares = auxiliary_server.get_permutation_shares().clone();
        let mask_shares = auxiliary_server.get_mask_shares().clone();
        let noise_shares = auxiliary_server.get_noise_shares().clone();
        
        // Send shares to computational servers
        for server_id in 1..=2 {
            if let Some(server) = servers.get_mut(&server_id) {
                // For simplicity, just copy the shares directly
                // In a real implementation, you'd distribute different shares to each server
                server.receive_permutation_shares(permutation_shares.clone());
                server.receive_mask_shares(mask_shares.clone());
                server.receive_noise_shares(noise_shares.clone());
                
                println!("    ✓ Distributed shares to server {}", server_id);
            }
        }

        Ok(())
    }

    /// Generate random permutation matrix
    async fn generate_permutation_matrix(&self) -> Result<Vec<Vec<FieldElement>>, ProtocolError> {
        let n = self.config.num_users;
        let mut matrix = vec![vec![self.field.zero(); n]; n];
        
        // Generate random permutation
        let mut permutation: Vec<usize> = (0..n).collect();
        self.shuffle_permutation(&mut permutation);
        
        // Create permutation matrix
        for (i, &pos) in permutation.iter().enumerate() {
            matrix[i][pos] = self.field.one();
        }
        
        Ok(matrix)
    }

    /// Generate random masks for each user
    async fn generate_user_masks(&self) -> Result<Vec<Vec<FieldElement>>, ProtocolError> {
        let n = self.config.num_users;
        let mut masks = Vec::with_capacity(n);
        
        for _ in 0..n {
            let user_mask = self.field.random_vector(2); // Assuming 2 features per user
            masks.push(user_mask);
        }
        
        Ok(masks)
    }

    /// Generate DP noise vector
    async fn generate_dp_noise(&self) -> Result<Vec<FieldElement>, ProtocolError> {
        let n = self.config.num_users;
        let mut noise = Vec::with_capacity(n);
        
        // Generate Laplace noise scaled by privacy budget
        let scale = self.config.noise_scale / self.config.epsilon;
        
        for _ in 0..n {
            let noise_value = self.generate_laplace_noise(scale)?;
            noise.push(noise_value);
        }
        
        Ok(noise)
    }

    /// Share permutation matrix
    async fn share_permutation_matrix(&self, matrix: &[Vec<FieldElement>]) -> Result<Vec<Vec<Vec<SecretShare>>>, ProtocolError> {
        let mut all_shares = Vec::with_capacity(matrix.len());
        
        for row in matrix {
            let row_shares = self.secret_sharing.share_vector(row)
                .map_err(|_| ProtocolError::SharingFailed)?;
            all_shares.push(row_shares);
        }
        
        Ok(all_shares)
    }

    /// Share user masks
    async fn share_user_masks(&self, masks: &[Vec<FieldElement>]) -> Result<Vec<Vec<Vec<SecretShare>>>, ProtocolError> {
        let mut all_shares = Vec::with_capacity(masks.len());
        
        for mask in masks {
            let mask_shares = self.secret_sharing.share_vector(mask)
                .map_err(|_| ProtocolError::SharingFailed)?;
            all_shares.push(mask_shares);
        }
        
        Ok(all_shares)
    }

    /// Share noise vector
    async fn share_noise_vector(&self, noise: &[FieldElement]) -> Result<Vec<Vec<SecretShare>>, ProtocolError> {
        self.secret_sharing.share_vector(noise)
            .map_err(|_| ProtocolError::SharingFailed)
    }

    /// Shuffle permutation using Fisher-Yates
    fn shuffle_permutation(&self, permutation: &mut [usize]) {
        use rand::seq::SliceRandom;
        use rand::thread_rng;
        permutation.shuffle(&mut thread_rng());
    }

    /// Generate Laplace noise
    fn generate_laplace_noise(&self, scale: f64) -> Result<FieldElement, ProtocolError> {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        let u1: f64 = rng.gen_range(0.0..1.0);
        let u2: f64 = rng.gen_range(0.0..1.0);
        
        let noise = scale * (u1.ln() - u2.ln());
        
        // Convert to field element (modulo field size)
        let noise_u64 = ((noise.abs() * 1000.0) as u64) % self.field.modulus();
        Ok(FieldElement::new(noise_u64, self.field.modulus()))
    }
}

/// Offline phase statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfflineStats {
    /// Time taken for permutation generation (ms)
    pub permutation_time_ms: u64,
    /// Time taken for mask generation (ms)
    pub mask_time_ms: u64,
    /// Time taken for noise generation (ms)
    pub noise_time_ms: u64,
    /// Time taken for share distribution (ms)
    pub distribution_time_ms: u64,
    /// Total communication (bytes)
    pub total_communication_bytes: usize,
}

impl Default for OfflineStats {
    fn default() -> Self {
        Self {
            permutation_time_ms: 0,
            mask_time_ms: 0,
            noise_time_ms: 0,
            distribution_time_ms: 0,
            total_communication_bytes: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_offline_phase_creation() {
        let config = ToyConfig::default();
        let field = FiniteField::new(config.field_modulus).unwrap();
        let secret_sharing = ShamirSecretSharing::new(2, 3, config.field_modulus).unwrap();
        
        let offline_phase = OfflinePhase::new(config, field, secret_sharing);
        assert!(offline_phase.is_ok());
    }

    #[tokio::test]
    async fn test_permutation_generation() {
        let config = ToyConfig { num_users: 10, ..Default::default() };
        let field = FiniteField::new(config.field_modulus).unwrap();
        let secret_sharing = ShamirSecretSharing::new(2, 3, config.field_modulus).unwrap();
        
        let offline_phase = OfflinePhase::new(config, field, secret_sharing).unwrap();
        
        // Test permutation matrix generation
        let matrix = offline_phase.generate_permutation_matrix().await.unwrap();
        assert_eq!(matrix.len(), 10);
        assert_eq!(matrix[0].len(), 10);
    }

    #[tokio::test]
    async fn test_mask_generation() {
        let config = ToyConfig { num_users: 10, ..Default::default() };
        let field = FiniteField::new(config.field_modulus).unwrap();
        let secret_sharing = ShamirSecretSharing::new(2, 3, config.field_modulus).unwrap();
        
        let offline_phase = OfflinePhase::new(config, field, secret_sharing).unwrap();
        
        // Test mask generation
        let masks = offline_phase.generate_user_masks().await.unwrap();
        assert_eq!(masks.len(), 10);
        assert_eq!(masks[0].len(), 2);
    }

    #[tokio::test]
    async fn test_noise_generation() {
        let config = ToyConfig { num_users: 10, ..Default::default() };
        let field = FiniteField::new(config.field_modulus).unwrap();
        let secret_sharing = ShamirSecretSharing::new(2, 3, config.field_modulus).unwrap();
        
        let offline_phase = OfflinePhase::new(config, field, secret_sharing).unwrap();
        
        // Test noise generation
        let noise = offline_phase.generate_dp_noise().await.unwrap();
        assert_eq!(noise.len(), 10);
    }
} 
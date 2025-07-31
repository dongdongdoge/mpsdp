use crate::finite_field::{FieldElement, FiniteField, FieldError};
use crate::secret_sharing::{SecretShare, ShamirSecretSharing};
use crate::server::{Server, ServerRole};
use crate::{UserData, ProtocolError};
use std::collections::HashMap;

/// Online phase implementation
pub struct OnlinePhase {
    /// Configuration
    config: crate::ToyConfig,
    /// Finite field
    field: FiniteField,
    /// Secret sharing scheme
    secret_sharing: ShamirSecretSharing,
    /// Field operation counter
    field_operations: usize,
}

impl OnlinePhase {
    /// Create new online phase
    pub fn new(
        config: crate::ToyConfig,
        field: FiniteField,
        secret_sharing: ShamirSecretSharing,
    ) -> Result<Self, ProtocolError> {
        Ok(Self {
            config,
            field,
            secret_sharing,
            field_operations: 0,
        })
    }

    /// Execute online phase
    pub async fn execute(&mut self, servers: &mut HashMap<usize, Server>, user_data: Vec<UserData>) -> Result<Vec<Vec<FieldElement>>, ProtocolError> {
        println!("  Processing user submissions...");
        let user_shares = self.process_user_submissions(servers, user_data).await?;

        println!("  Performing silent shuffle...");
        let shuffled_shares = self.silent_shuffle(servers, user_shares).await?;

        println!("  Performing silent randomization...");
        let randomized_shares = self.silent_randomization(servers, shuffled_shares).await?;

        println!("  Reconstructing final result...");
        let final_result = self.reconstruct_result(servers, randomized_shares).await?;

        Ok(final_result)
    }

    /// Process user submissions (Step 1)
    async fn process_user_submissions(&mut self, _servers: &mut HashMap<usize, Server>, user_data: Vec<UserData>) -> Result<Vec<Vec<FieldElement>>, ProtocolError> {
        let mut user_shares = Vec::with_capacity(user_data.len());

        for user in user_data {
            // User computes [x_i]_2 = x_i - a_i
            let user_mask = self.compute_user_mask(user.user_id, user.seed);
            let user_share = self.compute_user_share(&user.data, &user_mask)?;
            user_shares.push(user_share);
        }

        Ok(user_shares)
    }

    /// Silent shuffle (Step 2) - completely local computation
    async fn silent_shuffle(&mut self, servers: &mut HashMap<usize, Server>, user_shares: Vec<Vec<FieldElement>>) -> Result<Vec<Vec<FieldElement>>, ProtocolError> {
        let mut shuffled_shares = Vec::with_capacity(user_shares.len());

        // Each computational server performs local shuffle computation
        for server_id in 1..=2 {
            if let Some(server) = servers.get_mut(&server_id) {
                let server_shuffled = self.compute_local_shuffle(server, &user_shares).await?;
                
                if server_id == 1 {
                    // Use server 1's result as the primary shuffled data
                    shuffled_shares = server_shuffled;
                }
            }
        }

        Ok(shuffled_shares)
    }

    /// Silent randomization (Step 3) - completely local computation
    async fn silent_randomization(&mut self, servers: &mut HashMap<usize, Server>, shuffled_shares: Vec<Vec<FieldElement>>) -> Result<Vec<Vec<FieldElement>>, ProtocolError> {
        let mut randomized_shares = Vec::with_capacity(shuffled_shares.len());

        // Each computational server performs local randomization
        for server_id in 1..=2 {
            if let Some(server) = servers.get_mut(&server_id) {
                let server_randomized = self.compute_local_randomization(server, &shuffled_shares).await?;
                
                if server_id == 1 {
                    // Use server 1's result as the primary randomized data
                    randomized_shares = server_randomized;
                }
            }
        }

        Ok(randomized_shares)
    }

    /// Reconstruct final result (Step 4)
    async fn reconstruct_result(&mut self, servers: &mut HashMap<usize, Server>, _randomized_shares: Vec<Vec<FieldElement>>) -> Result<Vec<Vec<FieldElement>>, ProtocolError> {
        // Collect shares from both computational servers
        let mut server_shares = Vec::new();
        
        for server_id in 1..=2 {
            if let Some(server) = servers.get(&server_id) {
                let server_result = server.get_final_result();
                server_shares.push(server_result);
            }
        }

        // Reconstruct final result by combining shares
        let final_result = self.combine_server_results(&server_shares)?;

        Ok(final_result)
    }

    /// Compute user mask based on seed
    fn compute_user_mask(&self, user_id: usize, seed: u64) -> Vec<FieldElement> {
        // Deterministic mask generation using seed
        let mut mask = Vec::new();
        let mut rng_seed = seed + (user_id as u64);
        
        for _ in 0..2 { // Assuming 2 features per user
            rng_seed = rng_seed.wrapping_mul(1103515245).wrapping_add(12345);
            let mask_value = rng_seed % self.field.modulus();
            mask.push(FieldElement::new(mask_value, self.field.modulus()));
        }
        
        mask
    }

    /// Compute user share [x_i]_2 = x_i - a_i
    fn compute_user_share(&mut self, user_data: &[FieldElement], mask: &[FieldElement]) -> Result<Vec<FieldElement>, ProtocolError> {
        if user_data.len() != mask.len() {
            return Err(ProtocolError::DimensionMismatch);
        }

        let mut share = Vec::with_capacity(user_data.len());
        for (data, mask_val) in user_data.iter().zip(mask.iter()) {
            let share_val = data.sub(mask_val)
                .map_err(|_| ProtocolError::FieldOperationFailed)?;
            share.push(share_val);
            self.field_operations += 1;
        }

        Ok(share)
    }

    /// Compute local shuffle for a server
    async fn compute_local_shuffle(&mut self, server: &mut Server, user_shares: &[Vec<FieldElement>]) -> Result<Vec<Vec<FieldElement>>, ProtocolError> {
        // Get permutation shares from server
        let permutation_shares = server.get_permutation_shares();
        
        // Apply permutation locally
        let shuffled = self.apply_permutation_locally(user_shares, permutation_shares).await?;
        
        Ok(shuffled)
    }

    /// Compute local randomization for a server
    async fn compute_local_randomization(&mut self, server: &mut Server, shuffled_shares: &[Vec<FieldElement>]) -> Result<Vec<Vec<FieldElement>>, ProtocolError> {
        // Get noise shares from server
        let noise_shares = server.get_noise_shares();
        
        // Add noise locally
        let randomized = self.add_noise_locally(shuffled_shares, noise_shares).await?;
        
        // Store final result in server
        server.set_final_result(randomized.clone());
        
        Ok(randomized)
    }

    /// Apply permutation locally
    async fn apply_permutation_locally(&mut self, data: &[Vec<FieldElement>], permutation_shares: &[Vec<Vec<SecretShare>>]) -> Result<Vec<Vec<FieldElement>>, ProtocolError> {
        let n = data.len();
        let mut shuffled = vec![vec![self.field.zero(); 2]; n]; // Assuming 2 features per user
        
        // Apply permutation matrix to data
        for i in 0..n {
            for j in 0..n {
                // Get permutation matrix element [i][j]
                let perm_element = self.get_permutation_element(permutation_shares, i, j)?;
                
                if !perm_element.is_zero() {
                    // Apply permutation: shuffled[i] += perm_element * data[j]
                    for k in 0..2 { // 2 features
                        let product = perm_element.mul(&data[j][k])
                            .map_err(|_| ProtocolError::FieldOperationFailed)?;
                        shuffled[i][k] = shuffled[i][k].add(&product)
                            .map_err(|_| ProtocolError::FieldOperationFailed)?;
                        self.field_operations += 2;
                    }
                }
            }
        }
        
        Ok(shuffled)
    }

    /// Add noise locally
    async fn add_noise_locally(&mut self, data: &[Vec<FieldElement>], noise_shares: &[Vec<SecretShare>]) -> Result<Vec<Vec<FieldElement>>, ProtocolError> {
        let mut randomized = Vec::with_capacity(data.len());
        
        for (i, user_data) in data.iter().enumerate() {
            let mut noised_user_data = Vec::with_capacity(user_data.len());
            
            for (j, feature) in user_data.iter().enumerate() {
                // Get noise for this user and feature
                let noise = self.get_noise_element(noise_shares, i, j)?;
                
                // Add noise to feature
                let noised_feature = feature.add(&noise)
                    .map_err(|_| ProtocolError::FieldOperationFailed)?;
                noised_user_data.push(noised_feature);
                self.field_operations += 1;
            }
            
            randomized.push(noised_user_data);
        }
        
        Ok(randomized)
    }

    /// Get permutation matrix element
    fn get_permutation_element(&self, permutation_shares: &[Vec<Vec<SecretShare>>], i: usize, j: usize) -> Result<FieldElement, ProtocolError> {
        if i < permutation_shares.len() && j < permutation_shares[i].len() {
            let share = &permutation_shares[i][j];
            // For simplicity, just return the first share's value
            // In a real implementation, you'd reconstruct the actual value
            if !share.is_empty() {
                Ok(share[0].value())
            } else {
                Ok(self.field.zero())
            }
        } else {
            Ok(self.field.zero())
        }
    }

    /// Get noise element
    fn get_noise_element(&self, noise_shares: &[Vec<SecretShare>], user_id: usize, feature_id: usize) -> Result<FieldElement, ProtocolError> {
        if user_id < noise_shares.len() {
            let share = &noise_shares[user_id];
            if feature_id < share.len() {
                Ok(share[feature_id].value())
            } else {
                Ok(self.field.zero())
            }
        } else {
            Ok(self.field.zero())
        }
    }

    /// Combine server results
    fn combine_server_results(&mut self, server_results: &[Vec<Vec<FieldElement>>]) -> Result<Vec<Vec<FieldElement>>, ProtocolError> {
        if server_results.is_empty() {
            return Err(ProtocolError::EmptyInput);
        }

        let n = server_results[0].len();
        let mut combined = Vec::with_capacity(n);
        
        for i in 0..n {
            let mut combined_user = Vec::new();
            let feature_count = server_results[0][i].len();
            
            for j in 0..feature_count {
                let mut sum = self.field.zero();
                
                for server_result in server_results {
                    if i < server_result.len() && j < server_result[i].len() {
                        sum = sum.add(&server_result[i][j])
                            .map_err(|_| ProtocolError::FieldOperationFailed)?;
                        self.field_operations += 1;
                    }
                }
                
                combined_user.push(sum);
            }
            
            combined.push(combined_user);
        }
        
        Ok(combined)
    }

    /// Get field operation count
    pub fn field_operations(&self) -> usize {
        self.field_operations
    }
}

/// Online phase statistics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OnlineStats {
    /// Time taken for user submission processing (ms)
    pub submission_time_ms: u64,
    /// Time taken for silent shuffle (ms)
    pub shuffle_time_ms: u64,
    /// Time taken for silent randomization (ms)
    pub randomization_time_ms: u64,
    /// Time taken for result reconstruction (ms)
    pub reconstruction_time_ms: u64,
    /// Number of field operations
    pub field_operations: usize,
    /// Communication bytes (should be 0 for online phase)
    pub communication_bytes: usize,
}

impl Default for OnlineStats {
    fn default() -> Self {
        Self {
            submission_time_ms: 0,
            shuffle_time_ms: 0,
            randomization_time_ms: 0,
            reconstruction_time_ms: 0,
            field_operations: 0,
            communication_bytes: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_online_phase_creation() {
        let config = crate::ToyConfig::default();
        let field = FiniteField::new(config.field_modulus).unwrap();
        let secret_sharing = ShamirSecretSharing::new(2, 3, config.field_modulus).unwrap();
        
        let online_phase = OnlinePhase::new(config, field, secret_sharing);
        assert!(online_phase.is_ok());
    }

    #[tokio::test]
    async fn test_user_mask_computation() {
        let config = crate::ToyConfig::default();
        let field = FiniteField::new(config.field_modulus).unwrap();
        let secret_sharing = ShamirSecretSharing::new(2, 3, config.field_modulus).unwrap();
        
        let online_phase = OnlinePhase::new(config, field, secret_sharing).unwrap();
        
        let mask = online_phase.compute_user_mask(1, 12345);
        assert_eq!(mask.len(), 2);
    }

    #[tokio::test]
    async fn test_user_share_computation() {
        let config = crate::ToyConfig::default();
        let field = FiniteField::new(config.field_modulus).unwrap();
        let secret_sharing = ShamirSecretSharing::new(2, 3, config.field_modulus).unwrap();
        
        let online_phase = OnlinePhase::new(config, field, secret_sharing).unwrap();
        
        let user_data = vec![
            FieldElement::new(10, config.field_modulus),
            FieldElement::new(20, config.field_modulus),
        ];
        let mask = vec![
            FieldElement::new(3, config.field_modulus),
            FieldElement::new(7, config.field_modulus),
        ];
        
        let share = online_phase.compute_user_share(&user_data, &mask).unwrap();
        assert_eq!(share.len(), 2);
    }
} 
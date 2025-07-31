use crate::schema::DataPoint;
use crate::arith::PrivacyBudget;
use crate::multi_party::protocol::ProtocolError;
use crate::multi_party::share::{DataShare, ShareType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use rand::Rng;

/// Secret share for Shamir's secret sharing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretShare {
    /// Share ID
    pub id: usize,
    /// Share value
    pub value: u64,
    /// Share polynomial coefficient
    pub coefficient: u64,
    /// Prime modulus
    pub modulus: u64,
}

impl SecretShare {
    /// Create a new secret share
    pub fn new(id: usize, value: u64, coefficient: u64, modulus: u64) -> Self {
        Self {
            id,
            value,
            coefficient,
            modulus,
        }
    }

    /// Evaluate the share at a given point
    pub fn evaluate(&self, x: u64) -> u64 {
        let mut result = self.value;
        let mut power = 1;
        
        for _ in 0..self.coefficient {
            power = (power * x) % self.modulus;
            result = (result + power) % self.modulus;
        }
        
        result
    }
}

/// Shamir's secret sharing implementation
pub struct ShamirSecretSharing {
    /// Threshold (minimum shares needed)
    pub threshold: usize,
    /// Number of shares
    pub num_shares: usize,
    /// Prime modulus
    pub modulus: u64,
    /// Generator for field operations
    pub generator: u64,
}

impl ShamirSecretSharing {
    /// Create a new Shamir secret sharing scheme
    pub fn new(threshold: usize, num_shares: usize, modulus: u64) -> Result<Self, ProtocolError> {
        if threshold > num_shares {
            return Err(ProtocolError::InvalidConfiguration(
                "Threshold cannot exceed number of shares".to_string(),
            ));
        }

        if threshold < 2 {
            return Err(ProtocolError::InvalidConfiguration(
                "Threshold must be at least 2".to_string(),
            ));
        }

        Ok(Self {
            threshold,
            num_shares,
            modulus,
            generator: 5, // Common generator for small primes
        })
    }

    /// Share a secret value
    pub fn share_secret(&self, secret: u64) -> Result<Vec<SecretShare>, ProtocolError> {
        if secret >= self.modulus {
            return Err(ProtocolError::InvalidConfiguration(
                "Secret must be less than modulus".to_string(),
            ));
        }

        let mut shares = Vec::new();
        let mut rng = rand::thread_rng();

        for i in 0..self.num_shares {
            let coefficient = rng.gen_range(0..self.modulus);
            let share = SecretShare::new(i, secret, coefficient, self.modulus);
            shares.push(share);
        }

        Ok(shares)
    }

    /// Reconstruct secret from shares
    pub fn reconstruct_secret(&self, shares: &[SecretShare]) -> Result<u64, ProtocolError> {
        if shares.len() < self.threshold {
            return Err(ProtocolError::InsufficientServers {
                available: shares.len(),
                required: self.threshold,
            });
        }

        // Use Lagrange interpolation to reconstruct the secret
        let mut secret = 0u64;
        let n = shares.len() as u64;

        for i in 0..shares.len() {
            let mut numerator = 1u64;
            let mut denominator = 1u64;

            for j in 0..shares.len() {
                if i != j {
                    numerator = (numerator * (n - j as u64)) % self.modulus;
                    denominator = (denominator * ((i as u64 + 1) - (j as u64 + 1))) % self.modulus;
                }
            }

            let lagrange_coeff = (numerator * self.mod_inverse(denominator)) % self.modulus;
            secret = (secret + (shares[i].value * lagrange_coeff) % self.modulus) % self.modulus;
        }

        Ok(secret)
    }

    /// Modular multiplicative inverse
    fn mod_inverse(&self, a: u64) -> u64 {
        let mut t = 0u64;
        let mut new_t = 1u64;
        let mut r = self.modulus;
        let mut new_r = a;

        while new_r != 0 {
            let quotient = r / new_r;
            let temp_t = t;
            t = new_t;
            new_t = temp_t - quotient * new_t;
            let temp_r = r;
            r = new_r;
            new_r = temp_r - quotient * new_r;
        }

        if r > 1 {
            return 0; // No inverse exists
        }

        if t < 0 {
            t += self.modulus;
        }

        t
    }
}

/// Threshold encryption implementation
pub struct ThresholdEncryption {
    /// Shamir secret sharing scheme
    pub shamir: ShamirSecretSharing,
    /// Public key
    pub public_key: u64,
    /// Private key shares
    pub private_key_shares: Vec<u64>,
    /// Initialized flag
    pub initialized: bool,
}

impl ThresholdEncryption {
    /// Create a new threshold encryption scheme
    pub fn new(threshold: usize, num_servers: usize) -> Result<Self, ProtocolError> {
        let modulus = 0xFFFFFFFFFFFFFFC5; // 2^64 - 59
        let shamir = ShamirSecretSharing::new(threshold, num_servers, modulus)?;

        Ok(Self {
            shamir,
            public_key: 0,
            private_key_shares: Vec::new(),
            initialized: false,
        })
    }

    /// Initialize the threshold encryption scheme
    pub async fn initialize(&mut self) -> Result<(), ProtocolError> {
        // Generate a random private key
        let mut rng = rand::thread_rng();
        let private_key = rng.gen_range(1..self.shamir.modulus);

        // Share the private key
        let shares = self.shamir.share_secret(private_key)?;
        self.private_key_shares = shares.iter().map(|s| s.value).collect();

        // Compute public key (g^private_key mod p)
        self.public_key = self.modular_exponentiation(self.shamir.generator, private_key);

        self.initialized = true;
        Ok(())
    }

    /// Share data using threshold encryption
    pub async fn share_data(&self, data: DataPoint) -> Result<Vec<DataShare>, ProtocolError> {
        if !self.initialized {
            return Err(ProtocolError::InternalError {
                message: "Threshold encryption not initialized".to_string(),
            });
        }

        let mut shares = Vec::new();

        // Share each feature
        for (i, &feature) in data.features().iter().enumerate() {
            let feature_u64 = feature as u64;
            let feature_shares = self.shamir.share_secret(feature_u64)?;
            
            for (j, share) in feature_shares.iter().enumerate() {
                let data_share = DataShare::new(
                    j,
                    i,
                    ShareType::Feature,
                    share.value,
                    self.shamir.modulus,
                );
                shares.push(data_share);
            }
        }

        Ok(shares)
    }

    /// Reconstruct data from shares
    pub async fn reconstruct_data(&self, shares: Vec<DataShare>) -> Result<DataPoint, ProtocolError> {
        if !self.initialized {
            return Err(ProtocolError::InternalError {
                message: "Threshold encryption not initialized".to_string(),
            });
        }

        // Group shares by feature index
        let mut feature_shares: HashMap<usize, Vec<u64>> = HashMap::new();

        for share in shares {
            if let ShareType::Feature = share.share_type {
                feature_shares.entry(share.feature_index).or_insert_with(Vec::new).push(share.value);
            }
        }

        // Reconstruct each feature
        let mut features = Vec::new();
        let num_features = feature_shares.keys().max().unwrap_or(&0) + 1;

        for i in 0..num_features {
            if let Some(share_values) = feature_shares.get(&i) {
                // Convert back to SecretShare format for reconstruction
                let mut secret_shares = Vec::new();
                for (j, &value) in share_values.iter().enumerate() {
                    let share = SecretShare::new(j, value, 0, self.shamir.modulus);
                    secret_shares.push(share);
                }

                let reconstructed_value = self.shamir.reconstruct_secret(&secret_shares)?;
                features.push(reconstructed_value as f64);
            } else {
                features.push(0.0);
            }
        }

        Ok(DataPoint::new(features))
    }

    /// Generate noise for differential privacy
    pub async fn generate_noise(&self, privacy_budget: &PrivacyBudget) -> Result<f64, ProtocolError> {
        if !self.initialized {
            return Err(ProtocolError::InternalError {
                message: "Threshold encryption not initialized".to_string(),
            });
        }

        // Generate Laplace noise
        let scale = 1.0 / privacy_budget.epsilon();
        let mut rng = rand::thread_rng();
        
        let u1: f64 = rng.gen_range(0.0..1.0);
        let u2: f64 = rng.gen_range(0.0..1.0);
        
        let noise = scale * (u1.ln() - u2.ln());
        
        Ok(noise)
    }

    /// Encrypt a value using threshold encryption
    pub async fn encrypt(&self, value: u64) -> Result<u64, ProtocolError> {
        if !self.initialized {
            return Err(ProtocolError::InternalError {
                message: "Threshold encryption not initialized".to_string(),
            });
        }

        // Simple ElGamal encryption: (g^r, m * y^r)
        let mut rng = rand::thread_rng();
        let r = rng.gen_range(1..self.shamir.modulus);
        
        let c1 = self.modular_exponentiation(self.shamir.generator, r);
        let c2 = (value * self.modular_exponentiation(self.public_key, r)) % self.shamir.modulus;
        
        Ok(c2) // Return only the second component for simplicity
    }

    /// Decrypt a value using threshold decryption
    pub async fn decrypt(&self, encrypted_value: u64, shares: &[u64]) -> Result<u64, ProtocolError> {
        if !self.initialized {
            return Err(ProtocolError::InternalError {
                message: "Threshold encryption not initialized".to_string(),
            });
        }

        if shares.len() < self.shamir.threshold {
            return Err(ProtocolError::InsufficientServers {
                available: shares.len(),
                required: self.shamir.threshold,
            });
        }

        // Threshold decryption using Lagrange interpolation
        let mut decryption_share = 0u64;
        let n = shares.len() as u64;

        for i in 0..shares.len() {
            let mut numerator = 1u64;
            let mut denominator = 1u64;

            for j in 0..shares.len() {
                if i != j {
                    numerator = (numerator * (n - j as u64)) % self.shamir.modulus;
                    denominator = (denominator * ((i as u64 + 1) - (j as u64 + 1))) % self.shamir.modulus;
                }
            }

            let lagrange_coeff = (numerator * self.shamir.mod_inverse(denominator)) % self.shamir.modulus;
            decryption_share = (decryption_share + (shares[i] * lagrange_coeff) % self.shamir.modulus) % self.shamir.modulus;
        }

        let decrypted_value = (encrypted_value * self.modular_exponentiation(decryption_share, -1)) % self.shamir.modulus;
        Ok(decrypted_value)
    }

    /// Modular exponentiation (g^e mod p)
    fn modular_exponentiation(&self, mut base: u64, mut exponent: u64) -> u64 {
        let mut result = 1u64;
        base = base % self.shamir.modulus;

        while exponent > 0 {
            if exponent % 2 == 1 {
                result = (result * base) % self.shamir.modulus;
            }
            exponent = exponent >> 1;
            base = (base * base) % self.shamir.modulus;
        }

        result
    }

    /// Get threshold
    pub fn threshold(&self) -> usize {
        self.shamir.threshold
    }

    /// Get number of shares
    pub fn num_shares(&self) -> usize {
        self.shamir.num_shares
    }

    /// Check if initialized
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
}

/// Homomorphic encryption for secure computation
pub struct HomomorphicEncryption {
    /// Public key
    pub public_key: u64,
    /// Private key
    pub private_key: u64,
    /// Modulus
    pub modulus: u64,
}

impl HomomorphicEncryption {
    /// Create a new homomorphic encryption scheme
    pub fn new() -> Self {
        let modulus = 0xFFFFFFFFFFFFFFC5;
        let mut rng = rand::thread_rng();
        let private_key = rng.gen_range(1..modulus);
        let public_key = 5u64.pow(private_key as u32) % modulus;

        Self {
            public_key,
            private_key,
            modulus,
        }
    }

    /// Encrypt a value
    pub fn encrypt(&self, value: u64) -> u64 {
        let mut rng = rand::thread_rng();
        let r = rng.gen_range(1..self.modulus);
        
        let c1 = 5u64.pow(r as u32) % self.modulus;
        let c2 = (value * self.public_key.pow(r as u32)) % self.modulus;
        
        c2 // Return only the second component for simplicity
    }

    /// Decrypt a value
    pub fn decrypt(&self, encrypted_value: u64) -> u64 {
        let decryption_key = self.public_key.pow(self.private_key as u32) % self.modulus;
        let inverse = self.modular_inverse(decryption_key);
        (encrypted_value * inverse) % self.modulus
    }

    /// Add two encrypted values
    pub fn add(&self, a: u64, b: u64) -> u64 {
        (a * b) % self.modulus
    }

    /// Multiply encrypted value by plaintext
    pub fn multiply(&self, encrypted: u64, plaintext: u64) -> u64 {
        encrypted.pow(plaintext as u32) % self.modulus
    }

    /// Modular multiplicative inverse
    fn modular_inverse(&self, a: u64) -> u64 {
        let mut t = 0i64;
        let mut new_t = 1i64;
        let mut r = self.modulus as i64;
        let mut new_r = a as i64;

        while new_r != 0 {
            let quotient = r / new_r;
            let temp_t = t;
            t = new_t;
            new_t = temp_t - quotient * new_t;
            let temp_r = r;
            r = new_r;
            new_r = temp_r - quotient * new_r;
        }

        if r > 1 {
            return 0; // No inverse exists
        }

        if t < 0 {
            t += self.modulus as i64;
        }

        t as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arith::PrivacyBudget;

    #[test]
    fn test_shamir_secret_sharing() {
        let shamir = ShamirSecretSharing::new(2, 3, 97).unwrap();
        let secret = 42u64;
        
        let shares = shamir.share_secret(secret).unwrap();
        assert_eq!(shares.len(), 3);
        
        let reconstructed = shamir.reconstruct_secret(&shares[0..2]).unwrap();
        assert_eq!(reconstructed, secret);
    }

    #[tokio::test]
    async fn test_threshold_encryption() {
        let mut crypto = ThresholdEncryption::new(2, 3).unwrap();
        crypto.initialize().await.unwrap();
        
        assert!(crypto.is_initialized());
        assert_eq!(crypto.threshold(), 2);
        assert_eq!(crypto.num_shares(), 3);
    }

    #[tokio::test]
    async fn test_data_sharing() {
        let mut crypto = ThresholdEncryption::new(2, 3).unwrap();
        crypto.initialize().await.unwrap();
        
        let data = DataPoint::new(vec![1.0, 2.0, 3.0]);
        let shares = crypto.share_data(data).await.unwrap();
        
        assert!(!shares.is_empty());
    }

    #[tokio::test]
    async fn test_noise_generation() {
        let mut crypto = ThresholdEncryption::new(2, 3).unwrap();
        crypto.initialize().await.unwrap();
        
        let privacy_budget = PrivacyBudget::new(1.0, 1e-5);
        let noise = crypto.generate_noise(&privacy_budget).await.unwrap();
        
        assert!(noise.is_finite());
    }

    #[test]
    fn test_homomorphic_encryption() {
        let crypto = HomomorphicEncryption::new();
        
        let value = 42u64;
        let encrypted = crypto.encrypt(value);
        let decrypted = crypto.decrypt(encrypted);
        
        assert_eq!(decrypted, value);
    }
} 
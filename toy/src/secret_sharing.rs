use crate::finite_field::{FieldElement, FiniteField, FieldError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Secret share structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretShare {
    /// Share ID
    pub id: usize,
    /// Share value
    pub value: FieldElement,
    /// Evaluation point
    pub point: FieldElement,
}

impl SecretShare {
    /// Create a new secret share
    pub fn new(id: usize, value: FieldElement, point: FieldElement) -> Self {
        Self { id, value, point }
    }

    /// Get share ID
    pub fn id(&self) -> usize {
        self.id
    }

    /// Get share value
    pub fn value(&self) -> FieldElement {
        self.value
    }

    /// Get evaluation point
    pub fn point(&self) -> FieldElement {
        self.point
    }
}

/// Shamir's secret sharing implementation
#[derive(Clone)]
pub struct ShamirSecretSharing {
    /// Threshold (minimum shares needed)
    pub threshold: usize,
    /// Number of shares
    pub num_shares: usize,
    /// Finite field
    pub field: FiniteField,
}

impl ShamirSecretSharing {
    /// Create a new Shamir secret sharing scheme
    pub fn new(threshold: usize, num_shares: usize, modulus: u64) -> Result<Self, FieldError> {
        if threshold > num_shares {
            return Err(FieldError::DimensionMismatch);
        }

        if threshold < 2 {
            return Err(FieldError::DimensionMismatch);
        }

        let field = FiniteField::new(modulus)?;

        Ok(Self {
            threshold,
            num_shares,
            field,
        })
    }

    /// Share a secret value
    pub fn share_secret(&self, secret: FieldElement) -> Result<Vec<SecretShare>, FieldError> {
        if secret.modulus() != self.field.modulus() {
            return Err(FieldError::ModulusMismatch);
        }

        // Generate random coefficients for polynomial
        let mut coefficients = Vec::with_capacity(self.threshold);
        coefficients.push(secret); // Constant term is the secret

        // Generate random coefficients for higher degree terms
        for _ in 1..self.threshold {
            coefficients.push(self.field.random_element());
        }

        // Generate shares by evaluating polynomial at different points
        let mut shares = Vec::with_capacity(self.num_shares);
        for i in 0..self.num_shares {
            let point = self.field.element((i + 1) as u64);
            let value = self.evaluate_polynomial(&coefficients, &point)?;
            shares.push(SecretShare::new(i, value, point));
        }

        Ok(shares)
    }

    /// Reconstruct secret from shares
    pub fn reconstruct_secret(&self, shares: &[SecretShare]) -> Result<FieldElement, FieldError> {
        if shares.len() < self.threshold {
            return Err(FieldError::DimensionMismatch);
        }

        // Use Lagrange interpolation to reconstruct the secret
        let mut secret = self.field.zero();
        let n = shares.len() as u64;

        for (i, share) in shares.iter().enumerate() {
            let mut numerator = self.field.one();
            let mut denominator = self.field.one();

            for (j, other_share) in shares.iter().enumerate() {
                if i != j {
                    // numerator *= (n - j)
                    let n_minus_j = self.field.element(n - (j as u64 + 1));
                    numerator = numerator.mul(&n_minus_j)?;

                    // denominator *= (i - j)
                    let i_minus_j = self.field.element((i as u64 + 1) - (j as u64 + 1));
                    denominator = denominator.mul(&i_minus_j)?;
                }
            }

            // Compute Lagrange coefficient
            let lagrange_coeff = numerator.div(&denominator)?;
            
            // Add contribution to secret
            let contribution = share.value().mul(&lagrange_coeff)?;
            secret = secret.add(&contribution)?;
        }

        Ok(secret)
    }

    /// Evaluate polynomial at a given point
    fn evaluate_polynomial(&self, coefficients: &[FieldElement], point: &FieldElement) -> Result<FieldElement, FieldError> {
        if coefficients.is_empty() {
            return Err(FieldError::EmptyInput);
        }

        let mut result = coefficients[0]; // Constant term
        let mut power = self.field.one();

        for coefficient in coefficients.iter().skip(1) {
            power = power.mul(point)?;
            let term = coefficient.mul(&power)?;
            result = result.add(&term)?;
        }

        Ok(result)
    }

    /// Share a vector of secrets
    pub fn share_vector(&self, secrets: &[FieldElement]) -> Result<Vec<Vec<SecretShare>>, FieldError> {
        let mut all_shares = Vec::with_capacity(secrets.len());

        for secret in secrets {
            let shares = self.share_secret(*secret)?;
            all_shares.push(shares);
        }

        Ok(all_shares)
    }

    /// Reconstruct a vector of secrets
    pub fn reconstruct_vector(&self, shares: &[Vec<SecretShare>]) -> Result<Vec<FieldElement>, FieldError> {
        let mut secrets = Vec::with_capacity(shares.len());

        for share_group in shares {
            let secret = self.reconstruct_secret(share_group)?;
            secrets.push(secret);
        }

        Ok(secrets)
    }

    /// Share a matrix of secrets
    pub fn share_matrix(&self, matrix: &[Vec<FieldElement>]) -> Result<Vec<Vec<Vec<SecretShare>>>, FieldError> {
        let mut all_shares = Vec::with_capacity(matrix.len());

        for row in matrix {
            let row_shares = self.share_vector(row)?;
            all_shares.push(row_shares);
        }

        Ok(all_shares)
    }

    /// Reconstruct a matrix of secrets
    pub fn reconstruct_matrix(&self, shares: &[Vec<Vec<SecretShare>>]) -> Result<Vec<Vec<FieldElement>>, FieldError> {
        let mut matrix = Vec::with_capacity(shares.len());

        for row_shares in shares {
            let row = self.reconstruct_vector(row_shares)?;
            matrix.push(row);
        }

        Ok(matrix)
    }

    /// Add two shared values
    pub fn add_shares(&self, a: &[SecretShare], b: &[SecretShare]) -> Result<Vec<SecretShare>, FieldError> {
        if a.len() != b.len() {
            return Err(FieldError::DimensionMismatch);
        }

        let mut result = Vec::with_capacity(a.len());
        for (share_a, share_b) in a.iter().zip(b.iter()) {
            if share_a.id() != share_b.id() {
                return Err(FieldError::DimensionMismatch);
            }
            let sum = share_a.value().add(&share_b.value())?;
            result.push(SecretShare::new(share_a.id(), sum, share_a.point()));
        }

        Ok(result)
    }

    /// Multiply shared value by a constant
    pub fn multiply_by_constant(&self, shares: &[SecretShare], constant: FieldElement) -> Result<Vec<SecretShare>, FieldError> {
        let mut result = Vec::with_capacity(shares.len());
        for share in shares {
            let product = share.value().mul(&constant)?;
            result.push(SecretShare::new(share.id(), product, share.point()));
        }

        Ok(result)
    }

    /// Get threshold
    pub fn threshold(&self) -> usize {
        self.threshold
    }

    /// Get number of shares
    pub fn num_shares(&self) -> usize {
        self.num_shares
    }

    /// Get field
    pub fn field(&self) -> &FiniteField {
        &self.field
    }
}

/// Share distribution for multiple servers
pub struct ShareDistributor {
    /// Secret sharing scheme
    pub shamir: ShamirSecretSharing,
    /// Number of servers
    pub num_servers: usize,
}

impl ShareDistributor {
    /// Create a new share distributor
    pub fn new(shamir: ShamirSecretSharing, num_servers: usize) -> Self {
        Self { shamir, num_servers }
    }

    /// Distribute shares among servers
    pub fn distribute_shares(&self, shares: Vec<SecretShare>) -> HashMap<usize, Vec<SecretShare>> {
        let mut distribution = HashMap::new();

        for share in shares {
            let server_id = share.id() % self.num_servers;
            distribution.entry(server_id).or_insert_with(Vec::new).push(share);
        }

        distribution
    }

    /// Distribute vector shares
    pub fn distribute_vector_shares(&self, shares: Vec<Vec<SecretShare>>) -> HashMap<usize, Vec<Vec<SecretShare>>> {
        let mut distribution = HashMap::new();

        for (i, share_group) in shares.into_iter().enumerate() {
            let server_id = i % self.num_servers;
            distribution.entry(server_id).or_insert_with(Vec::new).push(share_group);
        }

        distribution
    }

    /// Collect shares from servers
    pub fn collect_shares(&self, server_shares: &HashMap<usize, Vec<SecretShare>>) -> Vec<SecretShare> {
        let mut all_shares = Vec::new();
        for shares in server_shares.values() {
            all_shares.extend(shares.clone());
        }
        all_shares
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shamir_secret_sharing() {
        let shamir = ShamirSecretSharing::new(2, 3, 7).unwrap();
        let secret = FieldElement::new(5, 7);
        
        let shares = shamir.share_secret(secret).unwrap();
        assert_eq!(shares.len(), 3);
        
        let reconstructed = shamir.reconstruct_secret(&shares[0..2]).unwrap();
        assert_eq!(reconstructed.value(), secret.value());
    }

    #[test]
    fn test_vector_sharing() {
        let shamir = ShamirSecretSharing::new(2, 3, 7).unwrap();
        let secrets = vec![
            FieldElement::new(1, 7),
            FieldElement::new(2, 7),
            FieldElement::new(3, 7),
        ];
        
        let shares = shamir.share_vector(&secrets).unwrap();
        assert_eq!(shares.len(), 3);
        
        let reconstructed = shamir.reconstruct_vector(&shares).unwrap();
        assert_eq!(reconstructed.len(), 3);
        for (original, reconstructed) in secrets.iter().zip(reconstructed.iter()) {
            assert_eq!(original.value(), reconstructed.value());
        }
    }

    #[test]
    fn test_share_operations() {
        let shamir = ShamirSecretSharing::new(2, 3, 7).unwrap();
        let a = FieldElement::new(3, 7);
        let b = FieldElement::new(4, 7);
        
        let shares_a = shamir.share_secret(a).unwrap();
        let shares_b = shamir.share_secret(b).unwrap();
        
        let sum_shares = shamir.add_shares(&shares_a, &shares_b).unwrap();
        let sum = shamir.reconstruct_secret(&sum_shares).unwrap();
        
        let expected = a.add(&b).unwrap();
        assert_eq!(sum.value(), expected.value());
    }

    #[test]
    fn test_share_distributor() {
        let shamir = ShamirSecretSharing::new(2, 3, 7).unwrap();
        let distributor = ShareDistributor::new(shamir, 3);
        
        let secret = FieldElement::new(5, 7);
        let shares = distributor.shamir.share_secret(secret).unwrap();
        
        let distribution = distributor.distribute_shares(shares);
        assert_eq!(distribution.len(), 3);
        
        let collected = distributor.collect_shares(&distribution);
        assert_eq!(collected.len(), 3);
    }
} 
use crate::finite_field::{FieldElement, FiniteField, FieldError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretShare {
    pub id: usize,
    pub value: FieldElement,
    pub point: FieldElement,
}

impl SecretShare {
    pub fn new(id: usize, value: FieldElement, point: FieldElement) -> Self {
        Self { id, value, point }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn value(&self) -> FieldElement {
        self.value
    }

    pub fn point(&self) -> FieldElement {
        self.point
    }
}

#[derive(Clone)]
pub struct AdditiveSecretSharing {
    pub field: FiniteField,
}

impl AdditiveSecretSharing {
    pub fn new(modulus: u64) -> Result<Self, FieldError> {
        let field = FiniteField::new(modulus)?;
        Ok(Self { field })
    }

    pub fn share_secret(&self, secret: FieldElement) -> Result<Vec<SecretShare>, FieldError> {
        if secret.modulus() != self.field.modulus() {
            return Err(FieldError::ModulusMismatch);
        }

        let s0 = self.field.random_element();
        let s1 = secret.sub(&s0)?;
        let p0 = self.field.zero();
        let p1 = self.field.zero();

        Ok(vec![
            SecretShare::new(0, s0, p0),
            SecretShare::new(1, s1, p1),
        ])
    }

    pub fn reconstruct_secret(&self, shares: &[SecretShare]) -> Result<FieldElement, FieldError> {
        if shares.len() < 2 {
            return Err(FieldError::ModulusMismatch);
        }
        let sum = shares[0].value().add(&shares[1].value())?;
        Ok(sum)
    }

    pub fn share_vector(&self, secrets: &[FieldElement]) -> Result<Vec<Vec<SecretShare>>, FieldError> {
        let mut all_shares = Vec::with_capacity(secrets.len());
        for secret in secrets {
            let shares = self.share_secret(*secret)?;
            all_shares.push(shares);
        }
        Ok(all_shares)
    }

    pub fn reconstruct_vector(&self, shares: &[Vec<SecretShare>]) -> Result<Vec<FieldElement>, FieldError> {
        let mut secrets = Vec::with_capacity(shares.len());
        for share_group in shares {
            let secret = self.reconstruct_secret(share_group)?;
            secrets.push(secret);
        }
        Ok(secrets)
    }

    pub fn share_matrix(&self, matrix: &[Vec<FieldElement>]) -> Result<Vec<Vec<Vec<SecretShare>>>, FieldError> {
        let mut all_shares = Vec::with_capacity(matrix.len());
        for row in matrix {
            let row_shares = self.share_vector(row)?;
            all_shares.push(row_shares);
        }
        Ok(all_shares)
    }

    pub fn reconstruct_matrix(&self, shares: &[Vec<Vec<SecretShare>>]) -> Result<Vec<Vec<FieldElement>>, FieldError> {
        let mut matrix = Vec::with_capacity(shares.len());
        for row_shares in shares {
            let row = self.reconstruct_vector(row_shares)?;
            matrix.push(row);
        }
        Ok(matrix)
    }
}

#[derive(Clone)]
pub struct ShareDistributor {
    secret_sharing: AdditiveSecretSharing,
    num_servers: usize,
}

impl ShareDistributor {
    pub fn new(secret_sharing: AdditiveSecretSharing, num_servers: usize) -> Self {
        Self {
            secret_sharing,
            num_servers,
        }
    }

    pub fn distribute_to_servers(&self, shares: Vec<Vec<SecretShare>>) -> HashMap<usize, Vec<FieldElement>> {
        let mut server_shares: HashMap<usize, Vec<FieldElement>> = HashMap::new();

        for server_id in 0..self.num_servers {
            let mut server_data = Vec::new();
            for share_group in &shares {
                if let Some(share) = share_group.get(server_id) {
                    server_data.push(share.value());
                }
            }
            server_shares.insert(server_id, server_data);
        }

        server_shares
    }

    pub fn distribute_matrix_to_servers(&self, matrix_shares: Vec<Vec<Vec<SecretShare>>>) -> HashMap<usize, Vec<Vec<FieldElement>>> {
        let mut server_shares: HashMap<usize, Vec<Vec<FieldElement>>> = HashMap::new();

        for server_id in 0..self.num_servers {
            let mut server_data = Vec::new();
            for row_shares in &matrix_shares {
                let mut row_data = Vec::new();
                for share_group in row_shares {
                    if let Some(share) = share_group.get(server_id) {
                        row_data.push(share.value());
                    }
                }
                if !row_data.is_empty() {
                    server_data.push(row_data);
                }
            }
            server_shares.insert(server_id, server_data);
        }

        server_shares
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_additive_secret_sharing() {
        let additive = AdditiveSecretSharing::new(7).unwrap();
        let secret = FieldElement::new(5, 7);
        
        let shares = additive.share_secret(secret).unwrap();
        assert_eq!(shares.len(), 2);
        
        let reconstructed = additive.reconstruct_secret(&shares).unwrap();
        assert_eq!(reconstructed.value(), 5);
    }

    #[test]
    fn test_vector_sharing() {
        let additive = AdditiveSecretSharing::new(7).unwrap();
        let secrets = vec![
            FieldElement::new(1, 7),
            FieldElement::new(2, 7),
            FieldElement::new(3, 7),
        ];
        
        let shares = additive.share_vector(&secrets).unwrap();
        assert_eq!(shares.len(), 3);
        
        let reconstructed = additive.reconstruct_vector(&shares).unwrap();
        assert_eq!(reconstructed.len(), 3);
        assert_eq!(reconstructed[0].value(), 1);
        assert_eq!(reconstructed[1].value(), 2);
        assert_eq!(reconstructed[2].value(), 3);
    }

    #[test]
    fn test_share_distribution() {
        let additive = AdditiveSecretSharing::new(7).unwrap();
        let distributor = ShareDistributor::new(additive, 3);
        
        let secrets = vec![
            FieldElement::new(1, 7),
            FieldElement::new(2, 7),
        ];
        
        let shares = distributor.secret_sharing.share_vector(&secrets).unwrap();
        let server_shares = distributor.distribute_to_servers(shares);
        
        assert_eq!(server_shares.len(), 3);
        assert!(server_shares.contains_key(&0));
        assert!(server_shares.contains_key(&1));
        assert!(server_shares.contains_key(&2));
    }
} 
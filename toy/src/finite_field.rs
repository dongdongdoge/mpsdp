use serde::{Deserialize, Serialize};
use std::ops::Neg;
use std::fmt;

/// Field element in a finite field
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldElement {
    /// Value in the field
    value: u64,
    /// Field modulus (prime)
    modulus: u64,
}

impl FieldElement {
    /// Create a new field element
    pub fn new(value: u64, modulus: u64) -> Self {
        Self {
            value: value % modulus,
            modulus,
        }
    }

    /// Create zero element
    pub fn zero(modulus: u64) -> Self {
        Self { value: 0, modulus }
    }

    /// Create one element
    pub fn one(modulus: u64) -> Self {
        Self { value: 1, modulus }
    }

    /// Get the value
    pub fn value(&self) -> u64 {
        self.value
    }

    /// Get the modulus
    pub fn modulus(&self) -> u64 {
        self.modulus
    }

    /// Check if element is zero
    pub fn is_zero(&self) -> bool {
        self.value == 0
    }

    /// Check if element is one
    pub fn is_one(&self) -> bool {
        self.value == 1
    }

    /// Modular addition
    pub fn add(&self, other: &FieldElement) -> Result<FieldElement, FieldError> {
        if self.modulus != other.modulus {
            return Err(FieldError::ModulusMismatch);
        }

        let sum = self.value + other.value;
        let result = if sum >= self.modulus {
            sum - self.modulus
        } else {
            sum
        };

        Ok(FieldElement::new(result, self.modulus))
    }

    /// Modular subtraction
    pub fn sub(&self, other: &FieldElement) -> Result<FieldElement, FieldError> {
        if self.modulus != other.modulus {
            return Err(FieldError::ModulusMismatch);
        }

        let diff = if self.value >= other.value {
            self.value - other.value
        } else {
            self.modulus - (other.value - self.value)
        };

        Ok(FieldElement::new(diff, self.modulus))
    }

    /// Modular multiplication
    pub fn mul(&self, other: &FieldElement) -> Result<FieldElement, FieldError> {
        if self.modulus != other.modulus {
            return Err(FieldError::ModulusMismatch);
        }

        let product = (self.value as u128) * (other.value as u128);
        let result = (product % (self.modulus as u128)) as u64;

        Ok(FieldElement::new(result, self.modulus))
    }

    /// Modular division (multiplication by inverse)
    pub fn div(&self, other: &FieldElement) -> Result<FieldElement, FieldError> {
        if self.modulus != other.modulus {
            return Err(FieldError::ModulusMismatch);
        }

        if other.is_zero() {
            return Err(FieldError::DivisionByZero);
        }

        let inverse = other.inverse()?;
        self.mul(&inverse)
    }

    /// Modular inverse
    pub fn inverse(&self) -> Result<FieldElement, FieldError> {
        if self.is_zero() {
            return Err(FieldError::DivisionByZero);
        }

        let mut t = 0i64;
        let mut new_t = 1i64;
        let mut r = self.modulus as i64;
        let mut new_r = self.value as i64;

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
            return Err(FieldError::NoInverse);
        }

        if t < 0 {
            t += self.modulus as i64;
        }

        Ok(FieldElement::new(t as u64, self.modulus))
    }

    /// Modular exponentiation
    pub fn pow(&self, mut exponent: u64) -> Result<FieldElement, FieldError> {
        if exponent == 0 {
            return Ok(FieldElement::one(self.modulus));
        }

        let mut base = *self;
        let mut result = FieldElement::one(self.modulus);

        while exponent > 0 {
            if exponent % 2 == 1 {
                result = result.mul(&base)?;
            }
            exponent = exponent >> 1;
            base = base.mul(&base)?;
        }

        Ok(result)
    }

    /// Random field element
    pub fn random(modulus: u64) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let value = rng.gen_range(0..modulus);
        Self::new(value, modulus)
    }

    /// Convert to u64 (for compatibility)
    pub fn to_u64(&self) -> u64 {
        self.value
    }

    /// Convert from u64
    pub fn from_u64(value: u64, modulus: u64) -> Self {
        Self::new(value, modulus)
    }
}

impl Neg for FieldElement {
    type Output = FieldElement;

    fn neg(self) -> FieldElement {
        if self.is_zero() {
            self
        } else {
            FieldElement::new(self.modulus - self.value, self.modulus)
        }
    }
}

impl fmt::Display for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (mod {})", self.value, self.modulus)
    }
}

/// Finite field implementation
#[derive(Debug, Clone)]
pub struct FiniteField {
    /// Field modulus (prime)
    modulus: u64,
    /// Field generator
    generator: u64,
}

impl FiniteField {
    /// Create a new finite field
    pub fn new(modulus: u64) -> Result<Self, FieldError> {
        if !Self::is_prime(modulus) {
            return Err(FieldError::NonPrimeModulus);
        }

        Ok(Self {
            modulus,
            generator: Self::find_generator(modulus),
        })
    }

    /// Check if a number is prime
    fn is_prime(n: u64) -> bool {
        if n < 2 {
            return false;
        }
        if n == 2 {
            return true;
        }
        if n % 2 == 0 {
            return false;
        }

        let mut i = 3;
        while i * i <= n {
            if n % i == 0 {
                return false;
            }
            i += 2;
        }
        true
    }

    /// Find a generator for the field
    fn find_generator(modulus: u64) -> u64 {
        // For simplicity, use 5 as generator for most primes
        // In practice, you'd want to find a proper generator
        if modulus > 5 {
            5
        } else {
            2
        }
    }

    /// Get field modulus
    pub fn modulus(&self) -> u64 {
        self.modulus
    }

    /// Get field generator
    pub fn generator(&self) -> u64 {
        self.generator
    }

    /// Create zero element
    pub fn zero(&self) -> FieldElement {
        FieldElement::zero(self.modulus)
    }

    /// Create one element
    pub fn one(&self) -> FieldElement {
        FieldElement::one(self.modulus)
    }

    /// Create random element
    pub fn random_element(&self) -> FieldElement {
        FieldElement::random(self.modulus)
    }

    /// Create element from u64
    pub fn element(&self, value: u64) -> FieldElement {
        FieldElement::new(value, self.modulus)
    }

    /// Vector addition
    pub fn vector_add(&self, a: &[FieldElement], b: &[FieldElement]) -> Result<Vec<FieldElement>, FieldError> {
        if a.len() != b.len() {
            return Err(FieldError::DimensionMismatch);
        }

        let mut result = Vec::with_capacity(a.len());
        for (x, y) in a.iter().zip(b.iter()) {
            result.push(x.add(y)?);
        }

        Ok(result)
    }

    /// Vector subtraction
    pub fn vector_sub(&self, a: &[FieldElement], b: &[FieldElement]) -> Result<Vec<FieldElement>, FieldError> {
        if a.len() != b.len() {
            return Err(FieldError::DimensionMismatch);
        }

        let mut result = Vec::with_capacity(a.len());
        for (x, y) in a.iter().zip(b.iter()) {
            result.push(x.sub(y)?);
        }

        Ok(result)
    }

    /// Vector multiplication (element-wise)
    pub fn vector_mul(&self, a: &[FieldElement], b: &[FieldElement]) -> Result<Vec<FieldElement>, FieldError> {
        if a.len() != b.len() {
            return Err(FieldError::DimensionMismatch);
        }

        let mut result = Vec::with_capacity(a.len());
        for (x, y) in a.iter().zip(b.iter()) {
            result.push(x.mul(y)?);
        }

        Ok(result)
    }

    /// Matrix-vector multiplication
    pub fn matrix_vector_mul(&self, matrix: &[Vec<FieldElement>], vector: &[FieldElement]) -> Result<Vec<FieldElement>, FieldError> {
        if matrix.is_empty() || vector.is_empty() {
            return Err(FieldError::EmptyInput);
        }

        let rows = matrix.len();
        let cols = matrix[0].len();

        if cols != vector.len() {
            return Err(FieldError::DimensionMismatch);
        }

        let mut result = Vec::with_capacity(rows);
        for row in matrix {
            let mut sum = self.zero();
            for (matrix_elem, vector_elem) in row.iter().zip(vector.iter()) {
                let product = matrix_elem.mul(vector_elem)?;
                sum = sum.add(&product)?;
            }
            result.push(sum);
        }

        Ok(result)
    }

    /// Generate random vector
    pub fn random_vector(&self, length: usize) -> Vec<FieldElement> {
        (0..length).map(|_| self.random_element()).collect()
    }

    /// Generate random matrix
    pub fn random_matrix(&self, rows: usize, cols: usize) -> Vec<Vec<FieldElement>> {
        (0..rows).map(|_| self.random_vector(cols)).collect()
    }
}

/// Field operation errors
#[derive(Debug, thiserror::Error)]
pub enum FieldError {
    #[error("Modulus mismatch between field elements")]
    ModulusMismatch,
    #[error("Division by zero")]
    DivisionByZero,
    #[error("No multiplicative inverse exists")]
    NoInverse,
    #[error("Non-prime modulus")]
    NonPrimeModulus,
    #[error("Dimension mismatch")]
    DimensionMismatch,
    #[error("Empty input")]
    EmptyInput,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_element_creation() {
        let elem = FieldElement::new(5, 7);
        assert_eq!(elem.value(), 5);
        assert_eq!(elem.modulus(), 7);
    }

    #[test]
    fn test_field_operations() {
        let a = FieldElement::new(5, 7);
        let b = FieldElement::new(3, 7);

        // Addition
        let sum = a.add(&b).unwrap();
        assert_eq!(sum.value(), 1); // (5 + 3) mod 7 = 1

        // Subtraction
        let diff = a.sub(&b).unwrap();
        assert_eq!(diff.value(), 2); // (5 - 3) mod 7 = 2

        // Multiplication
        let product = a.mul(&b).unwrap();
        assert_eq!(product.value(), 1); // (5 * 3) mod 7 = 1

        // Division
        let quotient = a.div(&b).unwrap();
        assert_eq!(quotient.value(), 4); // (5 * 3^(-1)) mod 7 = 4
    }

    #[test]
    fn test_finite_field() {
        let field = FiniteField::new(7).unwrap();
        assert_eq!(field.modulus(), 7);
        assert_eq!(field.generator(), 5);
    }

    #[test]
    fn test_vector_operations() {
        let field = FiniteField::new(7).unwrap();
        let a = vec![field.element(1), field.element(2), field.element(3)];
        let b = vec![field.element(4), field.element(5), field.element(6)];

        let sum = field.vector_add(&a, &b).unwrap();
        assert_eq!(sum[0].value(), 5); // (1 + 4) mod 7 = 5
        assert_eq!(sum[1].value(), 0); // (2 + 5) mod 7 = 0
        assert_eq!(sum[2].value(), 2); // (3 + 6) mod 7 = 2
    }

    #[test]
    fn test_matrix_vector_multiplication() {
        let field = FiniteField::new(7).unwrap();
        let matrix = vec![
            vec![field.element(1), field.element(2)],
            vec![field.element(3), field.element(4)],
        ];
        let vector = vec![field.element(5), field.element(6)];

        let result = field.matrix_vector_mul(&matrix, &vector).unwrap();
        assert_eq!(result.len(), 2);
    }
} 
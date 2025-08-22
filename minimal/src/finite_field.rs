use serde::{Deserialize, Serialize};
use std::ops::Neg;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldElement {
    value: u64,
    modulus: u64,
}

impl FieldElement {
    pub fn new(value: u64, modulus: u64) -> Self {
        Self {
            value: value % modulus,
            modulus,
        }
    }

    pub fn zero(modulus: u64) -> Self {
        Self { value: 0, modulus }
    }

    pub fn one(modulus: u64) -> Self {
        Self { value: 1, modulus }
    }

    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn modulus(&self) -> u64 {
        self.modulus
    }

    pub fn is_zero(&self) -> bool {
        self.value == 0
    }

    pub fn is_one(&self) -> bool {
        self.value == 1
    }

    pub fn add(&self, other: &FieldElement) -> Result<FieldElement, FieldError> {
        if self.modulus != other.modulus {
            return Err(FieldError::ModulusMismatch);
        }

        let sum = (self.value as u128) + (other.value as u128);
        let result = (sum % (self.modulus as u128)) as u64;

        Ok(FieldElement::new(result, self.modulus))
    }

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

    pub fn mul(&self, other: &FieldElement) -> Result<FieldElement, FieldError> {
        if self.modulus != other.modulus {
            return Err(FieldError::ModulusMismatch);
        }

        let product = (self.value as u128) * (other.value as u128);
        let result = (product % (self.modulus as u128)) as u64;

        Ok(FieldElement::new(result, self.modulus))
    }

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
            let temp_t = new_t;
            new_t = t - quotient * new_t;
            t = temp_t;
            let temp_r = new_r;
            new_r = r - quotient * new_r;
            r = temp_r;
        }

        if r > 1 {
            return Err(FieldError::NoInverse);
        }

        if t < 0 {
            t += self.modulus as i64;
        }

        Ok(FieldElement::new(t as u64, self.modulus))
    }

    pub fn pow(&self, mut exponent: u64) -> Result<FieldElement, FieldError> {
        if exponent == 0 {
            return Ok(FieldElement::one(self.modulus));
        }

        let mut base = *self;
        let mut result = FieldElement::one(self.modulus);

        while exponent > 0 {
            if exponent & 1 == 1 {
                result = result.mul(&base)?;
            }
            base = base.mul(&base)?;
            exponent >>= 1;
        }

        Ok(result)
    }
}

impl Neg for FieldElement {
    type Output = FieldElement;

    fn neg(self) -> Self::Output {
        if self.value == 0 {
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

#[derive(Debug, Clone)]
pub struct FiniteField {
    modulus: u64,
}

impl FiniteField {
    pub fn new(modulus: u64) -> Result<Self, FieldError> {
        if modulus < 2 {
            return Err(FieldError::InvalidModulus);
        }

        if !Self::is_prime(modulus) {
            return Err(FieldError::NonPrimeModulus);
        }

        Ok(Self { modulus })
    }

    pub fn modulus(&self) -> u64 {
        self.modulus
    }

    pub fn element(&self, value: u64) -> FieldElement {
        FieldElement::new(value, self.modulus)
    }

    pub fn zero(&self) -> FieldElement {
        FieldElement::zero(self.modulus)
    }

    pub fn one(&self) -> FieldElement {
        FieldElement::one(self.modulus)
    }

    pub fn random_element(&self) -> FieldElement {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let value = rng.gen_range(0..self.modulus);
        FieldElement::new(value, self.modulus)
    }

    fn is_prime(n: u64) -> bool {
        if n < 2 {
            return false;
        }
        if n < 4 {
            return true;
        }
        if n % 2 == 0 || n % 3 == 0 {
            return false;
        }

        let mut i = 5;
        while i <= n / i {
            if n % i == 0 || n % (i + 2) == 0 {
                return false;
            }
            i += 6;
        }
        true
    }
}

#[derive(Debug, Clone)]
pub enum FieldError {
    ModulusMismatch,
    DivisionByZero,
    NoInverse,
    InvalidModulus,
    NonPrimeModulus,
}

impl std::fmt::Display for FieldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldError::ModulusMismatch => write!(f, "Modulus mismatch"),
            FieldError::DivisionByZero => write!(f, "Division by zero"),
            FieldError::NoInverse => write!(f, "No multiplicative inverse"),
            FieldError::InvalidModulus => write!(f, "Invalid modulus"),
            FieldError::NonPrimeModulus => write!(f, "Non-prime modulus"),
        }
    }
}

impl std::error::Error for FieldError {}

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
        let field = FiniteField::new(7).unwrap();
        let a = field.element(5);
        let b = field.element(3);

        let sum = a.add(&b).unwrap();
        assert_eq!(sum.value(), 1);

        let diff = a.sub(&b).unwrap();
        assert_eq!(diff.value(), 2);

        let product = a.mul(&b).unwrap();
        assert_eq!(product.value(), 1);
    }

    #[test]
    fn test_field_inverse() {
        let field = FiniteField::new(7).unwrap();
        let a = field.element(3);
        let inv = a.inverse().unwrap();
        let product = a.mul(&inv).unwrap();
        assert_eq!(product.value(), 1);
    }

    #[test]
    fn test_field_power() {
        let field = FiniteField::new(7).unwrap();
        let a = field.element(2);
        let pow = a.pow(3).unwrap();
        assert_eq!(pow.value(), 1);
    }
} 
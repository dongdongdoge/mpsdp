// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT license.

#[derive(Debug, Clone, Copy)]
pub struct PrivacyBudget {
    pub epsilon: f64,
    pub delta: f64,
}

impl PrivacyBudget {
    pub fn new(epsilon: f64, delta: f64) -> Self {
        Self { epsilon, delta }
    }

    pub fn epsilon(&self) -> f64 {
        self.epsilon
    }

    pub fn delta(&self) -> f64 {
        self.delta
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_privacy_budget() {
        let budget = PrivacyBudget::new(1.0, 1e-5);
        assert_eq!(budget.epsilon(), 1.0);
        assert_eq!(budget.delta(), 1e-5);
    }
}

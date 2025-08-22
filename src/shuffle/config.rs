use crate::arith::PrivacyBudget;

#[derive(Debug, Clone)]
pub struct ShuffleConfig {
    pub shuffle_rounds: usize,
    pub privacy_budget: PrivacyBudget,
}

impl ShuffleConfig {
    pub fn new(shuffle_rounds: usize, privacy_budget: PrivacyBudget) -> Self {
        Self {
            shuffle_rounds,
            privacy_budget,
        }
    }

    pub fn builder() -> ShuffleConfigBuilder {
        ShuffleConfigBuilder::new()
    }
}

impl Default for ShuffleConfig {
    fn default() -> Self {
        Self {
            shuffle_rounds: 3,
            privacy_budget: PrivacyBudget::new(1.0, 1e-5),
        }
    }
}

pub struct ShuffleConfigBuilder {
    shuffle_rounds: usize,
    privacy_budget: PrivacyBudget,
}

impl ShuffleConfigBuilder {
    pub fn new() -> Self {
        Self {
            shuffle_rounds: 3,
            privacy_budget: PrivacyBudget::new(1.0, 1e-5),
        }
    }

    pub fn shuffle_rounds(mut self, rounds: usize) -> Self {
        self.shuffle_rounds = rounds;
        self
    }

    pub fn privacy_budget(mut self, budget: PrivacyBudget) -> Self {
        self.privacy_budget = budget;
        self
    }

    pub fn build(self) -> ShuffleConfig {
        ShuffleConfig {
            shuffle_rounds: self.shuffle_rounds,
            privacy_budget: self.privacy_budget,
        }
    }
} 
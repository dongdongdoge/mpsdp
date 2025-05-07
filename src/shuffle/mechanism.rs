use crate::schema::{DataPoint, Query, QueryResult};
use crate::arith::PrivacyBudget;
use crate::random;
use super::ShuffleError;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct ShuffleMechanism {
    rng: rand::rngs::ThreadRng,
}

impl ShuffleMechanism {
    pub fn new() -> Self {
        Self {
            rng: thread_rng(),
        }
    }

    pub fn shuffle(&mut self, mut data: Vec<DataPoint>, rounds: usize) -> Result<Vec<DataPoint>, ShuffleError> {
        if data.is_empty() {
            return Err(ShuffleError::InvalidInput);
        }

        for _ in 0..rounds {
            data.shuffle(&mut self.rng);
        }

        Ok(data)
    }

    pub fn process_query(&self, query: Query, data: Vec<DataPoint>, config: &super::ShuffleConfig) -> Result<QueryResult, ShuffleError> {
        if data.is_empty() {
            return Err(ShuffleError::InvalidInput);
        }

        // Apply shuffle
        let mut shuffled_data = data;
        shuffled_data.shuffle(&mut self.rng);

        // Process query based on type
        let result = match query.query_type {
            crate::schema::QueryType::Mean => self.process_mean_query(&shuffled_data, &query),
            crate::schema::QueryType::Histogram => self.process_histogram_query(&shuffled_data, &query),
            _ => return Err(ShuffleError::InvalidInput),
        };

        // Add noise based on privacy budget
        let noisy_result = self.add_noise(result, &config.privacy_budget)?;
        Ok(noisy_result)
    }

    fn process_mean_query(&self, data: &[DataPoint], query: &Query) -> QueryResult {
        let mut sums = vec![0.0; query.features.len()];
        let mut counts = vec![0; query.features.len()];

        for point in data {
            for (i, feature) in query.features.iter().enumerate() {
                if let Some(value) = point.get_feature(feature) {
                    sums[i] += value;
                    counts[i] += 1;
                }
            }
        }

        let means: Vec<f64> = sums.iter()
            .zip(counts.iter())
            .map(|(&sum, &count)| if count > 0 { sum / count as f64 } else { 0.0 })
            .collect();

        QueryResult::new(means)
    }

    fn process_histogram_query(&self, data: &[DataPoint], query: &Query) -> QueryResult {
        let mut histogram = std::collections::HashMap::new();

        for point in data {
            for feature in &query.features {
                if let Some(value) = point.get_feature(feature) {
                    *histogram.entry(value).or_insert(0) += 1;
                }
            }
        }

        let values: Vec<f64> = histogram.values().map(|&v| v as f64).collect();
        QueryResult::new(values)
    }

    fn add_noise(&self, mut result: QueryResult, budget: &PrivacyBudget) -> Result<QueryResult, ShuffleError> {
        let scale = 1.0 / budget.epsilon();
        
        for value in result.values_mut() {
            *value += random::laplace_noise(scale);
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::QueryType;

    #[test]
    fn test_shuffle_mechanism() {
        let mut mechanism = ShuffleMechanism::new();
        let data = vec![
            DataPoint::new(vec![1.0, 2.0]),
            DataPoint::new(vec![3.0, 4.0]),
            DataPoint::new(vec![5.0, 6.0]),
        ];

        let shuffled = mechanism.shuffle(data, 3).unwrap();
        assert_eq!(shuffled.len(), 3);
    }

    #[test]
    fn test_process_mean_query() {
        let mechanism = ShuffleMechanism::new();
        let data = vec![
            DataPoint::new(vec![1.0, 2.0]),
            DataPoint::new(vec![3.0, 4.0]),
            DataPoint::new(vec![5.0, 6.0]),
        ];

        let query = Query::new(
            QueryType::Mean,
            vec!["feature1".to_string()],
        );

        let config = super::super::ShuffleConfig::default();
        let result = mechanism.process_query(query, data, &config).unwrap();
        assert!(result.has_noise());
    }
} 
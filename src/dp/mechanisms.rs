use crate::schema::{DataPoint, Query, QueryResult};
use crate::arith::PrivacyBudget;
use crate::random;
use super::{DPConfig, DPError, MechanismType};

pub struct DPMechanismImpl {
    mechanism_type: MechanismType,
}

impl DPMechanismImpl {
    pub fn new(mechanism_type: MechanismType) -> Self {
        Self { mechanism_type }
    }

    pub fn apply(&self, data: Vec<DataPoint>, query: Query, config: &DPConfig) -> Result<QueryResult, DPError> {
        if data.is_empty() {
            return Err(DPError::InvalidInput);
        }

        // Calculate raw result
        let raw_result = self.compute_raw_result(&data, &query)?;
        
        // Add noise based on mechanism type
        let noisy_result = match self.mechanism_type {
            MechanismType::Laplace => self.add_laplace_noise(raw_result, config),
            MechanismType::Gaussian => self.add_gaussian_noise(raw_result, config),
            MechanismType::Exponential => self.add_exponential_noise(raw_result, config),
        };

        Ok(noisy_result)
    }

    pub fn get_sensitivity(&self, query: &Query) -> f64 {
        match query.query_type {
            crate::schema::QueryType::Mean => 1.0,
            crate::schema::QueryType::Histogram => 1.0,
            _ => 0.0,
        }
    }

    fn compute_raw_result(&self, data: &[DataPoint], query: &Query) -> Result<QueryResult, DPError> {
        match query.query_type {
            crate::schema::QueryType::Mean => self.compute_mean(data, query),
            crate::schema::QueryType::Histogram => self.compute_histogram(data, query),
            _ => Err(DPError::InvalidInput),
        }
    }

    fn compute_mean(&self, data: &[DataPoint], query: &Query) -> Result<QueryResult, DPError> {
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

        Ok(QueryResult::new(means))
    }

    fn compute_histogram(&self, data: &[DataPoint], query: &Query) -> Result<QueryResult, DPError> {
        let mut histogram = std::collections::HashMap::new();

        for point in data {
            for feature in &query.features {
                if let Some(value) = point.get_feature(feature) {
                    *histogram.entry(value).or_insert(0) += 1;
                }
            }
        }

        let values: Vec<f64> = histogram.values().map(|&v| v as f64).collect();
        Ok(QueryResult::new(values))
    }

    fn add_laplace_noise(&self, mut result: QueryResult, config: &DPConfig) -> QueryResult {
        let sensitivity = self.get_sensitivity(&result.query);
        let scale = sensitivity / config.privacy_budget.epsilon();
        
        for value in result.values_mut() {
            *value += random::laplace_noise(scale);
        }

        result
    }

    fn add_gaussian_noise(&self, mut result: QueryResult, config: &DPConfig) -> QueryResult {
        let sensitivity = self.get_sensitivity(&result.query);
        let sigma = sensitivity * (2.0 * config.privacy_budget.delta().ln()).sqrt() / config.privacy_budget.epsilon();
        
        for value in result.values_mut() {
            *value += random::gaussian_noise(sigma);
        }

        result
    }

    fn add_exponential_noise(&self, mut result: QueryResult, config: &DPConfig) -> QueryResult {
        let sensitivity = self.get_sensitivity(&result.query);
        let scale = sensitivity / config.privacy_budget.epsilon();
        
        for value in result.values_mut() {
            *value += random::exponential_noise(scale);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::QueryType;

    #[test]
    fn test_laplace_mechanism() {
        let mechanism = DPMechanismImpl::new(MechanismType::Laplace);
        let data = vec![
            DataPoint::new(vec![1.0, 2.0]),
            DataPoint::new(vec![3.0, 4.0]),
        ];

        let query = Query::new(
            QueryType::Mean,
            vec!["feature1".to_string()],
        );

        let config = DPConfig::default();
        let result = mechanism.apply(data, query, &config).unwrap();
        assert!(result.has_noise());
    }

    #[test]
    fn test_gaussian_mechanism() {
        let mechanism = DPMechanismImpl::new(MechanismType::Gaussian);
        let data = vec![
            DataPoint::new(vec![1.0, 2.0]),
            DataPoint::new(vec![3.0, 4.0]),
        ];

        let query = Query::new(
            QueryType::Mean,
            vec!["feature1".to_string()],
        );

        let config = DPConfig::default();
        let result = mechanism.apply(data, query, &config).unwrap();
        assert!(result.has_noise());
    }

    #[test]
    fn test_sensitivity_calculation() {
        let mechanism = DPMechanismImpl::new(MechanismType::Laplace);
        
        let mean_query = Query::new(
            QueryType::Mean,
            vec!["feature1".to_string()],
        );
        assert_eq!(mechanism.get_sensitivity(&mean_query), 1.0);

        let hist_query = Query::new(
            QueryType::Histogram,
            vec!["feature1".to_string()],
        );
        assert_eq!(mechanism.get_sensitivity(&hist_query), 1.0);
    }
} 
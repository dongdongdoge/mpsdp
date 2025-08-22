// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT license.

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QueryType {
    Mean,
    Variance,
    Histogram,
    Range,
    Count,
    Sum,
}

#[derive(Debug, Clone)]
pub struct DataPoint {
    features: Vec<f64>,
}

impl DataPoint {
    pub fn new(features: Vec<f64>) -> Self {
        Self { features }
    }

    pub fn features(&self) -> &[f64] {
        &self.features
    }

    pub fn features_mut(&mut self) -> &mut [f64] {
        &mut self.features
    }

    pub fn get_feature(&self, feature_name: &str) -> Option<f64> {
        match feature_name {
            "feature1" => self.features.get(0).copied(),
            "feature2" => self.features.get(1).copied(),
            "feature3" => self.features.get(2).copied(),
            _ => None,
        }
    }

    pub fn set_feature(&mut self, feature_name: &str, value: f64) -> Result<(), String> {
        match feature_name {
            "feature1" => {
                if self.features.len() > 0 {
                    self.features[0] = value;
                    Ok(())
                } else {
                    Err("Feature index out of bounds".to_string())
                }
            }
            "feature2" => {
                if self.features.len() > 1 {
                    self.features[1] = value;
                    Ok(())
                } else {
                    Err("Feature index out of bounds".to_string())
                }
            }
            "feature3" => {
                if self.features.len() > 2 {
                    self.features[2] = value;
                    Ok(())
                } else {
                    Err("Feature index out of bounds".to_string())
                }
            }
            _ => Err(format!("Unknown feature: {}", feature_name)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Query {
    pub query_type: QueryType,
    pub features: Vec<String>,
    pub parameters: HashMap<String, f64>,
}

impl Query {
    pub fn new(query_type: QueryType, features: Vec<String>) -> Self {
        Self {
            query_type,
            features,
            parameters: HashMap::new(),
        }
    }

    pub fn with_parameters(
        query_type: QueryType,
        features: Vec<String>,
        parameters: HashMap<String, f64>,
    ) -> Self {
        Self {
            query_type,
            features,
            parameters,
        }
    }

    pub fn add_parameter(&mut self, key: impl Into<String>, value: f64) {
        self.parameters.insert(key.into(), value);
    }

    pub fn get_parameter(&self, key: &str) -> Option<f64> {
        self.parameters.get(key).copied()
    }
}

#[derive(Debug, Clone)]
pub struct QueryResult {
    values: Vec<f64>,
    has_noise: bool,
    privacy_budget_used: f64,
}

impl QueryResult {
    pub fn new(values: Vec<f64>) -> Self {
        Self {
            values,
            has_noise: false,
            privacy_budget_used: 0.0,
        }
    }

    pub fn with_noise(values: Vec<f64>, privacy_budget_used: f64) -> Self {
        Self {
            values,
            has_noise: true,
            privacy_budget_used,
        }
    }

    pub fn values(&self) -> &[f64] {
        &self.values
    }

    pub fn values_mut(&mut self) -> &mut [f64] {
        &mut self.values
    }

    pub fn has_noise(&self) -> bool {
        self.has_noise
    }

    pub fn privacy_budget_used(&self) -> f64 {
        self.privacy_budget_used
    }

    pub fn set_privacy_budget_used(&mut self, budget: f64) {
        self.privacy_budget_used = budget;
    }

    pub fn mark_as_noisy(&mut self) {
        self.has_noise = true;
    }
}

#[cfg(test)]
mod query_tests {
    use super::*;

    #[test]
    fn test_data_point_creation() {
        let data = DataPoint::new(vec![1.0, 2.0, 3.0]);
        assert_eq!(data.features(), &[1.0, 2.0, 3.0]);
        assert_eq!(data.get_feature("feature1"), Some(1.0));
        assert_eq!(data.get_feature("feature2"), Some(2.0));
        assert_eq!(data.get_feature("feature3"), Some(3.0));
    }

    #[test]
    fn test_data_point_feature_setting() {
        let mut data = DataPoint::new(vec![1.0, 2.0, 3.0]);
        assert!(data.set_feature("feature1", 5.0).is_ok());
        assert_eq!(data.get_feature("feature1"), Some(5.0));
        assert!(data.set_feature("unknown", 1.0).is_err());
    }

    #[test]
    fn test_query_creation() {
        let query = Query::new(
            QueryType::Mean,
            vec!["feature1".to_string(), "feature2".to_string()],
        );
        assert_eq!(query.query_type, QueryType::Mean);
        assert_eq!(query.features.len(), 2);
    }

    #[test]
    fn test_query_with_parameters() {
        let mut query = Query::new(QueryType::Histogram, vec!["feature1".to_string()]);
        query.add_parameter("bins", 10.0);
        assert_eq!(query.get_parameter("bins"), Some(10.0));
        assert_eq!(query.get_parameter("nonexistent"), None);
    }

    #[test]
    fn test_query_result() {
        let result = QueryResult::new(vec![1.0, 2.0, 3.0]);
        assert_eq!(result.values(), &[1.0, 2.0, 3.0]);
        assert!(!result.has_noise());

        let noisy_result = QueryResult::with_noise(vec![1.0, 2.0], 0.5);
        assert!(noisy_result.has_noise());
        assert_eq!(noisy_result.privacy_budget_used(), 0.5);
    }
}

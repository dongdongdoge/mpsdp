use doppio::shuffle::{Shuffler, ShuffleConfig, ShuffleMechanism, ShuffleAlgorithm};
use doppio::schema::{DataPoint, Query, QueryType, Schema, AttributeType};
use doppio::arith::PrivacyBudget;

fn main() {
    // Example 1: Basic shuffle usage
    basic_shuffle_example();

    // Example 2: Shuffle with custom configuration
    custom_config_example();

    // Example 3: Query processing with shuffle
    query_processing_example();

    // Example 4: Different shuffle algorithms
    algorithm_comparison_example();
}

fn basic_shuffle_example() {
    println!("=== Basic Shuffle Example ===");
    
    // Create default shuffler
    let mut shuffler = Shuffler::new_default();
    
    // Create some sample data
    let data = vec![
        DataPoint::new(vec![1.0, 2.0]),
        DataPoint::new(vec![3.0, 4.0]),
        DataPoint::new(vec![5.0, 6.0]),
        DataPoint::new(vec![7.0, 8.0]),
    ];

    println!("Original data: {:?}", data.iter().map(|d| d.features()).collect::<Vec<_>>());

    // Shuffle the data
    match shuffler.shuffle_data(data) {
        Ok(shuffled) => {
            println!("Shuffled data: {:?}", shuffled.iter().map(|d| d.features()).collect::<Vec<_>>());
        }
        Err(e) => {
            println!("Shuffle failed: {}", e);
        }
    }
}

fn custom_config_example() {
    println!("\n=== Custom Configuration Example ===");
    
    // Create a schema for data validation
    let schema = Schema(vec![
        ("feature1".to_string(), AttributeType::C4),
        ("feature2".to_string(), AttributeType::N8(255)),
    ]);

    // Create custom configuration
    let config = ShuffleConfig::builder()
        .schema(schema)
        .shuffle_rounds(5)
        .privacy_budget(PrivacyBudget::new(0.5, 1e-6))
        .batch_size(500)
        .adaptive_noise(true)
        .seed(12345)
        .build();

    let mut shuffler = Shuffler::new(config);
    
    let data = vec![
        DataPoint::new(vec![1.0, 2.0]),
        DataPoint::new(vec![3.0, 4.0]),
        DataPoint::new(vec![5.0, 6.0]),
    ];

    match shuffler.shuffle_data(data) {
        Ok(shuffled) => {
            println!("Custom shuffled data: {:?}", shuffled.iter().map(|d| d.features()).collect::<Vec<_>>());
        }
        Err(e) => {
            println!("Custom shuffle failed: {}", e);
        }
    }
}

fn query_processing_example() {
    println!("\n=== Query Processing Example ===");
    
    let config = ShuffleConfig::default();
    let shuffler = Shuffler::new(config);
    
    let data = vec![
        DataPoint::new(vec![1.0, 2.0]),
        DataPoint::new(vec![3.0, 4.0]),
        DataPoint::new(vec![5.0, 6.0]),
        DataPoint::new(vec![7.0, 8.0]),
    ];

    // Process different types of queries
    let queries = vec![
        Query::new(QueryType::Mean, vec!["feature1".to_string(), "feature2".to_string()]),
        Query::new(QueryType::Variance, vec!["feature1".to_string()]),
        Query::new(QueryType::Histogram, vec!["feature1".to_string()]),
    ];

    for (i, query) in queries.iter().enumerate() {
        println!("Query {}: {:?}", i + 1, query.query_type);
        
        match shuffler.process_query(query.clone(), data.clone()) {
            Ok(result) => {
                println!("  Result: {:?}", result.values());
                println!("  Has noise: {}", result.has_noise());
            }
            Err(e) => {
                println!("  Query failed: {}", e);
            }
        }
    }
}

fn algorithm_comparison_example() {
    println!("\n=== Algorithm Comparison Example ===");
    
    let data = vec![
        DataPoint::new(vec![1.0, 2.0]),
        DataPoint::new(vec![3.0, 4.0]),
        DataPoint::new(vec![5.0, 6.0]),
    ];

    let algorithms = [
        ShuffleAlgorithm::FisherYates,
        ShuffleAlgorithm::Knuth,
        ShuffleAlgorithm::Deterministic,
    ];

    for algorithm in algorithms {
        println!("Testing {:?} algorithm:", algorithm);
        
        let mut mechanism = ShuffleMechanism::with_algorithm(algorithm);
        if algorithm == ShuffleAlgorithm::Deterministic {
            mechanism.set_seed(12345);
        }

        match mechanism.shuffle(data.clone(), 3, &PrivacyBudget::new(1.0, 1e-5)) {
            Ok(shuffled) => {
                println!("  Result: {:?}", shuffled.iter().map(|d| d.features()).collect::<Vec<_>>());
            }
            Err(e) => {
                println!("  Failed: {}", e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_shuffle() {
        let mut shuffler = Shuffler::new_default();
        let data = vec![
            DataPoint::new(vec![1.0, 2.0]),
            DataPoint::new(vec![3.0, 4.0]),
        ];

        let result = shuffler.shuffle_data(data);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 2);
    }

    #[test]
    fn test_query_processing() {
        let shuffler = Shuffler::new_default();
        let data = vec![
            DataPoint::new(vec![1.0, 2.0]),
            DataPoint::new(vec![3.0, 4.0]),
        ];

        let query = Query::new(QueryType::Mean, vec!["feature1".to_string()]);
        let result = shuffler.process_query(query, data);
        assert!(result.is_ok());
    }
} 
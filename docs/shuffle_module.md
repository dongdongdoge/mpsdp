# Shuffle Module Documentation

## Overview

The shuffle module provides a comprehensive implementation of shuffle differential privacy, offering better privacy-utility trade-offs compared to local differential privacy. The module is designed to be modular, extensible, and easy to use.

## Key Features

- **Multiple Shuffle Algorithms**: Fisher-Yates, Knuth, Cryptographic, and Deterministic shuffling
- **Flexible Configuration**: Builder pattern for easy configuration setup
- **Comprehensive Error Handling**: Detailed error types with recovery information
- **Privacy Guarantees**: Built-in differential privacy with noise addition
- **Query Processing**: Support for various query types (Mean, Variance, Histogram, Range)
- **Schema Validation**: Optional schema-based data validation
- **Performance Monitoring**: Built-in statistics and timing information

## Architecture

### Core Components

1. **Shuffler**: Main orchestrator for shuffle operations
2. **ShuffleMechanism**: Implements different shuffle algorithms
3. **ShuffleConfig**: Configuration management with builder pattern
4. **ShuffleError**: Comprehensive error handling
5. **Types**: Data structures for shuffle operations

### Module Structure

```
src/shuffle/
├── mod.rs          # Main module with Shuffler
├── config.rs       # Configuration management
├── error.rs        # Error handling
├── mechanism.rs    # Shuffle algorithms
└── types.rs        # Data structures
```

## Usage Examples

### Basic Usage

```rust
use doppio::shuffle::{Shuffler, ShuffleConfig};
use doppio::schema::{DataPoint, Query, QueryType};

// Create a default shuffler
let mut shuffler = Shuffler::new_default();

// Create sample data
let data = vec![
    DataPoint::new(vec![1.0, 2.0]),
    DataPoint::new(vec![3.0, 4.0]),
    DataPoint::new(vec![5.0, 6.0]),
];

// Shuffle the data
match shuffler.shuffle_data(data) {
    Ok(shuffled) => println!("Shuffled {} data points", shuffled.len()),
    Err(e) => println!("Shuffle failed: {}", e),
}
```

### Custom Configuration

```rust
use doppio::shuffle::{Shuffler, ShuffleConfig};
use doppio::schema::{Schema, AttributeType};
use doppio::arith::PrivacyBudget;

// Create a schema for validation
let schema = Schema(vec![
    ("feature1".to_string(), AttributeType::C4),
    ("feature2".to_string(), AttributeType::N8(255)),
]);

// Build custom configuration
let config = ShuffleConfig::builder()
    .schema(schema)
    .shuffle_rounds(5)
    .privacy_budget(PrivacyBudget::new(0.5, 1e-6))
    .batch_size(500)
    .adaptive_noise(true)
    .seed(12345)
    .build();

let mut shuffler = Shuffler::new(config);
```

### Query Processing

```rust
use doppio::shuffle::Shuffler;
use doppio::schema::{DataPoint, Query, QueryType};

let shuffler = Shuffler::new_default();
let data = vec![
    DataPoint::new(vec![1.0, 2.0]),
    DataPoint::new(vec![3.0, 4.0]),
    DataPoint::new(vec![5.0, 6.0]),
];

// Process different query types
let queries = vec![
    Query::new(QueryType::Mean, vec!["feature1".to_string()]),
    Query::new(QueryType::Variance, vec!["feature1".to_string()]),
    Query::new(QueryType::Histogram, vec!["feature1".to_string()]),
];

for query in queries {
    match shuffler.process_query(query, data.clone()) {
        Ok(result) => {
            println!("Result: {:?}", result.values());
            println!("Has noise: {}", result.has_noise());
        }
        Err(e) => println!("Query failed: {}", e),
    }
}
```

### Different Shuffle Algorithms

```rust
use doppio::shuffle::{ShuffleMechanism, ShuffleAlgorithm};
use doppio::arith::PrivacyBudget;

let algorithms = [
    ShuffleAlgorithm::FisherYates,
    ShuffleAlgorithm::Knuth,
    ShuffleAlgorithm::Deterministic,
];

for algorithm in algorithms {
    let mut mechanism = ShuffleMechanism::with_algorithm(algorithm);
    
    if algorithm == ShuffleAlgorithm::Deterministic {
        mechanism.set_seed(12345);
    }
    
    let data = vec![/* your data */];
    let result = mechanism.shuffle(data, 3, &PrivacyBudget::new(1.0, 1e-5));
}
```

## Configuration Options

### ShuffleConfig

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `shuffle_rounds` | `usize` | 3 | Number of shuffle rounds |
| `privacy_budget` | `PrivacyBudget` | (1.0, 1e-5) | Privacy budget (ε, δ) |
| `batch_size` | `usize` | 1000 | Maximum batch size |
| `schema` | `Option<Schema>` | None | Optional schema for validation |
| `adaptive_noise` | `bool` | false | Enable adaptive noise scaling |
| `seed` | `Option<u64>` | None | Seed for deterministic shuffling |
| `max_retries` | `usize` | 3 | Maximum retries for failed operations |

### Privacy Budget

The privacy budget controls the trade-off between privacy and utility:

- **Epsilon (ε)**: Controls the privacy level (lower = more private)
- **Delta (δ)**: Controls the probability of privacy failure

```rust
use doppio::arith::PrivacyBudget;

// More private (ε = 0.1)
let strict_budget = PrivacyBudget::new(0.1, 1e-6);

// Less private, more utility (ε = 2.0)
let relaxed_budget = PrivacyBudget::new(2.0, 1e-5);
```

## Error Handling

The module provides comprehensive error handling with detailed error types:

```rust
use doppio::shuffle::ShuffleError;

match shuffler.shuffle_data(data) {
    Ok(result) => { /* handle success */ },
    Err(ShuffleError::EmptyInput) => {
        println!("No data provided for shuffling");
    }
    Err(ShuffleError::PrivacyBudgetExceeded { epsilon, delta }) => {
        println!("Privacy budget exceeded: ε={}, δ={}", epsilon, delta);
    }
    Err(ShuffleError::SchemaMismatch { data_index, message }) => {
        println!("Schema mismatch at index {}: {}", data_index, message);
    }
    Err(e) => {
        println!("Other error: {}", e);
    }
}
```

## Query Types

The module supports various query types:

### Mean Query
Computes the mean of specified features.

### Variance Query
Computes the variance of specified features.

### Histogram Query
Creates a histogram of feature values.

### Range Query
Computes the range (max - min) of specified features.

## Performance Considerations

### Batch Processing
For large datasets, use batch processing:

```rust
let config = ShuffleConfig::builder()
    .batch_size(1000)  // Process in batches of 1000
    .build();
```

### Parallel Processing
Enable parallel processing for better performance:

```rust
let config = ShuffleConfig::builder()
    .parallel_processing(true)
    .build();
```

### Memory Usage
Monitor memory usage through statistics:

```rust
let result = shuffler.shuffle_data(data)?;
println!("Memory used: {} bytes", result.statistics.memory_usage_bytes);
```

## Testing

The module includes comprehensive tests:

```bash
# Run all tests
cargo test

# Run shuffle-specific tests
cargo test shuffle

# Run with verbose output
cargo test -- --nocapture
```

## Best Practices

1. **Privacy Budget Management**: Start with conservative privacy budgets and adjust based on utility requirements.

2. **Schema Validation**: Use schema validation when possible to catch data format issues early.

3. **Error Handling**: Always handle errors appropriately, especially privacy-related errors.

4. **Performance Monitoring**: Monitor processing times and memory usage for large datasets.

5. **Deterministic Testing**: Use deterministic shuffling for reproducible tests.

## Advanced Usage

### Custom Shuffle Algorithms

You can implement custom shuffle algorithms by extending the `ShuffleMechanism`:

```rust
impl ShuffleMechanism {
    fn custom_shuffle(&mut self, data: &mut [DataPoint]) {
        // Your custom shuffle implementation
    }
}
```

### Integration with Other Modules

The shuffle module integrates with other modules in the framework:

```rust
use doppio::shuffle::Shuffler;
use doppio::dp::DPMechanism;
use doppio::client::Client;

// Use shuffle in client operations
let mut client = Client::new();
let mut shuffler = Shuffler::new_default();

// Combine shuffle with differential privacy
let data = vec![/* your data */];
let shuffled = shuffler.shuffle_data(data)?;
let result = client.process_data(shuffled)?;
```

## Troubleshooting

### Common Issues

1. **Empty Input Error**: Ensure data is not empty before shuffling.

2. **Schema Mismatch**: Verify that data conforms to the specified schema.

3. **Privacy Budget Exceeded**: Reduce the number of operations or increase the privacy budget.

4. **Performance Issues**: Consider using batch processing or parallel processing.

### Debugging

Enable debug logging to troubleshoot issues:

```rust
use log::debug;

// Enable debug logging
env_logger::init();

// Debug information will be logged
let shuffler = Shuffler::new_default();
```

## Future Enhancements

Planned improvements include:

1. **More Shuffle Algorithms**: Additional cryptographic and statistical shuffle methods
2. **Advanced Privacy Mechanisms**: More sophisticated noise addition techniques
3. **Distributed Processing**: Support for distributed shuffle operations
4. **Machine Learning Integration**: Direct integration with ML frameworks
5. **Performance Optimizations**: GPU acceleration and memory optimizations 
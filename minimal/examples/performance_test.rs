use minimal_shuffle_dp::{MinimalProtocol, ProtocolConfig, UserData, FieldElement};
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Minimal Protocol Performance Test");
    println!("==================================");

    let test_sizes = vec![10, 100, 1000];
    
    for num_users in test_sizes {
        println!("\nTesting with {} users:", num_users);
        
        let config = ProtocolConfig {
            num_users,
            field_modulus: 0xFFFFFFFFFFFFFFC5,
            epsilon: 1.0,
            delta: 1e-5,
            noise_scale: 1.0,
        };

        let mut protocol = MinimalProtocol::new(config)?;
        
        let mut user_data = Vec::new();
        for i in 0..num_users {
            let data = vec![
                FieldElement::new(i as u64, protocol.field().modulus()),
                FieldElement::new((i * 2) as u64, protocol.field().modulus()),
            ];
            user_data.push(UserData::new(i, data, i as u64));
        }

        let start = Instant::now();
        let result = protocol.execute(user_data).await?;
        let duration = start.elapsed();

        println!("  - Total time: {:?}", duration);
        println!("  - Offline phase: {}ms", result.stats.offline_time_ms);
        println!("  - Online phase: {}ms", result.stats.online_time_ms);
        println!("  - Field operations: {}", result.stats.field_operations);
        println!("  - Result size: {} data points", result.result.len());
    }

    Ok(())
}

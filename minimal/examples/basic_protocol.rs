use minimal_shuffle_dp::{MinimalProtocol, ProtocolConfig, UserData, FieldElement};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Minimal 3-Server Multi-Party Shuffle DP Protocol");
    println!("=================================================");

    let config = ProtocolConfig {
        num_users: 10,
        field_modulus: 0xFFFFFFFFFFFFFFC5,
        epsilon: 1.0,
        delta: 1e-5,
        noise_scale: 1.0,
    };

    let mut protocol = MinimalProtocol::new(config)?;
    println!("✓ Protocol initialized");

    let mut user_data = Vec::new();
    for i in 0..10 {
        let data = vec![
            FieldElement::new(i as u64, protocol.field().modulus()),
            FieldElement::new((i * 2) as u64, protocol.field().modulus()),
        ];
        user_data.push(UserData::new(i, data, i as u64));
    }
    println!("✓ Created {} user data points", user_data.len());

    println!("\nExecuting protocol...");
    let result = protocol.execute(user_data).await?;
    
    println!("✓ Protocol completed successfully!");
    println!("  - Result contains {} data points", result.result.len());
    println!("  - Privacy guarantees: ε={}, δ={}", 
             result.privacy_guarantees.epsilon, 
             result.privacy_guarantees.delta);
    println!("  - Offline phase: {}ms", result.stats.offline_time_ms);
    println!("  - Online phase: {}ms", result.stats.online_time_ms);
    println!("  - Field operations: {}", result.stats.field_operations);

    Ok(())
}

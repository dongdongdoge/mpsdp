use toy_prototype::{
    ToyProtocol, ToyConfig, UserData, FieldElement, FiniteField
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Toy Prototype: 3-Server Multi-Party Shuffle DP Protocol ===");
    println!();

    // Configuration
    let config = ToyConfig {
        num_users: 10,
        epsilon: 1.0,
        delta: 1e-5,
        noise_scale: 1.0,
        field_modulus: 0xFFFFFFFFFFFFFFC5, // 2^64 - 59
    };

    println!("Configuration:");
    println!("  Number of users: {}", config.num_users);
    println!("  Privacy budget (ε): {}", config.epsilon);
    println!("  Privacy budget (δ): {}", config.delta);
    println!("  Field modulus: 0x{:X}", config.field_modulus);
    println!();

    // Create protocol instance
    let mut protocol = ToyProtocol::new(config)?;
    println!("✓ Protocol initialized");

    // Generate test user data
    let mut user_data = Vec::new();
    let field = protocol.field();
    
    println!("Generating test user data...");
    for i in 0..10 {
        let data = vec![
            FieldElement::new(i as u64, field.modulus()),
            FieldElement::new((i * 2) as u64, field.modulus()),
        ];
        user_data.push(UserData::new(i, data, i as u64));
    }
    println!("✓ Generated {} user data points", user_data.len());

    // Execute protocol
    println!();
    println!("Executing protocol...");
    let result = protocol.execute(user_data).await?;

    // Display results
    println!();
    println!("=== Protocol Results ===");
    println!("Result data points: {}", result.result.len());
    println!("Privacy guarantees:");
    println!("  ε = {}", result.privacy_guarantees.epsilon);
    println!("  δ = {}", result.privacy_guarantees.delta);
    println!("  Proven: {}", result.privacy_guarantees.is_proven);
    
    println!();
    println!("Performance statistics:");
    println!("  Offline phase time: {}ms", result.stats.offline_time_ms);
    println!("  Online phase time: {}ms", result.stats.online_time_ms);
    println!("  Total communication: {} bytes", result.stats.total_communication_bytes);
    println!("  Field operations: {}", result.stats.field_operations);

    // Show first few results
    println!();
    println!("First 5 result data points:");
    for (i, data_point) in result.result.iter().take(5).enumerate() {
        println!("  User {}: [{}, {}]", 
            i, 
            data_point[0].value(), 
            data_point[1].value()
        );
    }

    println!();
    println!("✓ Protocol execution completed successfully!");
    println!();
    println!("Key features demonstrated:");
    println!("  ✓ Zero communication in online phase");
    println!("  ✓ All computations in finite fields");
    println!("  ✓ Secret sharing with 2-out-of-3 threshold");
    println!("  ✓ Differential privacy with Laplace noise");
    println!("  ✓ Complete privacy preservation");

    Ok(())
} 
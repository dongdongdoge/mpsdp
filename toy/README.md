# Toy Prototype: 3-Server Multi-Party Shuffle DP Protocol

This directory contains a minimal but complete prototype of a 3-server multi-party shuffle differential privacy protocol, implementing the protocol described in `description`.

## Overview

This toy prototype demonstrates a novel approach to shuffle differential privacy that achieves:
- **Zero communication** in the online phase
- **Complete privacy** through secret sharing
- **Efficient computation** using finite field arithmetic
- **Provable differential privacy** guarantees

## Protocol Design

### Two-Phase Architecture

#### Phase 1: Offline Preparation
- **P₀ (Auxiliary Server)** generates correlated randomness:
  - Random permutation matrix `M`
  - User data masks `a_i` for each user
  - DP noise vector `r`
- All randomness is secret-shared among P₁ and P₂
- No user data is involved in this phase

#### Phase 2: Online Execution
- **Users** compute `[x_i]_2 = x_i - a_i` and submit to servers
- **P₁, P₂** perform completely local computations:
  - Apply permutation matrix to shuffled data
  - Add DP noise to randomized data
- **Zero communication** between servers during online phase
- **Result reconstruction** from server shares

### Key Innovations

1. **Silent Shuffle**: Permutation applied locally using pre-computed shares
2. **Silent Randomization**: DP noise added locally using pre-computed shares
3. **Finite Field MPC**: All computations in finite fields for correctness
4. **Threshold Security**: 2-out-of-3 secret sharing for fault tolerance

## Implementation

### Core Components

- **`finite_field.rs`**: Finite field arithmetic implementation
- **`secret_sharing.rs`**: Shamir's secret sharing scheme
- **`offline_phase.rs`**: P₀'s offline preparation logic
- **`online_phase.rs`**: P₁, P₂'s online computation logic
- **`server.rs`**: Server role implementations
- **`protocol.rs`**: Main protocol orchestration

### Finite Field Operations

All MPC computations are performed in finite fields:
```rust
// Field element creation
let a = FieldElement::new(5, modulus);
let b = FieldElement::new(3, modulus);

// Field operations
let sum = a.add(&b)?;      // Modular addition
let product = a.mul(&b)?;   // Modular multiplication
let inverse = a.inverse()?; // Modular inverse
```

### Secret Sharing

Shamir's secret sharing with threshold 2:
```rust
// Share a secret
let shares = shamir.share_secret(secret)?;

// Reconstruct from shares
let reconstructed = shamir.reconstruct_secret(&shares[0..2])?;
```

## Usage

### Running Tests
```bash
cd toy
cargo test
```

### Running Examples
```bash
cd toy
cargo run --example basic_protocol
```

### Configuration
```rust
let config = ToyConfig {
    field_modulus: 0xFFFFFFFFFFFFFFC5, // 2^64 - 59
    num_users: 1000,
    epsilon: 1.0,
    delta: 1e-5,
    noise_scale: 1.0,
};
```

## Protocol Correctness

### Privacy Guarantees
- **Differential Privacy**: ε-DP with Laplace noise
- **Information Theoretic Security**: Based on secret sharing
- **Zero Knowledge**: Servers learn nothing about individual data

### Efficiency Characteristics
- **Offline Communication**: O(n²) for n users
- **Online Communication**: O(1) - only user submissions
- **Computation**: O(n²) field operations
- **Storage**: O(n²) field elements per server

### Fault Tolerance
- **Threshold**: 2-out-of-3 servers required
- **Recovery**: Can tolerate 1 server failure
- **Consistency**: All honest servers produce same result

## Mathematical Foundation

### Finite Field Properties
- **Modulus**: Prime p = 2^64 - 59
- **Generator**: g = 5 (primitive root)
- **Operations**: Addition, multiplication, inversion

### Secret Sharing Properties
- **Threshold**: t = 2, n = 3
- **Reconstruction**: Lagrange interpolation
- **Security**: Information theoretic

### Differential Privacy
- **Mechanism**: Laplace noise
- **Sensitivity**: L1 norm
- **Scale**: Δf/ε where Δf is sensitivity

## Limitations

This is a **toy prototype** with the following limitations:
- Small field size (64-bit) for demonstration
- Simplified noise generation
- No optimization for large-scale deployment
- Basic error handling

## Future Work

- **Large-scale deployment**: Optimize for millions of users
- **Advanced cryptography**: Use more efficient MPC protocols
- **Performance optimization**: Parallel computation, batch processing
- **Security analysis**: Formal verification of privacy guarantees

## References

- Original protocol description: `description`
- Shamir's Secret Sharing: [Shamir79]
- Differential Privacy: [Dwork06]
- Finite Field Arithmetic: [Handbook of Applied Cryptography]

## License

Same as main project - MIT License 
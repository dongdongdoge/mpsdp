# Doppio

A secure and efficient framework for privacy-preserving data processing with shuffle differential privacy guarantees.

## Overview

Doppio is a framework that implements secure data processing with shuffle differential privacy guarantees. The framework provides privacy-preserving mechanisms that leverage the shuffle model to achieve better privacy-utility trade-offs compared to local differential privacy.

## Project Structure

- **`src/`**: Main framework implementation with modular components
- **`toy/`**: Minimal program prototype implementing a 3-server multi-party shuffle DP protocol
  - Contains a complete working prototype of the protocol described in `toy/description`
  - All MPC computations are performed in finite fields
  - Demonstrates offline preparation and online execution phases
  - Serves as a reference implementation for the described protocol

## Features

- **Privacy Mechanisms**:
  - Laplace mechanism
  - kRR mechanism

- **Query Types**:
  - Mean estimation
  - Variance estimation
  - Histogram computation
  - Range query
  - Multi-round query

## Toy Prototype

The `toy/` directory contains a minimal but complete prototype of a 3-server multi-party shuffle differential privacy protocol. This prototype demonstrates:

### Protocol Architecture
- **P₀ (Auxiliary Server)**: Generates correlated randomness in offline phase
- **P₁, P₂ (Computational Servers)**: Perform local computations in online phase

### Key Features
- **Offline Phase**: Pre-computation of shuffle correlation and DP noise
- **Online Phase**: Completely local computation with zero server communication
- **Finite Field Operations**: All MPC computations performed in finite fields
- **Secret Sharing**: Shamir's secret sharing with 2-out-of-3 threshold
- **Differential Privacy**: Laplace noise addition for privacy guarantees

### Implementation Details
- Written in Rust for performance and safety
- Modular design with separate components for each protocol phase
- Comprehensive test coverage
- Detailed documentation in `toy/description`

### Usage
```bash
cd toy
cargo test  # Run all tests
cargo run   # Run example (if implemented)
```

This toy prototype serves as a reference implementation and can be used to understand the protocol design and verify its correctness.
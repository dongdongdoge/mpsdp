# Doppio

A secure and efficient framework for privacy-preserving data processing with shuffle differential privacy guarantees.

## Overview

Doppio is a framework that implements secure data processing with shuffle differential privacy guarantees using a three-tier design: clients submit encoded reports to a dedicated shuffler service (middle server), which performs shuffling and applies DP noise under a privacy budget, then forwards privatized data to an analyzer/server that aggregates and serves results.

## Project Structure

- **`src/`**: Main framework implementation with modular components
- **`toy/`**: Minimal program prototype implementing a 2+1-server multi-party shuffle DP protocol (P0 assisting server + P1/P2 computational servers)
  - Contains a complete working prototype of the protocol described in `toy/description`
  - All MPC computations are performed in finite fields
  - Demonstrates offline preparation and online execution phases
  - Serves as a reference implementation for the described protocol

## Features

- **Privacy Mechanisms**:
  - (additive) Laplace mechanism
  - (non-additive) kRR mechanism

- **Query Types**:
  - Mean estimation
  - Variance estimation
  - Histogram estimation
  - Range query
  - Multi-round query

## Toy Prototype

The `toy/` directory contains a "minimal but complete" prototype of a 2+1-server multi-party shuffle differential privacy protocol. This prototype demonstrates the middle-tier shuffler concept via an assisting server (P0) and two computational servers (P1, P2):

### Protocol Architecture
- **P0 (Assisting Server)**: Generates correlated randomness in offline phase
- **P1, P2 (Computational Servers)**: Perform local computations in online phase

### Key Features
- **Offline Phase**: Pre-computation of shuffle correlation and DP correlation (together termed as "shuffle DP correlation")
- **Online Phase**: Local computation with zero inter-server communication
- **Finite Field Operations**: All MPC computations performed in finite fields
- **Secret Sharing**: Basic additive sharing (2-out-of-2) 
- **Differential Privacy**: Laplace noise addition/kRR noise perturbation for privacy guarantees

### Minimal Usage
```bash
cd toy
cargo test  # Run all tests
cargo run   # Run example (if implemented)
```

This toy prototype serves as a reference implementation and can be used to understand the protocol design and verify its correctness.

## TODO

- [ ] Add more tests
- [ ] Add documentation
- [ ] Add more examples
- [ ] Add detailed performance tests
- [ ] Add fault tolerance
- [ ] Add consistency checks
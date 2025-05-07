# Doppio Framework

A secure and efficient framework for privacy-preserving data processing with differential privacy guarantees.

## Overview

Doppio is a framework that implements secure data processing with differential privacy guarantees. It provides a robust solution for handling sensitive data while maintaining privacy and utility. The framework combines shuffle-based privacy with differential privacy mechanisms to provide strong privacy guarantees.

## Features

- **Privacy Mechanisms**:
  - Laplace mechanism
  - Gaussian mechanism
  - Exponential mechanism
  - Shuffle-based privacy
  - Composition of mechanisms

- **Query Types**:
  - Mean estimation
  - Histogram computation
  - Custom query support

- **Security Features**:
  - Differential privacy guarantees
  - Privacy budget management
  - Secure data shuffling
  - Input validation
  - Error handling

- **Performance Optimizations**:
  - Efficient data structures
  - Parallel processing
  - Optimized noise generation
  - Caching mechanisms

## Project Structure

```
doppio/
├── src/                    # Source code
│   ├── client/            # Client implementation
│   │   ├── mod.rs         # Client interface
│   │   ├── report.rs      # Report handling
│   │   └── query.rs       # Query processing
│   ├── server/            # Server implementation
│   │   ├── mod.rs         # Server interface
│   │   ├── histogram.rs   # Histogram computation
│   │   └── role.rs        # Server roles
│   ├── dp/                # Differential privacy
│   │   ├── mod.rs         # DP interface
│   │   └── mechanisms.rs  # DP mechanisms
│   ├── shuffle/           # Shuffle-based privacy
│   │   ├── mod.rs         # Shuffle interface
│   │   └── mechanism.rs   # Shuffle implementation
│   └── schema/            # Data schemas
├── tests/                 # Test suite
│   ├── unit/             # Unit tests
│   ├── integration/      # Integration tests
│   └── benchmarks/       # Performance benchmarks
├── examples/             # Example usage
├── docs/                # Documentation
└── scripts/             # Build and deployment scripts
```
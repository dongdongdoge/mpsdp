# Doppio Architecture

## System Overview

Doppio is designed as a modular framework for privacy-preserving data processing. The system follows a three-tier architecture where a dedicated shuffler service acts as an intermediary between data producers and the analysis server.

## Core Components

### 1. Client–Shuffler–Server Architecture

```
┌─────────────┐     ┌─────────────────┐     ┌──────────────────┐
│   Clients   │────▶│    Shuffler     │────▶│  Analyzer/Server │
│ (SDK/Apps)  │     │ (Middle Server) │     │  (Aggregation)   │
└─────────────┘     └─────────────────┘     └──────────────────┘
        │                    │                         │
        │  TLS + encoding    │  Batch + permute + DP   │  Aggregate + results
        ▼                    ▼                         ▼
```

### 2. Privacy Layers

The system implements multiple layers of privacy protection that compose:

1. **Anonymity via Shuffler (Middle tier)**
   - Removes linkability by batching and random permutation
   - Strips metadata and enforces per-batch limits
   - Optional thresholding before release

2. **Differential Privacy (Shuffler)**
   - Privacy budget management and accounting at the shuffler
   - DP mechanisms (e.g., Laplace, kRR) applied before release
   - Composition-aware report processing

3. **Optional Local Randomization (Client SDK)**
   - Lightweight local protections when required by policy
   - Report encoding and validation

### 3. Data Flow

```
┌─────────────┐   encode+TLS   ┌─────────────────┐   batch+permute+DP  ┌──────────────────┐
│   Clients   │ ─────────────▶ │    Shuffler     │ ───────────────────▶ │  Analyzer/Server │
└─────────────┘                 └─────────────────┘                      └──────────────────┘
       │                               │                                        │
       ▼                               ▼                                        ▼
  Reports (encoded)         Privatized reports (shuffled+noised)         Aggregated results
```

## Module Design

### 1. Client SDK

Responsibilities:
- Report encoding and schema validation
- Optional local randomization (policy-dependent)
- Submission over secure channels to the shuffler

Key components:
- `Report`: Client-side data container and validator
- `Schema`: Input contract

### 2. Shuffler Service (Middle Server)

Responsibilities:
- Batching, random permutation, and metadata stripping
- Rate limiting and per-user caps
- Privacy budget accounting and DP noise addition
- Optional thresholding and integrity checks

Key components:
- `Shuffler`: Main shuffling implementation
- `ShuffleConfig`: Configuration management
- `ShuffleMechanism`: Algorithm-specific shuffling

### 3. Analyzer Server

Responsibilities:
- Aggregation over already-privatized inputs
- Query orchestration and result serving
- Optional post-processing and caching

Key components:
- `QueryProcessor`: Executes queries over privatized inputs
- `ResultStore`: Optional caching and post-processing

### 3. Differential Privacy Module

The DP module provides:
- Multiple privacy mechanisms (Laplace, Gaussian, Exponential)
- Privacy budget tracking
- Noise generation and addition (used by the shuffler in deployment)

Key components:
- `DPMechanism`: Base mechanism implementation
- `PrivacyBudget`: Budget management and tracking
- `NoiseGenerator`: Noise generation utilities

### 4. Shuffle Module

The shuffle module is the core of the middle-tier shuffler service and implements:
- Secure data shuffling
- Batch processing and multi-round shuffles
- Service-facing configuration and instrumentation

Key components:
- `Shuffler`: Main shuffling implementation
- `ShuffleConfig`: Configuration management
- `ShuffleMechanism`: Mechanism-specific implementations

## Security Considerations

### 1. Privacy Guarantees

- Anonymity via shuffling in the middle tier
- Differential privacy applied at the shuffler prior to release
- Composition of privacy mechanisms and proper accounting
- Privacy budget management at the shuffler

### 2. Data Protection

- Secure data transmission
- Input validation
- Error handling
- Access control

### 3. Performance Optimization

- Efficient data structures
- Parallel processing
- Caching mechanisms
- Batch processing

## Implementation Details

### 1. Error Handling

The system uses a comprehensive error handling strategy:
- Custom error types for each module
- Error propagation and conversion
- Detailed error messages
- Error recovery mechanisms

### 2. Configuration Management

Configuration is handled through:
- Default configurations
- Custom configuration options
- Runtime configuration updates
- Configuration validation

### 3. Testing Strategy

The system implements:
- Unit tests for each module
- Integration tests for module interactions
- Performance benchmarks
- Security testing

## Future Improvements

1. **Scalability**
   - Distributed processing
   - Load balancing
   - Resource optimization

2. **Privacy Enhancements**
   - Advanced privacy mechanisms
   - Adaptive privacy budgets
   - Privacy-preserving machine learning

3. **Performance**
   - GPU acceleration
   - Memory optimization
   - Parallel processing improvements

4. **Usability**
   - Better documentation
   - More examples
   - Simplified API 
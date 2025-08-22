Minimal prototype for 2+1-server augmented multi-party shuffle DP protocol.

## Architecture Overview

```
┌─────────────┐     ┌─────────────────┐     ┌──────────────────┐
│   Clients   │────▶│    Shuffler     │────▶│  Analyzer/Server │
│ (SDK/Apps)  │     │ (Middle Server) │     │  (Aggregation)   │
└─────────────┘     └─────────────────┘     └──────────────────┘
```

## Data Flow Overview

```
┌─────────────┐   encode+TLS   ┌─────────────────┐   batch+permute+DP  ┌──────────────────┐
│   Clients   │ ─────────────▶ │    Shuffler     │ ───────────────────▶ │  Analyzer/Server │
└─────────────┘                 └─────────────────┘                      └──────────────────┘
```

## Features

- Laplace and kRR mechanisms
- Mean, variance, histogram estimation
- Range and multi-round queries

## Main Directory Structure

- **`src/`**: Full framework implementation
- **`minimal/`**: Minimal prototype of 2+1-server multi-party shuffle DP protocol (_you can follow this PoC to understand the core technical designs_)
- **`eval/`**: Evaluations and simulations for augmented multi-party shuffle DP
- **`tests/`**: Module tests

## Minimal Prototype

The `minimal/` directory contains a minimal prototype demonstrating the protocol:

- **P0**: Assisting server for randomness generation
- **P1, P2**: Computational servers for local computation
- **P3**: Data analysis server for result aggregation (ignored)

```bash
cd minimal
cargo test
cargo run
```
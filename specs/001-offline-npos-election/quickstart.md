# Quickstart Guide: Offline NPoS Election Tool

**Date**: 2025-01-27  
**Feature**: 001-offline-npos-election

## Overview

This guide provides a quick introduction to using the Offline NPoS Election Tool. You'll learn how to install the tool, run your first election simulation, and explore the main features.

## Installation

### Prerequisites

- Rust 1.70 or later
- Network access (for RPC data fetching, optional for file-based usage)

### Build from Source

```bash
git clone <repository-url>
cd offline-election-tool
cargo build --release
```

The binary will be available at `target/release/offline-election` (or `offline-election.exe` on Windows).

## Quick Start Examples

### 1. Run Election from RPC (On-Chain Data)

Fetch election data from a public Substrate node and run an election simulation:

```bash
offline-election run \
  --algorithm sequential-phragmen \
  --active-set-size 100 \
  --rpc-url https://rpc.polkadot.io \
  --block-number 12345678
```

**Output**: JSON results showing selected validators and stake distribution.

### 2. Run Election from JSON File

Load election data from a JSON file:

```bash
offline-election run \
  --algorithm parallel-phragmen \
  --active-set-size 50 \
  --input-file election_data.json
```

**Example JSON file structure** (`election_data.json`):

```json
{
  "candidates": [
    {
      "account_id": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
      "stake": "1000000000000"
    }
  ],
  "nominators": [
    {
      "account_id": "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
      "stake": "500000000000",
      "targets": ["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"]
    }
  ]
}
```

### 3. Run Election with Parameter Overrides

Override specific parameters like active set size or candidate stakes:

```bash
offline-election run \
  --algorithm sequential-phragmen \
  --active-set-size 75 \
  --rpc-url https://rpc.polkadot.io \
  --override-candidate-stake 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY=2000000000000
```

### 4. Get Detailed Diagnostics

Run an election and get detailed explanations:

```bash
offline-election run \
  --algorithm sequential-phragmen \
  --active-set-size 100 \
  --rpc-url https://rpc.polkadot.io \
  --diagnostics
```

**Output**: Includes explanations for why each validator was selected or not selected, stake distribution analysis, and algorithm-specific insights.

### 5. Compare Different Algorithms

Run the same data through different algorithms to compare results:

```bash
# Sequential Phragmen
offline-election run --algorithm sequential-phragmen --rpc-url https://rpc.polkadot.io --block-number 12345678 > results_seq.json

# Parallel Phragmen
offline-election run --algorithm parallel-phragmen --rpc-url https://rpc.polkadot.io --block-number 12345678 > results_par.json

# Multi-phase
offline-election run --algorithm multi-phase --rpc-url https://rpc.polkadot.io --block-number 12345678 > results_multi.json
```

Compare the JSON outputs to see differences in validator selection and stake distribution.

## CLI Commands

### Main Command: `run`

Execute an election simulation.

**Required Arguments**:
- `--algorithm <ALGORITHM>` - Election algorithm: `sequential-phragmen`, `parallel-phragmen`, or `multi-phase`
- `--active-set-size <SIZE>` - Number of validators to select (must be positive integer)

**Data Source Options** (choose one):
- `--rpc-url <URL>` - Fetch data from Substrate RPC endpoint
- `--input-file <PATH>` - Load data from JSON file
- `--synthetic` - Create synthetic data (requires additional flags)

**Optional Arguments**:
- `--block-number <NUMBER>` - Block number for RPC snapshot
- `--override-candidate-stake <ACCOUNT>=<STAKE>` - Override candidate stake (can be used multiple times)
- `--override-nominator-stake <ACCOUNT>=<STAKE>` - Override nominator stake (can be used multiple times)
- `--diagnostics` - Include detailed diagnostics in output
- `--output-file <PATH>` - Write results to file instead of stdout
- `--format <FORMAT>` - Output format: `json` (default) or `human-readable`

**Examples**:

```bash
# Basic RPC election
offline-election run --algorithm sequential-phragmen --active-set-size 100 --rpc-url https://rpc.polkadot.io

# With block number
offline-election run --algorithm parallel-phragmen --active-set-size 50 --rpc-url https://rpc.polkadot.io --block-number 12345678

# With diagnostics
offline-election run --algorithm sequential-phragmen --active-set-size 100 --rpc-url https://rpc.polkadot.io --diagnostics --output-file results.json

# With overrides
offline-election run \
  --algorithm sequential-phragmen \
  --active-set-size 100 \
  --rpc-url https://rpc.polkadot.io \
  --override-candidate-stake 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY=2000000000000 \
  --override-nominator-stake 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty=1000000000000
```

### Server Command: `server`

Start a REST API server for remote access.

```bash
offline-election server --port 3000
```

**Options**:
- `--port <PORT>` - Server port (default: 3000)
- `--host <HOST>` - Server host (default: 0.0.0.0)

**API Endpoints**:
- `POST /elections/run` - Run an election
- `GET /elections/{id}/results` - Get election results
- `GET /elections/{id}/diagnostics` - Get diagnostics

See `contracts/rest-api.yaml` for full API documentation.

## Programmatic API Usage

### Basic Example

```rust
use offline_election::{ElectionConfig, ElectionEngine, AlgorithmType};
use offline_election::input::RpcLoader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load data from RPC
    let loader = RpcLoader::new("https://rpc.polkadot.io");
    let election_data = loader.load_at_block(12345678)?;
    
    // Configure election
    let config = ElectionConfig::new()
        .algorithm(AlgorithmType::SequentialPhragmen)
        .active_set_size(100)
        .build()?;
    
    // Execute
    let engine = ElectionEngine::new();
    let result = engine.execute(&config, &election_data)?;
    
    // Access results
    println!("Selected {} validators", result.selected_validators().len());
    for validator in result.selected_validators() {
        println!("{}: {}", validator.account_id(), validator.total_backing_stake());
    }
    
    Ok(())
}
```

### Add to Your Cargo.toml

```toml
[dependencies]
offline-election = { path = "../offline-election-tool" }
```

See `contracts/programmatic-api.md` for complete API documentation.

## Output Format

### JSON Output

By default, results are output as JSON:

```json
{
  "selected_validators": [
    {
      "account_id": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
      "total_backing_stake": "1000000000000",
      "nominator_count": 5,
      "rank": 1
    }
  ],
  "stake_distribution": [
    {
      "nominator_id": "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
      "validator_id": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
      "amount": "500000000000",
      "proportion": 1.0
    }
  ],
  "total_stake": "500000000000",
  "algorithm_used": "sequential-phragmen"
}
```

### Human-Readable Output

Use `--format human-readable` for a more readable output:

```
Election Results
================
Algorithm: Sequential Phragmen
Active Set Size: 100
Total Stake: 5,000,000,000,000

Selected Validators (Top 10):
1. 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
   Stake: 1,000,000,000,000 | Nominators: 5
2. 5FLSigC9HGRKVhB9FkBSa33j4BWBsjRHJMPqvb6oFkDhRSY9
   Stake: 950,000,000,000 | Nominators: 3
...
```

## Common Use Cases

### Use Case 1: Predict Election Outcomes

Before an on-chain election occurs, simulate it to predict which validators will be selected:

```bash
offline-election run \
  --algorithm sequential-phragmen \
  --active-set-size 100 \
  --rpc-url https://rpc.polkadot.io \
  --block-number $(polkadot-cli query system number --rpc-url https://rpc.polkadot.io)
```

### Use Case 2: Test Parameter Changes

Test how changing the active set size affects results:

```bash
for size in 50 75 100 125 150; do
  echo "Testing active set size: $size"
  offline-election run \
    --algorithm sequential-phragmen \
    --active-set-size $size \
    --rpc-url https://rpc.polkadot.io \
    --output-file results_$size.json
done
```

### Use Case 3: Understand Validator Selection

Get detailed diagnostics to understand why validators were selected:

```bash
offline-election run \
  --algorithm sequential-phragmen \
  --active-set-size 100 \
  --rpc-url https://rpc.polkadot.io \
  --diagnostics \
  --output-file diagnostics.json
```

Then examine `diagnostics.json` to see explanations for each validator.

### Use Case 4: Research Different Algorithms

Compare how different algorithms select validators:

```bash
# Create a script to compare algorithms
cat > compare_algorithms.sh << 'EOF'
#!/bin/bash
for algo in sequential-phragmen parallel-phragmen multi-phase; do
  echo "Running $algo..."
  offline-election run \
    --algorithm $algo \
    --active-set-size 100 \
    --rpc-url https://rpc.polkadot.io \
    --output-file results_$algo.json
done
EOF
chmod +x compare_algorithms.sh
./compare_algorithms.sh
```

## Troubleshooting

### RPC Connection Errors

If you get RPC connection errors:

1. **Check URL**: Ensure the RPC URL is correct and accessible
2. **Check Network**: Verify you have network connectivity
3. **Try Different Node**: Some public nodes may be rate-limited

```bash
# Test RPC connection
curl -X POST https://rpc.polkadot.io \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"system_health","params":[],"id":1}'
```

### Validation Errors

If you get validation errors:

1. **Check Account IDs**: Ensure all account IDs are valid SS58 format
2. **Check Uniqueness**: Ensure no duplicate account IDs
3. **Check References**: Ensure all voting edges reference existing candidates

### Algorithm Convergence Errors

If an algorithm fails to converge:

1. **Check Data**: Ensure election data is valid
2. **Try Different Algorithm**: Some algorithms may work better with certain data
3. **Check Diagnostics**: Use `--diagnostics` to understand the failure

## Next Steps

- Read the full [Data Model documentation](./data-model.md) to understand the data structures
- Explore the [REST API documentation](./contracts/rest-api.yaml) for server usage
- Check the [Programmatic API documentation](./contracts/programmatic-api.md) for library usage
- Review the [Research documentation](./research.md) for technical details

## Getting Help

- Check error messages - they provide detailed information about what went wrong
- Use `--diagnostics` flag to get detailed explanations
- Review the feature specification in `spec.md` for requirements and constraints



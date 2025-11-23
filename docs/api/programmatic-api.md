# Programmatic API Documentation

This guide explains how to use the Offline NPoS Election Tool as a Rust library in your own applications.

## Overview

The `offline_election` crate provides a programmatic API for running election simulations in Rust code. This is useful for:

- Building custom tools and applications
- Integrating election simulation into larger systems
- Automated testing and analysis
- Custom workflows and pipelines

## Basic Usage

### Add Dependency

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
offline_election = { path = "../offline-election-tool" }
tokio = { version = "1", features = ["full"] }
```

### Basic Example

```rust
use offline_election::*;

#[tokio::main]
async fn main() -> Result<(), ElectionError> {
    // Load election data from RPC
    let data = ElectionData::from_rpc(
        "https://rpc.polkadot.io",
        Some(12345678)
    ).await?;

    // Create election configuration
    let config = ElectionConfiguration::new()
        .algorithm(AlgorithmType::SequentialPhragmen)
        .active_set_size(100)
        .build()?;

    // Execute election
    let engine = ElectionEngine::new();
    let result = engine.execute_with_diagnostics(&config, &data, true)?;

    // Access results
    println!("Selected {} validators", result.validator_count());
    if let Some(diagnostics) = result.diagnostics() {
        for explanation in diagnostics.validator_explanations() {
            println!("{}: {}", explanation.account_id, explanation.reason);
        }
    }

    Ok(())
}
```

## Data Sources

### From RPC Endpoint

```rust
use offline_election::*;

let data = ElectionData::from_rpc(
    "https://rpc.polkadot.io",
    Some(12345678) // Optional block number
).await?;
```

### From JSON File

```rust
use offline_election::*;
use std::fs;

let json_content = fs::read_to_string("election_data.json")?;
let data = ElectionData::from_json(&json_content)?;
```

### From Synthetic Data

```rust
use offline_election::*;

let mut builder = SyntheticDataBuilder::new();
builder
    .add_candidate("candidate1".to_string(), 1000000)?
    .add_candidate("candidate2".to_string(), 2000000)?
    .add_nominator("nominator1".to_string(), 500000, vec!["candidate1".to_string()])?;

let data = builder.build()?;
```

## Election Configuration

### Creating Configuration

```rust
use offline_election::*;

let config = ElectionConfiguration::new()
    .algorithm(AlgorithmType::SequentialPhragmen)
    .active_set_size(100)
    .build()?;
```

### Available Algorithms

```rust
use offline_election::AlgorithmType;

// Sequential Phragmen
AlgorithmType::SequentialPhragmen

// Parallel Phragmen
AlgorithmType::ParallelPhragmen

// Multi-phase
AlgorithmType::MultiPhase
```

### Configuration Builder

```rust
let config = ElectionConfiguration::new()
    .algorithm(AlgorithmType::SequentialPhragmen)
    .active_set_size(100)
    // Add more configuration options here
    .build()?;
```

## Running Elections

### Basic Execution

```rust
use offline_election::*;

let engine = ElectionEngine::new();
let result = engine.execute(&config, &data)?;
```

### With Diagnostics

```rust
let result = engine.execute_with_diagnostics(&config, &data, true)?;
```

The third parameter (`true`) enables diagnostics, which provide explanations for why validators were selected or not selected.

## Accessing Results

### Basic Result Information

```rust
// Number of selected validators
let count = result.validator_count();

// Total stake
let total_stake = result.total_stake();

// Algorithm used
let algorithm = result.algorithm_used();
```

### Selected Validators

```rust
for validator in result.selected_validators() {
    println!("Validator: {}", validator.account_id);
    println!("Total backing stake: {}", validator.total_backing_stake);
    println!("Nominator count: {}", validator.nominator_count);
    println!("Rank: {}", validator.rank);
}
```

### Stake Distribution

```rust
for allocation in result.stake_distribution() {
    println!("Nominator: {}", allocation.nominator_id);
    println!("Validator: {}", allocation.validator_id);
    println!("Amount: {}", allocation.amount);
    println!("Proportion: {}", allocation.proportion);
}
```

### Diagnostics

```rust
if let Some(diagnostics) = result.diagnostics() {
    for explanation in diagnostics.validator_explanations() {
        println!("{}: {}", explanation.account_id, explanation.reason);
    }
}
```

## Error Handling

### Error Types

```rust
use offline_election::ElectionError;

match result {
    Ok(election_result) => {
        // Process results
    }
    Err(ElectionError::ValidationError { message, field }) => {
        eprintln!("Validation error: {} (field: {:?})", message, field);
    }
    Err(ElectionError::RpcError { message, url }) => {
        eprintln!("RPC error: {} (URL: {:?})", message, url);
    }
    Err(e) => {
        eprintln!("Other error: {:?}", e);
    }
}
```

## Complete Examples

### Example 1: Compare Algorithms

```rust
use offline_election::*;

#[tokio::main]
async fn main() -> Result<(), ElectionError> {
    // Load data once
    let data = ElectionData::from_rpc("https://rpc.polkadot.io", None).await?;
    let engine = ElectionEngine::new();

    // Run with Sequential Phragmen
    let config_seq = ElectionConfiguration::new()
        .algorithm(AlgorithmType::SequentialPhragmen)
        .active_set_size(100)
        .build()?;
    let result_seq = engine.execute(&config_seq, &data)?;

    // Run with Parallel Phragmen
    let config_par = ElectionConfiguration::new()
        .algorithm(AlgorithmType::ParallelPhragmen)
        .active_set_size(100)
        .build()?;
    let result_par = engine.execute(&config_par, &data)?;

    // Compare results
    println!("Sequential Phragmen selected {} validators", result_seq.validator_count());
    println!("Parallel Phragmen selected {} validators", result_par.validator_count());

    Ok(())
}
```

### Example 2: What-If Analysis

```rust
use offline_election::*;

#[tokio::main]
async fn main() -> Result<(), ElectionError> {
    // Load base data
    let mut data = ElectionData::from_rpc("https://rpc.polkadot.io", None).await?;

    // Override candidate stake
    data.override_candidate_stake(
        "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
        2000000000000
    )?;

    // Run election
    let config = ElectionConfiguration::new()
        .algorithm(AlgorithmType::SequentialPhragmen)
        .active_set_size(100)
        .build()?;

    let engine = ElectionEngine::new();
    let result = engine.execute_with_diagnostics(&config, &data, true)?;

    println!("What-if scenario results:");
    println!("Selected {} validators", result.validator_count());

    Ok(())
}
```

### Example 3: Synthetic Data Testing

```rust
use offline_election::*;

fn main() -> Result<(), ElectionError> {
    // Build synthetic data
    let mut builder = SyntheticDataBuilder::new();
    builder
        .add_candidate("candidate1".to_string(), 1000000)?
        .add_candidate("candidate2".to_string(), 2000000)?
        .add_candidate("candidate3".to_string(), 1500000)?
        .add_nominator("nominator1".to_string(), 500000, vec![
            "candidate1".to_string(),
            "candidate2".to_string()
        ])?
        .add_nominator("nominator2".to_string(), 300000, vec![
            "candidate3".to_string()
        ])?;

    let data = builder.build()?;

    // Run election
    let config = ElectionConfiguration::new()
        .algorithm(AlgorithmType::SequentialPhragmen)
        .active_set_size(2)
        .build()?;

    let engine = ElectionEngine::new();
    let result = engine.execute(&config, &data)?;

    println!("Selected validators:");
    for validator in result.selected_validators() {
        println!("  {} (stake: {})", validator.account_id, validator.total_backing_stake);
    }

    Ok(())
}
```

## API Reference

### Core Types

- `ElectionData`: Election input data (candidates, nominators, stakes)
- `ElectionConfiguration`: Election configuration (algorithm, active set size)
- `ElectionResult`: Election results (selected validators, stake distribution)
- `ElectionEngine`: Main engine for executing elections
- `ElectionError`: Error type for election operations

### Algorithm Types

- `AlgorithmType::SequentialPhragmen`: Sequential Phragmen algorithm
- `AlgorithmType::ParallelPhragmen`: Parallel Phragmen algorithm
- `AlgorithmType::MultiPhase`: Multi-phase algorithm

### Data Loading

- `ElectionData::from_rpc(url, block_number)`: Load from RPC endpoint
- `ElectionData::from_json(json)`: Load from JSON string
- `SyntheticDataBuilder`: Build synthetic election data

### Execution

- `ElectionEngine::execute(config, data)`: Execute election
- `ElectionEngine::execute_with_diagnostics(config, data, enable_diagnostics)`: Execute with diagnostics

## Best Practices

1. **Error Handling**: Always handle `ElectionError` appropriately
2. **Async Runtime**: Use `tokio::main` for async operations (RPC data loading)
3. **Data Reuse**: Load data once and reuse for multiple elections
4. **Configuration**: Use builder pattern for flexible configuration
5. **Diagnostics**: Enable diagnostics when you need to understand selection reasons

## See Also

- [REST API Documentation](rest-api.md) - HTTP API for remote access
- [Algorithm Guide](../guides/algorithms.md) - How to add custom algorithms
- [Testing Overview](../testing/overview.md) - Testing with the library


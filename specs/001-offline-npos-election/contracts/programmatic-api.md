# Programmatic API: Offline NPoS Election Tool

**Date**: 2025-01-27  
**Feature**: 001-offline-npos-election

## Overview

This document defines the Rust programmatic API for the Offline NPoS Election Tool. The API allows developers to configure and execute elections programmatically without using the CLI (FR-012).

## Library Structure

The library is organized into the following modules:

- `offline_election::models` - Data models (ElectionData, ElectionConfig, etc.)
- `offline_election::algorithms` - Election algorithm implementations
- `offline_election::input` - Input data loading (RPC, JSON, synthetic)
- `offline_election::diagnostics` - Diagnostic generation
- `offline_election::error` - Error types

## Core API

### Election Execution

```rust
use offline_election::{ElectionConfig, ElectionData, ElectionEngine, AlgorithmType};

// Create election configuration
let config = ElectionConfig::new()
    .algorithm(AlgorithmType::SequentialPhragmen)
    .active_set_size(100)
    .build()?;

// Load election data (from RPC, JSON, or synthetic)
let election_data = ElectionData::from_rpc("https://rpc.polkadot.io", Some(12345678))?;

// Create election engine
let engine = ElectionEngine::new();

// Execute election
let result = engine.execute(&config, &election_data)?;

// Access results
println!("Selected {} validators", result.selected_validators().len());
for validator in result.selected_validators() {
    println!("Validator: {}, Stake: {}", validator.account_id(), validator.total_backing_stake());
}
```

### Data Loading

#### From RPC

```rust
use offline_election::input::RpcLoader;

let loader = RpcLoader::new("https://rpc.polkadot.io");
let election_data = loader.load_at_block(12345678)?;
```

#### From JSON File

```rust
use offline_election::input::JsonLoader;
use std::path::PathBuf;

let loader = JsonLoader::new();
let election_data = loader.load_from_file(PathBuf::from("election_data.json"))?;
```

#### Synthetic Data

```rust
use offline_election::{ElectionData, ValidatorCandidate, Nominator};

let mut election_data = ElectionData::new();

// Add candidates
election_data.add_candidate(ValidatorCandidate::new(
    "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".parse()?,
    1_000_000_000_000u128,
))?;

// Add nominators
let mut nominator = Nominator::new(
    "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty".parse()?,
    500_000_000_000u128,
);
nominator.add_target("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".parse()?);
election_data.add_nominator(nominator)?;
```

### Algorithm Selection

```rust
use offline_election::{AlgorithmType, ElectionConfig};

// Sequential Phragmen
let config = ElectionConfig::new()
    .algorithm(AlgorithmType::SequentialPhragmen)
    .active_set_size(100)
    .build()?;

// Parallel Phragmen
let config = ElectionConfig::new()
    .algorithm(AlgorithmType::ParallelPhragmen)
    .active_set_size(100)
    .build()?;

// Multi-phase
let config = ElectionConfig::new()
    .algorithm(AlgorithmType::MultiPhase)
    .active_set_size(100)
    .build()?;
```

### Parameter Overrides

```rust
use offline_election::{ElectionConfig, ElectionOverrides};

let mut overrides = ElectionOverrides::new();
overrides.set_candidate_stake(
    "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".parse()?,
    2_000_000_000_000u128,
)?;
overrides.set_nominator_stake(
    "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty".parse()?,
    1_000_000_000_000u128,
)?;

let config = ElectionConfig::new()
    .algorithm(AlgorithmType::SequentialPhragmen)
    .active_set_size(100)
    .overrides(overrides)
    .build()?;
```

### Diagnostics

```rust
use offline_election::diagnostics::DiagnosticsGenerator;

let generator = DiagnosticsGenerator::new();
let diagnostics = generator.generate(&result, &election_data)?;

// Access validator explanations
for explanation in diagnostics.validator_explanations() {
    println!("Validator {}: {}", explanation.account_id(), explanation.reason());
    if explanation.selected() {
        println!("  Key factors: {:?}", explanation.key_factors());
    }
}

// Access stake analysis
let analysis = diagnostics.stake_analysis();
println!("Total stake: {}", analysis.total_stake());
println!("Average stake per validator: {}", analysis.average_stake_per_validator());
```

## Type Definitions

### ElectionConfig

```rust
pub struct ElectionConfig {
    algorithm: AlgorithmType,
    active_set_size: u32,
    overrides: Option<ElectionOverrides>,
    block_number: Option<u64>,
}

impl ElectionConfig {
    pub fn new() -> Self;
    pub fn algorithm(mut self, algorithm: AlgorithmType) -> Self;
    pub fn active_set_size(mut self, size: u32) -> Self;
    pub fn overrides(mut self, overrides: ElectionOverrides) -> Self;
    pub fn block_number(mut self, block: u64) -> Self;
    pub fn build(self) -> Result<Self, ElectionError>;
}
```

### ElectionData

```rust
pub struct ElectionData {
    candidates: Vec<ValidatorCandidate>,
    nominators: Vec<Nominator>,
    metadata: Option<ElectionMetadata>,
}

impl ElectionData {
    pub fn new() -> Self;
    pub fn add_candidate(&mut self, candidate: ValidatorCandidate) -> Result<(), ElectionError>;
    pub fn add_nominator(&mut self, nominator: Nominator) -> Result<(), ElectionError>;
    pub fn candidates(&self) -> &[ValidatorCandidate];
    pub fn nominators(&self) -> &[Nominator];
    pub fn validate(&self) -> Result<(), ElectionError>;
}
```

### ElectionEngine

```rust
pub struct ElectionEngine;

impl ElectionEngine {
    pub fn new() -> Self;
    pub fn execute(
        &self,
        config: &ElectionConfig,
        data: &ElectionData,
    ) -> Result<ElectionResult, ElectionError>;
}
```

### ElectionResult

```rust
pub struct ElectionResult {
    selected_validators: Vec<SelectedValidator>,
    stake_distribution: Vec<StakeAllocation>,
    total_stake: u128,
    algorithm_used: AlgorithmType,
    execution_metadata: ExecutionMetadata,
}

impl ElectionResult {
    pub fn selected_validators(&self) -> &[SelectedValidator];
    pub fn stake_distribution(&self) -> &[StakeAllocation];
    pub fn total_stake(&self) -> u128;
    pub fn algorithm_used(&self) -> AlgorithmType;
    pub fn to_json(&self) -> Result<String, ElectionError>;
}
```

### AlgorithmType

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlgorithmType {
    SequentialPhragmen,
    ParallelPhragmen,
    MultiPhase,
}
```

### Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum ElectionError {
    #[error("Validation error: {message}")]
    ValidationError {
        message: String,
        field: Option<String>,
    },
    
    #[error("RPC error: {message} (URL: {url})")]
    RpcError {
        message: String,
        url: String,
    },
    
    #[error("Algorithm error: {message} (algorithm: {algorithm:?})")]
    AlgorithmError {
        message: String,
        algorithm: AlgorithmType,
    },
    
    #[error("Insufficient candidates: requested {requested}, available {available}")]
    InsufficientCandidates {
        requested: u32,
        available: u32,
    },
    
    #[error("Invalid data: {message}")]
    InvalidData { message: String },
    
    #[error("File error: {message} (path: {path:?})")]
    FileError {
        message: String,
        path: PathBuf,
    },
}
```

## Usage Examples

### Complete Example: RPC Election

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
    
    // Output results
    println!("{}", result.to_json()?);
    
    Ok(())
}
```

### Example: Synthetic Data with Overrides

```rust
use offline_election::{ElectionConfig, ElectionData, ElectionEngine, ElectionOverrides, AlgorithmType, ValidatorCandidate, Nominator};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create synthetic data
    let mut data = ElectionData::new();
    data.add_candidate(ValidatorCandidate::new("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".parse()?, 1_000_000_000_000u128))?;
    
    let mut nominator = Nominator::new("5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty".parse()?, 500_000_000_000u128);
    nominator.add_target("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".parse()?);
    data.add_nominator(nominator)?;
    
    // Create overrides
    let mut overrides = ElectionOverrides::new();
    overrides.set_candidate_stake("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".parse()?, 2_000_000_000_000u128)?;
    
    // Configure and execute
    let config = ElectionConfig::new()
        .algorithm(AlgorithmType::ParallelPhragmen)
        .active_set_size(1)
        .overrides(overrides)
        .build()?;
    
    let engine = ElectionEngine::new();
    let result = engine.execute(&config, &data)?;
    
    println!("Selected validators: {}", result.selected_validators().len());
    
    Ok(())
}
```

### Example: Compare Algorithms

```rust
use offline_election::{ElectionConfig, ElectionEngine, AlgorithmType};

fn compare_algorithms(data: &ElectionData) -> Result<(), ElectionError> {
    let engine = ElectionEngine::new();
    
    for algorithm in [AlgorithmType::SequentialPhragmen, AlgorithmType::ParallelPhragmen, AlgorithmType::MultiPhase] {
        let config = ElectionConfig::new()
            .algorithm(algorithm)
            .active_set_size(100)
            .build()?;
        
        let result = engine.execute(&config, data)?;
        println!("{:?}: Selected {} validators", algorithm, result.selected_validators().len());
    }
    
    Ok(())
}
```

## Error Handling

All API functions return `Result<T, ElectionError>`. Errors should be handled appropriately:

```rust
match engine.execute(&config, &data) {
    Ok(result) => {
        // Process results
    }
    Err(ElectionError::ValidationError { message, field }) => {
        eprintln!("Validation failed: {} (field: {:?})", message, field);
    }
    Err(ElectionError::RpcError { message, url }) => {
        eprintln!("RPC error: {} (URL: {})", message, url);
    }
    Err(ElectionError::AlgorithmError { message, algorithm }) => {
        eprintln!("Algorithm failed: {} ({:?})", message, algorithm);
    }
    Err(e) => {
        eprintln!("Error: {}", e);
    }
}
```

## Thread Safety

- `ElectionEngine` is `Send + Sync` and can be used across threads
- `ElectionData` and `ElectionConfig` are `Send + Sync` but not `Sync` (mutable operations require `&mut`)
- `ElectionResult` is `Send + Sync` and can be shared across threads

## Performance Considerations

- Election execution is CPU-bound and benefits from parallel processing where algorithms support it
- Large datasets (1000+ candidates, 10000+ nominators) may take several seconds to process
- Results are computed lazily - diagnostics are only generated when requested
- JSON serialization of results can be expensive for large elections - consider streaming for very large outputs



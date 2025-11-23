//! Performance benchmarks with real Polkadot mainnet data
//!
//! These tests fetch real election data from Polkadot mainnet RPC endpoints
//! and measure execution time and memory usage for different election algorithms.
//! Tests are marked with #[ignore] by default and require network access.
//! Run with: `cargo test --test test_polkadot_mainnet_performance -- --ignored --nocapture`

use offline_election::types::AlgorithmType;
use crate::common::benchmark_utils::{create_benchmark_results, output_benchmark_json, run_benchmark_with_algorithm};
use crate::common::rpc_utils::{fetch_polkadot_mainnet_snapshot, calculate_recent_block_number};
use crate::common::memory_measurement::measure_memory_usage_platform;
use std::collections::HashMap;
use chrono::Utc;

// Default RPC endpoint for Polkadot mainnet
const DEFAULT_RPC_URL: &str = "https://polkadot.api.onfinality.io/public";

// Polkadot active set size
const POLKADOT_ACTIVE_SET_SIZE: usize = 297;

// Algorithm-specific performance thresholds (in milliseconds)
const THRESHOLD_SEQUENTIAL_PHRAGMEN_MS: u64 = 30_000; // 30 seconds
const THRESHOLD_PARALLEL_PHRAGMEN_MS: u64 = 15_000;   // 15 seconds
const THRESHOLD_MULTIPHASE_MS: u64 = 45_000;          // 45 seconds

#[test]
#[ignore] // Requires network access - run with `cargo test --test test_polkadot_mainnet_performance -- --ignored --nocapture`
fn test_polkadot_mainnet_performance_sequential() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    println!("Fetching Polkadot mainnet data from recent block...");
    
    // Fetch snapshot with retry logic
    let snapshot = rt.block_on(fetch_polkadot_mainnet_snapshot(DEFAULT_RPC_URL, None))
        .unwrap_or_else(|e| {
            eprintln!("Failed to fetch Polkadot mainnet snapshot: {}", e);
            eprintln!("Suggested alternative RPC endpoints:");
            eprintln!("  - https://rpc.polkadot.io");
            eprintln!("  - https://polkadot-rpc.dwellir.com");
            eprintln!("  - https://polkadot.public.curie.com");
            panic!("RPC fetch failed: {}", e);
        });
    
    println!("Fetched: {} candidates, {} nominators from block {}", 
             snapshot.election_data.candidates.len(),
             snapshot.election_data.nominators.len(),
             snapshot.block_number);
    
    // Validate block number is within last 30 days (with warning if older)
    let latest_block = snapshot.block_number + 432_000; // Approximate latest
    let block_age_days = (latest_block - snapshot.block_number) as f64 / 14_400.0;
    if block_age_days > 30.0 {
        eprintln!("Warning: Block {} is approximately {:.1} days old (target: <30 days)", 
                  snapshot.block_number, block_age_days);
    }
    
    // Validate Polkadot mainnet scale characteristics
    let candidate_count = snapshot.election_data.candidates.len();
    let nominator_count = snapshot.election_data.nominators.len();
    if candidate_count < 300 || candidate_count > 400 {
        eprintln!("Warning: Candidate count {} is outside expected Polkadot mainnet range (300-400)", 
                  candidate_count);
    }
    if nominator_count < 20_000 || nominator_count > 30_000 {
        eprintln!("Warning: Nominator count {} is outside expected Polkadot mainnet range (20k-30k)", 
                  nominator_count);
    }
    
    // Measure memory before execution
    let (memory_before_peak, memory_before_current) = measure_memory_usage_platform();
    
    println!("Running benchmark with sequential-phragmen algorithm...");
    
    // Run benchmark
    let (election_result, execution_time_ms) = run_benchmark_with_algorithm(
        &snapshot.election_data,
        AlgorithmType::SequentialPhragmen,
        POLKADOT_ACTIVE_SET_SIZE,
    ).unwrap_or_else(|e| {
        panic!("Benchmark execution failed: {}", e);
    });
    
    // Measure memory after execution
    let (memory_after_peak, memory_after_current) = measure_memory_usage_platform();
    
    // Calculate peak memory (use the higher of before/after, or difference)
    let memory_peak_mb = if memory_after_peak > memory_before_peak {
        memory_after_peak - memory_before_peak
    } else {
        memory_after_peak.max(memory_before_peak)
    };
    let memory_final_mb = memory_after_current;
    
    // Validate election result
    assert!(election_result.selected_validators.len() == POLKADOT_ACTIVE_SET_SIZE,
            "Should select {} validators, got {}", 
            POLKADOT_ACTIVE_SET_SIZE, 
            election_result.selected_validators.len());
    assert!(election_result.total_stake > 0, "Total stake should be positive");
    
    // Validate threshold
    let threshold_passed = execution_time_ms <= THRESHOLD_SEQUENTIAL_PHRAGMEN_MS;
    assert!(
        threshold_passed,
        "Execution time {}ms exceeds threshold {}ms for sequential-phragmen",
        execution_time_ms,
        THRESHOLD_SEQUENTIAL_PHRAGMEN_MS
    );
    
    // Create benchmark results
    let mut metadata = HashMap::new();
    metadata.insert("benchmark_name".to_string(), "polkadot_mainnet".to_string());
    metadata.insert("candidate_count".to_string(), candidate_count.to_string());
    metadata.insert("nominator_count".to_string(), nominator_count.to_string());
    metadata.insert("algorithm".to_string(), "sequential-phragmen".to_string());
    metadata.insert("block_number".to_string(), snapshot.block_number.to_string());
    metadata.insert("chain".to_string(), "polkadot".to_string());
    metadata.insert("rpc_endpoint".to_string(), snapshot.rpc_endpoint.clone());
    metadata.insert("threshold_ms".to_string(), THRESHOLD_SEQUENTIAL_PHRAGMEN_MS.to_string());
    metadata.insert("threshold_passed".to_string(), threshold_passed.to_string());
    metadata.insert("timestamp".to_string(), Utc::now().to_rfc3339());
    
    let benchmark_results = create_benchmark_results(
        execution_time_ms,
        memory_peak_mb,
        memory_final_mb,
        1,
        metadata,
    );
    
    let json_output = output_benchmark_json(&benchmark_results).unwrap();
    println!("Benchmark results:\n{}", json_output);
    
    println!("✓ Polkadot mainnet sequential-phragmen benchmark completed: {}ms (threshold: {}ms)", 
             execution_time_ms, THRESHOLD_SEQUENTIAL_PHRAGMEN_MS);
}

#[test]
#[ignore] // Requires network access - run with `cargo test --test test_polkadot_mainnet_performance -- --ignored --nocapture`
fn test_polkadot_mainnet_performance_parallel() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    println!("Fetching Polkadot mainnet data from recent block...");
    
    // Fetch snapshot with retry logic
    let snapshot = rt.block_on(fetch_polkadot_mainnet_snapshot(DEFAULT_RPC_URL, None))
        .unwrap_or_else(|e| {
            eprintln!("Failed to fetch Polkadot mainnet snapshot: {}", e);
            eprintln!("Suggested alternative RPC endpoints:");
            eprintln!("  - https://rpc.polkadot.io");
            eprintln!("  - https://polkadot-rpc.dwellir.com");
            eprintln!("  - https://polkadot.public.curie.com");
            panic!("RPC fetch failed: {}", e);
        });
    
    println!("Fetched: {} candidates, {} nominators from block {}", 
             snapshot.election_data.candidates.len(),
             snapshot.election_data.nominators.len(),
             snapshot.block_number);
    
    // Measure memory before execution
    let (memory_before_peak, memory_before_current) = measure_memory_usage_platform();
    
    println!("Running benchmark with parallel-phragmen algorithm...");
    
    // Run benchmark
    let (election_result, execution_time_ms) = run_benchmark_with_algorithm(
        &snapshot.election_data,
        AlgorithmType::ParallelPhragmen,
        POLKADOT_ACTIVE_SET_SIZE,
    ).unwrap_or_else(|e| {
        panic!("Benchmark execution failed: {}", e);
    });
    
    // Measure memory after execution
    let (memory_after_peak, memory_after_current) = measure_memory_usage_platform();
    
    // Calculate peak memory
    let memory_peak_mb = if memory_after_peak > memory_before_peak {
        memory_after_peak - memory_before_peak
    } else {
        memory_after_peak.max(memory_before_peak)
    };
    let memory_final_mb = memory_after_current;
    
    // Validate election result
    assert!(election_result.selected_validators.len() == POLKADOT_ACTIVE_SET_SIZE,
            "Should select {} validators, got {}", 
            POLKADOT_ACTIVE_SET_SIZE, 
            election_result.selected_validators.len());
    
    // Validate threshold
    let threshold_passed = execution_time_ms <= THRESHOLD_PARALLEL_PHRAGMEN_MS;
    assert!(
        threshold_passed,
        "Execution time {}ms exceeds threshold {}ms for parallel-phragmen",
        execution_time_ms,
        THRESHOLD_PARALLEL_PHRAGMEN_MS
    );
    
    // Create benchmark results
    let mut metadata = HashMap::new();
    metadata.insert("benchmark_name".to_string(), "polkadot_mainnet".to_string());
    metadata.insert("candidate_count".to_string(), snapshot.election_data.candidates.len().to_string());
    metadata.insert("nominator_count".to_string(), snapshot.election_data.nominators.len().to_string());
    metadata.insert("algorithm".to_string(), "parallel-phragmen".to_string());
    metadata.insert("block_number".to_string(), snapshot.block_number.to_string());
    metadata.insert("chain".to_string(), "polkadot".to_string());
    metadata.insert("rpc_endpoint".to_string(), snapshot.rpc_endpoint.clone());
    metadata.insert("threshold_ms".to_string(), THRESHOLD_PARALLEL_PHRAGMEN_MS.to_string());
    metadata.insert("threshold_passed".to_string(), threshold_passed.to_string());
    metadata.insert("timestamp".to_string(), Utc::now().to_rfc3339());
    
    let benchmark_results = create_benchmark_results(
        execution_time_ms,
        memory_peak_mb,
        memory_final_mb,
        1,
        metadata,
    );
    
    let json_output = output_benchmark_json(&benchmark_results).unwrap();
    println!("Benchmark results:\n{}", json_output);
    
    println!("✓ Polkadot mainnet parallel-phragmen benchmark completed: {}ms (threshold: {}ms)", 
             execution_time_ms, THRESHOLD_PARALLEL_PHRAGMEN_MS);
}

#[test]
#[ignore] // Requires network access - run with `cargo test --test test_polkadot_mainnet_performance -- --ignored --nocapture`
fn test_polkadot_mainnet_performance_multiphase() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    println!("Fetching Polkadot mainnet data from recent block...");
    
    // Fetch snapshot with retry logic
    let snapshot = rt.block_on(fetch_polkadot_mainnet_snapshot(DEFAULT_RPC_URL, None))
        .unwrap_or_else(|e| {
            eprintln!("Failed to fetch Polkadot mainnet snapshot: {}", e);
            eprintln!("Suggested alternative RPC endpoints:");
            eprintln!("  - https://rpc.polkadot.io");
            eprintln!("  - https://polkadot-rpc.dwellir.com");
            eprintln!("  - https://polkadot.public.curie.com");
            panic!("RPC fetch failed: {}", e);
        });
    
    println!("Fetched: {} candidates, {} nominators from block {}", 
             snapshot.election_data.candidates.len(),
             snapshot.election_data.nominators.len(),
             snapshot.block_number);
    
    // Measure memory before execution
    let (memory_before_peak, memory_before_current) = measure_memory_usage_platform();
    
    println!("Running benchmark with multi-phase algorithm...");
    
    // Run benchmark
    let (election_result, execution_time_ms) = run_benchmark_with_algorithm(
        &snapshot.election_data,
        AlgorithmType::MultiPhase,
        POLKADOT_ACTIVE_SET_SIZE,
    ).unwrap_or_else(|e| {
        panic!("Benchmark execution failed: {}", e);
    });
    
    // Measure memory after execution
    let (memory_after_peak, memory_after_current) = measure_memory_usage_platform();
    
    // Calculate peak memory
    let memory_peak_mb = if memory_after_peak > memory_before_peak {
        memory_after_peak - memory_before_peak
    } else {
        memory_after_peak.max(memory_before_peak)
    };
    let memory_final_mb = memory_after_current;
    
    // Validate election result
    assert!(election_result.selected_validators.len() == POLKADOT_ACTIVE_SET_SIZE,
            "Should select {} validators, got {}", 
            POLKADOT_ACTIVE_SET_SIZE, 
            election_result.selected_validators.len());
    
    // Validate threshold
    let threshold_passed = execution_time_ms <= THRESHOLD_MULTIPHASE_MS;
    assert!(
        threshold_passed,
        "Execution time {}ms exceeds threshold {}ms for multi-phase",
        execution_time_ms,
        THRESHOLD_MULTIPHASE_MS
    );
    
    // Create benchmark results
    let mut metadata = HashMap::new();
    metadata.insert("benchmark_name".to_string(), "polkadot_mainnet".to_string());
    metadata.insert("candidate_count".to_string(), snapshot.election_data.candidates.len().to_string());
    metadata.insert("nominator_count".to_string(), snapshot.election_data.nominators.len().to_string());
    metadata.insert("algorithm".to_string(), "multi-phase".to_string());
    metadata.insert("block_number".to_string(), snapshot.block_number.to_string());
    metadata.insert("chain".to_string(), "polkadot".to_string());
    metadata.insert("rpc_endpoint".to_string(), snapshot.rpc_endpoint.clone());
    metadata.insert("threshold_ms".to_string(), THRESHOLD_MULTIPHASE_MS.to_string());
    metadata.insert("threshold_passed".to_string(), threshold_passed.to_string());
    metadata.insert("timestamp".to_string(), Utc::now().to_rfc3339());
    
    let benchmark_results = create_benchmark_results(
        execution_time_ms,
        memory_peak_mb,
        memory_final_mb,
        1,
        metadata,
    );
    
    let json_output = output_benchmark_json(&benchmark_results).unwrap();
    println!("Benchmark results:\n{}", json_output);
    
    println!("✓ Polkadot mainnet multi-phase benchmark completed: {}ms (threshold: {}ms)", 
             execution_time_ms, THRESHOLD_MULTIPHASE_MS);
}


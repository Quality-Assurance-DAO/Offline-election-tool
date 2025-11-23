//! Benchmark utilities for performance measurement

use crate::common::models::BenchmarkResults;
use crate::common::memory_measurement::measure_memory_usage_platform;
use serde_json;
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Measure execution time of a function
pub fn measure_execution_time<F, T>(func: F) -> (T, Duration)
where
    F: FnOnce() -> T,
{
    let start = Instant::now();
    let result = func();
    let duration = start.elapsed();
    (result, duration)
}

/// Measure memory usage using platform-specific implementation
/// 
/// Returns current memory usage in MB, or 0 if measurement is unavailable
/// (graceful degradation on unsupported platforms)
pub fn measure_memory_usage() -> u64 {
    let (_peak, current) = measure_memory_usage_platform();
    current
}

/// Output benchmark results as structured JSON
/// 
/// Includes Polkadot-specific metadata fields: block_number, chain, rpc_endpoint,
/// threshold_ms, threshold_passed, memory_measurement_available
pub fn output_benchmark_json(results: &BenchmarkResults) -> Result<String, serde_json::Error> {
    // Determine if memory measurement is available (non-zero indicates measurement succeeded)
    let memory_measurement_available = results.memory_peak_mb > 0 || results.memory_final_mb > 0;
    
    // Create output structure matching benchmark-output.md contract
    let mut metadata = serde_json::json!({
        "benchmark_name": results.metadata.get("benchmark_name").unwrap_or(&"unknown".to_string()),
        "candidate_count": results.metadata.get("candidate_count")
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(0),
        "nominator_count": results.metadata.get("nominator_count")
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(0),
        "algorithm": results.metadata.get("algorithm").unwrap_or(&"unknown".to_string()),
        "iterations": results.iterations,
        "mean_time_ms": results.mean_time_ms,
        "std_dev_ms": results.std_dev_ms,
        "timestamp": results.metadata.get("timestamp").unwrap_or(&"".to_string()),
        "memory_measurement_available": memory_measurement_available,
    });
    
    // Add Polkadot-specific fields if present
    if let Some(block_number) = results.metadata.get("block_number") {
        metadata["block_number"] = serde_json::Value::String(block_number.clone());
    }
    if let Some(chain) = results.metadata.get("chain") {
        metadata["chain"] = serde_json::Value::String(chain.clone());
    }
    if let Some(rpc_endpoint) = results.metadata.get("rpc_endpoint") {
        metadata["rpc_endpoint"] = serde_json::Value::String(rpc_endpoint.clone());
    }
    if let Some(threshold_ms) = results.metadata.get("threshold_ms") {
        metadata["threshold_ms"] = serde_json::Value::String(threshold_ms.clone());
    }
    if let Some(threshold_passed) = results.metadata.get("threshold_passed") {
        metadata["threshold_passed"] = serde_json::Value::String(threshold_passed.clone());
    }
    
    let output = serde_json::json!({
        "timing_ms": results.execution_time_ms,
        "memory_mb": results.memory_peak_mb,
        "metadata": metadata,
    });
    
    serde_json::to_string_pretty(&output)
}

/// Create a benchmark results structure
pub fn create_benchmark_results(
    execution_time_ms: u64,
    memory_peak_mb: u64,
    memory_final_mb: u64,
    iterations: usize,
    metadata: HashMap<String, String>,
) -> BenchmarkResults {
    BenchmarkResults {
        execution_time_ms,
        memory_peak_mb,
        memory_final_mb,
        iterations,
        mean_time_ms: None,
        std_dev_ms: None,
        metadata,
    }
}

/// Run a benchmark with a specific algorithm and active set size
/// 
/// Measures execution time and returns BenchmarkResult.
/// 
/// # Arguments
/// * `election_data` - Election data to benchmark
/// * `algorithm` - Algorithm type to use
/// * `active_set_size` - Active set size for the election
/// 
/// # Returns
/// BenchmarkResult with execution time and metadata
pub fn run_benchmark_with_algorithm(
    election_data: &offline_election::models::ElectionData,
    algorithm: offline_election::types::AlgorithmType,
    active_set_size: usize,
) -> Result<(offline_election::models::ElectionResult, u64), String> {
    use offline_election::engine::ElectionEngine;
    use offline_election::models::election_config::ElectionConfiguration;
    
    let engine = ElectionEngine::new();
    let config = ElectionConfiguration {
        active_set_size: active_set_size as u32,
        algorithm,
        overrides: None,
        block_number: None,
    };
    
    let (result, duration) = measure_execution_time(|| {
        engine.execute(&config, election_data)
    });
    
    let execution_time_ms = duration.as_millis() as u64;
    
    result.map(|r| (r, execution_time_ms))
        .map_err(|e| format!("Election execution failed: {}", e))
}


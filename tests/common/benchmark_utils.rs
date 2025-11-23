//! Benchmark utilities for performance measurement

use crate::common::models::BenchmarkResults;
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

/// Measure memory usage (basic implementation)
/// 
/// Note: This is a simplified implementation. For more accurate memory
/// measurement, consider using platform-specific APIs or jemalloc statistics.
pub fn measure_memory_usage() -> u64 {
    // Basic memory measurement using system allocator
    // In a real implementation, you might use:
    // - jemalloc statistics
    // - platform-specific APIs (e.g., /proc/self/status on Linux)
    // - memory profilers
    
    // For now, return 0 as a placeholder
    // This should be enhanced with actual memory measurement
    0
}

/// Output benchmark results as structured JSON
pub fn output_benchmark_json(results: &BenchmarkResults) -> Result<String, serde_json::Error> {
    // Create output structure matching benchmark-output.md contract
    let output = serde_json::json!({
        "timing_ms": results.execution_time_ms,
        "memory_mb": results.memory_peak_mb,
        "metadata": {
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
        }
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


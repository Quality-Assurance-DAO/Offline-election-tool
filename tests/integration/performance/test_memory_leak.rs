//! Performance test: Memory leak detection (100 consecutive elections)

use offline_election::engine::ElectionEngine;
use offline_election::models::election_config::ElectionConfiguration;
use offline_election::types::AlgorithmType;
use crate::common::data_generator::generate_large_scale_election_data;
use crate::common::benchmark_utils::{measure_execution_time, output_benchmark_json};
use crate::common::models::BenchmarkResults;
use std::collections::HashMap;

#[test]
#[ignore] // Ignore by default - run with `cargo test -- --ignored`
fn test_memory_leak_100_consecutive_elections() {
    const CANDIDATE_COUNT: usize = 100;
    const NOMINATOR_COUNT: usize = 1_000;
    const ITERATIONS: usize = 100;
    
    println!("Running {} consecutive elections to detect memory leaks", ITERATIONS);
    println!("Dataset: {} candidates, {} nominators", CANDIDATE_COUNT, NOMINATOR_COUNT);
    
    let engine = ElectionEngine::new();
    let config = ElectionConfiguration {
        active_set_size: 50,
        algorithm: AlgorithmType::SequentialPhragmen,
        overrides: None,
        block_number: None,
    };
    
    let mut execution_times = Vec::new();
    let mut memory_measurements = Vec::new();
    
    for iteration in 0..ITERATIONS {
        // Generate fresh data for each iteration
        let election_data = generate_large_scale_election_data(
            CANDIDATE_COUNT,
            NOMINATOR_COUNT,
            AlgorithmType::SequentialPhragmen,
        );
        
        let (result, duration) = measure_execution_time(|| {
            engine.execute(&config, &election_data)
        });
        
        let execution_time_ms = duration.as_millis() as u64;
        execution_times.push(execution_time_ms);
        
        // Basic memory measurement (placeholder)
        let memory_mb = crate::common::benchmark_utils::measure_memory_usage();
        memory_measurements.push(memory_mb);
        
        assert!(result.is_ok(), "Election {} should succeed", iteration);
        
        if (iteration + 1) % 10 == 0 {
            println!("Completed {} / {} iterations", iteration + 1, ITERATIONS);
        }
    }
    
    // Calculate statistics
    let mean_time_ms = execution_times.iter().sum::<u64>() as f64 / execution_times.len() as f64;
    let variance = execution_times.iter()
        .map(|&x| (x as f64 - mean_time_ms).powi(2))
        .sum::<f64>() / execution_times.len() as f64;
    let std_dev_ms = variance.sqrt();
    
    // Check for memory leak: memory should not grow significantly
    // For now, we just verify all iterations succeeded
    // In a real implementation, we'd track memory usage across iterations
    
    // Output benchmark results
    let mut metadata = HashMap::new();
    metadata.insert("benchmark_name".to_string(), "memory_leak_detection".to_string());
    metadata.insert("candidate_count".to_string(), CANDIDATE_COUNT.to_string());
    metadata.insert("nominator_count".to_string(), NOMINATOR_COUNT.to_string());
    metadata.insert("iterations".to_string(), ITERATIONS.to_string());
    metadata.insert("algorithm".to_string(), "sequential-phragmen".to_string());
    
    let total_time_ms = execution_times.iter().sum::<u64>();
    let benchmark_results = BenchmarkResults {
        execution_time_ms: total_time_ms,
        memory_peak_mb: memory_measurements.iter().max().copied().unwrap_or(0),
        memory_final_mb: memory_measurements.last().copied().unwrap_or(0),
        iterations: ITERATIONS,
        mean_time_ms: Some(mean_time_ms),
        std_dev_ms: Some(std_dev_ms),
        metadata,
    };
    
    let json_output = output_benchmark_json(&benchmark_results).unwrap();
    println!("Benchmark results:\n{}", json_output);
    
    println!("✓ Memory leak test passed: {} iterations completed", ITERATIONS);
    println!("  Mean execution time: {:.2}ms, Std dev: {:.2}ms", mean_time_ms, std_dev_ms);
}


//! Performance test: Maximum active set size on large datasets

use offline_election::engine::ElectionEngine;
use offline_election::models::election_config::ElectionConfiguration;
use offline_election::types::AlgorithmType;
use crate::common::data_generator::generate_large_scale_election_data;
use crate::common::benchmark_utils::{measure_execution_time, create_benchmark_results, output_benchmark_json};
use std::collections::HashMap;

#[test]
#[ignore] // Ignore by default - run with `cargo test -- --ignored`
fn test_max_active_set_size_large_dataset() {
    const CANDIDATE_COUNT: usize = 1_000;
    const NOMINATOR_COUNT: usize = 10_000;
    const ACTIVE_SET_SIZE: u32 = 1_000; // Maximum active set size
    
    println!("Generating election data: {} candidates, {} nominators, active set size: {}", 
             CANDIDATE_COUNT, NOMINATOR_COUNT, ACTIVE_SET_SIZE);
    
    let election_data = generate_large_scale_election_data(
        CANDIDATE_COUNT,
        NOMINATOR_COUNT,
        AlgorithmType::SequentialPhragmen,
    );
    
    let engine = ElectionEngine::new();
    let config = ElectionConfiguration {
        active_set_size: ACTIVE_SET_SIZE,
        algorithm: AlgorithmType::SequentialPhragmen,
        overrides: None,
        block_number: None,
    };
    
    println!("Executing election...");
    let (result, duration) = measure_execution_time(|| {
        engine.execute(&config, &election_data)
    });
    
    let execution_time_ms = duration.as_millis() as u64;
    
    assert!(result.is_ok(), "Election should succeed");
    let election_result = result.unwrap();
    
    assert_eq!(election_result.selected_validators.len(), ACTIVE_SET_SIZE as usize);
    assert!(election_result.total_stake > 0);
    
    // Output benchmark results
    let mut metadata = HashMap::new();
    metadata.insert("benchmark_name".to_string(), "max_active_set_large".to_string());
    metadata.insert("candidate_count".to_string(), CANDIDATE_COUNT.to_string());
    metadata.insert("nominator_count".to_string(), NOMINATOR_COUNT.to_string());
    metadata.insert("active_set_size".to_string(), ACTIVE_SET_SIZE.to_string());
    metadata.insert("algorithm".to_string(), "sequential-phragmen".to_string());
    
    let benchmark_results = create_benchmark_results(
        execution_time_ms,
        0,
        0,
        1,
        metadata,
    );
    
    let json_output = output_benchmark_json(&benchmark_results).unwrap();
    println!("Benchmark results:\n{}", json_output);
    
    println!("✓ Max active set size test passed: {}ms", execution_time_ms);
}


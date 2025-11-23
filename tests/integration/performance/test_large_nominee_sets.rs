//! Performance test: Large nominee sets (10k+ nominators)

use offline_election::engine::ElectionEngine;
use offline_election::models::election_config::ElectionConfiguration;
use offline_election::types::AlgorithmType;
use crate::common::data_generator::generate_large_scale_election_data;
use crate::common::benchmark_utils::{measure_execution_time, create_benchmark_results, output_benchmark_json};
use std::collections::HashMap;

#[test]
#[ignore] // Ignore by default - run with `cargo test -- --ignored`
fn test_large_nominee_sets_10k_nominators() {
    const CANDIDATE_COUNT: usize = 500;
    const NOMINATOR_COUNT: usize = 10_000;
    
    println!("Generating election data: {} candidates, {} nominators", CANDIDATE_COUNT, NOMINATOR_COUNT);
    
    let election_data = generate_large_scale_election_data(
        CANDIDATE_COUNT,
        NOMINATOR_COUNT,
        AlgorithmType::SequentialPhragmen,
    );
    
    let engine = ElectionEngine::new();
    let config = ElectionConfiguration {
        active_set_size: 100,
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
    
    assert_eq!(election_result.selected_validators.len(), 100);
    assert!(election_result.total_stake > 0);
    
    // Output benchmark results
    let mut metadata = HashMap::new();
    metadata.insert("benchmark_name".to_string(), "large_nominee_sets".to_string());
    metadata.insert("candidate_count".to_string(), CANDIDATE_COUNT.to_string());
    metadata.insert("nominator_count".to_string(), NOMINATOR_COUNT.to_string());
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
    
    println!("✓ Large nominee sets test passed: {}ms", execution_time_ms);
}


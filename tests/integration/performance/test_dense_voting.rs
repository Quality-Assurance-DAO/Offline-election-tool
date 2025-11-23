//! Performance test: Dense voting patterns (nominators vote for many candidates)

use offline_election::engine::ElectionEngine;
use offline_election::models::election_config::ElectionConfiguration;
use offline_election::models::election_data::ElectionData;
use offline_election::models::{Nominator, ValidatorCandidate};
use offline_election::types::AlgorithmType;
use crate::common::benchmark_utils::{measure_execution_time, create_benchmark_results, output_benchmark_json};
use std::collections::HashMap;

#[test]
#[ignore] // Ignore by default - run with `cargo test -- --ignored`
fn test_dense_voting_patterns() {
    const CANDIDATE_COUNT: usize = 500;
    const NOMINATOR_COUNT: usize = 5_000;
    const VOTES_PER_NOMINATOR: usize = 250; // Each nominator votes for 50% of candidates
    
    println!("Generating dense voting data: {} candidates, {} nominators, {} votes per nominator", 
             CANDIDATE_COUNT, NOMINATOR_COUNT, VOTES_PER_NOMINATOR);
    
    let mut election_data = ElectionData::new();
    
    // Generate candidates
    for i in 0..CANDIDATE_COUNT {
        let account_id = format!("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY{}", i);
        let stake = 1_000_000_000 + (i as u128 * 100_000_000);
        let candidate = ValidatorCandidate {
            account_id,
            stake,
            metadata: None,
        };
        election_data.add_candidate(candidate).unwrap();
    }
    
    // Generate nominators with dense voting (each votes for many candidates)
    for i in 0..NOMINATOR_COUNT {
        let account_id = format!("5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty{}", i);
        let stake = 500_000_000 + (i as u128 * 50_000_000);
        
        // Dense voting: vote for VOTES_PER_NOMINATOR candidates
        let targets: Vec<String> = (0..VOTES_PER_NOMINATOR)
            .map(|j| format!("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY{}", j))
            .collect();
        
        let nominator = Nominator {
            account_id,
            stake,
            targets,
            metadata: None,
        };
        election_data.add_nominator(nominator).unwrap();
    }
    
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
    metadata.insert("benchmark_name".to_string(), "dense_voting".to_string());
    metadata.insert("candidate_count".to_string(), CANDIDATE_COUNT.to_string());
    metadata.insert("nominator_count".to_string(), NOMINATOR_COUNT.to_string());
    metadata.insert("votes_per_nominator".to_string(), VOTES_PER_NOMINATOR.to_string());
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
    
    println!("✓ Dense voting test passed: {}ms", execution_time_ms);
}


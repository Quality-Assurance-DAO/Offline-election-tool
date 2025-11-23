//! Edge case test: zero candidates

mod common;

use offline_election::engine::ElectionEngine;
use offline_election::models::election_config::ElectionConfiguration;
use offline_election::models::election_data::ElectionData;
use offline_election::types::AlgorithmType;

#[test]
fn test_zero_candidates_should_fail() {
    let engine = ElectionEngine::new();
    let mut election_data = ElectionData::new();
    
    // Add a nominator but no candidates
    let nominator = offline_election::models::nominator::Nominator {
        account_id: "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty".to_string(),
        stake: 1_000_000_000,
        targets: vec![],
        metadata: None,
    };
    election_data.add_nominator(nominator).unwrap();
    
    let config = ElectionConfiguration {
        active_set_size: 3,
        algorithm: AlgorithmType::SequentialPhragmen,
        overrides: None,
        block_number: None,
    };
    
    let result = engine.execute(&config, &election_data);
    
    assert!(result.is_err(), "Election with zero candidates should fail");
    
    let error = result.unwrap_err();
    let error_message = format!("{}", error);
    
    // Check that error message mentions candidates
    assert!(
        error_message.to_lowercase().contains("candidate"),
        "Error message should mention candidates, got: {}",
        error_message
    );
}


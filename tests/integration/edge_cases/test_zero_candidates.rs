//! Edge case test: zero candidates

use offline_election::engine::ElectionEngine;
use offline_election::models::election_config::ElectionConfiguration;
use offline_election::models::election_data::ElectionData;
use offline_election::types::AlgorithmType;
use common::assertions::assert_error_message_contains;

#[test]
fn test_zero_candidates_should_fail() {
    let engine = ElectionEngine::new();
    let mut election_data = ElectionData::new();
    
    // Add a nominator but no candidates
    let nominator = offline_election::models::nominator::Nominator {
        account_id: "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty".to_string(),
        stake: 1_000_000_000,
        targets: vec![],
    };
    election_data.add_nominator(nominator).unwrap();
    
    let config = ElectionConfiguration {
        active_set_size: 3,
        algorithm: AlgorithmType::SequentialPhragmen,
        overrides: None,
    };
    
    let result = engine.execute(&config, &election_data);
    
    assert!(result.is_err(), "Election with zero candidates should fail");
    
    let error = result.unwrap_err();
    let error_message = format!("{}", error);
    
    assert_error_message_contains(
        &error_message,
        &["candidate".to_string(), "zero".to_string()],
    );
}


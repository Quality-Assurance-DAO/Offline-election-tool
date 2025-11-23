//! Edge case test: voting edges referencing non-existent candidates

use offline_election::engine::ElectionEngine;
use offline_election::models::election_config::ElectionConfiguration;
use offline_election::models::election_data::ElectionData;
use offline_election::models::nominator::Nominator;
use offline_election::models::validator::ValidatorCandidate;
use offline_election::types::AlgorithmType;
use common::assertions::assert_error_message_contains;

#[test]
fn test_nominator_votes_for_nonexistent_candidate() {
    let engine = ElectionEngine::new();
    let mut election_data = ElectionData::new();
    
    // Add one candidate
    let candidate = ValidatorCandidate::new(
        "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string(),
        1_000_000_000,
    );
    election_data.add_candidate(candidate).unwrap();
    
    // Create a nominator voting for a candidate that doesn't exist
    let nominator = Nominator {
        account_id: "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty".to_string(),
        stake: 1_000_000_000,
        targets: vec!["5FLSigC9HGRKVhB9F7BqHjXJxZJxZJxZJxZJxZJxZJxZJxZJxZ".to_string()], // Non-existent candidate
        metadata: None,
    };
    
    election_data.add_nominator(nominator).unwrap();
    
    let config = ElectionConfiguration {
        active_set_size: 1,
        algorithm: AlgorithmType::SequentialPhragmen,
        overrides: None,
        block_number: None,
    };
    
    let result = engine.execute(&config, &election_data);
    
    assert!(result.is_err(), "Election with invalid voting targets should fail");
    
    let error = result.unwrap_err();
    let error_message = format!("{}", error);
    
    assert_error_message_contains(
        &error_message,
        &["non-existent".to_string(), "candidate".to_string()],
    );
}

#[test]
fn test_nominator_votes_for_multiple_nonexistent_candidates() {
    let engine = ElectionEngine::new();
    let mut election_data = ElectionData::new();
    
    // Add one candidate
    let candidate = ValidatorCandidate::new(
        "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string(),
        1_000_000_000,
    );
    election_data.add_candidate(candidate).unwrap();
    
    // Create a nominator voting for multiple non-existent candidates
    let nominator = Nominator {
        account_id: "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty".to_string(),
        stake: 1_000_000_000,
        targets: vec![
            "5FLSigC9HGRKVhB9F7BqHjXJxZJxZJxZJxZJxZJxZJxZJxZJxZ".to_string(),
            "5DbKjhNLpqX3HYq2b3tS1J3Z6sF7X8Y9Z0A1B2C3D4E5F6G7H8".to_string(),
        ],
        metadata: None,
    };
    
    election_data.add_nominator(nominator).unwrap();
    
    let config = ElectionConfiguration {
        active_set_size: 1,
        algorithm: AlgorithmType::SequentialPhragmen,
        overrides: None,
        block_number: None,
    };
    
    let result = engine.execute(&config, &election_data);
    
    assert!(result.is_err(), "Election with multiple invalid voting targets should fail");
    
    let error = result.unwrap_err();
    let error_message = format!("{}", error);
    
    assert_error_message_contains(
        &error_message,
        &["non-existent".to_string(), "candidate".to_string()],
    );
}


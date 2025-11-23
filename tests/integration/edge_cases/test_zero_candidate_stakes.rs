//! Edge case test: all candidates have zero stake

use offline_election::engine::ElectionEngine;
use offline_election::models::election_config::ElectionConfiguration;
use offline_election::models::election_data::ElectionData;
use offline_election::types::AlgorithmType;
use common::assertions::assert_error_message_contains;

#[test]
fn test_zero_candidate_stakes_should_fail() {
    let engine = ElectionEngine::new();
    let mut election_data = ElectionData::new();
    
    // Add candidates with zero stake
    for i in 0..3 {
        let candidate = offline_election::models::validator::ValidatorCandidate {
            account_id: format!("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY{}", i),
            stake: 0,
        };
        election_data.add_candidate(candidate).unwrap();
    }
    
    // Add nominators with non-zero stake
    let nominator = offline_election::models::nominator::Nominator {
        account_id: "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty".to_string(),
        stake: 1_000_000_000,
        targets: vec![
            "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY0".to_string(),
        ],
    };
    election_data.add_nominator(nominator).unwrap();
    
    let config = ElectionConfiguration {
        active_set_size: 3,
        algorithm: AlgorithmType::SequentialPhragmen,
        overrides: None,
    };
    
    let result = engine.execute(&config, &election_data);
    
    assert!(result.is_err(), "Election with zero candidate stakes should fail");
    
    let error = result.unwrap_err();
    let error_message = format!("{}", error);
    
    assert_error_message_contains(
        &error_message,
        &["stake".to_string(), "zero".to_string()],
    );
}


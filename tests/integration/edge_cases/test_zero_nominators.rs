//! Edge case test: zero nominators

use offline_election::engine::ElectionEngine;
use offline_election::models::election_config::ElectionConfiguration;
use offline_election::models::election_data::ElectionData;
use offline_election::types::AlgorithmType;

#[test]
fn test_zero_nominators_should_succeed() {
    let engine = ElectionEngine::new();
    let mut election_data = ElectionData::new();
    
    // Add candidates but no nominators
    let candidate = offline_election::models::validator::ValidatorCandidate {
        account_id: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string(),
        stake: 1_000_000_000,
    };
    election_data.add_candidate(candidate).unwrap();
    
    let config = ElectionConfiguration {
        active_set_size: 1,
        algorithm: AlgorithmType::SequentialPhragmen,
        overrides: None,
    };
    
    let result = engine.execute(&config, &election_data);
    
    // Election should succeed even with zero nominators
    assert!(result.is_ok(), "Election with zero nominators should succeed");
    
    let election_result = result.unwrap();
    assert_eq!(election_result.selected_validators.len(), 1);
}


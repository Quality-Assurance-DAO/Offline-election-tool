//! Edge case test: single candidate

use offline_election::engine::ElectionEngine;
use offline_election::models::election_config::ElectionConfiguration;
use offline_election::models::election_data::ElectionData;
use offline_election::types::AlgorithmType;

#[test]
fn test_single_candidate_should_succeed() {
    let engine = ElectionEngine::new();
    let mut election_data = ElectionData::new();
    
    // Add a single candidate
    let candidate = offline_election::models::validator::ValidatorCandidate {
        account_id: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string(),
        stake: 1_000_000_000,
    };
    election_data.add_candidate(candidate).unwrap();
    
    // Add a nominator
    let nominator = offline_election::models::nominator::Nominator {
        account_id: "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty".to_string(),
        stake: 500_000_000,
        targets: vec!["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string()],
    };
    election_data.add_nominator(nominator).unwrap();
    
    let config = ElectionConfiguration {
        active_set_size: 1,
        algorithm: AlgorithmType::SequentialPhragmen,
        overrides: None,
    };
    
    let result = engine.execute(&config, &election_data);
    
    assert!(result.is_ok(), "Election with single candidate should succeed");
    
    let election_result = result.unwrap();
    assert_eq!(election_result.selected_validators.len(), 1);
    assert_eq!(
        election_result.selected_validators[0].account_id,
        "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
    );
}


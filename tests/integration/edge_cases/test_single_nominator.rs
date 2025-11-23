//! Edge case test: single nominator

use offline_election::engine::ElectionEngine;
use offline_election::models::election_config::ElectionConfiguration;
use offline_election::models::election_data::ElectionData;
use offline_election::types::AlgorithmType;

#[test]
fn test_single_nominator_should_succeed() {
    let engine = ElectionEngine::new();
    let mut election_data = ElectionData::new();
    
    // Add multiple candidates
    for i in 0..3 {
        let candidate = offline_election::models::validator::ValidatorCandidate {
            account_id: format!("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY{}", i),
            stake: 1_000_000_000 + (i as u128 * 100_000_000),
        };
        election_data.add_candidate(candidate).unwrap();
    }
    
    // Add a single nominator
    let nominator = offline_election::models::nominator::Nominator {
        account_id: "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty".to_string(),
        stake: 500_000_000,
        targets: vec![
            "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY0".to_string(),
            "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY1".to_string(),
        ],
    };
    election_data.add_nominator(nominator).unwrap();
    
    let config = ElectionConfiguration {
        active_set_size: 2,
        algorithm: AlgorithmType::SequentialPhragmen,
        overrides: None,
    };
    
    let result = engine.execute(&config, &election_data);
    
    assert!(result.is_ok(), "Election with single nominator should succeed");
    
    let election_result = result.unwrap();
    assert_eq!(election_result.selected_validators.len(), 2);
}


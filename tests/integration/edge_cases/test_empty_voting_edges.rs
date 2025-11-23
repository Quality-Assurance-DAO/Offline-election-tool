//! Edge case test: nominators voting for zero candidates

use offline_election::engine::ElectionEngine;
use offline_election::models::election_config::ElectionConfiguration;
use offline_election::models::election_data::ElectionData;
use offline_election::types::AlgorithmType;

#[test]
fn test_empty_voting_edges_should_succeed() {
    let engine = ElectionEngine::new();
    let mut election_data = ElectionData::new();
    
    // Add candidates
    for i in 0..3 {
        let candidate = offline_election::models::validator::ValidatorCandidate {
            account_id: format!("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY{}", i),
            stake: 1_000_000_000 + (i as u128 * 100_000_000),
        };
        election_data.add_candidate(candidate).unwrap();
    }
    
    // Add nominators with non-zero stake but empty targets (voting for zero candidates)
    for i in 0..2 {
        let nominator = offline_election::models::nominator::Nominator {
            account_id: format!("5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty{}", i),
            stake: 500_000_000 + (i as u128 * 100_000_000),
            targets: vec![], // Empty targets - not voting for anyone
        };
        election_data.add_nominator(nominator).unwrap();
    }
    
    let config = ElectionConfiguration {
        active_set_size: 2,
        algorithm: AlgorithmType::SequentialPhragmen,
        overrides: None,
    };
    
    let result = engine.execute(&config, &election_data);
    
    // Election should succeed even if nominators have empty voting edges
    // The candidates' own stake will determine the selection
    assert!(result.is_ok(), "Election with empty voting edges should succeed");
    
    let election_result = result.unwrap();
    // Should select validators based on their own stake
    assert_eq!(election_result.selected_validators.len(), 2);
    
    // Verify that validators with higher stake are selected
    // (candidates are ordered by stake: 0=1B, 1=1.1B, 2=1.2B)
    let selected_ids: Vec<&String> = election_result
        .selected_validators
        .iter()
        .map(|v| &v.account_id)
        .collect();
    
    // Should select the two highest stake candidates (indices 1 and 2)
    assert!(
        selected_ids.contains(&&"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY1".to_string()),
        "Should select candidate 1"
    );
    assert!(
        selected_ids.contains(&&"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY2".to_string()),
        "Should select candidate 2"
    );
}


//! Edge case test: active set size equals candidate count

use offline_election::engine::ElectionEngine;
use offline_election::models::election_config::ElectionConfiguration;
use offline_election::models::election_data::ElectionData;
use offline_election::types::AlgorithmType;

#[test]
fn test_max_active_set_size_should_succeed() {
    let engine = ElectionEngine::new();
    let mut election_data = ElectionData::new();
    
    // Add exactly 5 candidates
    let candidate_count = 5;
    for i in 0..candidate_count {
        let candidate = offline_election::models::validator::ValidatorCandidate {
            account_id: format!("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY{}", i),
            stake: 1_000_000_000 + (i as u128 * 100_000_000),
        };
        election_data.add_candidate(candidate).unwrap();
    }
    
    // Add nominators voting for all candidates
    for i in 0..3 {
        let nominator = offline_election::models::nominator::Nominator {
            account_id: format!("5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty{}", i),
            stake: 500_000_000 + (i as u128 * 100_000_000),
            targets: (0..candidate_count)
                .map(|j| format!("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY{}", j))
                .collect(),
        };
        election_data.add_nominator(nominator).unwrap();
    }
    
    // Set active set size equal to candidate count
    let config = ElectionConfiguration {
        active_set_size: candidate_count,
        algorithm: AlgorithmType::SequentialPhragmen,
        overrides: None,
    };
    
    let result = engine.execute(&config, &election_data);
    
    assert!(result.is_ok(), "Election with active set size equal to candidate count should succeed");
    
    let election_result = result.unwrap();
    // All candidates should be selected
    assert_eq!(election_result.selected_validators.len(), candidate_count);
    
    // Verify all candidate IDs are present in results
    let selected_ids: std::collections::HashSet<&String> = election_result
        .selected_validators
        .iter()
        .map(|v| &v.account_id)
        .collect();
    
    for i in 0..candidate_count {
        let expected_id = format!("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY{}", i);
        assert!(
            selected_ids.contains(&expected_id),
            "Candidate {} should be selected",
            expected_id
        );
    }
}


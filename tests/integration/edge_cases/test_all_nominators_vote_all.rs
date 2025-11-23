//! Edge case test: all nominators vote for all candidates

use offline_election::engine::ElectionEngine;
use offline_election::models::election_config::ElectionConfiguration;
use offline_election::models::election_data::ElectionData;
use offline_election::models::nominator::Nominator;
use offline_election::models::validator::ValidatorCandidate;
use offline_election::types::AlgorithmType;

#[test]
fn test_all_nominators_vote_all_candidates() {
    let engine = ElectionEngine::new();
    let mut election_data = ElectionData::new();
    
    // Create 3 candidates
    let candidate1 = ValidatorCandidate::new(
        "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string(),
        1_000_000_000,
    );
    let candidate2 = ValidatorCandidate::new(
        "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty".to_string(),
        2_000_000_000,
    );
    let candidate3 = ValidatorCandidate::new(
        "5FLSigC9HGRKVhB9F7BqHjXJxZJxZJxZJxZJxZJxZJxZJxZJxZ".to_string(),
        3_000_000_000,
    );
    
    election_data.add_candidate(candidate1).unwrap();
    election_data.add_candidate(candidate2).unwrap();
    election_data.add_candidate(candidate3).unwrap();
    
    // Create 2 nominators, both voting for all candidates
    let all_candidate_ids = vec![
        "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string(),
        "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty".to_string(),
        "5FLSigC9HGRKVhB9F7BqHjXJxZJxZJxZJxZJxZJxZJxZJxZJxZ".to_string(),
    ];
    
    let nominator1 = Nominator {
        account_id: "5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY".to_string(),
        stake: 10_000_000_000,
        targets: all_candidate_ids.clone(),
        metadata: None,
    };
    
    let nominator2 = Nominator {
        account_id: "5DbKjhNLpqX3HYq2b3tS1J3Z6sF7X8Y9Z0A1B2C3D4E5F6G7H8".to_string(),
        stake: 20_000_000_000,
        targets: all_candidate_ids.clone(),
        metadata: None,
    };
    
    election_data.add_nominator(nominator1).unwrap();
    election_data.add_nominator(nominator2).unwrap();
    
    let config = ElectionConfiguration {
        active_set_size: 3,
        algorithm: AlgorithmType::SequentialPhragmen,
        overrides: None,
        block_number: None,
    };
    
    let result = engine.execute(&config, &election_data);
    
    assert!(result.is_ok(), "Election with all nominators voting for all candidates should succeed");
    
    let election_result = result.unwrap();
    assert_eq!(election_result.selected_validators.len(), 3, "Should select all 3 candidates");
    assert!(election_result.total_stake > 0, "Total stake should be positive");
}


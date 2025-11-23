//! Edge case test: all nominators have zero stake

use offline_election::engine::ElectionEngine;
use offline_election::models::election_config::ElectionConfiguration;
use offline_election::models::election_data::ElectionData;
use offline_election::models::nominator::Nominator;
use offline_election::models::validator::ValidatorCandidate;
use offline_election::types::AlgorithmType;

#[test]
fn test_all_nominators_zero_stake() {
    let engine = ElectionEngine::new();
    let mut election_data = ElectionData::new();
    
    // Create candidates with non-zero stake
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
    
    // Create nominators with zero stake
    let nominator1 = Nominator {
        account_id: "5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY".to_string(),
        stake: 0,
        targets: vec![
            "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string(),
        ],
        metadata: None,
    };
    
    let nominator2 = Nominator {
        account_id: "5DbKjhNLpqX3HYq2b3tS1J3Z6sF7X8Y9Z0A1B2C3D4E5F6G7H8".to_string(),
        stake: 0,
        targets: vec![
            "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty".to_string(),
        ],
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
    
    // Election should succeed even with zero-stake nominators
    // The algorithm should select validators based on their own stake
    let result = engine.execute(&config, &election_data);
    
    match result {
        Ok(election_result) => {
            // Should select validators based on their own stake
            assert_eq!(election_result.selected_validators.len(), 3);
            // Total stake should be sum of candidate stakes (nominators have zero)
            assert!(election_result.total_stake > 0);
        }
        Err(e) => {
            // If it fails, it should be a clear error, not a panic
            let error_message = format!("{}", e);
            // Zero-stake nominators are allowed, so this should succeed
            // But if it fails, we document the behavior
            panic!("Election with zero-stake nominators failed: {}", error_message);
        }
    }
}


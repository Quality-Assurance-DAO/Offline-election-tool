//! Edge case test: extremely large stake values

use offline_election::engine::ElectionEngine;
use offline_election::models::election_config::ElectionConfiguration;
use offline_election::models::election_data::ElectionData;
use offline_election::models::nominator::Nominator;
use offline_election::models::validator::ValidatorCandidate;
use offline_election::types::AlgorithmType;

#[test]
fn test_maximum_u128_stakes() {
    let engine = ElectionEngine::new();
    let mut election_data = ElectionData::new();
    
    // Create candidates with maximum u128 stake values
    let max_stake = u128::MAX;
    
    let candidate1 = ValidatorCandidate::new(
        "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string(),
        max_stake,
    );
    let candidate2 = ValidatorCandidate::new(
        "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty".to_string(),
        max_stake / 2,
    );
    let candidate3 = ValidatorCandidate::new(
        "5FLSigC9HGRKVhB9F7BqHjXJxZJxZJxZJxZJxZJxZJxZJxZJxZ".to_string(),
        max_stake / 4,
    );
    
    election_data.add_candidate(candidate1).unwrap();
    election_data.add_candidate(candidate2).unwrap();
    election_data.add_candidate(candidate3).unwrap();
    
    // Create nominators with very large stakes
    let nominator1 = Nominator {
        account_id: "5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY".to_string(),
        stake: max_stake,
        targets: vec![
            "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string(),
            "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty".to_string(),
        ],
        metadata: None,
    };
    
    let nominator2 = Nominator {
        account_id: "5DbKjhNLpqX3HYq2b3tS1J3Z6sF7X8Y9Z0A1B2C3D4E5F6G7H8".to_string(),
        stake: max_stake / 2,
        targets: vec![
            "5FLSigC9HGRKVhB9F7BqHjXJxZJxZJxZJxZJxZJxZJxZJxZJxZ".to_string(),
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
    
    // The election should handle maximum stake values without panicking
    // Note: This might take longer to compute due to large numbers
    let result = engine.execute(&config, &election_data);
    
    // System should handle maximum stakes gracefully (either succeed or fail with clear error)
    match result {
        Ok(election_result) => {
            assert_eq!(election_result.selected_validators.len(), 3);
            assert!(election_result.total_stake > 0);
        }
        Err(e) => {
            // If it fails, error should be clear (not a panic)
            let error_message = format!("{}", e);
            assert!(
                !error_message.contains("panic") && !error_message.contains("overflow"),
                "Should not panic on maximum stakes: {}",
                error_message
            );
        }
    }
}

#[test]
fn test_very_large_stakes() {
    let engine = ElectionEngine::new();
    let mut election_data = ElectionData::new();
    
    // Use very large but not maximum stakes (10^18 * 10^9 = 10^27)
    let large_stake = 1_000_000_000_000_000_000_000_000_000u128;
    
    let candidate = ValidatorCandidate::new(
        "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string(),
        large_stake,
    );
    election_data.add_candidate(candidate).unwrap();
    
    let nominator = Nominator {
        account_id: "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty".to_string(),
        stake: large_stake * 2,
        targets: vec!["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string()],
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
    
    assert!(result.is_ok(), "Election with very large stakes should succeed");
    
    let election_result = result.unwrap();
    assert_eq!(election_result.selected_validators.len(), 1);
    assert!(election_result.total_stake >= large_stake);
}


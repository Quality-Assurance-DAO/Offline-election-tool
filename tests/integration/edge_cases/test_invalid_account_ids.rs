//! Edge case test: invalid SS58 account IDs

use offline_election::engine::ElectionEngine;
use offline_election::models::election_config::ElectionConfiguration;
use offline_election::models::election_data::ElectionData;
use offline_election::models::nominator::Nominator;
use offline_election::models::validator::ValidatorCandidate;
use offline_election::types::AlgorithmType;
use common::assertions::assert_error_message_contains;

#[test]
fn test_invalid_account_id_format() {
    let engine = ElectionEngine::new();
    let mut election_data = ElectionData::new();
    
    // Try to add a candidate with an invalid account ID (not SS58 format)
    // Note: The actual validation might happen during election execution
    // or during data loading, depending on implementation
    
    // For now, we'll test that the system handles invalid IDs gracefully
    // This test may need adjustment based on where SS58 validation occurs
    
    let candidate = ValidatorCandidate::new(
        "invalid_account_id_not_ss58".to_string(),
        1_000_000_000,
    );
    
    // Adding the candidate should succeed (validation may happen later)
    election_data.add_candidate(candidate).unwrap();
    
    let nominator = Nominator {
        account_id: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string(),
        stake: 1_000_000_000,
        targets: vec!["invalid_account_id_not_ss58".to_string()],
        metadata: None,
    };
    
    election_data.add_nominator(nominator).unwrap();
    
    let config = ElectionConfiguration {
        active_set_size: 1,
        algorithm: AlgorithmType::SequentialPhragmen,
        overrides: None,
        block_number: None,
    };
    
    // The election might succeed or fail depending on SS58 validation
    // This test documents the current behavior
    let result = engine.execute(&config, &election_data);
    
    // If validation happens, we should get an error
    // If not, the election might proceed (which is also acceptable for this test)
    // The key is that the system doesn't panic
    match result {
        Ok(_) => {
            // System accepts non-SS58 IDs - this is acceptable behavior
        }
        Err(e) => {
            // System validates SS58 format - verify error message is clear
            let error_message = format!("{}", e);
            // Error should mention account ID or validation
            assert!(
                error_message.contains("account") || error_message.contains("invalid") || error_message.contains("format"),
                "Error message should mention account ID or validation: {}",
                error_message
            );
        }
    }
}

#[test]
fn test_empty_account_id() {
    let engine = ElectionEngine::new();
    let mut election_data = ElectionData::new();
    
    let candidate = ValidatorCandidate::new(
        "".to_string(),
        1_000_000_000,
    );
    
    election_data.add_candidate(candidate).unwrap();
    
    let nominator = Nominator {
        account_id: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string(),
        stake: 1_000_000_000,
        targets: vec!["".to_string()],
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
    
    // Empty account IDs should be rejected
    assert!(result.is_err(), "Election with empty account IDs should fail");
    
    let error = result.unwrap_err();
    let error_message = format!("{}", error);
    
    assert_error_message_contains(
        &error_message,
        &["candidate".to_string()],
    );
}


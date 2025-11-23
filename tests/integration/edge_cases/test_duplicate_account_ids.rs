//! Edge case test: duplicate account IDs

use offline_election::engine::ElectionEngine;
use offline_election::models::election_config::ElectionConfiguration;
use offline_election::models::election_data::ElectionData;
use offline_election::types::AlgorithmType;
use common::assertions::assert_error_message_contains;

#[test]
fn test_duplicate_candidate_account_ids_should_fail() {
    let engine = ElectionEngine::new();
    let mut election_data = ElectionData::new();
    
    let duplicate_id = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string();
    
    // Add first candidate
    let candidate1 = offline_election::models::validator::ValidatorCandidate {
        account_id: duplicate_id.clone(),
        stake: 1_000_000_000,
    };
    election_data.add_candidate(candidate1).unwrap();
    
    // Try to add duplicate candidate
    let candidate2 = offline_election::models::validator::ValidatorCandidate {
        account_id: duplicate_id.clone(),
        stake: 2_000_000_000,
    };
    
    let result = election_data.add_candidate(candidate2);
    
    assert!(result.is_err(), "Adding duplicate candidate account ID should fail");
    
    let error = result.unwrap_err();
    let error_message = format!("{}", error);
    
    assert_error_message_contains(
        &error_message,
        &["duplicate".to_string(), "candidate".to_string()],
    );
}

#[test]
fn test_duplicate_nominator_account_ids_should_fail() {
    let engine = ElectionEngine::new();
    let mut election_data = ElectionData::new();
    
    // Add a candidate first
    let candidate = offline_election::models::validator::ValidatorCandidate {
        account_id: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string(),
        stake: 1_000_000_000,
    };
    election_data.add_candidate(candidate).unwrap();
    
    let duplicate_id = "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty".to_string();
    
    // Add first nominator
    let nominator1 = offline_election::models::nominator::Nominator {
        account_id: duplicate_id.clone(),
        stake: 500_000_000,
        targets: vec!["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string()],
    };
    election_data.add_nominator(nominator1).unwrap();
    
    // Try to add duplicate nominator
    let nominator2 = offline_election::models::nominator::Nominator {
        account_id: duplicate_id.clone(),
        stake: 1_000_000_000,
        targets: vec!["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string()],
    };
    
    let result = election_data.add_nominator(nominator2);
    
    assert!(result.is_err(), "Adding duplicate nominator account ID should fail");
    
    let error = result.unwrap_err();
    let error_message = format!("{}", error);
    
    assert_error_message_contains(
        &error_message,
        &["duplicate".to_string(), "nominator".to_string()],
    );
}


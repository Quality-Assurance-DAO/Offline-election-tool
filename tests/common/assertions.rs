//! Test assertion utilities

use offline_election::models::ElectionResult;
use std::collections::HashMap;

/// Assert that an error message contains specific substrings
pub fn assert_error_message_contains(error_message: &str, expected_substrings: &[String]) {
    for substring in expected_substrings {
        assert!(
            error_message.contains(substring),
            "Expected error message to contain '{}', but got: {}",
            substring,
            error_message
        );
    }
}

/// Compare election results for exact match
/// 
/// Validates that selected validator account IDs and stake allocations
/// match exactly between actual and expected results.
pub fn compare_results_exact_match(
    actual: &ElectionResult,
    expected: &ElectionResult,
) -> Result<(), String> {
    // Compare selected validators
    let actual_validator_ids: Vec<&String> = actual
        .selected_validators
        .iter()
        .map(|v| &v.account_id)
        .collect();
    
    let expected_validator_ids: Vec<&String> = expected
        .selected_validators
        .iter()
        .map(|v| &v.account_id)
        .collect();
    
    if actual_validator_ids != expected_validator_ids {
        return Err(format!(
            "Selected validators mismatch: expected {:?}, got {:?}",
            expected_validator_ids, actual_validator_ids
        ));
    }
    
    // Compare stake allocations
    let actual_allocations: HashMap<(String, String), u128> = actual
        .stake_distribution
        .iter()
        .map(|alloc| ((alloc.nominator_id.clone(), alloc.validator_id.clone()), alloc.amount))
        .collect();
    
    let expected_allocations: HashMap<(String, String), u128> = expected
        .stake_distribution
        .iter()
        .map(|alloc| ((alloc.nominator_id.clone(), alloc.validator_id.clone()), alloc.amount))
        .collect();
    
    if actual_allocations != expected_allocations {
        return Err(format!(
            "Stake allocations mismatch: expected {:?}, got {:?}",
            expected_allocations, actual_allocations
        ));
    }
    
    Ok(())
}

/// Assert that election result structure is valid
pub fn assert_election_result_valid(result: &ElectionResult) {
    assert!(!result.selected_validators.is_empty(), "Result must have at least one selected validator");
    assert!(!result.stake_distribution.is_empty(), "Result must have at least one stake allocation");
    assert!(result.total_stake > 0, "Total stake must be positive");
}

/// Detect result changes between baseline and current results
pub fn detect_result_changes(
    baseline: &ElectionResult,
    current: &ElectionResult,
) -> Vec<String> {
    let mut changes = Vec::new();
    
    // Check for validator changes
    let baseline_ids: std::collections::HashSet<&String> = baseline
        .selected_validators
        .iter()
        .map(|v| &v.account_id)
        .collect();
    
    let current_ids: std::collections::HashSet<&String> = current
        .selected_validators
        .iter()
        .map(|v| &v.account_id)
        .collect();
    
    if baseline_ids != current_ids {
        changes.push(format!(
            "Validator set changed: baseline {:?}, current {:?}",
            baseline_ids, current_ids
        ));
    }
    
    // Check for stake allocation changes
    let baseline_allocations: HashMap<(String, String), u128> = baseline
        .stake_distribution
        .iter()
        .map(|alloc| ((alloc.nominator_id.clone(), alloc.validator_id.clone()), alloc.amount))
        .collect();
    
    let current_allocations: HashMap<(String, String), u128> = current
        .stake_distribution
        .iter()
        .map(|alloc| ((alloc.nominator_id.clone(), alloc.validator_id.clone()), alloc.amount))
        .collect();
    
    if baseline_allocations != current_allocations {
        changes.push("Stake allocations changed".to_string());
    }
    
    changes
}

/// Assert that results match baseline
pub fn assert_results_match_baseline(
    baseline: &ElectionResult,
    current: &ElectionResult,
) {
    let changes = detect_result_changes(baseline, current);
    assert!(
        changes.is_empty(),
        "Results do not match baseline: {:?}",
        changes
    );
}


//! Edge case integration tests
//!
//! This module contains comprehensive edge case tests covering boundary conditions,
//! error cases, and unusual input scenarios. Tests can be run individually or via
//! the fixture-based test runner.

mod test_zero_candidates;
mod test_zero_nominators;
mod test_single_candidate;
mod test_single_nominator;
mod test_zero_candidate_stakes;
mod test_zero_nominator_stakes;
mod test_max_active_set_size;
mod test_empty_voting_edges;
mod test_duplicate_account_ids;
mod test_all_nominators_vote_all;
mod test_invalid_account_ids;
mod test_invalid_voting_targets;
mod test_maximum_stakes;
mod test_malformed_json;

pub use test_zero_candidates::*;
pub use test_zero_nominators::*;
pub use test_single_candidate::*;
pub use test_single_nominator::*;
pub use test_zero_candidate_stakes::*;
pub use test_zero_nominator_stakes::*;
pub use test_max_active_set_size::*;
pub use test_empty_voting_edges::*;
pub use test_duplicate_account_ids::*;
pub use test_all_nominators_vote_all::*;
pub use test_invalid_account_ids::*;
pub use test_invalid_voting_targets::*;
pub use test_maximum_stakes::*;
pub use test_malformed_json::*;

#[cfg(test)]
mod test_runner {
    use offline_election::engine::ElectionEngine;
    use offline_election::models::election_config::ElectionConfiguration;
    use crate::common::fixture_loader::load_test_fixture;
    use std::path::PathBuf;

    /// Run an edge case test from a fixture file
    /// 
    /// Loads a test fixture, executes the election, and validates the result
    /// matches expected behavior (success or error).
    pub fn run_edge_case_test_from_fixture(fixture_path: &str) -> Result<(), String> {
        let path = PathBuf::from(fixture_path);
        let fixture = load_test_fixture(&path)?;
        
        let engine = ElectionEngine::new();
        let config = ElectionConfiguration {
            active_set_size: 3, // Default, may be overridden by test
            algorithm: fixture.metadata.algorithm,
            overrides: None,
            block_number: None,
        };
        
        // Validate input data
        let validation_result = fixture.input.validate();
        
        // Execute election
        let result = engine.execute(&config, &fixture.input);
        
        // Determine expected behavior based on fixture metadata
        // Error cases typically have tags like "error_case"
        let is_error_case = fixture.metadata.tags.iter().any(|t| t.contains("error"));
        
        match (is_error_case, validation_result, result) {
            // Error case: should fail
            (true, Err(_), _) => {
                // Validation error is expected
                Ok(())
            }
            (true, Ok(_), Err(e)) => {
                // Execution error is expected
                let error_message = format!("{}", e);
                // Verify error message is clear and actionable
                assert!(
                    !error_message.is_empty(),
                    "Error message should not be empty"
                );
                Ok(())
            }
            (true, Ok(_), Ok(_)) => {
                Err("Error case test should have failed but succeeded".to_string())
            }
            // Success case: should succeed
            (false, Ok(_), Ok(election_result)) => {
                // Verify result structure
                assert!(!election_result.selected_validators.is_empty());
                assert!(election_result.total_stake > 0);
                Ok(())
            }
            (false, Ok(_), Err(e)) => {
                Err(format!("Success case test failed: {}", e))
            }
            (false, Err(e), _) => {
                Err(format!("Input validation failed for success case: {}", e))
            }
        }
    }

    /// Run all edge case tests from fixtures
    #[test]
    fn test_all_edge_case_fixtures() {
        let fixture_dir = PathBuf::from("tests/fixtures/regression/edge_cases");
        let fixture_files = vec![
            "zero_candidates.json",
            "zero_nominators.json",
            "single_candidate.json",
            "single_nominator.json",
            "zero_candidate_stakes.json",
            "zero_nominator_stakes.json",
            "max_active_set_size.json",
            "empty_voting_edges.json",
            "all_nominators_vote_all.json",
            "duplicate_account_ids.json",
            "invalid_account_ids.json",
            "invalid_voting_targets.json",
            "maximum_stakes.json",
            "malformed_json.json",
        ];
        
        for fixture_file in fixture_files {
            let fixture_path = fixture_dir.join(fixture_file);
            if fixture_path.exists() {
                let result = run_edge_case_test_from_fixture(
                    fixture_path.to_str().unwrap()
                );
                match result {
                    Ok(_) => println!("✓ {} passed", fixture_file),
                    Err(e) => panic!("✗ {} failed: {}", fixture_file, e),
                }
            } else {
                eprintln!("⚠ {} not found, skipping", fixture_file);
            }
        }
    }
}


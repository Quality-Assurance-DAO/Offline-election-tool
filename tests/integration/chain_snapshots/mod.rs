//! Chain snapshot integration tests
//!
//! This module contains tests that validate election accuracy by comparing
//! simulation results against actual on-chain election outcomes from historical blocks.
//!
//! Tests are marked with #[ignore] by default and require network access.
//! Run with: `cargo test -- --ignored`

mod test_polkadot;
mod test_kusama;
mod test_westend;

pub use test_polkadot::*;
pub use test_kusama::*;
pub use test_westend::*;

use offline_election::engine::ElectionEngine;
use offline_election::models::election_config::ElectionConfiguration;
use offline_election::types::AlgorithmType;
use crate::common::fixture_loader::load_chain_snapshot;
use crate::common::assertions::compare_results_exact_match;
use crate::common::rpc_retry::retry_with_backoff;
use std::path::PathBuf;

/// Test result indicating whether a test passed, failed, or was skipped
#[derive(Debug, Clone)]
pub enum TestResult {
    Passed,
    Failed(String),
    Skipped(String),
}

/// Run a chain snapshot test from a fixture file
/// 
/// Loads a chain snapshot fixture, executes the election, and compares
/// results to expected on-chain outcomes.
pub async fn run_chain_snapshot_test_from_fixture(
    fixture_path: &str,
) -> TestResult {
    let path = PathBuf::from(fixture_path);
    
    // Load snapshot
    let snapshot = match load_chain_snapshot(&path) {
        Ok(s) => s,
        Err(e) => {
            return TestResult::Skipped(format!("Failed to load snapshot: {}", e));
        }
    };
    
    let engine = ElectionEngine::new();
    let config = ElectionConfiguration {
        active_set_size: snapshot.expected_result.selected_validators.len() as u32,
        algorithm: AlgorithmType::SequentialPhragmen,
        overrides: None,
        block_number: Some(snapshot.metadata.block_number),
    };
    
    // Execute election
    let result = match engine.execute(&config, &snapshot.election_data) {
        Ok(r) => r,
        Err(e) => {
            return TestResult::Failed(format!("Election execution failed: {}", e));
        }
    };
    
    // Compare results
    match compare_results_exact_match(&result, &snapshot.expected_result) {
        Ok(_) => TestResult::Passed,
        Err(e) => TestResult::Failed(format!("Result mismatch: {}", e)),
    }
}

/// Fetch chain snapshot from RPC with retry logic
pub async fn fetch_chain_snapshot_with_retry(
    rpc_endpoint: &str,
    block_number: u64,
    chain: &str,
) -> Result<crate::common::models::ChainSnapshot, String> {
    retry_with_backoff(
        || async {
            crate::common::rpc_utils::fetch_chain_snapshot(rpc_endpoint, block_number).await
        },
        3, // max attempts
        std::time::Duration::from_secs(1), // initial delay
    )
    .await
    .map_err(|e| format!("Failed to fetch chain snapshot for {} block {} after retries: {}", chain, block_number, e))
}


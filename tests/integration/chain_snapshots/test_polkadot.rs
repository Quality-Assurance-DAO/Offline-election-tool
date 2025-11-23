//! Chain snapshot tests for Polkadot

use super::*;
use offline_election::engine::ElectionEngine;
use offline_election::models::election_config::ElectionConfiguration;
use offline_election::types::AlgorithmType;
use crate::common::fixture_loader::load_chain_snapshot;
use crate::common::assertions::compare_results_exact_match;
use std::path::PathBuf;

const POLKADOT_RPC: &str = "https://rpc.polkadot.io";

#[test]
#[ignore] // Requires network access
async fn test_polkadot_block_1() {
    let fixture_path = PathBuf::from("tests/fixtures/chain_snapshots/polkadot/block_1.json");
    
    if !fixture_path.exists() {
        eprintln!("⚠ Fixture not found: {:?}. Skipping test.", fixture_path);
        return;
    }
    
    let snapshot = load_chain_snapshot(&fixture_path)
        .expect("Failed to load chain snapshot");
    
    let engine = ElectionEngine::new();
    let config = ElectionConfiguration {
        active_set_size: snapshot.expected_result.selected_validators.len() as u32,
        algorithm: AlgorithmType::SequentialPhragmen,
        overrides: None,
        block_number: Some(snapshot.metadata.block_number),
    };
    
    let result = engine.execute(&config, &snapshot.election_data)
        .expect("Election execution should succeed");
    
    compare_results_exact_match(&result, &snapshot.expected_result)
        .expect("Results should match exactly");
}

#[test]
#[ignore] // Requires network access
async fn test_polkadot_block_2() {
    let fixture_path = PathBuf::from("tests/fixtures/chain_snapshots/polkadot/block_2.json");
    
    if !fixture_path.exists() {
        eprintln!("⚠ Fixture not found: {:?}. Skipping test.", fixture_path);
        return;
    }
    
    let snapshot = load_chain_snapshot(&fixture_path)
        .expect("Failed to load chain snapshot");
    
    let engine = ElectionEngine::new();
    let config = ElectionConfiguration {
        active_set_size: snapshot.expected_result.selected_validators.len() as u32,
        algorithm: AlgorithmType::SequentialPhragmen,
        overrides: None,
        block_number: Some(snapshot.metadata.block_number),
    };
    
    let result = engine.execute(&config, &snapshot.election_data)
        .expect("Election execution should succeed");
    
    compare_results_exact_match(&result, &snapshot.expected_result)
        .expect("Results should match exactly");
}

#[test]
#[ignore] // Requires network access
async fn test_polkadot_block_3() {
    let fixture_path = PathBuf::from("tests/fixtures/chain_snapshots/polkadot/block_3.json");
    
    if !fixture_path.exists() {
        eprintln!("⚠ Fixture not found: {:?}. Skipping test.", fixture_path);
        return;
    }
    
    let snapshot = load_chain_snapshot(&fixture_path)
        .expect("Failed to load chain snapshot");
    
    let engine = ElectionEngine::new();
    let config = ElectionConfiguration {
        active_set_size: snapshot.expected_result.selected_validators.len() as u32,
        algorithm: AlgorithmType::SequentialPhragmen,
        overrides: None,
        block_number: Some(snapshot.metadata.block_number),
    };
    
    let result = engine.execute(&config, &snapshot.election_data)
        .expect("Election execution should succeed");
    
    compare_results_exact_match(&result, &snapshot.expected_result)
        .expect("Results should match exactly");
}

#[test]
#[ignore] // Requires network access
async fn test_polkadot_block_4() {
    let fixture_path = PathBuf::from("tests/fixtures/chain_snapshots/polkadot/block_4.json");
    
    if !fixture_path.exists() {
        eprintln!("⚠ Fixture not found: {:?}. Skipping test.", fixture_path);
        return;
    }
    
    let snapshot = load_chain_snapshot(&fixture_path)
        .expect("Failed to load chain snapshot");
    
    let engine = ElectionEngine::new();
    let config = ElectionConfiguration {
        active_set_size: snapshot.expected_result.selected_validators.len() as u32,
        algorithm: AlgorithmType::SequentialPhragmen,
        overrides: None,
        block_number: Some(snapshot.metadata.block_number),
    };
    
    let result = engine.execute(&config, &snapshot.election_data)
        .expect("Election execution should succeed");
    
    compare_results_exact_match(&result, &snapshot.expected_result)
        .expect("Results should match exactly");
}


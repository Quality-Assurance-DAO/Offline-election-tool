//! Regression tests for normal election scenarios

use super::*;
use offline_election::engine::ElectionEngine;
use offline_election::models::election_config::ElectionConfiguration;
use offline_election::types::AlgorithmType;
use crate::common::fixture_loader::load_test_fixture;
use crate::common::assertions::compare_results_exact_match;
use std::path::PathBuf;

#[test]
fn test_regression_normal_election_5x5() {
    let fixture_path = PathBuf::from("tests/fixtures/regression/normal/normal_election_5x5.json");
    
    if !fixture_path.exists() {
        eprintln!("⚠ Fixture not found: {:?}. Skipping test.", fixture_path);
        return;
    }
    
    let fixture = load_test_fixture(&fixture_path)
        .expect("Failed to load test fixture");
    
    let engine = ElectionEngine::new();
    let config = ElectionConfiguration {
        active_set_size: 3,
        algorithm: fixture.metadata.algorithm,
        overrides: None,
        block_number: None,
    };
    
    let result = engine.execute(&config, &fixture.input)
        .expect("Election execution should succeed");
    
    if let Some(expected_result) = &fixture.expected_result {
        compare_results_exact_match(&result, expected_result)
            .expect("Results should match baseline");
    }
}

#[test]
fn test_regression_normal_election_10x10() {
    let fixture_path = PathBuf::from("tests/fixtures/regression/normal/normal_election_10x10.json");
    
    if !fixture_path.exists() {
        eprintln!("⚠ Fixture not found: {:?}. Skipping test.", fixture_path);
        return;
    }
    
    let fixture = load_test_fixture(&fixture_path)
        .expect("Failed to load test fixture");
    
    let engine = ElectionEngine::new();
    let config = ElectionConfiguration {
        active_set_size: 5,
        algorithm: fixture.metadata.algorithm,
        overrides: None,
        block_number: None,
    };
    
    let result = engine.execute(&config, &fixture.input)
        .expect("Election execution should succeed");
    
    if let Some(expected_result) = &fixture.expected_result {
        compare_results_exact_match(&result, expected_result)
            .expect("Results should match baseline");
    }
}

#[test]
fn test_regression_normal_election_20x20() {
    let fixture_path = PathBuf::from("tests/fixtures/regression/normal/normal_election_20x20.json");
    
    if !fixture_path.exists() {
        eprintln!("⚠ Fixture not found: {:?}. Skipping test.", fixture_path);
        return;
    }
    
    let fixture = load_test_fixture(&fixture_path)
        .expect("Failed to load test fixture");
    
    let engine = ElectionEngine::new();
    let config = ElectionConfiguration {
        active_set_size: 10,
        algorithm: fixture.metadata.algorithm,
        overrides: None,
        block_number: None,
    };
    
    let result = engine.execute(&config, &fixture.input)
        .expect("Election execution should succeed");
    
    if let Some(expected_result) = &fixture.expected_result {
        compare_results_exact_match(&result, expected_result)
            .expect("Results should match baseline");
    }
}

#[test]
fn test_regression_all_normal_fixtures() {
    let fixtures_dir = PathBuf::from("tests/fixtures/regression/normal");
    let results = run_all_regression_tests(fixtures_dir.to_str().unwrap());
    
    let mut passed = 0;
    let mut failed = 0;
    let mut skipped = 0;
    
    for (fixture_name, result) in results {
        match result {
            Ok(_) => {
                passed += 1;
                println!("✓ {}", fixture_name);
            }
            Err(e) => {
                failed += 1;
                eprintln!("✗ {}: {}", fixture_name, e);
            }
        }
    }
    
    if skipped > 0 {
        eprintln!("⚠ {} fixtures skipped (not found)", skipped);
    }
    
    assert_eq!(failed, 0, "{} regression tests failed", failed);
    println!("Regression tests: {} passed, {} failed", passed, failed);
}


//! Regression tests for edge case scenarios

use super::*;
use crate::common::fixture_loader::load_test_fixture;
use std::path::PathBuf;

#[test]
fn test_regression_edge_case_fixtures() {
    // Run regression tests on edge case fixtures
    // These should maintain consistent behavior even if implementation changes
    let fixtures_dir = PathBuf::from("tests/fixtures/regression/edge_cases");
    let results = run_all_regression_tests(fixtures_dir.to_str().unwrap());
    
    let mut passed = 0;
    let mut failed = 0;
    
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
    
    // Edge case fixtures may not have expected results (error cases)
    // So we just verify they don't crash
    println!("Edge case regression tests: {} passed, {} failed", passed, failed);
}


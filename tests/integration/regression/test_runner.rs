//! Regression test runner

use offline_election::engine::ElectionEngine;
use offline_election::models::election_config::ElectionConfiguration;
use offline_election::types::AlgorithmType;
use crate::common::fixture_loader::load_test_fixture;
use crate::common::assertions::{compare_results_exact_match, assert_results_match_baseline};
use std::path::PathBuf;

/// Run a regression test from a fixture file
/// 
/// Loads a test fixture, executes the election, and compares results
/// to expected baseline results. Fails if results differ.
pub fn run_regression_test_from_fixture(fixture_path: &str) -> Result<(), String> {
    let path = PathBuf::from(fixture_path);
    let fixture = load_test_fixture(&path)?;
    
    let engine = ElectionEngine::new();
    let config = ElectionConfiguration {
        active_set_size: 100, // Default, may be overridden by fixture
        algorithm: fixture.metadata.algorithm,
        overrides: None,
        block_number: None,
    };
    
    // Execute election
    let result = engine.execute(&config, &fixture.input)
        .map_err(|e| format!("Election execution failed: {}", e))?;
    
    // Compare to expected result if provided
    if let Some(expected_result) = &fixture.expected_result {
        compare_results_exact_match(&result, expected_result)
            .map_err(|e| format!("Result mismatch: {}", e))?;
    }
    
    Ok(())
}

/// Run all regression tests from fixtures directory
pub fn run_all_regression_tests(fixtures_dir: &str) -> Vec<(String, Result<(), String>)> {
    let mut results = Vec::new();
    let dir = PathBuf::from(fixtures_dir);
    
    if !dir.exists() {
        eprintln!("⚠ Regression fixtures directory not found: {:?}", dir);
        return results;
    }
    
    // Find all JSON fixture files
    let entries = std::fs::read_dir(&dir)
        .unwrap_or_else(|e| {
            eprintln!("⚠ Failed to read fixtures directory: {}", e);
            return std::fs::read_dir(std::path::Path::new(".")).unwrap();
        });
    
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let fixture_name = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string();
            
            let result = run_regression_test_from_fixture(
                path.to_str().unwrap()
            );
            
            results.push((fixture_name, result));
        }
    }
    
    results
}

/// Track baseline results for regression detection
pub struct BaselineTracker {
    baseline_dir: PathBuf,
}

impl BaselineTracker {
    pub fn new(baseline_dir: &str) -> Self {
        Self {
            baseline_dir: PathBuf::from(baseline_dir),
        }
    }
    
    /// Save baseline result for a fixture
    pub fn save_baseline(&self, fixture_name: &str, result: &offline_election::models::ElectionResult) -> Result<(), String> {
        std::fs::create_dir_all(&self.baseline_dir)
            .map_err(|e| format!("Failed to create baseline directory: {}", e))?;
        
        let baseline_path = self.baseline_dir.join(format!("{}.json", fixture_name));
        let json = serde_json::to_string_pretty(result)
            .map_err(|e| format!("Failed to serialize baseline: {}", e))?;
        
        std::fs::write(&baseline_path, json)
            .map_err(|e| format!("Failed to write baseline: {}", e))?;
        
        Ok(())
    }
    
    /// Load baseline result for a fixture
    pub fn load_baseline(&self, fixture_name: &str) -> Result<offline_election::models::ElectionResult, String> {
        let baseline_path = self.baseline_dir.join(format!("{}.json", fixture_name));
        let content = std::fs::read_to_string(&baseline_path)
            .map_err(|e| format!("Failed to read baseline: {}", e))?;
        
        serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse baseline: {}", e))
    }
}


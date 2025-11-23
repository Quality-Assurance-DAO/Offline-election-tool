//! Test fixture loading utilities

use crate::common::models::{ChainSnapshot, TestFixture};
use serde_json;
use std::fs;
use std::path::Path;

/// Load a test fixture from a JSON file
pub fn load_test_fixture<P: AsRef<Path>>(path: P) -> Result<TestFixture, String> {
    let content = fs::read_to_string(path.as_ref())
        .map_err(|e| format!("Failed to read fixture file {:?}: {}", path.as_ref(), e))?;
    
    let fixture: TestFixture = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse fixture JSON: {}", e))?;
    
    validate_fixture_schema(&fixture)?;
    
    Ok(fixture)
}

/// Load a chain snapshot from a JSON file
pub fn load_chain_snapshot<P: AsRef<Path>>(path: P) -> Result<ChainSnapshot, String> {
    let content = fs::read_to_string(path.as_ref())
        .map_err(|e| format!("Failed to read chain snapshot file {:?}: {}", path.as_ref(), e))?;
    
    let snapshot: ChainSnapshot = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse chain snapshot JSON: {}", e))?;
    
    Ok(snapshot)
}

/// Load a regression test fixture from a JSON file
/// 
/// This is a convenience alias for `load_test_fixture()` specifically for regression tests.
pub fn load_regression_fixture<P: AsRef<Path>>(path: P) -> Result<TestFixture, String> {
    load_test_fixture(path)
}

/// Validate fixture schema
pub fn validate_fixture_schema(fixture: &TestFixture) -> Result<(), String> {
    // Validate metadata
    if fixture.metadata.test_name.is_empty() {
        return Err("test_name must be non-empty".to_string());
    }
    
    if fixture.metadata.description.is_empty() {
        return Err("description must be non-empty".to_string());
    }
    
    // Validate input data
    // Check for duplicate candidate account IDs
    let mut candidate_ids = std::collections::HashSet::new();
    for candidate in &fixture.input.candidates {
        if !candidate_ids.insert(&candidate.account_id) {
            return Err(format!("Duplicate candidate account ID: {}", candidate.account_id));
        }
    }
    
    // Check for duplicate nominator account IDs
    let mut nominator_ids = std::collections::HashSet::new();
    for nominator in &fixture.input.nominators {
        if !nominator_ids.insert(&nominator.account_id) {
            return Err(format!("Duplicate nominator account ID: {}", nominator.account_id));
        }
    }
    
    // Check that voting targets reference existing candidates
    let candidate_id_set: std::collections::HashSet<&String> = 
        fixture.input.candidates.iter().map(|c| &c.account_id).collect();
    
    for nominator in &fixture.input.nominators {
        for target in &nominator.targets {
            if !candidate_id_set.contains(target) {
                return Err(format!(
                    "Nominator '{}' votes for non-existent candidate '{}'",
                    nominator.account_id, target
                ));
            }
        }
    }
    
    Ok(())
}


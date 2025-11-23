//! Edge case test: malformed JSON structure

use std::fs;
use std::path::PathBuf;
use offline_election::models::election_data::ElectionData;
use serde_json;

#[test]
fn test_load_malformed_json_missing_candidates() {
    // Create a temporary JSON file with missing candidates field
    let temp_dir = std::env::temp_dir();
    let temp_file = temp_dir.join("test_malformed_missing_candidates.json");
    
    let malformed_json = r#"
    {
        "nominators": [
            {
                "account_id": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
                "stake": 1000000000,
                "targets": []
            }
        ]
    }
    "#;
    
    fs::write(&temp_file, malformed_json).unwrap();
    
    // Try to deserialize - should fail or handle gracefully
    let file_content = fs::read_to_string(&temp_file).unwrap();
    let result: Result<ElectionData, _> = serde_json::from_str(&file_content);
    
    // Deserialization should fail for malformed JSON
    assert!(result.is_err(), "Malformed JSON with missing candidates should fail to deserialize");
    
    // Cleanup
    let _ = fs::remove_file(&temp_file);
}

#[test]
fn test_load_malformed_json_invalid_structure() {
    // Create a temporary JSON file with invalid structure
    let temp_dir = std::env::temp_dir();
    let temp_file = temp_dir.join("test_malformed_invalid_structure.json");
    
    let malformed_json = r#"
    {
        "candidates": "not_an_array",
        "nominators": null
    }
    "#;
    
    fs::write(&temp_file, malformed_json).unwrap();
    
    let file_content = fs::read_to_string(&temp_file).unwrap();
    let result: Result<ElectionData, _> = serde_json::from_str(&file_content);
    
    assert!(result.is_err(), "Malformed JSON with invalid structure should fail to deserialize");
    
    // Cleanup
    let _ = fs::remove_file(&temp_file);
}

#[test]
fn test_load_malformed_json_missing_required_fields() {
    // Create a temporary JSON file with missing required fields
    let temp_dir = std::env::temp_dir();
    let temp_file = temp_dir.join("test_malformed_missing_fields.json");
    
    let malformed_json = r#"
    {
        "candidates": [
            {
                "account_id": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
            }
        ],
        "nominators": []
    }
    "#;
    
    fs::write(&temp_file, malformed_json).unwrap();
    
    let file_content = fs::read_to_string(&temp_file).unwrap();
    let result: Result<ElectionData, _> = serde_json::from_str(&file_content);
    
    // Missing stake field should cause deserialization to fail
    assert!(result.is_err(), "Malformed JSON with missing required fields should fail to deserialize");
    
    // Cleanup
    let _ = fs::remove_file(&temp_file);
}

#[test]
fn test_load_malformed_json_invalid_json_syntax() {
    // Create a temporary JSON file with invalid JSON syntax
    let temp_dir = std::env::temp_dir();
    let temp_file = temp_dir.join("test_malformed_syntax.json");
    
    let malformed_json = r#"
    {
        "candidates": [
            {
                "account_id": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
                "stake": 1000000000
            }
        ],
        "nominators": [
            {
                "account_id": "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
                "stake": 1000000000,
                "targets": []
            }
        ]
        // Missing closing brace
    "#;
    
    fs::write(&temp_file, malformed_json).unwrap();
    
    let file_content = fs::read_to_string(&temp_file).unwrap();
    let result: Result<ElectionData, _> = serde_json::from_str(&file_content);
    
    assert!(result.is_err(), "Malformed JSON with invalid syntax should fail to deserialize");
    
    // Cleanup
    let _ = fs::remove_file(&temp_file);
}


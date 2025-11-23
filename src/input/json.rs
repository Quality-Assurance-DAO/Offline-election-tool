//! JSON input loader for loading election data from JSON files

use crate::error::ElectionError;
use crate::models::election_data::ElectionData;
use std::path::PathBuf;

/// JSON loader for loading election data from files
pub struct JsonLoader;

impl JsonLoader {
    /// Create a new JSON loader
    pub fn new() -> Self {
        Self
    }

    /// Load election data from a JSON file
    pub fn load_from_file(&self, path: PathBuf) -> Result<ElectionData, ElectionError> {
        let content = std::fs::read_to_string(&path).map_err(|e| ElectionError::FileError {
            message: format!("Failed to read file: {}", e),
            path: path.clone(),
        })?;

        let data: ElectionData = serde_json::from_str(&content).map_err(|e| ElectionError::FileError {
            message: format!("Failed to parse JSON: {}", e),
            path: path.clone(),
        })?;

        // Validate the loaded data
        data.validate()?;

        Ok(data)
    }
}

impl Default for JsonLoader {
    fn default() -> Self {
        Self::new()
    }
}

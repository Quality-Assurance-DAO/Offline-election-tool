//! REST API server
//! Will be implemented in Phase 8 (T111-T116)

use crate::error::ElectionError;

/// REST API server for election operations
pub struct ApiServer {
    // Implementation will be added in Phase 8 (T111-T116)
}

impl ApiServer {
    /// Create a new API server
    pub fn new() -> Self {
        Self {}
    }

    /// Start the server
    pub async fn start(&self, _port: u16) -> Result<(), ElectionError> {
        // Implementation will be added in Phase 8 (T111-T116)
        Err(ElectionError::InvalidData {
            message: "API server not yet implemented".to_string(),
        })
    }
}

impl Default for ApiServer {
    fn default() -> Self {
        Self::new()
    }
}


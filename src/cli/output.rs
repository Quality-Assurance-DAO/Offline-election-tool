//! CLI output formatting
//! Will be implemented in Phase 3 (T052-T053)

use crate::models::election_result::ElectionResult;

/// Format election result as JSON
pub fn format_json(result: &ElectionResult) -> Result<String, crate::error::ElectionError> {
    // Implementation will be added in Phase 3 (T052-T053)
    result.to_json()
}



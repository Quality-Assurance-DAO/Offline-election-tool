//! Election algorithm trait definition

use crate::error::ElectionError;
use crate::models::election_config::ElectionConfiguration;
use crate::models::election_data::ElectionData;
use crate::models::election_result::ElectionResult;

/// Trait for election algorithm implementations
pub trait ElectionAlgorithm {
    /// Execute the election algorithm with the given data and configuration
    fn execute(
        &self,
        data: &ElectionData,
        config: &ElectionConfiguration,
    ) -> Result<ElectionResult, ElectionError>;

    /// Get the name of the algorithm
    fn name(&self) -> &'static str;
}



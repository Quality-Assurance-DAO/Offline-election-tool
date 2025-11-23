//! Election configuration model

use crate::error::ElectionError;
use crate::models::election_overrides::ElectionOverrides;
use crate::types::AlgorithmType;
use serde::{Deserialize, Serialize};

/// Configuration for how an election should be executed
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ElectionConfiguration {
    /// Election algorithm to use
    pub algorithm: AlgorithmType,
    /// Number of validators to select (must be positive)
    pub active_set_size: u32,
    /// Optional parameter overrides
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrides: Option<ElectionOverrides>,
    /// Optional block number for RPC snapshot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_number: Option<u64>,
}

impl ElectionConfiguration {
    /// Create a new election configuration
    pub fn new() -> Self {
        Self {
            algorithm: AlgorithmType::SequentialPhragmen,
            active_set_size: 100,
            overrides: None,
            block_number: None,
        }
    }

    /// Set the algorithm
    pub fn algorithm(mut self, algorithm: AlgorithmType) -> Self {
        self.algorithm = algorithm;
        self
    }

    /// Set the active set size
    pub fn active_set_size(mut self, size: u32) -> Self {
        self.active_set_size = size;
        self
    }

    /// Set parameter overrides
    pub fn overrides(mut self, overrides: ElectionOverrides) -> Self {
        self.overrides = Some(overrides);
        self
    }

    /// Set block number
    pub fn block_number(mut self, block: u64) -> Self {
        self.block_number = Some(block);
        self
    }

    /// Build and validate the configuration
    pub fn build(self) -> Result<Self, ElectionError> {
        self.validate()?;
        Ok(self)
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<(), ElectionError> {
        // Active set size must be positive
        if self.active_set_size == 0 {
            return Err(ElectionError::ValidationError {
                message: "Active set size must be positive".to_string(),
                field: Some("active_set_size".to_string()),
            });
        }

        Ok(())
    }

    /// Validate that active set size doesn't exceed available candidates
    pub fn validate_against_data(&self, candidate_count: usize) -> Result<(), ElectionError> {
        if self.active_set_size as usize > candidate_count {
            return Err(ElectionError::InsufficientCandidates {
                requested: self.active_set_size,
                available: candidate_count as u32,
            });
        }
        Ok(())
    }
}

impl Default for ElectionConfiguration {
    fn default() -> Self {
        Self::new()
    }
}



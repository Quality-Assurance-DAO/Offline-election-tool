//! Election result model

use crate::types::AlgorithmType;
use serde::{Deserialize, Serialize};

/// Outcome of an election execution
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ElectionResult {
    /// List of validators selected for the active set
    pub selected_validators: Vec<SelectedValidator>,
    /// How nominator stakes are allocated across validators
    pub stake_distribution: Vec<StakeAllocation>,
    /// Total stake participating in election
    pub total_stake: u128,
    /// Algorithm that produced these results
    pub algorithm_used: AlgorithmType,
    /// Execution metadata (timing, block number, etc.)
    pub execution_metadata: ExecutionMetadata,
}

/// Validator that was selected in the election
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SelectedValidator {
    /// Account ID of the selected validator
    pub account_id: String,
    /// Total stake backing this validator
    pub total_backing_stake: u128,
    /// Number of nominators backing this validator
    pub nominator_count: u32,
    /// Optional rank/position in the active set
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<u32>,
}

/// How a nominator's stake is allocated to a validator
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StakeAllocation {
    /// Account ID of the nominator
    pub nominator_id: String,
    /// Account ID of the validator receiving stake
    pub validator_id: String,
    /// Amount of stake allocated
    pub amount: u128,
    /// Proportion of nominator's total stake (0.0 to 1.0)
    pub proportion: f64,
}

/// Execution metadata
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionMetadata {
    /// Block number if data came from RPC
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_number: Option<u64>,
    /// Execution timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_timestamp: Option<String>,
    /// Data source identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<String>,
}

impl ElectionResult {
    /// Create a new election result
    pub fn new(
        selected_validators: Vec<SelectedValidator>,
        stake_distribution: Vec<StakeAllocation>,
        total_stake: u128,
        algorithm_used: AlgorithmType,
    ) -> Self {
        Self {
            selected_validators,
            stake_distribution,
            total_stake,
            algorithm_used,
            execution_metadata: ExecutionMetadata {
                block_number: None,
                execution_timestamp: None,
                data_source: None,
            },
        }
    }

    /// Get reference to selected validators
    pub fn selected_validators(&self) -> &[SelectedValidator] {
        &self.selected_validators
    }

    /// Get reference to stake distribution
    pub fn stake_distribution(&self) -> &[StakeAllocation] {
        &self.stake_distribution
    }

    /// Get total stake
    pub fn total_stake(&self) -> u128 {
        self.total_stake
    }

    /// Get algorithm used
    pub fn algorithm_used(&self) -> AlgorithmType {
        self.algorithm_used
    }

    /// Convert result to JSON string
    pub fn to_json(&self) -> Result<String, crate::error::ElectionError> {
        serde_json::to_string_pretty(self).map_err(|e| crate::error::ElectionError::InvalidData {
            message: format!("Failed to serialize result to JSON: {}", e),
        })
    }
}


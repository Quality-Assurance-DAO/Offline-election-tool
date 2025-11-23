//! Diagnostics data models

use serde::{Deserialize, Serialize};

/// Detailed diagnostics explaining election results
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Diagnostics {
    /// Explanations for each validator
    pub validator_explanations: Vec<ValidatorExplanation>,
    /// Stake distribution analysis
    pub stake_analysis: StakeAnalysis,
    /// Algorithm-specific insights
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm_insights: Option<serde_json::Value>,
    /// Warnings or notable conditions
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<String>,
}

/// Explanation for why a validator was selected or not selected
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidatorExplanation {
    /// Account ID of the validator
    pub account_id: String,
    /// Whether this validator was selected
    pub selected: bool,
    /// Human-readable explanation
    pub reason: String,
    /// Key factors that influenced selection
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub key_factors: Vec<String>,
}

/// Analysis of stake distribution
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StakeAnalysis {
    /// Total stake
    pub total_stake: u128,
    /// Average stake per validator
    pub average_stake_per_validator: u128,
}

impl Diagnostics {
    /// Get validator explanations
    pub fn validator_explanations(&self) -> &[ValidatorExplanation] {
        &self.validator_explanations
    }

    /// Get stake analysis
    pub fn stake_analysis(&self) -> &StakeAnalysis {
        &self.stake_analysis
    }
}



//! Validator candidate model

use serde::{Deserialize, Serialize};

/// Validator candidate in an election
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidatorCandidate {
    /// SS58-encoded account identifier (must be unique)
    pub account_id: String,
    /// Total stake amount (can be zero or overridden)
    pub stake: u128,
    /// Optional metadata (e.g., commission rate, on-chain status)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<CandidateMetadata>,
}

/// Metadata for a validator candidate
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CandidateMetadata {
    /// Commission rate (0-100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commission_rate: Option<u8>,
    /// On-chain status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_chain_status: Option<String>,
}

impl ValidatorCandidate {
    /// Create a new validator candidate
    pub fn new(account_id: String, stake: u128) -> Self {
        Self {
            account_id,
            stake,
            metadata: None,
        }
    }

    /// Create a validator candidate with metadata
    pub fn with_metadata(
        account_id: String,
        stake: u128,
        metadata: CandidateMetadata,
    ) -> Self {
        Self {
            account_id,
            stake,
            metadata: Some(metadata),
        }
    }
}



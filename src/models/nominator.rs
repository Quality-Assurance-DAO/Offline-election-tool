//! Nominator model

use serde::{Deserialize, Serialize};

/// Nominator in an election
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Nominator {
    /// SS58-encoded account identifier (must be unique)
    pub account_id: String,
    /// Total stake amount available for voting
    pub stake: u128,
    /// List of candidate account IDs this nominator votes for
    pub targets: Vec<String>,
    /// Optional metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<NominatorMetadata>,
}

/// Metadata for a nominator
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NominatorMetadata {
    /// Additional metadata fields as needed
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Nominator {
    /// Create a new nominator
    pub fn new(account_id: String, stake: u128) -> Self {
        Self {
            account_id,
            stake,
            targets: Vec::new(),
            metadata: None,
        }
    }

    /// Add a target candidate to vote for
    pub fn add_target(&mut self, candidate_id: String) {
        if !self.targets.contains(&candidate_id) {
            self.targets.push(candidate_id);
        }
    }

    /// Remove a target candidate
    pub fn remove_target(&mut self, candidate_id: &str) {
        self.targets.retain(|id| id != candidate_id);
    }
}



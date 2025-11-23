//! Voting edge model representing a nominator's vote for a candidate

use serde::{Deserialize, Serialize};

/// Voting edge representing a nominator's preference to vote for a validator candidate
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VotingEdge {
    /// Account ID of the nominator
    pub nominator_id: String,
    /// Account ID of the candidate being voted for
    pub candidate_id: String,
    /// Optional explicit weight (if None, uses nominator's stake proportionally)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<u128>,
}

impl VotingEdge {
    /// Create a new voting edge
    pub fn new(nominator_id: String, candidate_id: String) -> Self {
        Self {
            nominator_id,
            candidate_id,
            weight: None,
        }
    }

    /// Create a voting edge with explicit weight
    pub fn with_weight(nominator_id: String, candidate_id: String, weight: u128) -> Self {
        Self {
            nominator_id,
            candidate_id,
            weight: Some(weight),
        }
    }
}



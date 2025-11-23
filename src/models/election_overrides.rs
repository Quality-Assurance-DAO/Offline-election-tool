//! Election parameter overrides model

use crate::error::ElectionError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Optional parameter overrides that modify election data before execution
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ElectionOverrides {
    /// Override stake for specific candidates (account_id -> stake)
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub candidate_stakes: HashMap<String, u128>,
    /// Override stake for specific nominators (account_id -> stake)
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub nominator_stakes: HashMap<String, u128>,
    /// Voting edge modifications
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub voting_edges: Vec<EdgeModification>,
    /// Override active set size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_set_size: Option<u32>,
}

/// Modification to a voting edge
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EdgeModification {
    /// Action to perform
    pub action: EdgeAction,
    /// Nominator account ID
    pub nominator_id: String,
    /// Candidate account ID
    pub candidate_id: String,
    /// Optional weight for the edge
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<u128>,
}

/// Action to perform on a voting edge
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EdgeAction {
    /// Add a new voting edge
    Add,
    /// Remove an existing voting edge
    Remove,
    /// Modify an existing voting edge
    Modify,
}

impl ElectionOverrides {
    /// Create a new empty overrides structure
    pub fn new() -> Self {
        Self::default()
    }

    /// Set candidate stake override
    pub fn set_candidate_stake(
        &mut self,
        account_id: String,
        stake: u128,
    ) -> Result<(), ElectionError> {
        self.candidate_stakes.insert(account_id, stake);
        Ok(())
    }

    /// Set nominator stake override
    pub fn set_nominator_stake(
        &mut self,
        account_id: String,
        stake: u128,
    ) -> Result<(), ElectionError> {
        self.nominator_stakes.insert(account_id, stake);
        Ok(())
    }

    /// Add a voting edge
    pub fn add_voting_edge(
        &mut self,
        nominator_id: String,
        candidate_id: String,
    ) -> Result<(), ElectionError> {
        self.voting_edges.push(EdgeModification {
            action: EdgeAction::Add,
            nominator_id,
            candidate_id,
            weight: None,
        });
        Ok(())
    }

    /// Remove a voting edge
    pub fn remove_voting_edge(
        &mut self,
        nominator_id: String,
        candidate_id: String,
    ) -> Result<(), ElectionError> {
        self.voting_edges.push(EdgeModification {
            action: EdgeAction::Remove,
            nominator_id,
            candidate_id,
            weight: None,
        });
        Ok(())
    }

    /// Modify a voting edge
    pub fn modify_voting_edge(
        &mut self,
        nominator_id: String,
        candidate_id: String,
        weight: Option<u128>,
    ) -> Result<(), ElectionError> {
        self.voting_edges.push(EdgeModification {
            action: EdgeAction::Modify,
            nominator_id,
            candidate_id,
            weight,
        });
        Ok(())
    }
}



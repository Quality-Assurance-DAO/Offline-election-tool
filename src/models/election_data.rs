//! Election data model containing all candidates, nominators, and voting edges

use crate::error::ElectionError;
use crate::models::nominator::Nominator;
use crate::models::validator::ValidatorCandidate;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Complete state needed to run an election
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ElectionData {
    /// List of validator candidates
    pub candidates: Vec<ValidatorCandidate>,
    /// List of nominators with their stakes and votes
    pub nominators: Vec<Nominator>,
    /// Optional metadata about the election data source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ElectionMetadata>,
}

/// Metadata about the election data source
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ElectionMetadata {
    /// Block number if data came from RPC
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_number: Option<u64>,
    /// Chain identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain: Option<String>,
}

impl ElectionData {
    /// Create a new empty election data structure
    pub fn new() -> Self {
        Self {
            candidates: Vec::new(),
            nominators: Vec::new(),
            metadata: None,
        }
    }

    /// Add a validator candidate
    pub fn add_candidate(&mut self, candidate: ValidatorCandidate) -> Result<(), ElectionError> {
        // Check for duplicate account IDs
        if self.candidates.iter().any(|c| c.account_id == candidate.account_id) {
            return Err(ElectionError::ValidationError {
                message: format!("Duplicate candidate account ID: {}", candidate.account_id),
                field: Some("candidates".to_string()),
            });
        }
        self.candidates.push(candidate);
        Ok(())
    }

    /// Add a nominator
    pub fn add_nominator(&mut self, nominator: Nominator) -> Result<(), ElectionError> {
        // Check for duplicate account IDs
        if self.nominators.iter().any(|n| n.account_id == nominator.account_id) {
            return Err(ElectionError::ValidationError {
                message: format!("Duplicate nominator account ID: {}", nominator.account_id),
                field: Some("nominators".to_string()),
            });
        }
        self.nominators.push(nominator);
        Ok(())
    }

    /// Validate election data
    pub fn validate(&self) -> Result<(), ElectionError> {
        // Must contain at least one validator candidate
        if self.candidates.is_empty() {
            return Err(ElectionError::ValidationError {
                message: format!(
                    "Election data must contain at least one validator candidate, but found {}. Please add at least one candidate.",
                    self.candidates.len()
                ),
                field: Some("candidates".to_string()),
            });
        }

        // Nominators are optional - election can run with just validators (no nominator votes)
        // This allows the tool to work when RPC endpoints don't support storage queries

        // All candidate account IDs must be unique
        let mut candidate_ids = HashSet::new();
        for candidate in &self.candidates {
            if !candidate_ids.insert(&candidate.account_id) {
                return Err(ElectionError::ValidationError {
                    message: format!("Duplicate candidate account ID: {}", candidate.account_id),
                    field: Some("candidates".to_string()),
                });
            }
        }

        // All nominator account IDs must be unique
        let mut nominator_ids = HashSet::new();
        for nominator in &self.nominators {
            if !nominator_ids.insert(&nominator.account_id) {
                return Err(ElectionError::ValidationError {
                    message: format!("Duplicate nominator account ID: {}", nominator.account_id),
                    field: Some("nominators".to_string()),
                });
            }
        }

        // All voting edges must reference existing candidates
        let candidate_id_set: HashSet<&String> = self.candidates.iter().map(|c| &c.account_id).collect();
        for nominator in &self.nominators {
            for target in &nominator.targets {
                if !candidate_id_set.contains(target) {
                    let available_candidates: Vec<String> = self.candidates.iter().take(5).map(|c| c.account_id.clone()).collect();
                    let candidate_list = if self.candidates.len() > 5 {
                        format!("{} (and {} more)", available_candidates.join(", "), self.candidates.len() - 5)
                    } else {
                        available_candidates.join(", ")
                    };
                    return Err(ElectionError::ValidationError {
                        message: format!(
                            "Nominator '{}' votes for non-existent candidate '{}'. Available candidates: {}",
                            nominator.account_id, target, candidate_list
                        ),
                        field: Some("nominators.targets".to_string()),
                    });
                }
            }
        }

        Ok(())
    }

    /// Get reference to candidates
    pub fn candidates(&self) -> &[ValidatorCandidate] {
        &self.candidates
    }

    /// Get reference to nominators
    pub fn nominators(&self) -> &[Nominator] {
        &self.nominators
    }

    /// Load election data from an RPC endpoint
    /// 
    /// # Arguments
    /// * `url` - RPC endpoint URL
    /// * `block_number` - Optional block number to snapshot state (None for latest)
    /// 
    /// # Returns
    /// Returns `Ok(ElectionData)` if successful, or `Err` if RPC call fails
    /// 
    /// # Example
    /// ```no_run
    /// use offline_election::ElectionData;
    /// 
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let data = ElectionData::from_rpc(
    ///     "https://rpc.polkadot.io",
    ///     Some(10000000)
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn from_rpc(
        url: &str,
        block_number: Option<u64>,
    ) -> Result<Self, ElectionError> {
        let loader = crate::input::rpc::RpcLoader::new(url)?;
        if let Some(block) = block_number {
            loader.load_at_block(block).await
        } else {
            loader.load_latest().await
        }
    }
}

impl Default for ElectionData {
    fn default() -> Self {
        Self::new()
    }
}


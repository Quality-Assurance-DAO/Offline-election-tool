//! Synthetic data builder for creating election data programmatically
//! Allows creation of candidates and nominators that don't exist on-chain

use crate::error::ElectionError;
use crate::models::election_data::ElectionData;
use crate::models::nominator::Nominator;
use crate::models::validator::ValidatorCandidate;

/// Builder for creating synthetic election data
/// This allows creating accounts that don't exist on-chain or have zero stake
pub struct SyntheticDataBuilder {
    candidates: Vec<(String, u128)>,
    nominators: Vec<(String, u128, Vec<String>)>,
}

impl SyntheticDataBuilder {
    /// Create a new synthetic data builder
    pub fn new() -> Self {
        Self {
            candidates: Vec::new(),
            nominators: Vec::new(),
        }
    }

    /// Add a candidate (validator)
    /// 
    /// # Arguments
    /// * `account_id` - Account identifier (can be any string, doesn't need to exist on-chain)
    /// * `stake` - Stake amount (can be zero)
    /// 
    /// # Returns
    /// Returns `Ok(&mut Self)` for chaining, or `Err` if account_id is duplicate
    pub fn add_candidate(
        &mut self,
        account_id: String,
        stake: u128,
    ) -> Result<&mut Self, ElectionError> {
        // Check for duplicate account IDs
        if self.candidates.iter().any(|(id, _)| id == &account_id) {
            return Err(ElectionError::ValidationError {
                message: format!("Duplicate candidate account ID: {}", account_id),
                field: Some("candidates".to_string()),
            });
        }
        self.candidates.push((account_id, stake));
        Ok(self)
    }

    /// Add a nominator
    /// 
    /// # Arguments
    /// * `account_id` - Account identifier (can be any string, doesn't need to exist on-chain)
    /// * `stake` - Stake amount (can be zero)
    /// * `targets` - List of candidate account IDs to vote for (can be empty)
    /// 
    /// # Returns
    /// Returns `Ok(&mut Self)` for chaining, or `Err` if account_id is duplicate
    pub fn add_nominator(
        &mut self,
        account_id: String,
        stake: u128,
        targets: Vec<String>,
    ) -> Result<&mut Self, ElectionError> {
        // Check for duplicate account IDs
        if self.nominators.iter().any(|(id, _, _)| id == &account_id) {
            return Err(ElectionError::ValidationError {
                message: format!("Duplicate nominator account ID: {}", account_id),
                field: Some("nominators".to_string()),
            });
        }
        self.nominators.push((account_id, stake, targets));
        Ok(self)
    }

    /// Add a voting edge (nominator votes for a candidate)
    /// 
    /// # Arguments
    /// * `nominator_id` - Account ID of the nominator
    /// * `candidate_id` - Account ID of the candidate to vote for
    /// 
    /// # Returns
    /// Returns `Ok(&mut Self)` for chaining, or `Err` if nominator doesn't exist
    pub fn add_voting_edge(
        &mut self,
        nominator_id: String,
        candidate_id: String,
    ) -> Result<&mut Self, ElectionError> {
        // Find the nominator and add the target
        if let Some((_, _, targets)) = self.nominators.iter_mut().find(|(id, _, _)| id == &nominator_id) {
            if !targets.contains(&candidate_id) {
                targets.push(candidate_id);
            }
            Ok(self)
        } else {
            Err(ElectionError::ValidationError {
                message: format!("Nominator not found: {}", nominator_id),
                field: Some("nominators".to_string()),
            })
        }
    }

    /// Build the election data from the collected candidates and nominators
    /// 
    /// # Returns
    /// Returns `Ok(ElectionData)` if valid, or `Err` if validation fails
    pub fn build(&self) -> Result<ElectionData, ElectionError> {
        let mut election_data = ElectionData::new();

        // Add all candidates
        for (account_id, stake) in &self.candidates {
            let candidate = ValidatorCandidate::new(account_id.clone(), *stake);
            election_data.add_candidate(candidate)?;
        }

        // Add all nominators with their targets
        for (account_id, stake, targets) in &self.nominators {
            let mut nominator = Nominator::new(account_id.clone(), *stake);
            for target in targets {
                nominator.add_target(target.clone());
            }
            election_data.add_nominator(nominator)?;
        }

        // Validate the election data (checks for duplicate IDs and valid edges)
        election_data.validate()?;

        Ok(election_data)
    }
}

impl Default for SyntheticDataBuilder {
    fn default() -> Self {
        Self::new()
    }
}



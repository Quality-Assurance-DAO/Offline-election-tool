//! Election engine for executing elections

use crate::algorithms::trait_def::ElectionAlgorithm;
use crate::algorithms::sequential_phragmen::SequentialPhragmen;
use crate::error::ElectionError;
use crate::models::election_config::ElectionConfiguration;
use crate::models::election_data::ElectionData;
use crate::models::election_result::ElectionResult;
use crate::types::AlgorithmType;

/// Election engine for executing elections with various algorithms
pub struct ElectionEngine;

impl ElectionEngine {
    /// Create a new election engine
    pub fn new() -> Self {
        Self
    }

    /// Execute an election with the given configuration and data
    pub fn execute(
        &self,
        config: &ElectionConfiguration,
        data: &ElectionData,
    ) -> Result<ElectionResult, ElectionError> {
        // Validate election data
        data.validate()?;

        // Validate configuration against data
        config.validate_against_data(data.candidates().len())?;

        // Select algorithm based on configuration
        let algorithm: Box<dyn ElectionAlgorithm> = match config.algorithm {
            AlgorithmType::SequentialPhragmen => Box::new(SequentialPhragmen),
            AlgorithmType::ParallelPhragmen => Box::new(crate::algorithms::parallel_phragmen::ParallelPhragmen),
            AlgorithmType::MultiPhase => Box::new(crate::algorithms::multi_phase::MultiPhase),
        };

        // Apply overrides if present
        let mut modified_data = data.clone();
        if let Some(ref overrides) = config.overrides {
            self.apply_overrides(&mut modified_data, overrides)?;
        }

        // Execute algorithm
        let result = algorithm.execute(&modified_data, config)?;

        // Validate result
        self.validate_result(&result, config)?;

        Ok(result)
    }

    /// Apply parameter overrides to election data
    fn apply_overrides(
        &self,
        data: &mut ElectionData,
        overrides: &crate::models::election_overrides::ElectionOverrides,
    ) -> Result<(), ElectionError> {
        // Apply candidate stake overrides
        for (account_id, stake) in &overrides.candidate_stakes {
            if let Some(candidate) = data.candidates.iter_mut().find(|c| c.account_id == *account_id) {
                candidate.stake = *stake;
            }
        }

        // Apply nominator stake overrides
        for (account_id, stake) in &overrides.nominator_stakes {
            if let Some(nominator) = data.nominators.iter_mut().find(|n| n.account_id == *account_id) {
                nominator.stake = *stake;
            }
        }

        // Apply voting edge modifications
        for edge_mod in &overrides.voting_edges {
            match edge_mod.action {
                crate::models::election_overrides::EdgeAction::Add => {
                    if let Some(nominator) = data.nominators.iter_mut().find(|n| n.account_id == edge_mod.nominator_id) {
                        nominator.add_target(edge_mod.candidate_id.clone());
                    }
                }
                crate::models::election_overrides::EdgeAction::Remove => {
                    if let Some(nominator) = data.nominators.iter_mut().find(|n| n.account_id == edge_mod.nominator_id) {
                        nominator.remove_target(&edge_mod.candidate_id);
                    }
                }
                crate::models::election_overrides::EdgeAction::Modify => {
                    // Modify is similar to remove + add
                    if let Some(nominator) = data.nominators.iter_mut().find(|n| n.account_id == edge_mod.nominator_id) {
                        nominator.remove_target(&edge_mod.candidate_id);
                        nominator.add_target(edge_mod.candidate_id.clone());
                    }
                }
            }
        }

        Ok(())
    }

    /// Validate election result
    fn validate_result(
        &self,
        result: &ElectionResult,
        config: &ElectionConfiguration,
    ) -> Result<(), ElectionError> {
        // Check that number of selected validators matches active set size
        if result.selected_validators.len() != config.active_set_size as usize {
            return Err(ElectionError::ValidationError {
                message: format!(
                    "Result has {} validators but expected {}",
                    result.selected_validators.len(),
                    config.active_set_size
                ),
                field: Some("selected_validators".to_string()),
            });
        }

        // Check that total stake matches
        let total_allocated: u128 = result.stake_distribution.iter().map(|a| a.amount).sum();
        if total_allocated != result.total_stake {
            return Err(ElectionError::ValidationError {
                message: format!(
                    "Stake distribution total {} doesn't match total stake {}",
                    total_allocated, result.total_stake
                ),
                field: Some("stake_distribution".to_string()),
            });
        }

        Ok(())
    }
}

impl Default for ElectionEngine {
    fn default() -> Self {
        Self::new()
    }
}



//! Parallel Phragmen algorithm implementation using sp-npos-elections

use crate::algorithms::trait_def::ElectionAlgorithm;
use crate::error::ElectionError;
use crate::models::election_config::ElectionConfiguration;
use crate::models::election_data::ElectionData;
use crate::models::election_result::{ElectionResult, SelectedValidator, StakeAllocation, ExecutionMetadata};
use sp_runtime::Perbill;
use std::collections::HashMap;

/// Parallel Phragmen algorithm implementation
pub struct ParallelPhragmen;

impl ElectionAlgorithm for ParallelPhragmen {
    fn execute(
        &self,
        data: &ElectionData,
        config: &ElectionConfiguration,
    ) -> Result<ElectionResult, ElectionError> {
        if data.candidates.is_empty() {
            return Err(ElectionError::ValidationError {
                message: "Cannot run election with zero candidates".to_string(),
                field: None,
            });
        }

        let candidate_lookup: HashMap<String, &crate::models::validator::ValidatorCandidate> = data
            .candidates
            .iter()
            .map(|candidate| (candidate.account_id.clone(), candidate))
            .collect();

        let nominator_lookup: HashMap<String, &crate::models::nominator::Nominator> = data
            .nominators
            .iter()
            .map(|nominator| (nominator.account_id.clone(), nominator))
            .collect();

        // Preserve the original ordering of candidates
        let candidates: Vec<String> = data
            .candidates
            .iter()
            .map(|candidate| candidate.account_id.clone())
            .collect();

        let mut voters: Vec<(String, u64, Vec<String>)> = Vec::new();
        for nominator in data.nominators.iter() {
            let targets: Vec<String> = nominator
                .targets
                .iter()
                .filter(|id| candidate_lookup.contains_key(*id))
                .cloned()
                .collect();

            if targets.is_empty() {
                continue;
            }

            let stake_u64 = nominator.stake.min(u64::MAX as u128) as u64;
            voters.push((nominator.account_id.clone(), stake_u64, targets));
        }

        // Use phragmms algorithm from sp-npos-elections (parallel phragmen variant)
        let solution = sp_npos_elections::phragmms::<String, Perbill>(
            config.active_set_size as usize,
            candidates,
            voters,
            None,
        )
        .map_err(|e| ElectionError::AlgorithmError {
            message: format!("Parallel phragmen algorithm failed: {:?}", e),
            algorithm: crate::types::AlgorithmType::ParallelPhragmen,
        })?;

        // Convert results back to our format
        let mut selected_validators = Vec::new();
        for (rank, (winner_id, total_backing)) in solution.winners.iter().enumerate() {
            if let Some(candidate) = candidate_lookup.get(winner_id) {
                let nominator_count = solution
                    .assignments
                    .iter()
                    .filter(|assignment| {
                        assignment
                            .distribution
                            .iter()
                            .any(|(target, _)| target == winner_id)
                    })
                    .count() as u32;

                selected_validators.push(SelectedValidator {
                    account_id: candidate.account_id.clone(),
                    total_backing_stake: *total_backing,
                    nominator_count,
                    rank: Some(rank as u32 + 1),
                });
            }
        }

        let mut stake_distribution = Vec::new();
        let perbill_denominator = Perbill::one().deconstruct() as f64;

        for assignment in &solution.assignments {
            if let Some(nominator) = nominator_lookup.get(&assignment.who) {
                for (validator_id, portion) in &assignment.distribution {
                    let proportion = portion.deconstruct() as f64 / perbill_denominator;
                    let amount = (*portion * nominator.stake) as u128;

                    stake_distribution.push(StakeAllocation {
                        nominator_id: nominator.account_id.clone(),
                        validator_id: validator_id.clone(),
                        amount,
                        proportion,
                    });
                }
            }
        }

        // Calculate total stake from all nominators
        let total_nominator_stake: u128 = data.nominators.iter().map(|n| n.stake).sum();

        Ok(ElectionResult {
            selected_validators,
            stake_distribution,
            total_stake: total_nominator_stake,
            algorithm_used: crate::types::AlgorithmType::ParallelPhragmen,
            execution_metadata: ExecutionMetadata {
                block_number: config.block_number,
                execution_timestamp: Some(chrono::Utc::now().to_rfc3339()),
                data_source: None,
            },
            diagnostics: None,
        })
    }

    fn name(&self) -> &'static str {
        "parallel-phragmen"
    }
}



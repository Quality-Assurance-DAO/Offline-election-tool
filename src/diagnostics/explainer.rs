//! Diagnostics generator for explaining election results

use crate::diagnostics::models::{Diagnostics, StakeAnalysis, ValidatorExplanation};
use crate::error::ElectionError;
use crate::models::election_data::ElectionData;
use crate::models::election_result::ElectionResult;
use crate::types::AlgorithmType;
use std::collections::{HashMap, HashSet};

/// Generator for election diagnostics
pub struct DiagnosticsGenerator;

impl DiagnosticsGenerator {
    /// Create a new diagnostics generator
    pub fn new() -> Self {
        Self
    }

    /// Generate diagnostics for an election result
    pub fn generate(
        &self,
        result: &ElectionResult,
        data: &ElectionData,
    ) -> Result<Diagnostics, ElectionError> {
        let mut validator_explanations = Vec::new();
        let mut warnings = Vec::new();

        // Create lookup maps for efficient access
        let selected_validator_set: HashSet<&String> = result
            .selected_validators
            .iter()
            .map(|v| &v.account_id)
            .collect();

        let selected_validator_map: HashMap<&String, &crate::models::election_result::SelectedValidator> =
            result
                .selected_validators
                .iter()
                .map(|v| (&v.account_id, v))
                .collect();

        let stake_by_validator: HashMap<&String, u128> = result
            .stake_distribution
            .iter()
            .map(|alloc| (&alloc.validator_id, alloc.amount))
            .fold(HashMap::new(), |mut acc, (id, amount)| {
                *acc.entry(id).or_insert(0) += amount;
                acc
            });

        let nominator_count_by_validator: HashMap<&String, u32> = result
            .stake_distribution
            .iter()
            .map(|alloc| &alloc.validator_id)
            .fold(HashMap::new(), |mut acc, id| {
                *acc.entry(id).or_insert(0) += 1;
                acc
            });

        // Generate explanations for all candidates
        for candidate in &data.candidates {
            let is_selected = selected_validator_set.contains(&candidate.account_id);
            let explanation = if is_selected {
                self.explain_selected_validator(
                    candidate,
                    selected_validator_map.get(&candidate.account_id).copied(),
                    &stake_by_validator,
                    &nominator_count_by_validator,
                    &result.selected_validators,
                )
            } else {
                self.explain_unselected_validator(
                    candidate,
                    &selected_validator_set,
                    &stake_by_validator,
                    &result.selected_validators,
                )
            };
            validator_explanations.push(explanation);
        }

        // Generate stake analysis
        let stake_analysis = self.generate_stake_analysis(result);

        // Generate algorithm-specific insights
        let algorithm_insights = self.generate_algorithm_insights(result, data);

        // Check for warnings
        if result.selected_validators.len() < result.selected_validators.len() {
            warnings.push(format!(
                "Only {} validators selected, but {} were requested",
                result.selected_validators.len(),
                result.selected_validators.len()
            ));
        }

        if result.total_stake == 0 {
            warnings.push("Total stake is zero - election may not be meaningful".to_string());
        }

        let zero_stake_validators = result
            .selected_validators
            .iter()
            .filter(|v| v.total_backing_stake == 0)
            .count();
        if zero_stake_validators > 0 {
            warnings.push(format!(
                "{} selected validators have zero backing stake",
                zero_stake_validators
            ));
        }

        Ok(Diagnostics {
            validator_explanations,
            stake_analysis,
            algorithm_insights: Some(algorithm_insights),
            warnings,
        })
    }

    /// Explain why a validator was selected
    fn explain_selected_validator(
        &self,
        candidate: &crate::models::validator::ValidatorCandidate,
        selected_info: Option<&crate::models::election_result::SelectedValidator>,
        _stake_by_validator: &HashMap<&String, u128>,
        _nominator_count_by_validator: &HashMap<&String, u32>,
        all_selected: &[crate::models::election_result::SelectedValidator],
    ) -> ValidatorExplanation {
        let mut key_factors = Vec::new();
        let mut reason_parts = Vec::new();

        if let Some(info) = selected_info {
            reason_parts.push(format!(
                "Selected as validator #{} in the active set",
                info.rank.unwrap_or(0)
            ));

            // Analyze stake
            let total_stake = info.total_backing_stake;
            if total_stake > 0 {
                key_factors.push(format!("Total backing stake: {}", total_stake));
                reason_parts.push(format!("with {} total backing stake", total_stake));

                // Compare to other validators
                if let Some(max_stake) = all_selected.iter().map(|v| v.total_backing_stake).max() {
                    if total_stake == max_stake {
                        key_factors.push("Highest total stake".to_string());
                        reason_parts.push("(highest total stake)".to_string());
                    } else {
                        let percentile = (total_stake as f64 / max_stake as f64) * 100.0;
                        if percentile >= 80.0 {
                            key_factors.push("Top tier stake".to_string());
                        }
                    }
                }
            } else {
                key_factors.push("Zero backing stake".to_string());
                reason_parts.push("with zero backing stake".to_string());
            }

            // Analyze nominator count
            let nominator_count = info.nominator_count;
            if nominator_count > 0 {
                key_factors.push(format!("Backed by {} nominators", nominator_count));
                reason_parts.push(format!("backed by {} nominators", nominator_count));

                // Compare to other validators
                if let Some(max_nominators) = all_selected.iter().map(|v| v.nominator_count).max() {
                    if nominator_count == max_nominators {
                        key_factors.push("Most nominators".to_string());
                    }
                }
            } else {
                key_factors.push("No nominators".to_string());
            }

            // Self-stake analysis
            if candidate.stake > 0 {
                key_factors.push(format!("Self-stake: {}", candidate.stake));
            }
        }

        ValidatorExplanation {
            account_id: candidate.account_id.clone(),
            selected: true,
            reason: reason_parts.join(" "),
            key_factors,
        }
    }

    /// Explain why a validator was not selected
    fn explain_unselected_validator(
        &self,
        candidate: &crate::models::validator::ValidatorCandidate,
        _selected_set: &HashSet<&String>,
        stake_by_validator: &HashMap<&String, u128>,
        all_selected: &[crate::models::election_result::SelectedValidator],
    ) -> ValidatorExplanation {
        let mut key_factors = Vec::new();
        let mut reason_parts = Vec::new();

        reason_parts.push("Not selected".to_string());

        // Analyze why not selected
        if candidate.stake == 0 {
            key_factors.push("Zero self-stake".to_string());
            reason_parts.push("due to zero self-stake".to_string());
        }

        // Compare to selected validators
        if let Some(min_selected_stake) = all_selected
            .iter()
            .map(|v| v.total_backing_stake)
            .min()
        {
            if candidate.stake < min_selected_stake {
                key_factors.push(format!(
                    "Self-stake ({}) below minimum selected ({})",
                    candidate.stake, min_selected_stake
                ));
                reason_parts.push(format!(
                    "self-stake of {} is below the minimum selected validator stake of {}",
                    candidate.stake, min_selected_stake
                ));
            }
        }

        // Check if this validator had any nominator votes
        let had_nominator_votes = stake_by_validator.contains_key(&candidate.account_id);
        if !had_nominator_votes {
            key_factors.push("No nominator votes".to_string());
            reason_parts.push("and no nominator votes".to_string());
        } else {
            let backing_stake = stake_by_validator.get(&candidate.account_id).unwrap_or(&0);
            if let Some(min_selected_stake) = all_selected
                .iter()
                .map(|v| v.total_backing_stake)
                .min()
            {
                if *backing_stake < min_selected_stake {
                    key_factors.push(format!(
                        "Total backing stake ({}) below minimum selected ({})",
                        backing_stake, min_selected_stake
                    ));
                }
            }
        }

        ValidatorExplanation {
            account_id: candidate.account_id.clone(),
            selected: false,
            reason: reason_parts.join(", "),
            key_factors,
        }
    }

    /// Generate stake analysis
    fn generate_stake_analysis(&self, result: &ElectionResult) -> StakeAnalysis {
        let total_stake = result.total_stake;
        let validator_count = result.selected_validators.len() as u128;
        let average_stake = if validator_count > 0 {
            total_stake / validator_count
        } else {
            0
        };

        StakeAnalysis {
            total_stake,
            average_stake_per_validator: average_stake,
        }
    }

    /// Generate algorithm-specific insights
    fn generate_algorithm_insights(
        &self,
        result: &ElectionResult,
        _data: &ElectionData,
    ) -> serde_json::Value {
        let mut insights = serde_json::Map::new();

        insights.insert(
            "algorithm".to_string(),
            serde_json::Value::String(format!("{:?}", result.algorithm_used)),
        );

        // Algorithm-specific insights
        match result.algorithm_used {
            AlgorithmType::SequentialPhragmen => {
                insights.insert(
                    "description".to_string(),
                    serde_json::Value::String(
                        "Sequential Phragmen selects validators one by one, optimizing stake distribution at each step".to_string(),
                    ),
                );
            }
            AlgorithmType::ParallelPhragmen => {
                insights.insert(
                    "description".to_string(),
                    serde_json::Value::String(
                        "Parallel Phragmen selects all validators simultaneously, optimizing overall stake distribution".to_string(),
                    ),
                );
            }
            AlgorithmType::MultiPhase => {
                insights.insert(
                    "description".to_string(),
                    serde_json::Value::String(
                        "Multi-phase uses sequential phragmen internally, simulating the on-chain multi-phase election process".to_string(),
                    ),
                );
            }
        }

        // Distribution statistics
        if !result.selected_validators.is_empty() {
            let stakes: Vec<u128> = result
                .selected_validators
                .iter()
                .map(|v| v.total_backing_stake)
                .collect();
            let min_stake = stakes.iter().min().copied().unwrap_or(0);
            let max_stake = stakes.iter().max().copied().unwrap_or(0);
            let median_stake = if stakes.len() > 0 {
                let mut sorted = stakes.clone();
                sorted.sort();
                sorted[sorted.len() / 2]
            } else {
                0
            };

            insights.insert(
                "stake_distribution".to_string(),
                serde_json::json!({
                    "min": min_stake,
                    "max": max_stake,
                    "median": median_stake,
                    "validator_count": result.selected_validators.len(),
                }),
            );
        }

        serde_json::Value::Object(insights)
    }
}

impl Default for DiagnosticsGenerator {
    fn default() -> Self {
        Self::new()
    }
}

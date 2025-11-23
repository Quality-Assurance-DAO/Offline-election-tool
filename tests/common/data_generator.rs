//! Synthetic data generation utilities for testing

use offline_election::models::{ElectionData, Nominator, ValidatorCandidate};
use offline_election::types::AlgorithmType;

/// Generate large-scale election data for performance testing
/// 
/// # Arguments
/// * `candidate_count` - Number of validator candidates
/// * `nominator_count` - Number of nominators
/// * `algorithm` - Algorithm type to use
/// 
/// # Returns
/// ElectionData with synthetic candidates and nominators
pub fn generate_large_scale_election_data(
    candidate_count: usize,
    nominator_count: usize,
    _algorithm: AlgorithmType,
) -> ElectionData {
    let mut election_data = ElectionData::new();
    
    // Generate candidates
    for i in 0..candidate_count {
        let account_id = format!("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY{}", i);
        let stake = 1_000_000_000 + (i as u128 * 100_000_000);
        let candidate = ValidatorCandidate {
            account_id,
            stake,
            metadata: None,
        };
        election_data.add_candidate(candidate).unwrap();
    }
    
    // Generate nominators
    for i in 0..nominator_count {
        let account_id = format!("5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty{}", i);
        let stake = 500_000_000 + (i as u128 * 50_000_000);
        
        // Each nominator votes for a subset of candidates
        // Distribute votes across candidates to create realistic voting patterns
        let targets: Vec<String> = (0..candidate_count)
            .step_by((candidate_count / 10).max(1)) // Vote for ~10% of candidates
            .map(|j| format!("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY{}", j))
            .collect();
        
        let nominator = Nominator {
            account_id,
            stake,
            targets,
            metadata: None,
        };
        election_data.add_nominator(nominator).unwrap();
    }
    
    election_data
}

/// Generate synthetic election data for general testing
pub fn generate_synthetic_election_data(
    candidate_count: usize,
    nominator_count: usize,
) -> ElectionData {
    generate_large_scale_election_data(candidate_count, nominator_count, AlgorithmType::SequentialPhragmen)
}


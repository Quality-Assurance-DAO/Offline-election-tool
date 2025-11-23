//! Criterion benchmarks for large-scale election performance

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use offline_election::engine::ElectionEngine;
use offline_election::models::election_config::ElectionConfiguration;
use offline_election::types::AlgorithmType;

// Import test utilities
use offline_election::models::election_data::ElectionData;
use offline_election::models::{Nominator, ValidatorCandidate};

/// Generate election data for benchmarking
fn generate_benchmark_data(candidate_count: usize, nominator_count: usize) -> ElectionData {
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
        let targets: Vec<String> = (0..candidate_count)
            .step_by((candidate_count / 10).max(1))
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

fn benchmark_election_execution(c: &mut Criterion) {
    let engine = ElectionEngine::new();
    
    // Benchmark different scales
    let scales = vec![
        (100, 1_000, "100c_1kn"),
        (500, 5_000, "500c_5kn"),
        (1_000, 10_000, "1kc_10kn"),
        (2_000, 20_000, "2kc_20kn"),
        (5_000, 50_000, "5kc_50kn"),
    ];
    
    let mut group = c.benchmark_group("election_execution");
    group.sample_size(10); // Reduce sample size for large benchmarks
    
    for (candidate_count, nominator_count, name) in scales {
        let election_data = generate_benchmark_data(candidate_count, nominator_count);
        let config = ElectionConfiguration {
            active_set_size: 100,
            algorithm: AlgorithmType::SequentialPhragmen,
            overrides: None,
            block_number: None,
        };
        
        group.bench_with_input(
            BenchmarkId::new("sequential_phragmen", name),
            &(config, election_data),
            |b, (config, election_data)| {
                b.iter(|| {
                    let result = engine.execute(black_box(config), black_box(election_data));
                    black_box(result)
                })
            },
        );
    }
    
    group.finish();
}

fn benchmark_different_active_set_sizes(c: &mut Criterion) {
    let engine = ElectionEngine::new();
    let election_data = generate_benchmark_data(1_000, 10_000);
    
    let mut group = c.benchmark_group("active_set_size");
    
    let active_set_sizes = vec![10, 50, 100, 200, 500];
    
    for active_set_size in active_set_sizes {
        let config = ElectionConfiguration {
            active_set_size,
            algorithm: AlgorithmType::SequentialPhragmen,
            overrides: None,
            block_number: None,
        };
        
        group.bench_with_input(
            BenchmarkId::from_parameter(active_set_size),
            &config,
            |b, config| {
                b.iter(|| {
                    let result = engine.execute(black_box(config), black_box(&election_data));
                    black_box(result)
                })
            },
        );
    }
    
    group.finish();
}

criterion_group!(benches, benchmark_election_execution, benchmark_different_active_set_sizes);
criterion_main!(benches);


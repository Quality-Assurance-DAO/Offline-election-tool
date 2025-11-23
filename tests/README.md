# Test Suite Documentation

This document provides comprehensive documentation for the test suite of the Offline Election Tool.

For general testing information, including how to run elections and understand test results, see [Testing Overview](../docs/testing/overview.md).

## Overview

The test suite is organized into several categories:
- **Unit Tests** - Tests in `src/` files (marked with `#[cfg(test)]`)
- **Integration Tests** - Tests in `tests/integration/` directory
  - Edge case tests (`tests/integration/edge_cases/`)
  - Chain snapshot tests (`tests/integration/chain_snapshots/`)
  - Performance tests (`tests/integration/performance/`)
  - Regression tests (`tests/integration/regression/`)

## Test Organization

### Edge Case Tests (`tests/integration/edge_cases/`)

Comprehensive edge case test suite covering boundary conditions, error cases, and unusual input scenarios:

- `test_zero_candidates.rs` - Zero candidates scenario
- `test_zero_nominators.rs` - Zero nominators scenario
- `test_single_candidate.rs` - Single candidate scenario
- `test_single_nominator.rs` - Single nominator scenario
- `test_zero_candidate_stakes.rs` - All candidates have zero stake
- `test_zero_nominator_stakes.rs` - All nominators have zero stake
- `test_max_active_set_size.rs` - Active set size equals candidate count
- `test_empty_voting_edges.rs` - Nominators voting for zero candidates
- `test_duplicate_account_ids.rs` - Duplicate account IDs
- `test_all_nominators_vote_all.rs` - All nominators vote for all candidates
- `test_invalid_account_ids.rs` - Invalid SS58 account IDs
- `test_invalid_voting_targets.rs` - Voting edges referencing non-existent candidates
- `test_maximum_stakes.rs` - Extremely large stake values
- `test_malformed_json.rs` - Malformed JSON structure

**Fixtures**: `tests/fixtures/regression/edge_cases/` (14 fixtures)

**Usage**:
```bash
# Run all edge case tests
cargo test --test integration_edge_cases_zero_candidates

# Run specific edge case test
cargo test test_zero_candidates_should_fail
```

### Chain Snapshot Tests (`tests/integration/chain_snapshots/`)

Tests that validate election accuracy by comparing simulation results against actual on-chain election outcomes:

- `test_polkadot.rs` - Polkadot chain snapshot tests (4 blocks)
- `test_kusama.rs` - Kusama chain snapshot tests (4 blocks)
- `test_westend.rs` - Westend chain snapshot tests (2 blocks)

**Fixtures**: `tests/fixtures/chain_snapshots/{chain}/block_{n}.json`

**Usage**:
```bash
# Run chain snapshot tests (requires network access)
cargo test --test integration_edge_cases_zero_candidates -- --ignored

# Run specific chain test
cargo test test_polkadot_block_1 -- --ignored
```

**Note**: Tests are marked with `#[ignore]` by default and require network access to fetch chain data.

### Performance Tests (`tests/integration/performance/`)

Large-scale performance tests validating execution times and memory usage:

- `test_large_scale_1k.rs` - 1k candidates, 10k nominators (< 60s target)
- `test_large_scale_5k.rs` - 5k candidates, 50k nominators (< 5min target)
- `test_large_scale_10k.rs` - 10k candidates, 100k nominators (no OOM target)
- `test_large_nominee_sets.rs` - Large nominee sets (10k+ nominators)
- `test_max_active_set_large.rs` - Maximum active set size on large datasets
- `test_dense_voting.rs` - Dense voting patterns
- `test_sparse_voting.rs` - Sparse voting patterns
- `test_memory_leak.rs` - Memory leak detection (100 consecutive elections)

**Note**: These tests use **synthetic data**. For real-world Polkadot mainnet performance benchmarks, see [Performance Guide](../docs/guides/performance.md).

**Usage**:
```bash
# Run performance tests (may take a long time)
cargo test --test integration_edge_cases_zero_candidates -- --ignored

# Run specific performance test
cargo test test_large_scale_1k -- --ignored
```

**Benchmarks**: Use Criterion for standard benchmarks:
```bash
cargo bench
```

**Performance Documentation**: See [Performance Guide](../docs/guides/performance.md) for:
- Current benchmark status
- How to benchmark with real Polkadot mainnet data
- Performance characteristics and gaps
- Recommendations for improvement

### Regression Tests (`tests/integration/regression/`)

Regression test suite ensuring results remain consistent across code changes:

- `test_normal_elections.rs` - Normal election scenarios
- `test_edge_case_regression.rs` - Edge case regression tests
- `test_performance_regression.rs` - Performance regression tests
- `test_runner.rs` - Test runner with baseline tracking

**Fixtures**: `tests/fixtures/regression/normal/` (53+ fixtures)

**Usage**:
```bash
# Run all regression tests
cargo test --test integration_edge_cases_zero_candidates

# Run specific regression test
cargo test test_regression_normal_election_5x5
```

## Test Fixtures

### Fixture Structure

Test fixtures are JSON files following the `TestFixture` schema:

```json
{
  "metadata": {
    "test_name": "string",
    "description": "string",
    "created": "ISO 8601 datetime",
    "algorithm": "sequential-phragmen | parallel-phragmen | multi-phase",
    "category": "edge_case | performance | regression | chain_snapshot",
    "tags": ["string"]
  },
  "input": {
    "candidates": [...],
    "nominators": [...]
  },
  "expected_result": {...} // Optional, for regression tests
}
```

### Fixture Locations

- Edge case fixtures: `tests/fixtures/regression/edge_cases/`
- Normal regression fixtures: `tests/fixtures/regression/normal/`
- Chain snapshots: `tests/fixtures/chain_snapshots/{chain}/`
- Performance benchmarks: `tests/fixtures/benchmarks/`

## Common Test Utilities

### Assertions (`tests/common/assertions.rs`)

- `assert_error_message_contains()` - Validate error messages
- `compare_results_exact_match()` - Compare election results exactly
- `assert_election_result_valid()` - Validate election result structure
- `detect_result_changes()` - Detect changes between baseline and current results
- `assert_results_match_baseline()` - Assert results match baseline

### Data Generation (`tests/common/data_generator.rs`)

- `generate_large_scale_election_data()` - Generate large-scale test datasets
- `generate_synthetic_election_data()` - Generate synthetic election data

### Benchmark Utilities (`tests/common/benchmark_utils.rs`)

- `measure_execution_time()` - Measure function execution time
- `measure_memory_usage()` - Measure memory usage (placeholder)
- `output_benchmark_json()` - Output structured JSON benchmark results
- `create_benchmark_results()` - Create benchmark results structure

### Fixture Loading (`tests/common/fixture_loader.rs`)

- `load_test_fixture()` - Load test fixture from JSON file
- `load_chain_snapshot()` - Load chain snapshot from JSON file
- `load_regression_fixture()` - Load regression fixture (alias)
- `validate_fixture_schema()` - Validate fixture schema

### RPC Utilities (`tests/common/rpc_utils.rs`)

- `fetch_chain_snapshot()` - Fetch chain snapshot from RPC endpoint
- `save_chain_snapshot()` - Save chain snapshot to JSON file

### RPC Retry (`tests/common/rpc_retry.rs`)

- `retry_with_backoff()` - Retry function with exponential backoff
- `retry_with_backoff_default()` - Retry with default settings (3 attempts, 1s initial delay)

## Running Tests

### Run All Tests

```bash
cargo test
```

### Run Only Unit Tests

```bash
cargo test --lib
```

### Run Only Integration Tests

```bash
cargo test --test '*'
```

### Run Tests with Output

```bash
cargo test -- --nocapture
```

### Run Ignored Tests (Network/Performance)

```bash
cargo test -- --ignored
```

### Run Specific Test

```bash
cargo test test_name
```

### Run Tests Matching Pattern

```bash
cargo test pattern
```

## Test Categories

### Edge Cases
- **Count**: 14 test files, 14 fixtures
- **Purpose**: Validate boundary conditions and error handling
- **Run**: `cargo test --test integration_edge_cases_zero_candidates`

### Chain Snapshots
- **Count**: 3 test files, 10+ fixtures (Polkadot: 4, Kusama: 4, Westend: 2+)
- **Purpose**: Validate accuracy against on-chain results
- **Run**: `cargo test -- --ignored`

### Performance
- **Count**: 8 test files
- **Purpose**: Validate scalability and performance
- **Run**: `cargo test -- --ignored` or `cargo bench`

### Regression
- **Count**: 3 test files, 53+ fixtures
- **Purpose**: Detect result changes across code modifications
- **Run**: `cargo test --test integration_edge_cases_zero_candidates`

## Success Criteria

The test suite validates:

- ✅ **20+ edge cases** - 14 edge case tests covering various scenarios
- ✅ **50+ regression fixtures** - 53 regression test fixtures
- ✅ **10+ chain snapshots** - 10+ chain snapshot fixtures (Polkadot: 4, Kusama: 4, Westend: 2+)
- ✅ **Performance targets** - Performance tests with timing targets

## Continuous Integration

For CI/CD, use:

```bash
# Run all tests
cargo test --all-features

# Run tests with output
cargo test -- --nocapture --test-threads=1

# Run tests and fail on warnings
RUSTFLAGS="-D warnings" cargo test
```

## Troubleshooting

### Tests Fail Due to Network Issues

If chain snapshot tests fail:
1. Check internet connection
2. Verify RPC endpoints are accessible
3. Skip network-dependent tests: `cargo test -- --skip rpc`

### Tests Fail Due to Missing Fixtures

Ensure fixture files exist:
```bash
ls tests/fixtures/regression/
ls tests/fixtures/chain_snapshots/
```

### Performance Tests Timeout

Performance tests may take a long time. Increase timeout or skip:
```bash
cargo test -- --skip slow
```

## Additional Resources

- See [Testing Overview](../docs/testing/overview.md) for detailed test running instructions
- See `specs/002-comprehensive-testing/` for test design documents
- See `specs/002-comprehensive-testing/contracts/` for fixture and benchmark contracts


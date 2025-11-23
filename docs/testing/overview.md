# Testing Overview

This guide explains how to test the offline election tool, run the test suite, and understand test results.

## Quick Start

### 1. Build the Tool

```bash
cargo build --release
```

### 2. Run Your First Election Test

Create a JSON file with election data. See `test-data.json` for an example format.

The JSON structure should have:
- `candidates`: Array of validator candidates with `account_id` and `stake`
- `nominators`: Array of nominators with `account_id`, `stake`, and `targets` (array of candidate account IDs)

#### Human-Readable Output

```bash
cargo run -- run \
  --algorithm sequential-phragmen \
  --active-set-size 3 \
  --input-file test-data.json \
  --format human-readable
```

#### JSON Output

```bash
cargo run -- run \
  --algorithm sequential-phragmen \
  --active-set-size 3 \
  --input-file test-data.json \
  --format json
```

#### Save Results to File

```bash
cargo run -- run \
  --algorithm sequential-phragmen \
  --active-set-size 3 \
  --input-file test-data.json \
  --format json \
  --output-file results.json
```

### 3. Run the Test Suite

```bash
cargo test
```

This will run all unit tests and integration tests.

## Available Algorithms

The tool supports three election algorithms:

- **`sequential-phragmen`**: Sequential Phragmen algorithm - Standard algorithm used by most Substrate chains
- **`parallel-phragmen`**: Parallel Phragmen algorithm - Alternative algorithm that can produce different results
- **`multi-phase`**: Multi-phase election algorithm - Multi-phase election process with signed/unsigned submissions and fallback phases

All three algorithms are fully implemented and ready to use.

## Test Data Format

### Example Test Data

A sample test file `test-data.json` is included in the repository with:
- 5 validator candidates
- 5 nominators with various stake distributions
- Voting patterns that create an interesting election scenario

### Custom Test Data

Create your own JSON file following the same structure as `test-data.json`:

```json
{
  "candidates": [
    {
      "account_id": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
      "stake": 1000000000000
    }
  ],
  "nominators": [
    {
      "account_id": "5DTestNominator1",
      "stake": 500000000000,
      "targets": ["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"]
    }
  ]
}
```

## Expected Output Format

### Human-Readable Format

```
Election Results
================
Algorithm: SequentialPhragmen
Total Stake: 4000000000000
Selected Validators: 3

Selected Validators:
1. <account_id> - Stake: <total_backing>, Nominators: <count>
2. <account_id> - Stake: <total_backing>, Nominators: <count>
...
```

### JSON Format

```json
{
  "selected_validators": [
    {
      "account_id": "...",
      "total_backing_stake": 1234567890,
      "nominator_count": 5,
      "rank": 1
    }
  ],
  "stake_distribution": [
    {
      "nominator_id": "...",
      "validator_id": "...",
      "amount": 123456789,
      "proportion": 0.5
    }
  ],
  "total_stake": 4000000000000,
  "algorithm_used": "SequentialPhragmen",
  "execution_metadata": {
    "block_number": null,
    "execution_timestamp": "2025-01-27T...",
    "data_source": null
  }
}
```

## Testing Different Scenarios

### Test with Different Active Set Sizes

```bash
# Select top 2 validators
cargo run -- run --algorithm sequential-phragmen --active-set-size 2 --input-file test-data.json

# Select top 5 validators
cargo run -- run --algorithm sequential-phragmen --active-set-size 5 --input-file test-data.json
```

### Test with Different Algorithms

```bash
# Sequential Phragmen
cargo run -- run --algorithm sequential-phragmen --active-set-size 100 --rpc-url https://rpc.polkadot.io

# Parallel Phragmen
cargo run -- run --algorithm parallel-phragmen --active-set-size 100 --rpc-url https://rpc.polkadot.io

# Multi-phase
cargo run -- run --algorithm multi-phase --active-set-size 100 --rpc-url https://rpc.polkadot.io
```

## Running the Test Suite

### Prerequisites

- Rust 1.70 or later installed
- Cargo (comes with Rust)
- Network access (for integration tests that use RPC endpoints)

### Run All Tests

```bash
cargo test
```

This will run all unit tests and integration tests.

## Test Types

The project has several types of tests:

1. **Unit Tests** - Tests in `src/` files (marked with `#[cfg(test)]`)
2. **Integration Tests** - Tests in `tests/` directory
   - Edge case tests (`tests/integration/edge_cases/`)
   - Chain snapshot tests (`tests/integration/chain_snapshots/`)
   - Performance tests (`tests/integration/performance/`)
   - Regression tests (`tests/integration/regression/`)

## Running Specific Test Categories

### Run Only Unit Tests

```bash
cargo test --lib
```

### Run Only Integration Tests

```bash
cargo test --test '*'
```

Or run a specific integration test file:

```bash
cargo test --test integration_edge_cases_zero_candidates
```

### Run Edge Case Tests

```bash
# Run all edge case tests
cargo test --test integration_edge_cases_zero_candidates

# Run specific edge case test
cargo test test_zero_candidates_should_fail
```

The edge case tests verify handling of boundary conditions:

- `test_zero_candidates.rs` - Tests with no candidates
- `test_zero_nominators.rs` - Tests with no nominators
- `test_single_candidate.rs` - Tests with single candidate
- `test_single_nominator.rs` - Tests with single nominator
- `test_zero_candidate_stakes.rs` - Tests with zero-stake candidates
- `test_max_active_set_size.rs` - Tests with maximum active set size
- `test_empty_voting_edges.rs` - Tests with no voting edges
- `test_duplicate_account_ids.rs` - Tests with duplicate account IDs

### Run Tests by Name Pattern

```bash
# Run all tests containing "zero" in their name
cargo test zero

# Run all tests containing "candidate" in their name
cargo test candidate
```

### Chain Snapshot Tests

These tests use real on-chain data:

```bash
# Run chain snapshot tests (requires network access)
cargo test --test chain_snapshots
```

**Note**: These tests may require network access and could be slow.

### Performance Tests

Run performance benchmarks:

```bash
cargo test --test performance
```

Or use Criterion benchmarks:

```bash
cargo bench
```

## Running Tests with Output

### Show Test Output (for passing tests)

```bash
cargo test -- --nocapture
```

### Show Test Output (for all tests)

```bash
cargo test -- --show-output
```

### Run Tests in Single Thread (useful for debugging)

```bash
cargo test -- --test-threads=1
```

## Running Specific Tests

### Run a Single Test by Name

```bash
cargo test test_zero_candidates_should_fail
```

### Run Tests Matching a Pattern

```bash
# Run all tests starting with "test_zero"
cargo test test_zero

# Run all tests in a specific module
cargo test integration::edge_cases
```

## Running Tests with Filtering

### Skip Tests That Require Network Access

If you want to skip tests that require RPC access:

```bash
# Run tests but skip those marked with #[ignore]
cargo test -- --skip rpc
```

### Run Only Ignored Tests

```bash
cargo test -- --ignored
```

## Running Tests in Different Modes

### Run Tests in Release Mode

```bash
cargo test --release
```

This compiles with optimizations and may catch different issues.

### Run Tests with Verbose Output

```bash
cargo test --verbose
```

## Test Output Examples

### Successful Test Run

```
running 15 tests
test test_zero_candidates_should_fail ... ok
test test_single_candidate ... ok
test test_zero_nominators ... ok
...
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### Failed Test Run

```
running 1 test
test test_zero_candidates_should_fail ... FAILED

failures:

---- test_zero_candidates_should_fail stdout ----
thread 'test_zero_candidates_should_fail' panicked at 'assertion failed: ...'
```

## Debugging Tests

### Run Tests with Backtrace

```bash
RUST_BACKTRACE=1 cargo test
```

Or on Windows:

```cmd
set RUST_BACKTRACE=1
cargo test
```

### Run Tests with Full Backtrace

```bash
RUST_BACKTRACE=full cargo test
```

### Run a Single Test with Debug Output

```bash
RUST_LOG=debug cargo test test_zero_candidates_should_fail -- --nocapture
```

## Continuous Testing

### Watch for Changes and Re-run Tests

Install `cargo-watch`:

```bash
cargo install cargo-watch
```

Then run:

```bash
cargo watch -x test
```

This will automatically re-run tests when files change.

## Test Coverage

### Generate Test Coverage Report

Install `cargo-tarpaulin`:

```bash
cargo install cargo-tarpaulin
```

Then run:

```bash
cargo tarpaulin --out Html
```

This generates an HTML coverage report in `tarpaulin-report.html`.

## Common Test Commands Reference

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run tests matching pattern
cargo test pattern

# Run only unit tests
cargo test --lib

# Run only integration tests
cargo test --test '*'

# Run tests in release mode
cargo test --release

# Run tests with backtrace
RUST_BACKTRACE=1 cargo test

# Run tests in single thread
cargo test -- --test-threads=1

# Run ignored tests
cargo test -- --ignored

# Skip tests matching pattern
cargo test -- --skip pattern
```

## Troubleshooting

### Common Election Errors

1. **"Must specify one of: --rpc-url, --input-file, or --synthetic"**
   - Solution: Provide one of the required input sources

2. **"Election data must contain at least one validator candidate"**
   - Solution: Ensure your JSON file has at least one candidate

3. **"Nominator votes for non-existent candidate"**
   - Solution: Check that all `targets` in nominators reference valid candidate `account_id`s

4. **"Cannot run election with zero candidates or voters"**
   - Solution: Ensure both candidates and nominators arrays are non-empty

### Test Suite Errors

#### Tests Fail Due to Network Issues

If tests that require RPC access are failing:

1. Check your internet connection
2. Verify the RPC endpoint is accessible
3. Consider skipping network-dependent tests: `cargo test -- --skip rpc`

#### Tests Fail Due to Timeout

Some RPC tests may timeout. Increase timeout or skip them:

```bash
# Skip slow tests
cargo test -- --skip slow
```

#### Tests Fail Due to Missing Fixtures

Some tests require fixture files. Ensure test fixtures are present:

```bash
# Check if fixtures exist
ls tests/fixtures/
```

#### Compilation Errors in Tests

If tests fail to compile:

```bash
# Clean and rebuild
cargo clean
cargo test
```

## CI/CD Integration

For continuous integration, use:

```bash
# Run all tests
cargo test --all-features

# Run tests with output
cargo test -- --nocapture --test-threads=1

# Run tests and fail on warnings
RUSTFLAGS="-D warnings" cargo test
```

## Next Steps

- See [README.md](../README.md) for general usage instructions
- Check test source files in `tests/` for test implementation details
- See [Performance Guide](../guides/performance.md) for performance benchmarks
- See [RPC Usage Guide](../guides/rpc-usage.md) for RPC testing information


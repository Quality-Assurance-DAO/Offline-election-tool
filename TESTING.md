# Testing the Offline Election Tool

This guide explains how to test the offline election tool and retrieve election results.

## Quick Start

### 1. Build the Tool

```bash
cargo build --release
```

### 2. Prepare Test Data

Create a JSON file with election data. See `test-data.json` for an example format.

The JSON structure should have:
- `candidates`: Array of validator candidates with `account_id` and `stake`
- `nominators`: Array of nominators with `account_id`, `stake`, and `targets` (array of candidate account IDs)

### 3. Run an Election

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

## Available Algorithms

- `sequential-phragmen`: Sequential Phragmen algorithm (currently implemented)
- `parallel-phragmen`: Parallel Phragmen algorithm (to be implemented)
- `multi-phase`: Multi-phase election algorithm (to be implemented)

## Example Test Data

A sample test file `test-data.json` is included in the repository with:
- 5 validator candidates
- 5 nominators with various stake distributions
- Voting patterns that create an interesting election scenario

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

### Test with Custom Data

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

## Running Unit Tests

```bash
cargo test
```

## Troubleshooting

### Common Errors

1. **"Must specify one of: --rpc-url, --input-file, or --synthetic"**
   - Solution: Provide one of the required input sources

2. **"Election data must contain at least one validator candidate"**
   - Solution: Ensure your JSON file has at least one candidate

3. **"Nominator votes for non-existent candidate"**
   - Solution: Check that all `targets` in nominators reference valid candidate `account_id`s

4. **"Cannot run election with zero candidates or voters"**
   - Solution: Ensure both candidates and nominators arrays are non-empty

## Next Steps

- RPC data fetching (currently returns error - needs async runtime implementation)
- Synthetic data generation (currently not implemented)
- Additional algorithms (parallel-phragmen, multi-phase)


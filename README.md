# Offline NPoS Election Tool

A Rust-based offline NPoS (Nominated Proof of Stake) election tool that exactly mirrors the election logic of any Substrate chain. This tool allows you to run election simulations offline, compare different election algorithms, and analyze election outcomes.

## Features

- **Multiple Election Algorithms**: Support for sequential phragmen, parallel phragmen, and multi-phase algorithms
- **Flexible Data Sources**: Fetch data from Substrate RPC endpoints, load from JSON files, or create synthetic data
- **Parameter Overrides**: Modify election parameters (active set size, stakes, voting edges) without changing source data
- **Detailed Diagnostics**: Get explanations for why validators were selected or not selected
- **Multiple Interfaces**: Use via CLI, REST API, or programmatic library API
- **Bit-for-Bit Accuracy**: Produces identical results to on-chain elections using Substrate's native crates

## Installation

### Prerequisites

- Rust 1.70 or later
- Network access (for RPC data fetching, optional for file-based usage)

### Build from Source

```bash
git clone <repository-url>
cd offline-election-tool
cargo build --release
```

The binary will be available at `target/release/offline-election` (or `offline-election.exe` on Windows).

## Quick Start

### Run Election from RPC (On-Chain Data)

```bash
# Latest block (any RPC endpoint)
offline-election run \
  --algorithm sequential-phragmen \
  --active-set-size 100 \
  --rpc-url https://rpc.polkadot.io

# Historical block (requires archive node endpoint)
offline-election run \
  --algorithm sequential-phragmen \
  --active-set-size 100 \
  --rpc-url https://polkadot.api.onfinality.io/public \
  --block-number 12345678
```

**Note**: For historical blocks (`--block-number`), use archive node endpoints. See [RPC_ARCHIVE_NODES.md](RPC_ARCHIVE_NODES.md) for details.

### Run Election from JSON File

```bash
offline-election run \
  --algorithm parallel-phragmen \
  --active-set-size 50 \
  --input-file election_data.json
```

### Get Detailed Diagnostics

```bash
offline-election run \
  --algorithm sequential-phragmen \
  --active-set-size 100 \
  --rpc-url https://rpc.polkadot.io \
  --diagnostics
```

### Start REST API Server

```bash
# Start server on default port 3000
offline-election server

# Start server on custom port
offline-election server --port 8080
```

### Use All Three Algorithms

**Via CLI:**
```bash
# Sequential Phragmen
offline-election run --algorithm sequential-phragmen --active-set-size 100 --rpc-url https://rpc.polkadot.io

# Parallel Phragmen
offline-election run --algorithm parallel-phragmen --active-set-size 100 --rpc-url https://rpc.polkadot.io

# Multi-phase
offline-election run --algorithm multi-phase --active-set-size 100 --rpc-url https://rpc.polkadot.io
```

**Via REST API:**
```bash
# Start server first
offline-election server

# Then use curl or any HTTP client
curl -X POST http://localhost:3000/elections/run \
  -H "Content-Type: application/json" \
  -d '{"algorithm": "sequential-phragmen", "active_set_size": 100, "data_source": {"type": "rpc", "url": "https://rpc.polkadot.io"}}'
```

See [API_USAGE.md](API_USAGE.md) for detailed API documentation and examples.

## Usage

### CLI Commands

#### Run Election

The `run` command executes an election simulation:

```bash
offline-election run [OPTIONS]
```

**Options:**
- `--algorithm <ALGORITHM>` - Election algorithm: `sequential-phragmen`, `parallel-phragmen`, or `multi-phase` (required)
- `--active-set-size <SIZE>` - Number of validators to select (required)
- `--rpc-url <URL>` - RPC endpoint URL (conflicts with `--input-file` and `--synthetic`)
- `--block-number <NUMBER>` - Block number for RPC snapshot (requires `--rpc-url`). **Note**: Historical blocks require archive node endpoints. See [RPC_ARCHIVE_NODES.md](RPC_ARCHIVE_NODES.md) for details.
- `--input-file <PATH>` - Path to JSON file with election data (conflicts with `--rpc-url` and `--synthetic`)
- `--synthetic` - Use synthetic data (conflicts with `--rpc-url` and `--input-file`)
- `--override-candidate-stake <ACCOUNT_ID=STAKE>` - Override candidate stake (can be repeated)
- `--override-nominator-stake <ACCOUNT_ID=STAKE>` - Override nominator stake (can be repeated)
- `--diagnostics` - Include detailed diagnostics in output
- `--output-file <PATH>` - Write output to file (default: stdout)
- `--format <FORMAT>` - Output format: `json` or `human-readable` (default: `json`)

**Examples:**

```bash
# Run election from RPC with diagnostics
offline-election run \
  --algorithm sequential-phragmen \
  --active-set-size 100 \
  --rpc-url https://rpc.polkadot.io \
  --block-number 12345678 \
  --diagnostics \
  --output-file results.json

# Run election from JSON file with parameter overrides
offline-election run \
  --algorithm parallel-phragmen \
  --active-set-size 50 \
  --input-file election_data.json \
  --override-candidate-stake "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY=1000000" \
  --override-nominator-stake "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty=500000"

# Run election with synthetic data
offline-election run \
  --algorithm multi-phase \
  --active-set-size 10 \
  --synthetic \
  --format human-readable
```

#### Start REST API Server

The `server` command starts a REST API server:

```bash
offline-election server [OPTIONS]
```

**Options:**
- `--port <PORT>` - Port to listen on (default: 3000)

**Example:**

```bash
# Start server on default port 3000
offline-election server

# Start server on custom port
offline-election server --port 8080
```

### Programmatic API

The library can be used programmatically in Rust:

```rust
use offline_election::*;

#[tokio::main]
async fn main() -> Result<(), ElectionError> {
    // Load election data from RPC
    let data = ElectionData::from_rpc(
        "https://rpc.polkadot.io",
        Some(12345678)
    ).await?;

    // Create election configuration
    let config = ElectionConfiguration::new()
        .algorithm(AlgorithmType::SequentialPhragmen)
        .active_set_size(100)
        .build()?;

    // Execute election
    let engine = ElectionEngine::new();
    let result = engine.execute_with_diagnostics(&config, &data, true)?;

    // Access results
    println!("Selected {} validators", result.validator_count());
    if let Some(diagnostics) = result.diagnostics() {
        for explanation in diagnostics.validator_explanations() {
            println!("{}: {}", explanation.account_id, explanation.reason);
        }
    }

    Ok(())
}
```

### REST API

The REST API provides HTTP endpoints for election operations:

- `POST /elections/run` - Run an election simulation
- `GET /elections/:id/results` - Get election results by ID
- `GET /elections/:id/diagnostics` - Get detailed diagnostics for an election
- `GET /health` - Health check endpoint

See [API_USAGE.md](API_USAGE.md) for detailed API documentation and examples.

## Troubleshooting

### Common Issues

**RPC Connection Errors:**
- Ensure the RPC endpoint URL is correct and accessible
- Check network connectivity
- Try alternative RPC endpoints (the tool suggests alternatives automatically)
- Some RPC endpoints may have rate limits - wait and retry
- **For historical blocks (`--block-number`)**: Use archive node endpoints (see [RPC_ARCHIVE_NODES.md](RPC_ARCHIVE_NODES.md))

**Validation Errors:**
- Ensure all candidate account IDs are unique
- Ensure all nominator account IDs are unique
- Check that voting edges reference existing candidates
- Verify that active set size doesn't exceed available candidates

**Algorithm Errors:**
- Ensure there are enough candidates for the requested active set size
- Check that nominators have valid voting targets
- Verify stake amounts are valid (non-negative)

**File Errors:**
- Ensure JSON files are valid and properly formatted
- Check file permissions
- Verify file paths are correct

### Getting Help

- Check the [Quickstart Guide](specs/001-offline-npos-election/quickstart.md) for detailed examples
- Review the [Feature Specification](specs/001-offline-npos-election/spec.md) for requirements
- See [API_USAGE.md](API_USAGE.md) for API documentation
- Check [TESTING.md](TESTING.md) for testing examples
- **For historical block queries**: See [RPC_ARCHIVE_NODES.md](RPC_ARCHIVE_NODES.md) for archive node requirements and troubleshooting

## Examples

### Example 1: Compare Algorithms

```bash
# Run same data through different algorithms
# Note: For historical blocks, use archive node endpoint
DATA="--rpc-url https://polkadot.api.onfinality.io/public --block-number 12345678 --active-set-size 100"

offline-election run --algorithm sequential-phragmen $DATA --output-file seq_results.json
offline-election run --algorithm parallel-phragmen $DATA --output-file par_results.json
offline-election run --algorithm multi-phase $DATA --output-file multi_results.json

# Compare results
diff seq_results.json par_results.json
```

### Example 2: What-If Analysis

```bash
# Run election with modified parameters
offline-election run \
  --algorithm sequential-phragmen \
  --active-set-size 50 \
  --rpc-url https://rpc.polkadot.io \
  --override-candidate-stake "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY=2000000" \
  --diagnostics \
  --output-file whatif_results.json
```

### Example 3: Synthetic Data Testing

```rust
use offline_election::*;

let mut builder = SyntheticDataBuilder::new();
builder
    .add_candidate("candidate1".to_string(), 1000000)?
    .add_candidate("candidate2".to_string(), 2000000)?
    .add_nominator("nominator1".to_string(), 500000, vec!["candidate1".to_string()])?;

let data = builder.build()?;
let config = ElectionConfiguration::new()
    .algorithm(AlgorithmType::SequentialPhragmen)
    .active_set_size(2)
    .build()?;

let engine = ElectionEngine::new();
let result = engine.execute(&config, &data)?;
```

See the [Quickstart Guide](specs/001-offline-npos-election/quickstart.md) for more detailed usage examples and documentation.

## Project Structure

```
src/
├── lib.rs                    # Library entry point
├── main.rs                   # CLI binary entry point
├── models/                   # Data models
├── algorithms/               # Election algorithm implementations
├── input/                    # Input data loading (RPC, JSON, synthetic)
├── diagnostics/              # Diagnostic generation
├── cli/                      # CLI interface
└── api/                      # REST API server

tests/
├── unit/                     # Unit tests
├── integration/              # Integration tests
└── contract/                 # Contract tests
```

## Documentation

- [Feature Specification](specs/001-offline-npos-election/spec.md)
- [Implementation Plan](specs/001-offline-npos-election/plan.md)
- [Data Model](specs/001-offline-npos-election/data-model.md)
- [Quickstart Guide](specs/001-offline-npos-election/quickstart.md)
- [Programmatic API](specs/001-offline-npos-election/contracts/programmatic-api.md)
- [REST API](specs/001-offline-npos-election/contracts/rest-api.yaml)

## License

MIT OR Apache-2.0

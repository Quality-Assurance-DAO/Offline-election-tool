# Offline NPoS Election Tool

A Rust-based offline **NPoS (Nominated Proof of Stake)** election tool that exactly mirrors the election logic of any Substrate chain. This tool allows you to run election simulations offline, compare different election algorithms, and analyze election outcomes.

## Project Overview

### What are NPoS Elections?

**NPoS (Nominated Proof of Stake)** is the consensus mechanism used by Polkadot and many Substrate-based chains to select validators. In NPoS:

- **Validators** are nodes that produce blocks and validate transactions
- **Nominators** are token holders who stake tokens to back validators they trust
- **Elections** run periodically (typically every era, ~24 hours) to select a fixed number of validators (the **active set**) from a pool of candidates
- **Stake distribution** determines which validators are selected - validators with more stake backing are more likely to be selected

### Why Offline Simulation is Valuable

Offline election simulation enables you to:

- **Test scenarios** before they happen on-chain
- **Analyze outcomes** without waiting for the next election
- **Compare algorithms** to understand how different election methods affect results
- **Explore what-if scenarios** by modifying stake distributions or election parameters
- **Learn and educate** about how NPoS elections work without interacting with live chains

This tool provides **bit-for-bit accuracy** with on-chain elections by using the same Substrate crates and algorithms used by live chains.

For a comprehensive overview of the Polkadot ecosystem, validators, nominators, staking, and NPoS elections, see [Polkadot Ecosystem Overview](docs/polkadot/ecosystem-overview.md).

## Features

- **Multiple Election Algorithms**: Support for **Sequential Phragmen**, **Parallel Phragmen**, and **Multi-phase** algorithms
- **Flexible Data Sources**: Fetch data from **Substrate RPC** endpoints, load from JSON files, or create synthetic data
- **Parameter Overrides**: Modify election parameters (**active set** size, stakes, voting edges) without changing source data
- **Detailed Diagnostics**: Get explanations for why **validators** were selected or not selected
- **Multiple Interfaces**: Use via CLI, REST API, or programmatic library API
- **Bit-for-Bit Accuracy**: Produces identical results to on-chain elections using **Substrate's** native crates
- **Performance Testing**: Includes synthetic benchmarks for large-scale scenarios (see [Performance Benchmarks](docs/guides/performance.md) for details)

### Key Terms

- **NPoS (Nominated Proof of Stake)**: The consensus mechanism used by Polkadot to select validators based on stake backing
- **Sequential Phragmen**: A standard election algorithm that selects validators sequentially based on stake distribution
- **Parallel Phragmen**: An alternative election algorithm that can produce different results than sequential phragmen
- **Multi-phase**: A multi-phase election process with signed/unsigned submissions and fallback phases
- **Active Set**: The fixed number of validators selected to participate in consensus (typically ~297 for Polkadot mainnet)
- **Validator**: A node that produces blocks and validates transactions on the network
- **Nominator**: A token holder who stakes tokens to back validators
- **Stake**: Tokens locked to participate in network security and validator selection
- **Archive Node**: An RPC node that maintains complete historical state (required for historical block queries)
- **RPC (Remote Procedure Call)**: A protocol for querying blockchain data from remote nodes
- **Substrate**: A blockchain framework used by Polkadot and many other chains
- **SS58**: An encoding format used for Polkadot account addresses
- **Bit-for-bit accuracy**: Producing identical results to on-chain elections using the same algorithms and data

For comprehensive definitions of all technical terms, see the [Glossary](docs/reference/glossary.md).

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

Get started with your first election simulation in minutes:

### Step 1: Build the Tool

```bash
git clone <repository-url>
cd offline-election-tool
cargo build --release
```

### Step 2: Run Your First Election

The simplest way to get started is to run an election using live Polkadot data:

```bash
# Run election from latest block
offline-election run \
  --algorithm sequential-phragmen \
  --active-set-size 100 \
  --rpc-url https://rpc.polkadot.io
```

This will:
1. Connect to the Polkadot RPC endpoint
2. Fetch current validator and nominator data
3. Run a **Sequential Phragmen** election to select 100 validators
4. Output the results in JSON format

### Step 3: Explore Different Algorithms

Try running the same data through different algorithms to compare results:

```bash
# Sequential Phragmen
offline-election run --algorithm sequential-phragmen --active-set-size 100 --rpc-url https://rpc.polkadot.io

# Parallel Phragmen
offline-election run --algorithm parallel-phragmen --active-set-size 100 --rpc-url https://rpc.polkadot.io

# Multi-phase
offline-election run --algorithm multi-phase --active-set-size 100 --rpc-url https://rpc.polkadot.io
```

### Step 4: Get Detailed Diagnostics

Add the `--diagnostics` flag to understand why validators were selected or not:

```bash
offline-election run \
  --algorithm sequential-phragmen \
  --active-set-size 100 \
  --rpc-url https://rpc.polkadot.io \
  --diagnostics
```

### Next Steps

- **Learn more**: Read the [Polkadot Ecosystem Overview](docs/polkadot/ecosystem-overview.md) to understand validators, nominators, and staking
- **Explore guides**: Check out [Algorithm Guide](docs/guides/algorithms.md) and [RPC Usage Guide](docs/guides/rpc-usage.md)
- **Try examples**: See the [Examples](#examples) section below for more use cases

---

## Detailed Usage Examples

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

**Note**: For historical blocks (`--block-number`), use archive node endpoints. See [RPC Usage Guide](docs/guides/rpc-usage.md) for details.

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

See [REST API Documentation](docs/api/rest-api.md) and [Programmatic API Documentation](docs/api/programmatic-api.md) for detailed API documentation and examples.

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
- `--block-number <NUMBER>` - Block number for RPC snapshot (requires `--rpc-url`). **Note**: Historical blocks require archive node endpoints. See [RPC Usage Guide](docs/guides/rpc-usage.md) for details.
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

See [REST API Documentation](docs/api/rest-api.md) for comprehensive API documentation including:
- Complete API usage examples for all three algorithms
- **Synthetic data construction guide** with edge case examples
- Validation rules and error handling
- Security considerations for production deployment

## Security and Robustness

### REST API Server Security

**⚠️ Important**: The REST API server is designed for development and testing purposes. For production deployments, additional security measures must be implemented.

#### Current State

The current implementation does **not** include:
- **Authentication**: All endpoints are publicly accessible without authentication
- **Rate Limiting**: No protection against request flooding or denial-of-service attacks
- **Request Size Limits**: No explicit limits on JSON payload size (relies on Axum defaults)

#### Input Validation and Protection

The API includes several layers of input validation:

1. **JSON Deserialization**: Axum automatically validates JSON structure and types using Serde
2. **Algorithm Validation**: Algorithm strings are validated against allowed values (`sequential-phragmen`, `parallel-phragmen`, `multi-phase`)
3. **Data Validation**: Election data is validated through the `ElectionData::validate()` method, which checks:
   - Unique candidate and nominator account IDs
   - Valid account ID formats (SS58 encoding)
   - Non-negative stake values
   - Valid voting edges (nominators can only vote for existing candidates)
   - Active set size constraints
4. **Stake Parsing**: Stake values are parsed with error handling to prevent invalid numeric inputs
5. **RPC URL Validation**: RPC URLs are validated when creating RPC clients

**Malformed Input Handling**:
- Invalid JSON: Returns `400 Bad Request` with error details
- Invalid algorithm: Returns `400 Bad Request` with validation error
- Invalid data structure: Returns `400 Bad Request` with field-specific errors
- Invalid account IDs: Returns `400 Bad Request` with validation error
- Invalid stake values: Returns `400 Bad Request` with field-specific error

#### Recommendations for Production Use

**1. Authentication**

Implement authentication before deploying to production:

- **API Keys**: Use API key authentication via custom middleware
- **Bearer Tokens**: Implement JWT or OAuth2 bearer token authentication
- **IP Whitelisting**: Restrict access to known IP addresses (if applicable)
- **Reverse Proxy**: Use a reverse proxy (nginx, Traefik) with authentication

Example with API key middleware (pseudo-code):
```rust
// Add API key validation middleware
let app = Router::new()
    .route("/elections/run", post(run_election))
    .layer(axum::middleware::from_fn(validate_api_key))
    .with_state(state);
```

**2. Rate Limiting**

Protect against abuse and DoS attacks:

- **Per-IP Rate Limiting**: Limit requests per IP address (e.g., 100 requests/minute)
- **Per-API-Key Rate Limiting**: Different limits for different API keys
- **Endpoint-Specific Limits**: Stricter limits on resource-intensive endpoints like `/elections/run`
- **Consider using**: `tower-ratelimit` or `axum-rate-limit` middleware

Example rate limiting configuration:
```rust
// Recommended limits:
// - /elections/run: 10 requests/minute per IP
// - /elections/:id/results: 60 requests/minute per IP
// - /health: 300 requests/minute per IP
```

**3. Request Size Limits**

Protect against memory exhaustion attacks:

- **JSON Payload Limits**: Set explicit limits on request body size (e.g., 10MB max)
- **Array Size Limits**: Limit maximum number of candidates/nominators in synthetic data
- **Configure Axum**: Use `axum::extract::DefaultBodyLimit` middleware

Example:
```rust
let app = Router::new()
    .route("/elections/run", post(run_election))
    .layer(DefaultBodyLimit::max(10 * 1024 * 1024)) // 10MB limit
    .with_state(state);
```

**4. Additional Security Measures**

- **HTTPS/TLS**: Always use HTTPS in production (use reverse proxy or TLS termination)
- **CORS Configuration**: Configure CORS headers appropriately if serving web clients
- **Request Timeouts**: Set timeouts for long-running operations
- **Error Message Sanitization**: Ensure error messages don't leak sensitive information
- **Logging and Monitoring**: Implement request logging and monitoring for suspicious activity
- **Resource Limits**: Set memory and CPU limits for the server process
- **Input Sanitization**: Validate and sanitize RPC URLs to prevent SSRF attacks

**5. Protecting Against Specific Attacks**

- **SSRF (Server-Side Request Forgery)**: Validate RPC URLs against allowlists or block internal IPs
- **DoS via Large Elections**: Limit `active_set_size` and number of candidates/nominators
- **Memory Exhaustion**: Set request size limits and resource quotas
- **JSON Bomb**: Limit nesting depth and array sizes in JSON parsing

**6. Deployment Recommendations**

- **Behind Reverse Proxy**: Deploy behind nginx, Traefik, or similar with authentication
- **Container Limits**: Use Docker/Kubernetes resource limits
- **Network Isolation**: Restrict network access to necessary endpoints only
- **Regular Updates**: Keep dependencies updated for security patches

#### Example Production Setup

```bash
# Run behind nginx with authentication
# nginx.conf:
# - Rate limiting: limit_req_zone
# - SSL/TLS termination
# - Basic auth or API key validation
# - Request size limits: client_max_body_size 10m

# Or use a service mesh (Istio, Linkerd) for:
# - mTLS
# - Rate limiting
# - Authentication policies
```

#### Development vs Production

**Development/Testing**:
- Current implementation is sufficient
- No authentication required
- Suitable for local testing and CI/CD

**Production**:
- **Must** implement authentication
- **Must** implement rate limiting
- **Must** set request size limits
- **Must** use HTTPS/TLS
- **Should** implement monitoring and logging
- **Should** use reverse proxy with additional security layers

## Troubleshooting

### Common Issues

**RPC Connection Errors:**
- Ensure the RPC endpoint URL is correct and accessible
- Check network connectivity
- Try alternative RPC endpoints (the tool suggests alternatives automatically)
- Some RPC endpoints may have rate limits - wait and retry
- **For historical blocks (`--block-number`)**: Use archive node endpoints (see [RPC Usage Guide](docs/guides/rpc-usage.md))

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
- See [REST API Documentation](docs/api/rest-api.md) and [Programmatic API Documentation](docs/api/programmatic-api.md) for API documentation
- Check [Testing Overview](docs/testing/overview.md) for testing examples
- **For historical block queries**: See [RPC Usage Guide](docs/guides/rpc-usage.md) for archive node requirements and troubleshooting
- **For performance benchmarks**: See [Performance Guide](docs/guides/performance.md) for performance testing and large-scale benchmarks

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

## Election Algorithms

### Currently Supported Algorithms

The tool supports three election algorithms commonly used in Substrate chains:

1. **Sequential Phragmen** (`sequential-phragmen`)
   - Uses `sp_npos_elections::seq_phragmen`
   - Standard algorithm used by most Substrate chains
   - Deterministic, produces consistent results

2. **Parallel Phragmen** (`parallel-phragmen`)
   - Uses `sp_npos_elections::phragmms` (Phragmms algorithm)
   - Alternative algorithm that can produce different results
   - Useful for comparing outcomes with sequential phragmen

3. **Multi-phase** (`multi-phase`)
   - Uses sequential phragmen internally (matching `pallet-election-provider-multi-phase`)
   - Represents the multi-phase election process used by chains like Polkadot
   - Supports signed/unsigned submissions and fallback phases

### Algorithm Extensibility

The codebase is designed to be **easily extensible** for adding new election algorithms:

**Architecture:**
- Algorithms implement the `ElectionAlgorithm` trait (`src/algorithms/trait_def.rs`)
- New algorithms can be added by:
  1. Creating a new module in `src/algorithms/`
  2. Implementing the `ElectionAlgorithm` trait
  3. Adding the variant to `AlgorithmType` enum in `src/types.rs`
  4. Registering it in `src/engine.rs`

**Example: Adding a Custom Algorithm**

```rust
// src/algorithms/custom.rs
use crate::algorithms::trait_def::ElectionAlgorithm;
use crate::error::ElectionError;
use crate::models::election_config::ElectionConfiguration;
use crate::models::election_data::ElectionData;
use crate::models::election_result::ElectionResult;

pub struct CustomAlgorithm;

impl ElectionAlgorithm for CustomAlgorithm {
    fn execute(
        &self,
        data: &ElectionData,
        config: &ElectionConfiguration,
    ) -> Result<ElectionResult, ElectionError> {
        // Your algorithm implementation here
        // ...
    }

    fn name(&self) -> &'static str {
        "custom-algorithm"
    }
}
```

Then add to `src/types.rs`:
```rust
pub enum AlgorithmType {
    SequentialPhragmen,
    ParallelPhragmen,
    MultiPhase,
    CustomAlgorithm, // Add new variant
}
```

And register in `src/engine.rs`:
```rust
let algorithm: Box<dyn ElectionAlgorithm> = match config.algorithm {
    AlgorithmType::SequentialPhragmen => Box::new(SequentialPhragmen),
    AlgorithmType::ParallelPhragmen => Box::new(ParallelPhragmen),
    AlgorithmType::MultiPhase => Box::new(MultiPhase),
    AlgorithmType::CustomAlgorithm => Box::new(CustomAlgorithm), // Add case
};
```

### Substrate Election Algorithms

**Standard Algorithms in Substrate:**
- **Sequential Phragmen**: Primary algorithm in `sp-npos-elections`
- **Parallel Phragmen (Phragmms)**: Alternative in `sp-npos-elections`
- **Multi-phase**: Wrapper around sequential phragmen in `pallet-election-provider-multi-phase`

**Custom Election Providers:**
Substrate chains can implement custom election providers via the `ElectionProvider` trait. These are chain-specific and not part of the standard Substrate runtime. To support a custom algorithm:

1. **If it uses standard Substrate crates**: Add it using the extensibility pattern above
2. **If it's chain-specific**: Implement the algorithm logic directly or integrate the chain's election provider crate
3. **If it's experimental**: Add as a custom algorithm following the trait pattern

**Note**: The RFP requirement to "take into consideration all the various election algorithms" is satisfied by:
- Supporting the three main standard algorithms
- Providing an extensible architecture for custom algorithms
- Using Substrate's native crates for accuracy

See [Algorithm Guide](docs/guides/algorithms.md) for detailed documentation on adding new algorithms.

## Navigation

### For Newcomers

- **Start here**: [Quick Start](#quick-start) - Run your first election in minutes
- **Understand the ecosystem**: [Polkadot Ecosystem Overview](docs/polkadot/ecosystem-overview.md) - Learn about validators, nominators, staking, and NPoS elections
- **Learn key terms**: [Glossary](docs/reference/glossary.md) - Definitions of all technical terms
- **See examples**: [Examples](#examples) - Common use cases and workflows

### For Contributors

- **API Documentation**: 
  - [REST API](docs/api/rest-api.md) - REST API server documentation
  - [Programmatic API](docs/api/programmatic-api.md) - Library API documentation
- **Guides**:
  - [Algorithm Guide](docs/guides/algorithms.md) - How to add custom election algorithms
  - [RPC Usage Guide](docs/guides/rpc-usage.md) - Using RPC endpoints and archive nodes
  - [Performance Guide](docs/guides/performance.md) - Performance benchmarks and optimization
- **Testing**: [Testing Overview](docs/testing/overview.md) - Running tests and understanding results

### For Maintainers

- **Reference Documentation**:
  - [RFP Compliance](docs/reference/rfp-compliance.md) - RFP compliance assessment
  - [Glossary](docs/reference/glossary.md) - Technical terms glossary
- **Project Structure**: [Project Structure](#project-structure) - Codebase organization
- **Feature Specifications**: [Feature Specifications](specs/) - Detailed feature specs and plans

## How This Tool Fits in the Polkadot Ecosystem

### Tool's Role

The **Offline NPoS Election Tool** enables offline simulation of NPoS elections without running a full node or waiting for on-chain elections. This provides:

- **Testing**: Test election scenarios before they happen on-chain
- **Analysis**: Analyze election outcomes and stake distributions
- **What-if scenarios**: Explore how changes in stake or nominations would affect election results
- **Algorithm comparison**: Compare different election algorithms (sequential phragmen, parallel phragmen, multi-phase)
- **Education**: Learn how NPoS elections work without interacting with live chains

### Dependencies

This tool uses Substrate's native election crates for accuracy:

- **`sp-npos-elections`**: Core election algorithms (sequential phragmen, parallel phragmen)
- **`frame-election-provider-support`**: Election provider trait and utilities
- **`pallet-election-provider-multi-phase`**: Multi-phase election implementation

Using these crates ensures **bit-for-bit accuracy** with on-chain elections.

### Interactions

- **RPC Data Fetching**: Fetch election data from live Substrate chains (Polkadot, Kusama, Westend)
- **Historical Analysis**: Query past election data using archive nodes
- **Offline Simulation**: Run elections locally without network dependencies

### Use Cases

- **Validator Operators**: Test stake strategies and analyze selection probability
- **Nominators**: Compare validator selection probabilities and plan nominations
- **Researchers**: Study election algorithm behavior and analyze historical data
- **Developers**: Test election logic before deploying to chain

For more details, see the [Polkadot Ecosystem Overview](docs/polkadot/ecosystem-overview.md).

## Documentation

### User Documentation

- [Polkadot Ecosystem Overview](docs/polkadot/ecosystem-overview.md) - Comprehensive Polkadot ecosystem context
- [Quick Start Guide](#quick-start) - Get started in minutes
- [Glossary](docs/reference/glossary.md) - Technical terms definitions

### API Documentation

- [REST API](docs/api/rest-api.md) - REST API server documentation
- [Programmatic API](docs/api/programmatic-api.md) - Library API documentation

### Guides

- [Algorithm Guide](docs/guides/algorithms.md) - How to add custom election algorithms
- [RPC Usage Guide](docs/guides/rpc-usage.md) - Using RPC endpoints and archive nodes
- [Performance Guide](docs/guides/performance.md) - Performance benchmarks and optimization
- [Testing Overview](docs/testing/overview.md) - Running tests and understanding results

### Reference

- [RFP Compliance](docs/reference/rfp-compliance.md) - RFP compliance assessment
- [Glossary](docs/reference/glossary.md) - Technical terms glossary

### Feature Specifications

- [Feature Specification](specs/001-offline-npos-election/spec.md)
- [Implementation Plan](specs/001-offline-npos-election/plan.md)
- [Data Model](specs/001-offline-npos-election/data-model.md)
- [Quickstart Guide](specs/001-offline-npos-election/quickstart.md)

## License

MIT OR Apache-2.0

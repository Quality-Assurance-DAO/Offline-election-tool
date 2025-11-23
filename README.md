# Offline NPoS Election Tool

A Rust-based offline NPoS (Nominated Proof of Stake) election tool that exactly mirrors the election logic of any Substrate chain. This tool allows you to run election simulations offline, compare different election algorithms, and analyze election outcomes.

## Features

- **Multiple Election Algorithms**: Support for sequential phragmen, parallel phragmen, and multi-phase algorithms
- **Flexible Data Sources**: Fetch data from Substrate RPC endpoints, load from JSON files, or create synthetic data
- **Parameter Overrides**: Modify election parameters (active set size, stakes, voting edges) without changing source data
- **Detailed Diagnostics**: Get explanations for why validators were selected or not selected
- **Multiple Interfaces**: Use via CLI, REST API, or programmatic library API
- **Bit-for-Bit Accuracy**: Produces identical results to on-chain elections using Substrate's native crates
- **Performance Testing**: Includes synthetic benchmarks for large-scale scenarios (see [PERFORMANCE_BENCHMARKS.md](PERFORMANCE_BENCHMARKS.md) for details)

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

See [API_USAGE.md](API_USAGE.md) for comprehensive API documentation including:
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
- **For performance benchmarks**: See [PERFORMANCE_BENCHMARKS.md](PERFORMANCE_BENCHMARKS.md) for performance testing and large-scale benchmarks

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

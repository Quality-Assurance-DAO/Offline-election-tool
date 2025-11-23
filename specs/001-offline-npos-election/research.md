# Research: Offline NPoS Election Tool

**Date**: 2025-01-27  
**Feature**: 001-offline-npos-election

## Overview

This document consolidates research findings for technical decisions required to implement the Offline NPoS Election Tool. All items marked as "NEEDS CLARIFICATION" in the Technical Context section of `plan.md` are resolved here.

## Technical Decisions

### 1. Rust Version Compatibility

**Decision**: Target Rust 1.70+ (MSRV - Minimum Supported Rust Version)

**Rationale**:
- Substrate crates (`sp-npos-elections`, `frame-election-provider-support`, `pallet-election-provider-multi-phase`) typically require Rust 1.70+ as their MSRV
- Rust 1.70 was released in June 2023 and provides stable async/await, const generics, and other features needed by Substrate
- Using a stable, well-supported version ensures compatibility with Substrate runtime dependencies
- Allows use of modern Rust features while maintaining broad compatibility

**Alternatives Considered**:
- **Rust 1.75+**: Latest stable provides newest features but may have compatibility issues with older Substrate versions
- **Rust 1.65**: Too old - lacks features required by modern Substrate crates
- **Nightly Rust**: Provides cutting-edge features but introduces instability and maintenance burden

**Implementation Note**: Specify `rust-version = "1.70"` in `Cargo.toml` to enforce MSRV, while allowing users to use newer Rust versions.

---

### 2. Substrate RPC Client Library

**Decision**: Use `jsonrpsee` for Substrate RPC communication

**Rationale**:
- `jsonrpsee` is the modern, officially recommended RPC client library for Substrate/Polkadot ecosystems
- Provides async/await support, which aligns with modern Rust practices
- Supports both HTTP and WebSocket transports (HTTP sufficient for this use case)
- Well-maintained by Parity Technologies, ensuring compatibility with Substrate updates
- Lightweight and focused on JSON-RPC 2.0 protocol
- Good error handling and type safety through code generation
- Active development and community support

**Alternatives Considered**:
- **`substrate-api-client`**: Older library, less actively maintained, more complex API
- **`polkadot-js` via FFI**: Would require Node.js dependency, adds complexity
- **Raw HTTP client (`reqwest`)**: Would require manual JSON-RPC implementation, error-prone

**Implementation Details**:
- Use `jsonrpsee-http-client` for HTTP-based RPC calls
- Use `jsonrpsee-types` for type definitions
- Implement RPC calls for: `chain_getBlock`, `state_getStorage`, `state_call` (for election data)
- Handle connection errors gracefully with informative messages (FR-020)

---

### 3. REST API Framework

**Decision**: Use `axum` for the REST API server

**Rationale**:
- Modern, async-first web framework built on `tokio` and `hyper`
- Excellent performance characteristics (comparable to or better than actix-web)
- Type-safe routing and request/response handling
- Built-in JSON support via `serde` integration
- Minimal boilerplate, clean API design
- Active development and strong community adoption
- Good documentation and examples
- Compatible with `tokio` async runtime used by `jsonrpsee`
- Supports graceful shutdown and middleware patterns

**Alternatives Considered**:
- **`actix-web`**: Mature and performant, but heavier API, more complex for simple REST endpoints
- **`warp`**: Functional style can be harder to read, less mainstream adoption
- **`rocket`**: Requires nightly Rust, less suitable for production stability needs

**Performance Considerations**:
- Axum's performance meets the requirement of <5 seconds response time for typical election sizes (SC-010)
- Async architecture allows handling multiple concurrent requests efficiently
- Low overhead for JSON serialization/deserialization

**Implementation Details**:
- Use `axum` with `tokio` runtime
- Implement REST endpoints: `POST /elections/run`, `GET /elections/{id}/results`, `GET /elections/{id}/diagnostics`
- Use `serde_json` for request/response serialization
- Return appropriate HTTP status codes (200, 400, 500) with JSON error responses
- No authentication middleware required (FR-021)

---

### 4. Election Algorithm Trait Design

**Decision**: Create a trait-based abstraction for election algorithms

**Rationale**:
- Enables runtime algorithm swapping (FR-010) without changing input data
- Provides clean separation between algorithm implementations and election execution
- Allows easy addition of new algorithms in the future
- Supports testing each algorithm independently
- Aligns with Rust's trait system for polymorphism

**Trait Design**:
```rust
pub trait ElectionAlgorithm {
    fn execute(
        &self,
        candidates: &[ValidatorCandidate],
        nominators: &[Nominator],
        config: &ElectionConfig,
    ) -> Result<ElectionResult, ElectionError>;
    
    fn name(&self) -> &'static str;
}
```

**Implementation Strategy**:
- Implement trait for: SequentialPhragmen, ParallelPhragmen, MultiPhase
- Use Substrate's native algorithm implementations from `sp-npos-elections` where possible
- Wrap Substrate algorithms in trait implementations to maintain compatibility

---

### 5. Input Data Format Design

**Decision**: Use JSON for file-based input with a well-defined schema

**Rationale**:
- JSON is human-readable and widely supported
- Easy to validate and parse with `serde_json`
- Supports nested structures needed for election data (candidates, nominators, edges)
- Can be easily generated from on-chain data or created synthetically
- Standard format that users can understand and modify

**Schema Structure**:
```json
{
  "candidates": [
    {
      "account_id": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
      "stake": "1000000000000"
    }
  ],
  "nominators": [
    {
      "account_id": "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
      "stake": "500000000000",
      "targets": ["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"]
    }
  ],
  "metadata": {
    "block_number": 12345678,
    "chain": "polkadot"
  }
}
```

**Validation Rules**:
- Reject duplicate account identifiers (FR-022)
- Validate account ID format (SS58 encoding)
- Ensure stake values are non-negative
- Validate that voting edges reference existing candidates

---

## Integration Patterns

### RPC Data Fetching Pattern

**Pattern**: Fetch election data from Substrate RPC endpoints

**Approach**:
1. Connect to RPC endpoint using `jsonrpsee-http-client`
2. If block number specified, fetch block hash first
3. Query storage for validator set, nominator data, and stake information
4. Transform RPC responses into internal `ElectionData` structure
5. Handle errors (connection failures, missing blocks) with clear messages

**Error Handling**:
- Connection failures → Return error with RPC endpoint URL and failure reason
- Block not found → Return error indicating block number doesn't exist
- Partial data → Return error indicating incomplete data fetch

### Algorithm Selection Pattern

**Pattern**: Runtime algorithm selection via trait objects

**Approach**:
1. Parse algorithm name from CLI/config (e.g., "sequential-phragmen", "parallel-phragmen", "multi-phase")
2. Create appropriate algorithm instance based on name
3. Execute election using trait method
4. Return results in standardized format regardless of algorithm

**Benefits**:
- Single code path for election execution
- Easy to add new algorithms
- Consistent error handling across algorithms

---

## Best Practices

### Error Handling
- Use `Result<T, ElectionError>` pattern throughout
- Provide detailed error messages explaining failure reasons (FR-023)
- Use custom error types for different failure modes (RPC errors, validation errors, algorithm errors)
- Return errors early - fail fast on invalid input

### Testing Strategy
- Unit tests for each algorithm implementation
- Integration tests comparing results to on-chain elections
- Deterministic test harnesses using fixed input data (FR-016)
- Test edge cases: zero validators, zero nominators, insufficient candidates, etc.

### Performance Optimization
- Use efficient data structures (Vec for candidates/nominators, HashMap for lookups)
- Minimize allocations in hot paths
- Consider parallel processing for large datasets (if algorithm supports it)
- Profile and optimize based on real-world usage patterns

### Compatibility Maintenance
- Pin Substrate dependency versions in `Cargo.toml` initially
- Test against multiple Substrate versions
- Monitor Substrate releases for breaking changes
- Update dependencies regularly to maintain compatibility (FR-017)

---

## Dependencies Summary

**Core Dependencies**:
- `sp-npos-elections` - Election algorithm implementations
- `frame-election-provider-support` - Election provider utilities
- `pallet-election-provider-multi-phase` - Multi-phase algorithm support
- `parity-scale-codec` - SCALE encoding/decoding
- `jsonrpsee-http-client` - RPC client
- `axum` - REST API framework
- `tokio` - Async runtime
- `clap` - CLI interface
- `serde`, `serde_json` - Serialization

**Development Dependencies**:
- `cargo-test` - Testing framework (built-in)
- Additional testing utilities as needed

---

## Open Questions Resolved

✅ **Rust Version**: Rust 1.70+ (MSRV)  
✅ **RPC Client**: `jsonrpsee-http-client`  
✅ **REST Framework**: `axum`  
✅ **Algorithm Abstraction**: Trait-based design  
✅ **Input Format**: JSON with defined schema  

All technical unknowns from the Technical Context section have been resolved.



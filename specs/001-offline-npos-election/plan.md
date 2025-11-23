# Implementation Plan: Offline NPoS Election Tool

**Branch**: `001-offline-npos-election` | **Date**: 2025-01-27 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-offline-npos-election/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Build a Rust-based offline NPoS (Nominated Proof of Stake) election tool that exactly mirrors the election logic of any Substrate chain. Refactor the legacy substrate-debug-kit/offline-election into a modern Rust project with clean library/CLI architecture, using Substrate's native crates (sp-npos-elections, frame-election-provider-support, pallet-election-provider-multi-phase, parity-scale-codec) to ensure bit-for-bit matching with on-chain elections. The tool will support multiple election algorithms (sequential phragmen, parallel phragmen, multi-phase) via trait-based abstraction, flexible input sources (RPC, JSON files, synthetic data), parameter overrides, and provide both CLI and REST API interfaces with extensive diagnostics.

## Technical Context

**Language/Version**: Rust 1.70+ (MSRV - Minimum Supported Rust Version)  
**Primary Dependencies**: 
- Substrate crates: `sp-npos-elections`, `frame-election-provider-support`, `pallet-election-provider-multi-phase`
- `parity-scale-codec` for encoding/decoding
- `clap` for CLI interface
- `jsonrpsee-http-client` for Substrate RPC communication
- `axum` for REST API framework
- `tokio` for async runtime
- JSON serialization: `serde`, `serde_json`

**Storage**: N/A (stateless tool - processes election data in memory)  
**Testing**: `cargo test` with unit tests, integration tests, and deterministic test harnesses  
**Target Platform**: Cross-platform (Linux, macOS, Windows) - Rust native compilation  
**Project Type**: Single Rust project with library + CLI + optional REST server  
**Performance Goals**: 
- RPC data fetching: <30 seconds for typical network conditions (SC-002)
- REST API response: <5 seconds for typical election sizes (SC-010)
- Process up to 1,000 validator candidates and 10,000 nominators without degradation (SC-011)

**Constraints**: 
- Must produce bit-for-bit identical results to on-chain elections (SC-001)
- Must maintain compatibility with latest Substrate runtime versions (FR-017)
- Deterministic results - same inputs produce identical outputs (SC-008)
- Offline-capable (file-based and synthetic data modes)

**Scale/Scope**: 
- Support up to 1,000 validator candidates and 10,000 nominators (SC-011)
- Multiple election algorithms (sequential phragmen, parallel phragmen, multi-phase)
- Multiple input sources (RPC, JSON files, synthetic data)
- CLI + REST API + programmatic library API

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

**Status**: NEEDS CLARIFICATION - Constitution file is a template and needs to be populated with actual project principles.

**Note**: The constitution file at `.specify/memory/constitution.md` appears to be a template. The following checks are based on common Rust project best practices and the feature requirements:

- ✅ **Library-First**: Project structure includes both library (`lib/`) and CLI (`cli/`) components
- ✅ **CLI Interface**: CLI is explicitly required (FR-011) with JSON output support (FR-013)
- ⚠️ **Test-First**: Feature spec mentions deterministic test harnesses (FR-016) but TDD approach needs clarification
- ✅ **Integration Testing**: Required for verifying bit-for-bit matching with on-chain elections
- ✅ **Observability**: JSON output and diagnostics (FR-015) provide structured output
- ✅ **Simplicity**: Single Rust project structure, no unnecessary complexity

**Action Required**: Constitution file needs to be populated with actual project principles before final gate evaluation.

## Project Structure

### Documentation (this feature)

```text
specs/[###-feature]/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)
<!--
  ACTION REQUIRED: Replace the placeholder tree below with the concrete layout
  for this feature. Delete unused options and expand the chosen structure with
  real paths (e.g., apps/admin, packages/something). The delivered plan must
  not include Option labels.
-->

```text
src/
├── lib.rs                    # Library entry point
├── models/                   # Data models (ElectionData, ElectionConfig, ElectionResult, etc.)
│   ├── mod.rs
│   ├── election_data.rs
│   ├── election_config.rs
│   ├── election_result.rs
│   ├── validator.rs
│   ├── nominator.rs
│   └── voting_edge.rs
├── algorithms/               # Election algorithm implementations
│   ├── mod.rs
│   ├── trait.rs              # ElectionAlgorithm trait
│   ├── sequential_phragmen.rs
│   ├── parallel_phragmen.rs
│   └── multi_phase.rs
├── input/                    # Input data loading (RPC, JSON, synthetic)
│   ├── mod.rs
│   ├── rpc.rs
│   ├── json.rs
│   └── synthetic.rs
├── diagnostics/              # Diagnostic generation
│   ├── mod.rs
│   └── explainer.rs
├── cli/                      # CLI interface
│   ├── mod.rs
│   ├── commands.rs
│   └── output.rs
└── api/                      # REST API server (optional)
    ├── mod.rs
    ├── server.rs
    └── handlers.rs

tests/
├── unit/                     # Unit tests for individual modules
├── integration/              # Integration tests for full election flows
└── contract/                 # Contract tests for API compatibility
```

**Structure Decision**: Single Rust project with modular library structure. The library (`src/lib.rs`) exposes the core election functionality, CLI (`src/cli/`) provides command-line interface, and optional REST API (`src/api/`) enables server mode. This structure supports all three usage modes: programmatic API, CLI, and REST API server.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| [e.g., 4th project] | [current need] | [why 3 projects insufficient] |
| [e.g., Repository pattern] | [specific problem] | [why direct DB access insufficient] |

# Implementation Tasks: Offline NPoS Election Tool

**Feature**: 001-offline-npos-election  
**Date**: 2025-01-27  
**Branch**: `001-offline-npos-election`

## Overview

This document provides an actionable, dependency-ordered task list for implementing the Offline NPoS Election Tool. Tasks are organized by phase, with each user story forming an independently testable increment.

**Total Tasks**: 154  
**MVP Scope**: Phase 1-3 (Setup + Foundational + User Story 1) - 57 tasks

## Implementation Strategy

**MVP First**: Start with Phase 1 (Setup), Phase 2 (Foundational), and Phase 3 (User Story 1) to deliver core election simulation capability. This enables immediate value: users can fetch on-chain data and run elections.

**Incremental Delivery**: Each user story phase is independently testable and delivers value. Complete phases sequentially by priority (P1 → P2 → P3).

**Parallel Execution**: Tasks marked with `[P]` can be executed in parallel within the same phase, as they work on different files with no dependencies on incomplete tasks.

## Dependencies

### User Story Completion Order

```
Phase 1: Setup
    ↓
Phase 2: Foundational (blocking prerequisites)
    ↓
Phase 3: User Story 1 (P1) - Run Election from On-Chain State
    ↓
Phase 4: User Story 2 (P1) - Run Election with Custom Parameters
    ↓
Phase 5: User Story 3 (P2) - Compare Different Election Algorithms
    ↓
Phase 6: User Story 4 (P2) - Get Detailed Election Diagnostics
    ↓
Phase 7: User Story 5 (P3) - Use Tool Programmatically via API
    ↓
Phase 8: User Story 6 (P3) - Access Tool via REST API Server
    ↓
Phase 9: Polish & Cross-Cutting Concerns
```

### Story Dependencies

- **US1** (Phase 3): Requires Phase 1 & 2 (setup + foundational models/algorithms)
- **US2** (Phase 4): Requires US1 (needs working election execution)
- **US3** (Phase 5): Requires US1 (needs working election execution)
- **US4** (Phase 6): Requires US1 (needs election results)
- **US5** (Phase 7): Requires US1 (needs working library API)
- **US6** (Phase 8): Requires US1 & US5 (needs working library API)

## Phase 1: Setup

**Goal**: Initialize Rust project structure and dependencies.

**Independent Test**: Project builds successfully with `cargo build`, all dependencies resolve, and project structure matches plan.md.

- [X] T001 Create Cargo.toml with project metadata and dependencies in project root
- [X] T002 Create src/lib.rs as library entry point
- [X] T003 Create src/main.rs as CLI binary entry point
- [X] T004 Create src/models/mod.rs module declaration
- [X] T005 Create src/algorithms/mod.rs module declaration
- [X] T006 Create src/input/mod.rs module declaration
- [X] T007 Create src/diagnostics/mod.rs module declaration
- [X] T008 Create src/cli/mod.rs module declaration
- [X] T009 Create src/api/mod.rs module declaration
- [X] T010 Create tests/unit/ directory for unit tests
- [X] T011 Create tests/integration/ directory for integration tests
- [X] T012 Create tests/contract/ directory for contract tests
- [X] T013 Create .gitignore with Rust patterns in project root
- [X] T014 Create README.md with project overview in project root

## Phase 2: Foundational

**Goal**: Implement core data models, error types, and algorithm trait abstraction. These are blocking prerequisites for all user stories.

**Independent Test**: Models can be instantiated, serialized to JSON, and validated. Algorithm trait can be implemented. Error types provide clear messages.

### Data Models

- [X] T015 [P] Create src/models/election_data.rs with ElectionData struct
- [X] T016 [P] Create src/models/election_config.rs with ElectionConfiguration struct
- [X] T017 [P] Create src/models/election_result.rs with ElectionResult struct
- [X] T018 [P] Create src/models/validator.rs with ValidatorCandidate struct
- [X] T019 [P] Create src/models/nominator.rs with Nominator struct
- [X] T020 [P] Create src/models/voting_edge.rs with VotingEdge struct
- [X] T021 [P] Create src/models/election_overrides.rs with ElectionOverrides struct
- [X] T022 Implement ElectionData validation logic in src/models/election_data.rs (unique account IDs, minimum candidates/nominators)
- [X] T023 Implement ElectionConfiguration validation logic in src/models/election_config.rs (active set size, algorithm validation)
- [X] T024 Export all models from src/models/mod.rs

### Error Types

- [X] T025 Create src/error.rs with ElectionError enum (ValidationError, RpcError, AlgorithmError, InsufficientCandidates, InvalidData, FileError)
- [X] T026 Implement Display and Error traits for ElectionError in src/error.rs

### Algorithm Trait

- [X] T027 Create src/algorithms/trait.rs with ElectionAlgorithm trait definition
- [X] T028 Create src/algorithms/mod.rs and export trait

### Type Definitions

- [X] T029 Create src/types.rs with AlgorithmType enum (SequentialPhragmen, ParallelPhragmen, MultiPhase)
- [X] T030 Create src/types.rs with DataSource enum (Rpc, JsonFile, Synthetic)

## Phase 3: User Story 1 - Run Election Simulation from On-Chain State (P1)

**Goal**: Enable users to fetch election data from a public Substrate node and run an election simulation.

**Independent Test**: Can connect to public Substrate RPC, fetch state at a specific block, run an election, and produce results. Compare results to actual on-chain election outcome at that block.

**Story**: A validator operator needs to predict election outcomes before they occur on-chain. They connect to a public Substrate node, specify a block number to snapshot the state, and run an election simulation to see which validators would be selected and how stake would be distributed.

### RPC Input Layer

- [X] T031 [US1] Create src/input/rpc.rs with RpcLoader struct
- [X] T032 [US1] Implement RpcLoader::new() constructor in src/input/rpc.rs
- [X] T033 [US1] Implement RpcLoader::load_at_block() method in src/input/rpc.rs
- [X] T034 [US1] Implement RPC calls for fetching validator candidates in src/input/rpc.rs
- [X] T035 [US1] Implement RPC calls for fetching nominator data in src/input/rpc.rs
- [X] T036 [US1] Implement RPC calls for fetching stake information in src/input/rpc.rs
- [X] T037 [US1] Implement error handling for RPC connection failures in src/input/rpc.rs
- [X] T038 [US1] Implement error handling for missing blocks in src/input/rpc.rs
- [X] T039 [US1] Export RpcLoader from src/input/mod.rs

### Algorithm Implementations

- [X] T040 [US1] Create src/algorithms/sequential_phragmen.rs with SequentialPhragmen struct
- [X] T041 [US1] Implement ElectionAlgorithm trait for SequentialPhragmen in src/algorithms/sequential_phragmen.rs
- [X] T042 [US1] Integrate sp-npos-elections crate for sequential phragmen algorithm in src/algorithms/sequential_phragmen.rs
- [X] T043 [US1] Export SequentialPhragmen from src/algorithms/mod.rs

### Election Engine

- [X] T044 [US1] Create src/engine.rs with ElectionEngine struct
- [X] T045 [US1] Implement ElectionEngine::new() constructor in src/engine.rs
- [X] T046 [US1] Implement ElectionEngine::execute() method in src/engine.rs
- [X] T047 [US1] Implement algorithm selection logic in src/engine.rs
- [X] T048 [US1] Implement result validation in src/engine.rs

### CLI Interface

- [X] T049 [US1] Create src/cli/commands.rs with RunCommand struct
- [X] T050 [US1] Implement clap command definition for run command in src/cli/commands.rs
- [X] T051 [US1] Implement run command execution logic in src/cli/commands.rs
- [X] T052 [US1] Create src/cli/output.rs with JSON output formatting
- [X] T053 [US1] Implement JSON serialization for ElectionResult in src/cli/output.rs
- [X] T054 [US1] Export CLI modules from src/cli/mod.rs
- [X] T055 [US1] Integrate CLI commands into src/main.rs

### Library API

- [X] T056 [US1] Export ElectionEngine, ElectionConfig, ElectionData from src/lib.rs
- [X] T057 [US1] Export input loaders from src/lib.rs

## Phase 4: User Story 2 - Run Election with Custom Parameters (P1)

**Goal**: Enable users to override election parameters (active set size, stakes, voting edges) and run elections with modified data.

**Independent Test**: Can load election data, apply parameter overrides (e.g., change active set size from 100 to 50), run election, and verify results reflect changes.

**Story**: A researcher needs to test "what-if" scenarios by modifying election parameters. They load election data from a file or RPC, then override specific parameters like active validator set size, add synthetic candidates with no stake, modify voting edges, or adjust stake amounts to understand how these changes affect election outcomes.

### Parameter Overrides

- [ ] T058 [US2] Implement ElectionOverrides::set_candidate_stake() in src/models/election_overrides.rs
- [ ] T059 [US2] Implement ElectionOverrides::set_nominator_stake() in src/models/election_overrides.rs
- [ ] T060 [US2] Implement ElectionOverrides::add_voting_edge() in src/models/election_overrides.rs
- [ ] T061 [US2] Implement ElectionOverrides::remove_voting_edge() in src/models/election_overrides.rs
- [ ] T062 [US2] Implement ElectionOverrides::modify_voting_edge() in src/models/election_overrides.rs
- [ ] T063 [US2] Implement override application logic in ElectionEngine::execute() in src/engine.rs

### JSON Input Layer

- [ ] T064 [US2] Create src/input/json.rs with JsonLoader struct
- [ ] T065 [US2] Implement JsonLoader::load_from_file() method in src/input/json.rs
- [ ] T066 [US2] Implement JSON deserialization for ElectionData in src/input/json.rs
- [ ] T067 [US2] Implement error handling for invalid JSON in src/input/json.rs
- [ ] T068 [US2] Export JsonLoader from src/input/mod.rs

### Synthetic Data Input

- [ ] T069 [US2] Create src/input/synthetic.rs with SyntheticDataBuilder struct
- [ ] T070 [US2] Implement SyntheticDataBuilder::new() constructor in src/input/synthetic.rs
- [ ] T071 [US2] Implement SyntheticDataBuilder::add_candidate() method in src/input/synthetic.rs
- [ ] T072 [US2] Implement SyntheticDataBuilder::add_nominator() method in src/input/synthetic.rs
- [ ] T073 [US2] Implement SyntheticDataBuilder::build() method in src/input/synthetic.rs
- [ ] T074 [US2] Export SyntheticDataBuilder from src/input/mod.rs

### CLI Enhancements

- [ ] T075 [US2] Add --input-file flag to run command in src/cli/commands.rs
- [ ] T076 [US2] Add --override-candidate-stake flag to run command in src/cli/commands.rs
- [ ] T077 [US2] Add --override-nominator-stake flag to run command in src/cli/commands.rs
- [ ] T078 [US2] Add --synthetic flag and synthetic data options to run command in src/cli/commands.rs

## Phase 5: User Story 3 - Compare Different Election Algorithms (P2)

**Goal**: Enable users to run the same election data through multiple algorithms and compare results.

**Independent Test**: Can load a single election dataset, run it through each available algorithm, and compare output results showing differences in selected validators and stake distribution.

**Story**: A developer needs to understand how different election algorithms (sequential phragmen, parallel phragmen, multi-phase) produce different results given the same input data. They run the same election data through multiple algorithms and compare the selected validators and stake distribution.

### Additional Algorithm Implementations

- [ ] T079 [US3] Create src/algorithms/parallel_phragmen.rs with ParallelPhragmen struct
- [ ] T080 [US3] Implement ElectionAlgorithm trait for ParallelPhragmen in src/algorithms/parallel_phragmen.rs
- [ ] T081 [US3] Integrate sp-npos-elections crate for parallel phragmen algorithm in src/algorithms/parallel_phragmen.rs
- [ ] T082 [US3] Export ParallelPhragmen from src/algorithms/mod.rs
- [ ] T083 [US3] Create src/algorithms/multi_phase.rs with MultiPhase struct
- [ ] T084 [US3] Implement ElectionAlgorithm trait for MultiPhase in src/algorithms/multi_phase.rs
- [ ] T085 [US3] Integrate pallet-election-provider-multi-phase crate for multi-phase algorithm in src/algorithms/multi_phase.rs
- [ ] T086 [US3] Export MultiPhase from src/algorithms/mod.rs

### Algorithm Selection

- [ ] T087 [US3] Update ElectionEngine::execute() to support all algorithm types in src/engine.rs
- [ ] T088 [US3] Update AlgorithmType enum parsing in src/types.rs

### CLI Enhancements

- [ ] T089 [US3] Update run command to accept all algorithm types in src/cli/commands.rs

## Phase 6: User Story 4 - Get Detailed Election Diagnostics (P2)

**Goal**: Provide detailed diagnostics explaining why each validator was selected or not selected, stake distribution analysis, and algorithm-specific insights.

**Independent Test**: Can run any election and verify that output includes explanations for validator selection decisions, stake distribution details, and algorithm-specific insights.

**Story**: A validator wants to understand why they were or weren't selected in an election. They run an election and receive detailed diagnostics explaining the selection criteria, stake calculations, voting patterns, and reasoning for each validator's inclusion or exclusion.

### Diagnostics Generation

- [ ] T090 [US4] Create src/diagnostics/explainer.rs with DiagnosticsGenerator struct
- [ ] T091 [US4] Implement DiagnosticsGenerator::new() constructor in src/diagnostics/explainer.rs
- [ ] T092 [US4] Implement DiagnosticsGenerator::generate() method in src/diagnostics/explainer.rs
- [ ] T093 [US4] Implement validator explanation generation in src/diagnostics/explainer.rs
- [ ] T094 [US4] Implement stake analysis generation in src/diagnostics/explainer.rs
- [ ] T095 [US4] Implement algorithm-specific insights generation in src/diagnostics/explainer.rs
- [ ] T096 [US4] Create src/diagnostics/models.rs with Diagnostics, ValidatorExplanation, StakeAnalysis structs
- [ ] T097 [US4] Export DiagnosticsGenerator from src/diagnostics/mod.rs

### Integration

- [ ] T098 [US4] Integrate diagnostics generation into ElectionEngine::execute() in src/engine.rs
- [ ] T099 [US4] Add diagnostics to ElectionResult struct in src/models/election_result.rs

### CLI Enhancements

- [ ] T100 [US4] Add --diagnostics flag to run command in src/cli/commands.rs
- [ ] T101 [US4] Include diagnostics in JSON output when flag is set in src/cli/output.rs

## Phase 7: User Story 5 - Use Tool Programmatically via API (P3)

**Goal**: Provide a clean programmatic API for developers to integrate election simulation into their applications.

**Independent Test**: Can write a simple program that uses the API to load data, configure an election, run it, and retrieve results programmatically.

**Story**: A developer wants to integrate election simulation into their own application. They use the programmatic API to configure elections, execute them, and retrieve results without using the command-line interface.

### API Refinement

- [ ] T102 [US5] Refine ElectionConfig builder API in src/models/election_config.rs
- [ ] T103 [US5] Add ElectionData::from_rpc() convenience method in src/models/election_data.rs
- [ ] T104 [US5] Add ElectionResult accessor methods in src/models/election_result.rs
- [ ] T105 [US5] Add ElectionResult::to_json() method in src/models/election_result.rs

### Documentation

- [ ] T106 [US5] Add doc comments to all public API types in src/lib.rs
- [ ] T107 [US5] Add doc comments to ElectionEngine in src/engine.rs
- [ ] T108 [US5] Add doc comments to input loaders in src/input/mod.rs

### Library Organization

- [ ] T109 [US5] Organize public API exports in src/lib.rs according to contracts/programmatic-api.md
- [ ] T110 [US5] Ensure all API types are Send + Sync where appropriate

## Phase 8: User Story 6 - Access Tool via REST API Server (P3)

**Goal**: Provide a REST API server for remote access to election simulation capabilities.

**Independent Test**: Can start server, send HTTP requests with election data, and verify JSON responses contain valid results.

**Story**: A team wants to run election simulations through a web service without installing the tool locally. They start a server, send HTTP requests with election parameters, and receive JSON responses with results.

### REST API Server

- [ ] T111 [US6] Create src/api/server.rs with ApiServer struct
- [ ] T112 [US6] Implement ApiServer::new() constructor in src/api/server.rs
- [ ] T113 [US6] Implement ApiServer::start() method in src/api/server.rs
- [ ] T114 [US6] Set up axum router with /elections/run endpoint in src/api/server.rs
- [ ] T115 [US6] Set up axum router with /elections/{id}/results endpoint in src/api/server.rs
- [ ] T116 [US6] Set up axum router with /elections/{id}/diagnostics endpoint in src/api/server.rs

### Request Handlers

- [ ] T117 [US6] Create src/api/handlers.rs with election request handlers
- [ ] T118 [US6] Implement POST /elections/run handler in src/api/handlers.rs
- [ ] T119 [US6] Implement GET /elections/{id}/results handler in src/api/handlers.rs
- [ ] T120 [US6] Implement GET /elections/{id}/diagnostics handler in src/api/handlers.rs
- [ ] T121 [US6] Implement request validation in src/api/handlers.rs
- [ ] T122 [US6] Implement error response formatting in src/api/handlers.rs

### Request/Response Models

- [ ] T123 [US6] Create src/api/models.rs with ElectionRequest, ElectionResponse structs
- [ ] T124 [US6] Implement JSON serialization for API models in src/api/models.rs
- [ ] T125 [US6] Implement JSON deserialization for API models in src/api/models.rs

### Election Storage

- [ ] T126 [US6] Create in-memory election storage in src/api/server.rs
- [ ] T127 [US6] Implement election ID generation in src/api/server.rs

### CLI Integration

- [ ] T128 [US6] Create src/cli/commands.rs with ServerCommand struct
- [ ] T129 [US6] Implement server command in src/cli/commands.rs
- [ ] T130 [US6] Add --port and --host flags to server command in src/cli/commands.rs
- [ ] T131 [US6] Integrate server command into src/main.rs

### Export

- [ ] T132 [US6] Export API modules from src/api/mod.rs

## Phase 9: Polish & Cross-Cutting Concerns

**Goal**: Finalize implementation, add error handling improvements, validation enhancements, and ensure all requirements are met.

**Independent Test**: All functional requirements pass, error messages are clear, validation catches edge cases, and system handles all specified error conditions gracefully.

### Error Handling Improvements

- [ ] T133 Implement detailed error messages for all error types in src/error.rs
- [ ] T134 Add error context for RPC failures in src/input/rpc.rs
- [ ] T135 Add error context for algorithm failures in src/algorithms/mod.rs
- [ ] T136 Add error context for validation failures in src/models/mod.rs

### Validation Enhancements

- [ ] T137 Implement duplicate account ID detection in ElectionData::validate() in src/models/election_data.rs
- [ ] T138 Implement zero validator/nominator detection in ElectionData::validate() in src/models/election_data.rs
- [ ] T139 Implement active set size validation in ElectionConfiguration::validate() in src/models/election_config.rs
- [ ] T140 Implement voting edge reference validation in ElectionData::validate() in src/models/election_data.rs

### Algorithm Error Handling

- [ ] T141 Implement algorithm convergence detection in src/algorithms/sequential_phragmen.rs
- [ ] T142 Implement algorithm convergence detection in src/algorithms/parallel_phragmen.rs
- [ ] T143 Implement algorithm convergence detection in src/algorithms/multi_phase.rs
- [ ] T144 Return detailed diagnostics on algorithm failure in src/engine.rs

### Output Formatting

- [ ] T145 Add human-readable output format option in src/cli/output.rs
- [ ] T146 Implement --format flag in run command in src/cli/commands.rs
- [ ] T147 Add --output-file flag to run command in src/cli/commands.rs

### Documentation

- [ ] T148 Update README.md with installation and usage instructions
- [ ] T149 Add code examples to README.md
- [ ] T150 Add troubleshooting section to README.md

### Performance Optimization

- [ ] T151 Optimize data structures for large datasets (1000+ candidates, 10000+ nominators)
- [ ] T152 Add performance benchmarks for election execution

### Compatibility

- [ ] T153 Verify compatibility with latest Substrate runtime versions
- [ ] T154 Test with multiple Substrate chain configurations

## Parallel Execution Examples

### Phase 3 (US1) - Parallel Opportunities

Tasks T031-T039 (RPC input layer) can be worked on in parallel with T040-T043 (algorithm implementations), as they are independent modules. However, T044-T048 (Election Engine) depends on both, so should be done after.

**Example parallel execution**:
- Developer A: T031-T039 (RPC input layer)
- Developer B: T040-T043 (Algorithm implementations)
- Developer C: T044-T048 (Election Engine) - after A and B complete

### Phase 4 (US2) - Parallel Opportunities

Tasks T058-T062 (parameter overrides) can be done in parallel with T064-T068 (JSON input) and T069-T074 (synthetic data), as they are independent features.

**Example parallel execution**:
- Developer A: T058-T062 (Parameter overrides)
- Developer B: T064-T068 (JSON input)
- Developer C: T069-T074 (Synthetic data)

### Phase 5 (US3) - Parallel Opportunities

Tasks T079-T082 (parallel phragmen) and T083-T086 (multi-phase) can be implemented in parallel.

**Example parallel execution**:
- Developer A: T079-T082 (Parallel Phragmen)
- Developer B: T083-T086 (Multi-phase)

### Phase 6 (US4) - Parallel Opportunities

Tasks T090-T097 (diagnostics generation) can be worked on independently of T098-T101 (integration).

**Example parallel execution**:
- Developer A: T090-T097 (Diagnostics generation)
- Developer B: T098-T101 (Integration) - after A completes

## Task Summary

- **Phase 1 (Setup)**: 14 tasks
- **Phase 2 (Foundational)**: 16 tasks
- **Phase 3 (US1 - P1)**: 27 tasks
- **Phase 4 (US2 - P1)**: 21 tasks
- **Phase 5 (US3 - P2)**: 11 tasks
- **Phase 6 (US4 - P2)**: 12 tasks
- **Phase 7 (US5 - P3)**: 9 tasks
- **Phase 8 (US6 - P3)**: 22 tasks
- **Phase 9 (Polish)**: 22 tasks

**Total**: 154 tasks

## MVP Scope

For initial delivery, focus on **Phases 1-3** (57 tasks total):
- Phase 1: Setup (14 tasks)
- Phase 2: Foundational (16 tasks)
- Phase 3: User Story 1 (27 tasks)

This delivers the core capability: fetching on-chain data and running election simulations, which provides immediate value to users.


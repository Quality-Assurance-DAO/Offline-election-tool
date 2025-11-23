---
description: "Task list for Polkadot Mainnet Performance Benchmarks feature implementation"
---

# Tasks: Polkadot Mainnet Performance Benchmarks

**Input**: Design documents from `/specs/003-polkadot-mainnet-benchmarks/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: Tests are REQUIRED for this feature - integration tests for performance benchmarks are the core deliverable.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3, US4)
- Include exact file paths in descriptions

## Path Conventions

- **Single project**: `src/`, `tests/` at repository root
- Paths shown below assume single Rust project structure

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic infrastructure

- [X] T001 Create benchmark results directory structure: tests/fixtures/benchmarks/
- [X] T002 [P] Add libc dependency to Cargo.toml for macOS memory measurement (version 0.2 or later)
- [X] T003 [P] Add winapi dependency to Cargo.toml for Windows memory measurement (version 0.3 or later, features: winbase, psapi)
- [X] T004 [P] Verify existing dependencies in Cargo.toml: jsonrpsee, tokio, serde, serde_json, chrono

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [X] T005 Create MemoryMeasurementError enum in tests/common/memory_measurement.rs with variants: UnsupportedPlatform, MeasurementFailed(String), PlatformError(String)
- [X] T006 [P] Create MemoryMeasurer trait in tests/common/memory_measurement.rs with methods: measure_peak_memory_mb() -> Result<u64, MemoryMeasurementError>, measure_current_memory_mb() -> Result<u64, MemoryMeasurementError>
- [X] T007 [P] [US2] Implement LinuxMemoryMeasurer in tests/common/memory_measurement.rs using /proc/self/status to read VmPeak and VmRSS
- [X] T008 [P] [US2] Implement MacOSMemoryMeasurer in tests/common/memory_measurement.rs using mach_task_basic_info via libc to get resident_size and virtual_size
- [X] T009 [P] [US2] Implement WindowsMemoryMeasurer in tests/common/memory_measurement.rs using GetProcessMemoryInfo from winapi to get PeakWorkingSetSize and WorkingSetSize
- [X] T010 [P] [US2] Implement UnsupportedMemoryMeasurer in tests/common/memory_measurement.rs for unsupported platforms returning UnsupportedPlatform error
- [X] T011 Enhance measure_memory_usage() function in tests/common/benchmark_utils.rs to use platform-specific MemoryMeasurer implementations with graceful degradation
- [X] T012 Enhance output_benchmark_json() function in tests/common/benchmark_utils.rs to include Polkadot-specific metadata fields: block_number, chain, rpc_endpoint, threshold_ms, threshold_passed, memory_measurement_available
- [X] T013 Create helper function calculate_recent_block_number() in tests/common/rpc_utils.rs to calculate block number within last 30 days (Polkadot block time ~6 seconds, ~432,000 blocks per 30 days)

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Benchmark Performance with Real Polkadot Mainnet Data (Priority: P1) 🎯 MVP

**Goal**: Enable developers to run performance benchmarks with real Polkadot mainnet data, measuring execution time and validating against algorithm-specific thresholds.

**Independent Test**: Can be fully tested by fetching real Polkadot mainnet data from a recent block, running an election simulation, and measuring execution time. The test outputs structured benchmark results that can be compared against targets and tracked over time.

### Tests for User Story 1 ⚠️

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [X] T014 [P] [US1] Create integration test test_polkadot_mainnet_performance_sequential in tests/integration/performance/test_polkadot_mainnet.rs with #[ignore] attribute, fetching data from recent block and measuring execution time for sequential-phragmen algorithm
- [X] T015 [P] [US1] Create integration test test_polkadot_mainnet_performance_parallel in tests/integration/performance/test_polkadot_mainnet.rs with #[ignore] attribute, fetching data from recent block and measuring execution time for parallel-phragmen algorithm
- [X] T016 [P] [US1] Create integration test test_polkadot_mainnet_performance_multiphase in tests/integration/performance/test_polkadot_mainnet.rs with #[ignore] attribute, fetching data from recent block and measuring execution time for multi-phase algorithm
- [X] T017 [US1] Add threshold validation assertions to all three benchmark tests: sequential-phragmen < 30s, parallel-phragmen < 15s, multi-phase < 45s in tests/integration/performance/test_polkadot_mainnet.rs

### Implementation for User Story 1

- [X] T018 [US1] Enhance RpcLoader::load_at_block() in src/input/rpc.rs to use retry_with_backoff from tests/common/rpc_retry.rs with max_attempts: 3, initial_delay: Duration::from_secs(1) for benchmark tests (implemented via fetch_polkadot_mainnet_snapshot wrapper)
- [X] T019 [US1] Create helper function fetch_polkadot_mainnet_snapshot() in tests/common/rpc_utils.rs that uses RpcLoader with retry logic and returns PolkadotMainnetSnapshot with election_data, block_number, rpc_endpoint, fetch_timestamp
- [X] T020 [US1] Create helper function run_benchmark_with_algorithm() in tests/common/benchmark_utils.rs that takes ElectionData, AlgorithmType, and active_set_size, measures execution time, and returns BenchmarkResult
- [X] T021 [US1] Implement benchmark execution logic in test_polkadot_mainnet_performance_sequential test: fetch snapshot, run benchmark, validate threshold, output results
- [X] T022 [US1] Implement benchmark execution logic in test_polkadot_mainnet_performance_parallel test: fetch snapshot, run benchmark, validate threshold, output results
- [X] T023 [US1] Implement benchmark execution logic in test_polkadot_mainnet_performance_multiphase test: fetch snapshot, run benchmark, validate threshold, output results
- [X] T024 [US1] Add error handling for RPC failures with alternative endpoint suggestions in all three benchmark tests in tests/integration/performance/test_polkadot_mainnet.rs
- [X] T025 [US1] Add block number validation (within last 30 days) with optional override parameter in benchmark tests in tests/integration/performance/test_polkadot_mainnet.rs

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently. Developers can run benchmarks with real Polkadot mainnet data and receive execution time measurements.

---

## Phase 4: User Story 2 - Measure Memory Usage During Benchmarks (Priority: P2)

**Goal**: Enable developers to understand memory consumption patterns when running elections with real Polkadot mainnet data by measuring and reporting peak memory usage.

**Independent Test**: Can be fully tested by running a benchmark test and verifying that memory usage is measured and reported in the benchmark results. The measurement should capture peak memory usage during election execution.

### Tests for User Story 2 ⚠️

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [ ] T026 [P] [US2] Create unit test test_memory_measurement_linux in tests/unit/test_memory_measurement.rs for LinuxMemoryMeasurer (if running on Linux)
- [ ] T027 [P] [US2] Create unit test test_memory_measurement_macos in tests/unit/test_memory_measurement.rs for MacOSMemoryMeasurer (if running on macOS)
- [ ] T028 [P] [US2] Create unit test test_memory_measurement_windows in tests/unit/test_memory_measurement.rs for WindowsMemoryMeasurer (if running on Windows)
- [ ] T029 [P] [US2] Create unit test test_memory_measurement_unsupported_platform in tests/unit/test_memory_measurement.rs for UnsupportedMemoryMeasurer returning UnsupportedPlatform error
- [ ] T030 [US2] Enhance test_polkadot_mainnet_performance_sequential in tests/integration/performance/test_polkadot_mainnet.rs to verify memory measurement is included in benchmark results
- [ ] T031 [US2] Add test assertion for graceful degradation when memory measurement fails (memory_mb = 0, memory_measurement_available = false) in tests/integration/performance/test_polkadot_mainnet.rs

### Implementation for User Story 2

- [ ] T032 [US2] Integrate memory measurement into run_benchmark_with_algorithm() function in tests/common/benchmark_utils.rs: measure memory before and after execution, calculate peak memory
- [ ] T033 [US2] Update create_benchmark_results() function in tests/common/benchmark_utils.rs to include memory_peak_mb and memory_final_mb in BenchmarkResults
- [ ] T034 [US2] Add memory measurement to all three benchmark tests (sequential, parallel, multiphase) in tests/integration/performance/test_polkadot_mainnet.rs
- [ ] T035 [US2] Add memory_measurement_available flag to benchmark metadata in output_benchmark_json() function in tests/common/benchmark_utils.rs
- [ ] T036 [US2] Add warning message output when memory measurement is unavailable (unsupported platform) in benchmark tests in tests/integration/performance/test_polkadot_mainnet.rs

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently. Benchmarks measure both execution time and memory usage.

---

## Phase 5: User Story 3 - Document Performance Characteristics (Priority: P2)

**Goal**: Enable developers and users to understand expected performance characteristics for Polkadot mainnet scale elections by documenting benchmark results in structured JSON files and markdown documentation.

**Independent Test**: Can be fully tested by running benchmarks with real Polkadot mainnet data and verifying that results are documented in a format that can be referenced and updated over time. Documentation should include execution times, memory usage, and hardware specifications.

### Tests for User Story 3 ⚠️

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [ ] T037 [P] [US3] Create unit test test_save_benchmark_json() in tests/unit/test_benchmark_storage.rs to verify JSON file is created in tests/fixtures/benchmarks/ with correct filename pattern: polkadot_mainnet_{block_number}_{algorithm}_{timestamp}.json
- [ ] T038 [P] [US3] Create unit test test_benchmark_json_schema() in tests/unit/test_benchmark_storage.rs to validate JSON output matches BenchmarkResults schema with all required Polkadot-specific fields
- [ ] T039 [US3] Create unit test test_append_markdown_documentation() in tests/unit/test_benchmark_storage.rs to verify results are appended to PERFORMANCE_BENCHMARKS.md with correct table format

### Implementation for User Story 3

- [ ] T040 [US3] Create helper function save_benchmark_json() in tests/common/benchmark_utils.rs that writes BenchmarkResults to JSON file in tests/fixtures/benchmarks/ with filename pattern: polkadot_mainnet_{block_number}_{algorithm}_{timestamp}.json
- [ ] T041 [US3] Create helper function append_performance_documentation() in tests/common/benchmark_utils.rs that appends benchmark results to PERFORMANCE_BENCHMARKS.md markdown file with table row format
- [ ] T042 [US3] Integrate save_benchmark_json() into all three benchmark tests (sequential, parallel, multiphase) in tests/integration/performance/test_polkadot_mainnet.rs
- [ ] T043 [US3] Integrate append_performance_documentation() into all three benchmark tests (sequential, parallel, multiphase) in tests/integration/performance/test_polkadot_mainnet.rs
- [ ] T044 [US3] Add hardware_info collection (cpu_cores, ram_gb, os, cpu_model) in benchmark tests using system information APIs in tests/integration/performance/test_polkadot_mainnet.rs
- [ ] T045 [US3] Add environment_info collection (rust_version, cargo_version, test_runner) in benchmark tests in tests/integration/performance/test_polkadot_mainnet.rs
- [ ] T046 [US3] Include hardware_info and environment_info in benchmark metadata when saving JSON and appending markdown in tests/common/benchmark_utils.rs

**Checkpoint**: At this point, User Stories 1, 2, AND 3 should all work independently. Benchmark results are documented in both JSON and markdown formats.

---

## Phase 6: User Story 4 - Integrate Performance Benchmarks into Test Suite (Priority: P3)

**Goal**: Enable performance benchmarks to run automatically as part of the test suite to detect performance regressions and ensure consistent performance characteristics over time.

**Independent Test**: Can be fully tested by running the test suite and verifying that performance benchmarks execute automatically, output results, and can be configured with performance thresholds that fail builds if exceeded.

### Tests for User Story 4 ⚠️

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [ ] T047 [US4] Create integration test test_benchmark_test_suite_integration() in tests/integration/performance/test_polkadot_mainnet.rs to verify benchmarks can be run with cargo test -- --ignored flag
- [ ] T048 [US4] Create integration test test_threshold_validation_failure() in tests/integration/performance/test_polkadot_mainnet.rs to verify test fails when execution time exceeds algorithm-specific threshold

### Implementation for User Story 4

- [ ] T049 [US4] Verify all benchmark tests have #[ignore] attribute in tests/integration/performance/test_polkadot_mainnet.rs to prevent automatic execution during standard test runs
- [ ] T050 [US4] Add clear error messages when threshold is exceeded in benchmark test assertions in tests/integration/performance/test_polkadot_mainnet.rs
- [ ] T051 [US4] Document benchmark execution instructions in PERFORMANCE_BENCHMARKS.md: cargo test --test test_polkadot_mainnet_performance -- --ignored --nocapture
- [ ] T052 [US4] Add threshold override capability for different hardware configurations in benchmark tests via environment variable or test parameter in tests/integration/performance/test_polkadot_mainnet.rs
- [ ] T053 [US4] Ensure benchmark results are still written to JSON and markdown even when threshold is exceeded in tests/integration/performance/test_polkadot_mainnet.rs

**Checkpoint**: All user stories should now be independently functional. Performance benchmarks are integrated into the test suite with proper threshold validation.

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [ ] T054 [P] Update PERFORMANCE_BENCHMARKS.md with Polkadot mainnet benchmark results section and table format documentation
- [ ] T055 [P] Add comprehensive error handling documentation for RPC failures, memory measurement failures, and threshold violations in tests/integration/performance/test_polkadot_mainnet.rs
- [ ] T056 [P] Add code comments explaining platform-specific memory measurement implementations in tests/common/memory_measurement.rs
- [ ] T057 [P] Verify all benchmark tests follow quickstart.md examples and scenarios in tests/integration/performance/test_polkadot_mainnet.rs
- [ ] T058 [P] Add validation for block number age (warn if >30 days old) in calculate_recent_block_number() function in tests/common/rpc_utils.rs
- [ ] T059 [P] Add validation for Polkadot mainnet scale characteristics (candidate_count ~300-400, nominator_count ~20k-30k) with warnings in benchmark tests in tests/integration/performance/test_polkadot_mainnet.rs
- [ ] T060 [P] Run quickstart.md validation: execute all example commands and verify they work correctly
- [ ] T061 [P] Code cleanup and refactoring: ensure consistent error handling patterns across all benchmark utilities
- [ ] T062 [P] Update tests/README.md with documentation for new Polkadot mainnet benchmark tests and memory measurement utilities

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3+)**: All depend on Foundational phase completion
  - User stories can then proceed in parallel (if staffed)
  - Or sequentially in priority order (P1 → P2 → P3)
- **Polish (Final Phase)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P2)**: Can start after Foundational (Phase 2) - Depends on memory measurement infrastructure from Foundational phase, integrates with US1 benchmark execution
- **User Story 3 (P2)**: Can start after Foundational (Phase 2) - Depends on US1 benchmark execution, integrates with US1 and US2 results
- **User Story 4 (P3)**: Can start after Foundational (Phase 2) - Depends on US1, US2, US3 completion for full test suite integration

### Within Each User Story

- Tests (if included) MUST be written and FAIL before implementation
- Models/traits before implementations
- Core implementation before integration
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel (T002, T003, T004)
- All Foundational tasks marked [P] can run in parallel within Phase 2 (T007, T008, T009, T010)
- Once Foundational phase completes, User Stories 1, 2, and 3 can start in parallel (if team capacity allows)
- All tests for a user story marked [P] can run in parallel
- Platform-specific memory measurers (T007, T008, T009, T010) can be implemented in parallel
- Different user stories can be worked on in parallel by different team members

---

## Parallel Example: User Story 1

```bash
# Launch all tests for User Story 1 together:
Task: "Create integration test test_polkadot_mainnet_performance_sequential in tests/integration/performance/test_polkadot_mainnet.rs"
Task: "Create integration test test_polkadot_mainnet_performance_parallel in tests/integration/performance/test_polkadot_mainnet.rs"
Task: "Create integration test test_polkadot_mainnet_performance_multiphase in tests/integration/performance/test_polkadot_mainnet.rs"
```

---

## Parallel Example: User Story 2

```bash
# Launch all platform-specific memory measurers together:
Task: "Implement LinuxMemoryMeasurer in tests/common/memory_measurement.rs"
Task: "Implement MacOSMemoryMeasurer in tests/common/memory_measurement.rs"
Task: "Implement WindowsMemoryMeasurer in tests/common/memory_measurement.rs"
Task: "Implement UnsupportedMemoryMeasurer in tests/common/memory_measurement.rs"

# Launch all memory measurement unit tests together:
Task: "Create unit test test_memory_measurement_linux in tests/unit/test_memory_measurement.rs"
Task: "Create unit test test_memory_measurement_macos in tests/unit/test_memory_measurement.rs"
Task: "Create unit test test_memory_measurement_windows in tests/unit/test_memory_measurement.rs"
Task: "Create unit test test_memory_measurement_unsupported_platform in tests/unit/test_memory_measurement.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1
4. **STOP and VALIDATE**: Test User Story 1 independently
5. Deploy/demo if ready

**MVP Deliverable**: Developers can run performance benchmarks with real Polkadot mainnet data and receive execution time measurements with threshold validation.

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Deploy/Demo (MVP!)
3. Add User Story 2 → Test independently → Deploy/Demo (Memory measurement)
4. Add User Story 3 → Test independently → Deploy/Demo (Documentation)
5. Add User Story 4 → Test independently → Deploy/Demo (Test suite integration)
6. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1 (benchmark execution)
   - Developer B: User Story 2 (memory measurement) - can start in parallel with US1
   - Developer C: User Story 3 (documentation) - can start after US1 completes
   - Developer D: User Story 4 (test suite integration) - can start after US1, US2, US3 complete
3. Stories complete and integrate independently

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Verify tests fail before implementing
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence
- Memory measurement gracefully degrades on unsupported platforms (returns 0 with warning)
- RPC retry logic uses existing retry_with_backoff function from tests/common/rpc_retry.rs
- Benchmark tests marked with #[ignore] attribute require --ignored flag to run
- Threshold validation: sequential-phragmen < 30s, parallel-phragmen < 15s, multi-phase < 45s

---

## Task Summary

**Total Tasks**: 62

**Tasks by Phase**:
- Phase 1 (Setup): 4 tasks
- Phase 2 (Foundational): 9 tasks
- Phase 3 (User Story 1): 12 tasks
- Phase 4 (User Story 2): 11 tasks
- Phase 5 (User Story 3): 10 tasks
- Phase 6 (User Story 4): 7 tasks
- Phase 7 (Polish): 9 tasks

**Tasks by User Story**:
- User Story 1 (P1): 12 tasks
- User Story 2 (P2): 20 tasks (includes foundational memory measurement tasks)
- User Story 3 (P2): 10 tasks
- User Story 4 (P3): 7 tasks

**Parallel Opportunities Identified**: 
- Setup phase: 3 parallel tasks (T002, T003, T004)
- Foundational phase: 4 parallel platform-specific memory measurers (T007, T008, T009, T010)
- User Story 1: 3 parallel test creation tasks (T014, T015, T016)
- User Story 2: 4 parallel platform-specific implementations (T007, T008, T009, T010) and 4 parallel unit tests (T026, T027, T028, T029)
- User Story 3: 3 parallel unit tests (T037, T038, T039)

**Independent Test Criteria**:
- **User Story 1**: Fetch real Polkadot mainnet data from recent block, run election simulation, measure execution time, validate threshold, output structured results
- **User Story 2**: Run benchmark test, verify memory usage is measured and reported in results, verify graceful degradation on unsupported platforms
- **User Story 3**: Run benchmarks, verify JSON files created in tests/fixtures/benchmarks/, verify markdown documentation appended to PERFORMANCE_BENCHMARKS.md
- **User Story 4**: Run test suite with --ignored flag, verify benchmarks execute, verify threshold validation fails test when exceeded

**Suggested MVP Scope**: User Story 1 only (Phase 1 + Phase 2 + Phase 3) - enables core benchmarking capability with real Polkadot mainnet data and execution time measurement.

**Format Validation**: ✅ All tasks follow the checklist format: `- [ ] [TaskID] [P?] [Story?] Description with file path`


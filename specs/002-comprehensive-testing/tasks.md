---
description: "Task list for comprehensive test enhancement feature implementation"
---

# Tasks: Comprehensive Test Enhancement

**Input**: Design documents from `/specs/002-comprehensive-testing/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: Tests are REQUIRED for this feature - comprehensive test suite enhancement is the core deliverable.

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

**Purpose**: Project initialization and basic test infrastructure

- [X] T001 Create test directory structure: tests/unit/, tests/integration/edge_cases/, tests/integration/performance/, tests/integration/chain_snapshots/, tests/integration/regression/
- [X] T002 Create test fixtures directory structure: tests/fixtures/chain_snapshots/, tests/fixtures/regression/, tests/fixtures/benchmarks/
- [X] T003 [P] Add criterion dev-dependency to Cargo.toml for performance benchmarking
- [X] T004 [P] Create tests/common/mod.rs with shared test utilities and helper functions
- [X] T005 [P] Create tests/common/fixture_loader.rs for loading test fixture JSON files
- [X] T006 [P] Create tests/common/rpc_retry.rs with exponential backoff retry logic for RPC calls

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core test infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [X] T007 Create TestFixtureMetadata struct in tests/common/models.rs with fields: test_name, description, created, algorithm, category, tags
- [X] T008 Create TestFixture struct in tests/common/models.rs with fields: metadata, input (ElectionData), expected_result (Option<ElectionResult>)
- [X] T009 Create ChainSnapshotMetadata struct in tests/common/models.rs with fields: chain, block_number, timestamp, rpc_endpoint, expected_validators, expected_stake_allocations
- [X] T010 Create ChainSnapshot struct in tests/common/models.rs with fields: metadata, election_data, expected_result
- [X] T011 Create BenchmarkMetadata struct in tests/common/models.rs with fields: benchmark_name, description, candidate_count, nominator_count, algorithm, target_time_ms, target_memory_mb
- [X] T012 Create BenchmarkResults struct in tests/common/models.rs with fields: execution_time_ms, memory_peak_mb, memory_final_mb, iterations, mean_time_ms, std_dev_ms, metadata
- [X] T013 Create EdgeCaseScenario struct in tests/common/models.rs with fields: scenario_name, description, input, expected_behavior, tags
- [X] T014 Create ExpectedBehavior struct in tests/common/models.rs with fields: should_succeed, expected_result, expected_error, error_message_contains
- [X] T015 Implement TestCategory enum in tests/common/models.rs with variants: EdgeCase, Performance, Regression, ChainSnapshot, Integration
- [X] T016 Implement fixture loader functions in tests/common/fixture_loader.rs: load_test_fixture(), load_chain_snapshot(), validate_fixture_schema()
- [X] T017 Implement RPC retry logic in tests/common/rpc_retry.rs: retry_with_backoff() with exponential backoff (1s, 2s, 4s) and max 3 attempts
- [X] T018 Create tests/common/benchmark_utils.rs with functions: measure_execution_time(), measure_memory_usage(), output_benchmark_json()

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Validate Edge Case Handling (Priority: P1) 🎯 MVP

**Goal**: Comprehensive edge case test suite covering zero candidates, zero nominators, single candidate, single nominator, maximum active set size, malformed data, boundary conditions, and error message validation

**Independent Test**: Can be fully tested by creating test fixtures for each edge case scenario, running them through the election engine, and verifying correct behavior (either valid results or appropriate error messages). Delivers immediate value by preventing production failures and ensuring predictable behavior.

### Tests for User Story 1

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [ ] T019 [P] [US1] Create edge case test for zero candidates in tests/integration/edge_cases/test_zero_candidates.rs
- [ ] T020 [P] [US1] Create edge case test for zero nominators in tests/integration/edge_cases/test_zero_nominators.rs
- [ ] T021 [P] [US1] Create edge case test for single candidate in tests/integration/edge_cases/test_single_candidate.rs
- [ ] T022 [P] [US1] Create edge case test for single nominator in tests/integration/edge_cases/test_single_nominator.rs
- [ ] T023 [P] [US1] Create edge case test for all candidates zero stake in tests/integration/edge_cases/test_zero_candidate_stakes.rs
- [ ] T024 [P] [US1] Create edge case test for all nominators zero stake in tests/integration/edge_cases/test_zero_nominator_stakes.rs
- [ ] T025 [P] [US1] Create edge case test for active set size equals candidate count in tests/integration/edge_cases/test_max_active_set_size.rs
- [ ] T026 [P] [US1] Create edge case test for nominators voting for zero candidates in tests/integration/edge_cases/test_empty_voting_edges.rs
- [ ] T027 [P] [US1] Create edge case test for nominators voting for all candidates in tests/integration/edge_cases/test_all_nominators_vote_all.rs
- [ ] T028 [P] [US1] Create edge case test for duplicate account IDs in tests/integration/edge_cases/test_duplicate_account_ids.rs
- [ ] T029 [P] [US1] Create edge case test for invalid SS58 account IDs in tests/integration/edge_cases/test_invalid_account_ids.rs
- [ ] T030 [P] [US1] Create edge case test for voting edges referencing non-existent candidates in tests/integration/edge_cases/test_invalid_voting_targets.rs
- [ ] T031 [P] [US1] Create edge case test for extremely large stake values in tests/integration/edge_cases/test_maximum_stakes.rs
- [ ] T032 [P] [US1] Create edge case test for malformed JSON structure in tests/integration/edge_cases/test_malformed_json.rs

### Test Fixtures for User Story 1

- [ ] T033 [P] [US1] Create test fixture tests/fixtures/regression/edge_cases/zero_candidates.json with zero candidates scenario
- [ ] T034 [P] [US1] Create test fixture tests/fixtures/regression/edge_cases/zero_nominators.json with zero nominators scenario
- [ ] T035 [P] [US1] Create test fixture tests/fixtures/regression/edge_cases/single_candidate.json with single candidate scenario
- [ ] T036 [P] [US1] Create test fixture tests/fixtures/regression/edge_cases/single_nominator.json with single nominator scenario
- [ ] T037 [P] [US1] Create test fixture tests/fixtures/regression/edge_cases/zero_candidate_stakes.json with all candidates zero stake
- [ ] T038 [P] [US1] Create test fixture tests/fixtures/regression/edge_cases/zero_nominator_stakes.json with all nominators zero stake
- [ ] T039 [P] [US1] Create test fixture tests/fixtures/regression/edge_cases/max_active_set_size.json with active set size equals candidate count
- [ ] T040 [P] [US1] Create test fixture tests/fixtures/regression/edge_cases/empty_voting_edges.json with nominators voting for zero candidates
- [ ] T041 [P] [US1] Create test fixture tests/fixtures/regression/edge_cases/all_nominators_vote_all.json with all nominators voting for all candidates
- [ ] T042 [P] [US1] Create test fixture tests/fixtures/regression/edge_cases/duplicate_account_ids.json with duplicate account IDs
- [ ] T043 [P] [US1] Create test fixture tests/fixtures/regression/edge_cases/invalid_account_ids.json with invalid SS58 account IDs
- [ ] T044 [P] [US1] Create test fixture tests/fixtures/regression/edge_cases/invalid_voting_targets.json with voting edges referencing non-existent candidates
- [ ] T045 [P] [US1] Create test fixture tests/fixtures/regression/edge_cases/maximum_stakes.json with extremely large stake values
- [ ] T046 [P] [US1] Create test fixture tests/fixtures/regression/edge_cases/malformed_json.json with malformed JSON structure

### Implementation for User Story 1

- [ ] T047 [US1] Implement error message validation in edge case tests to ensure clear, actionable error messages
- [ ] T048 [US1] Add test helper function assert_error_message_contains() in tests/common/assertions.rs for error message validation
- [ ] T049 [US1] Create integration test runner in tests/integration/edge_cases/mod.rs that loads fixtures and executes edge case tests
- [ ] T050 [US1] Verify all 14 edge case acceptance scenarios pass with appropriate behavior (success or error)

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently with at least 14 distinct edge case scenarios

---

## Phase 4: User Story 2 - Validate Large Scale Performance (Priority: P1)

**Goal**: Performance test suite validating election completion times and memory usage for large-scale datasets (1k-10k candidates, 10k-100k nominators) with structured JSON benchmark output

**Independent Test**: Can be fully tested by generating synthetic large-scale election datasets, running elections with performance monitoring, and verifying completion times and resource usage stay within acceptable bounds. Delivers immediate value by ensuring scalability and identifying performance regressions.

### Tests for User Story 2

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [ ] T051 [P] [US2] Create performance test for 1k candidates and 10k nominators in tests/integration/performance/test_large_scale_1k.rs (target: < 60s)
- [ ] T052 [P] [US2] Create performance test for 5k candidates and 50k nominators in tests/integration/performance/test_large_scale_5k.rs (target: < 5min)
- [ ] T053 [P] [US2] Create performance test for 10k candidates and 100k nominators in tests/integration/performance/test_large_scale_10k.rs (target: no OOM)
- [ ] T054 [P] [US2] Create performance test for large nominee sets (10k+ nominators) in tests/integration/performance/test_large_nominee_sets.rs
- [ ] T055 [P] [US2] Create performance test for maximum active set size on large datasets in tests/integration/performance/test_max_active_set_large.rs
- [ ] T056 [P] [US2] Create performance test for dense voting patterns in tests/integration/performance/test_dense_voting.rs
- [ ] T057 [P] [US2] Create performance test for sparse voting patterns in tests/integration/performance/test_sparse_voting.rs
- [ ] T058 [P] [US2] Create memory leak detection test running 100 consecutive elections in tests/integration/performance/test_memory_leak.rs

### Implementation for User Story 2

- [ ] T059 [US2] Implement synthetic data generator in tests/common/data_generator.rs: generate_large_scale_election_data() for creating test datasets
- [ ] T060 [US2] Implement performance measurement utilities in tests/common/benchmark_utils.rs: measure_execution_time(), measure_memory_usage()
- [ ] T061 [US2] Implement structured JSON benchmark output in tests/common/benchmark_utils.rs: output_benchmark_json() following benchmark-output.md contract
- [ ] T062 [US2] Create criterion benchmark configuration in benches/large_scale_benchmark.rs for standard benchmarks (1k-5k candidates)
- [ ] T063 [US2] Implement custom performance test runner in tests/integration/performance/mod.rs that outputs structured JSON
- [ ] T064 [US2] Implement memory leak detection logic in tests/integration/performance/test_memory_leak.rs monitoring memory across 100 consecutive elections
- [ ] T065 [US2] Verify all 8 performance acceptance scenarios pass with timing and memory within acceptable bounds

**Checkpoint**: At this point, User Story 2 should be fully functional with performance benchmarks outputting structured JSON and validating scalability requirements

---

## Phase 5: User Story 3 - Validate Real-World Chain Snapshot Accuracy (Priority: P1)

**Goal**: Chain snapshot regression tests validating that election simulations match actual on-chain results with 100% accuracy (exact match on selected validator account IDs and stake allocations)

**Independent Test**: Can be fully tested by fetching election data from known historical blocks on public Substrate chains, running elections, and comparing results to actual on-chain election outcomes documented at those blocks. Delivers immediate value by proving accuracy and catching regressions.

### Tests for User Story 3

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [ ] T066 [P] [US3] Create chain snapshot test framework in tests/integration/chain_snapshots/mod.rs
- [ ] T067 [P] [US3] Create chain snapshot test for Polkadot block in tests/integration/chain_snapshots/test_polkadot.rs
- [ ] T068 [P] [US3] Create chain snapshot test for Kusama block in tests/integration/chain_snapshots/test_kusama.rs
- [ ] T069 [P] [US3] Create chain snapshot test for third Substrate chain block in tests/integration/chain_snapshots/test_other_chain.rs
- [ ] T070 [P] [US3] Create test helper function compare_results_exact_match() in tests/common/assertions.rs for comparing election results to on-chain outcomes
- [ ] T071 [P] [US3] Create test helper function fetch_chain_snapshot() in tests/common/rpc_utils.rs for fetching election data from RPC endpoints
- [ ] T072 [P] [US3] Create test helper function save_chain_snapshot() in tests/common/rpc_utils.rs for saving snapshots as JSON files

### Chain Snapshot Fixtures for User Story 3

- [ ] T073 [P] [US3] Fetch and save Polkadot chain snapshot for block 1 in tests/fixtures/chain_snapshots/polkadot/block_1.json
- [ ] T074 [P] [US3] Fetch and save Polkadot chain snapshot for block 2 in tests/fixtures/chain_snapshots/polkadot/block_2.json
- [ ] T075 [P] [US3] Fetch and save Polkadot chain snapshot for block 3 in tests/fixtures/chain_snapshots/polkadot/block_3.json
- [ ] T076 [P] [US3] Fetch and save Polkadot chain snapshot for block 4 in tests/fixtures/chain_snapshots/polkadot/block_4.json
- [ ] T077 [P] [US3] Fetch and save Kusama chain snapshot for block 1 in tests/fixtures/chain_snapshots/kusama/block_1.json
- [ ] T078 [P] [US3] Fetch and save Kusama chain snapshot for block 2 in tests/fixtures/chain_snapshots/kusama/block_2.json
- [ ] T079 [P] [US3] Fetch and save Kusama chain snapshot for block 3 in tests/fixtures/chain_snapshots/kusama/block_3.json
- [ ] T080 [P] [US3] Fetch and save Kusama chain snapshot for block 4 in tests/fixtures/chain_snapshots/kusama/block_4.json
- [ ] T081 [P] [US3] Fetch and save third chain snapshot for block 1 in tests/fixtures/chain_snapshots/[chain]/block_1.json
- [ ] T082 [P] [US3] Fetch and save third chain snapshot for block 2 in tests/fixtures/chain_snapshots/[chain]/block_2.json

### Implementation for User Story 3

- [ ] T083 [US3] Implement RPC retry logic with exponential backoff in tests/common/rpc_retry.rs integrated with chain snapshot fetching
- [ ] T084 [US3] Implement chain snapshot loading from JSON files in tests/common/fixture_loader.rs: load_chain_snapshot()
- [ ] T085 [US3] Implement result comparison logic in tests/common/assertions.rs: compare_results_exact_match() validating exact match on validator IDs and stake allocations
- [ ] T086 [US3] Implement test skipping mechanism for RPC failures in tests/integration/chain_snapshots/mod.rs marking tests as skipped with reason after retries exhausted
- [ ] T087 [US3] Create integration test runner in tests/integration/chain_snapshots/mod.rs that loads snapshots and validates accuracy
- [ ] T088 [US3] Verify all 7 chain snapshot acceptance scenarios pass with 100% accuracy matching on-chain results

**Checkpoint**: At this point, User Story 3 should be fully functional with chain snapshot tests validating accuracy against at least 10 historical blocks from at least 3 different chains

---

## Phase 6: User Story 4 - Maintain Regression Test Suite (Priority: P2)

**Goal**: Regression test suite with at least 50 test fixtures preserving historical test cases and detecting result changes when code is modified

**Independent Test**: Can be fully tested by creating a curated set of test fixtures with known expected results, running them as part of continuous integration, and verifying results remain consistent across code changes. Delivers value by catching regressions early and enabling confident refactoring.

### Tests for User Story 4

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [ ] T089 [P] [US4] Create regression test framework in tests/integration/regression/mod.rs
- [ ] T090 [P] [US4] Create regression test runner that loads fixtures and compares results in tests/integration/regression/test_runner.rs
- [ ] T091 [P] [US4] Create regression test for normal election scenarios in tests/integration/regression/test_normal_elections.rs
- [ ] T092 [P] [US4] Create regression test for edge case scenarios in tests/integration/regression/test_edge_case_regression.rs
- [ ] T093 [P] [US4] Create regression test for performance scenarios in tests/integration/regression/test_performance_regression.rs

### Regression Test Fixtures for User Story 4

- [ ] T094 [P] [US4] Create regression test fixture tests/fixtures/regression/normal_election_5x5.json with 5 candidates and 5 nominators
- [ ] T095 [P] [US4] Create regression test fixture tests/fixtures/regression/normal_election_10x10.json with 10 candidates and 10 nominators
- [ ] T096 [P] [US4] Create regression test fixture tests/fixtures/regression/normal_election_20x20.json with 20 candidates and 20 nominators
- [ ] T097 [P] [US4] Create at least 47 additional regression test fixtures in tests/fixtures/regression/ covering various scenarios (total 50 fixtures)

### Implementation for User Story 4

- [ ] T098 [US4] Implement regression test fixture loader in tests/common/fixture_loader.rs: load_regression_fixture()
- [ ] T099 [US4] Implement result comparison and change detection in tests/common/assertions.rs: detect_result_changes() with clear reporting
- [ ] T100 [US4] Implement baseline result tracking in tests/integration/regression/test_runner.rs preserving historical results
- [ ] T101 [US4] Create test helper function assert_results_match_baseline() in tests/common/assertions.rs for regression validation
- [ ] T102 [US4] Verify all 5 regression test acceptance scenarios pass with change detection working correctly

**Checkpoint**: At this point, User Story 4 should be fully functional with at least 50 regression test fixtures detecting result changes

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories and final validation

- [ ] T103 [P] Create tests/common/mod.rs exporting all test utilities and models
- [ ] T104 [P] Add comprehensive test documentation in tests/README.md explaining test organization and usage
- [ ] T105 [P] Update Cargo.toml with proper test configuration and criterion benchmark setup
- [ ] T106 Create test helper function assert_election_result_valid() in tests/common/assertions.rs for validating election result structure
- [ ] T107 Create test helper function generate_synthetic_election_data() in tests/common/data_generator.rs for creating test datasets
- [ ] T108 Implement concurrent election execution test in tests/integration/performance/test_concurrent_execution.rs using tokio::task::spawn
- [ ] T109 Add test for deterministic results across multiple runs in tests/integration/regression/test_determinism.rs
- [ ] T110 Add test for algorithm convergence validation in tests/integration/edge_cases/test_algorithm_convergence.rs
- [ ] T111 Run quickstart.md validation ensuring all test scenarios work as documented
- [ ] T112 Verify test suite execution completes in under 10 minutes including large-scale performance tests
- [ ] T113 Verify all success criteria from spec.md are met: 20+ edge cases, 50+ regression fixtures, 10+ chain snapshots, performance targets achieved

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3-6)**: All depend on Foundational phase completion
  - User stories can then proceed in parallel (if staffed)
  - Or sequentially in priority order (US1 → US2 → US3 → US4)
- **Polish (Phase 7)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 3 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 4 (P2)**: Can start after Foundational (Phase 2) - May reuse fixtures from US1 but should be independently testable

### Within Each User Story

- Tests MUST be written and FAIL before implementation
- Test fixtures before test implementations
- Core test infrastructure before specific test cases
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel
- All Foundational tasks marked [P] can run in parallel (within Phase 2)
- Once Foundational phase completes, all user stories can start in parallel (if team capacity allows)
- All tests for a user story marked [P] can run in parallel
- Test fixtures within a story marked [P] can run in parallel
- Different user stories can be worked on in parallel by different team members

---

## Parallel Example: User Story 1

```bash
# Launch all edge case tests together:
Task: "Create edge case test for zero candidates in tests/integration/edge_cases/test_zero_candidates.rs"
Task: "Create edge case test for zero nominators in tests/integration/edge_cases/test_zero_nominators.rs"
Task: "Create edge case test for single candidate in tests/integration/edge_cases/test_single_candidate.rs"
Task: "Create edge case test for single nominator in tests/integration/edge_cases/test_single_nominator.rs"
# ... (all 14 edge case tests)

# Launch all test fixtures together:
Task: "Create test fixture tests/fixtures/regression/edge_cases/zero_candidates.json"
Task: "Create test fixture tests/fixtures/regression/edge_cases/zero_nominators.json"
# ... (all 14 edge case fixtures)
```

---

## Parallel Example: User Story 2

```bash
# Launch all performance tests together:
Task: "Create performance test for 1k candidates and 10k nominators in tests/integration/performance/test_large_scale_1k.rs"
Task: "Create performance test for 5k candidates and 50k nominators in tests/integration/performance/test_large_scale_5k.rs"
Task: "Create performance test for 10k candidates and 100k nominators in tests/integration/performance/test_large_scale_10k.rs"
# ... (all 8 performance tests)
```

---

## Parallel Example: User Story 3

```bash
# Launch all chain snapshot fixture fetching together:
Task: "Fetch and save Polkadot chain snapshot for block 1 in tests/fixtures/chain_snapshots/polkadot/block_1.json"
Task: "Fetch and save Polkadot chain snapshot for block 2 in tests/fixtures/chain_snapshots/polkadot/block_2.json"
Task: "Fetch and save Kusama chain snapshot for block 1 in tests/fixtures/chain_snapshots/kusama/block_1.json"
# ... (all 10+ chain snapshots)
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1 (Edge Case Handling)
4. **STOP and VALIDATE**: Test User Story 1 independently
5. Deploy/demo if ready

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Deploy/Demo (MVP!)
3. Add User Story 2 → Test independently → Deploy/Demo
4. Add User Story 3 → Test independently → Deploy/Demo
5. Add User Story 4 → Test independently → Deploy/Demo
6. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1 (Edge Cases)
   - Developer B: User Story 2 (Performance)
   - Developer C: User Story 3 (Chain Snapshots)
   - Developer D: User Story 4 (Regression)
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
- Test fixtures must follow JSON schema from contracts/test-fixtures.md
- Benchmark output must follow JSON format from contracts/benchmark-output.md
- RPC retry logic must use exponential backoff (1s, 2s, 4s) with max 3 attempts
- Chain snapshots must be stored as version-controlled JSON files
- Regression fixtures must preserve historical test cases

---

## Summary

**Total Task Count**: 113 tasks

**Task Count Per User Story**:
- Phase 1 (Setup): 6 tasks
- Phase 2 (Foundational): 12 tasks
- Phase 3 (User Story 1 - Edge Cases): 32 tasks (14 tests + 14 fixtures + 4 implementation)
- Phase 4 (User Story 2 - Performance): 15 tasks (8 tests + 7 implementation)
- Phase 5 (User Story 3 - Chain Snapshots): 23 tasks (7 tests + 10 fixtures + 6 implementation)
- Phase 6 (User Story 4 - Regression): 14 tasks (5 tests + 50 fixtures + 4 implementation)
- Phase 7 (Polish): 11 tasks

**Parallel Opportunities Identified**:
- 85+ tasks marked [P] can run in parallel
- All user stories can proceed in parallel after Foundational phase
- Test fixtures can be created in parallel within each story

**Independent Test Criteria**:
- **US1**: Edge case tests can run independently with fixtures, validate behavior (success or error)
- **US2**: Performance tests can run independently with synthetic data, validate timing/memory bounds
- **US3**: Chain snapshot tests can run independently with stored fixtures, validate accuracy against on-chain results
- **US4**: Regression tests can run independently with fixtures, validate result consistency

**Suggested MVP Scope**: User Story 1 (Edge Case Handling) - delivers immediate value by preventing production failures and ensuring predictable behavior

**Format Validation**: ✅ All tasks follow checklist format (checkbox, ID, labels, file paths)


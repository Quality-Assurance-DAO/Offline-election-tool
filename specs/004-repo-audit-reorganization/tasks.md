---
description: "Task list for Repository Audit and Reorganization feature implementation"
---

# Tasks: Repository Audit and Reorganization

**Input**: Design documents from `/specs/004-repo-audit-reorganization/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: No test tasks included - this is a documentation reorganization task, not a code feature.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Documentation**: Root-level markdown files and `docs/` directory structure
- **Source code**: Unchanged (this feature only reorganizes documentation)

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Create documentation structure and prepare for reorganization

- [X] T001 Create docs/ directory structure: docs/api/, docs/guides/, docs/testing/, docs/polkadot/, docs/reference/
- [X] T002 [P] Audit all root-level markdown files and create inventory in specs/004-repo-audit-reorganization/checklists/file-inventory.md
- [X] T003 [P] Identify all internal links in documentation files and create link inventory in specs/004-repo-audit-reorganization/checklists/link-inventory.md

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core audit and analysis that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [X] T004 Analyze root-level markdown files for duplicates (>95% similarity) and document findings
- [X] T005 Analyze root-level markdown files for redundancies (>60% overlap) and identify unique content to preserve
- [X] T006 Identify outdated files (non-existent features OR 18+ months old AND contradicted) and document findings
- [X] T007 [P] Map file relationships and cross-references between documentation files
- [X] T008 [P] Extract all technical terms from documentation and create term inventory
- [X] T009 Create file mapping table: old paths → new paths per contracts/documentation-structure.md

**Checkpoint**: Foundation ready - file audit complete, mapping established, user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Newcomer Discovers and Understands the Project (Priority: P1) 🎯 MVP

**Goal**: A developer new to Polkadot/Substrate can understand what the tool does, why it exists, and how to get started quickly within 5 minutes of reading the main README.

**Independent Test**: Have a new developer (who has never seen the repository) read the main README and successfully answer: "What is this tool?", "Why would I use it?", "How do I get started?", and "How does this relate to Polkadot?" within 5 minutes of reading.

### Implementation for User Story 1

- [X] T010 [US1] Enhance README.md with project overview section explaining what NPoS elections are and why offline simulation is valuable
- [X] T011 [US1] Add Quick Start section to README.md with step-by-step instructions to run first election simulation
- [X] T012 [US1] Create comprehensive Polkadot ecosystem overview in docs/polkadot/ecosystem-overview.md covering validators, nominators, staking, and NPoS elections
- [X] T013 [US1] Add Polkadot ecosystem context summary section to README.md with link to detailed overview in docs/polkadot/ecosystem-overview.md
- [X] T014 [US1] Create technical terms glossary in docs/reference/glossary.md with definitions for all identified terms
- [X] T015 [US1] Add inline definitions for technical terms in README.md where they first appear (NPoS, Sequential Phragmen, archive node, active set, etc.)
- [X] T016 [US1] Add navigation section to README.md with clear links to all documentation sections
- [X] T017 [US1] Add "How this tool fits in Polkadot ecosystem" section to README.md explaining role, dependencies, and interactions

**Checkpoint**: At this point, User Story 1 should be fully functional - a newcomer can understand the project and get started within 5 minutes

---

## Phase 4: User Story 2 - Contributor Finds Relevant Documentation (Priority: P1)

**Goal**: An experienced developer can find relevant documentation for any major feature (algorithms, API, testing, RPC usage) within 2 clicks from the main README, with no duplicate or conflicting information.

**Independent Test**: Have a contributor successfully find documentation for: "How to add a new algorithm?", "How to run tests?", "What is the API structure?", and "How does RPC data loading work?" - each within 2 clicks from the main README, with no duplicate or conflicting information.

### Implementation for User Story 2

- [X] T018 [US2] Consolidate TESTING.md and TEST_RUNNING_INSTRUCTIONS.md into docs/testing/overview.md, preserving all unique content and examples
- [X] T019 [US2] Consolidate RPC_ARCHIVE_NODES.md and RPC_TESTING.md into docs/guides/rpc-usage.md, preserving all unique content
- [X] T020 [US2] Split API_USAGE.md into docs/api/rest-api.md (REST API server documentation) and docs/api/programmatic-api.md (programmatic library API)
- [X] T021 [US2] Move ALGORITHM_EXTENSIBILITY.md to docs/guides/algorithms.md
- [X] T022 [US2] Move PERFORMANCE_BENCHMARKS.md to docs/guides/performance.md
- [X] T023 [US2] Move RFP_COMPLIANCE_ASSESSMENT.md to docs/reference/rfp-compliance.md
- [X] T024 [US2] Update README.md navigation links to point to new docs/ structure (api/, guides/, testing/, reference/)
- [X] T025 [US2] Add project structure section to README.md explaining major components and where to find documentation
- [X] T026 [US2] Update tests/README.md with links to docs/testing/overview.md
- [X] T027 [US2] Remove duplicate and redundant files after consolidation: ALGORITHM_EXTENSIBILITY.md, API_USAGE.md, PERFORMANCE_BENCHMARKS.md, RPC_ARCHIVE_NODES.md, RPC_TESTING.md, TESTING.md, TEST_RUNNING_INSTRUCTIONS.md, RFP_COMPLIANCE_ASSESSMENT.md

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently - newcomers can understand the project and contributors can find all documentation easily

---

## Phase 5: User Story 3 - Maintainer Audits and Updates Documentation (Priority: P2)

**Goal**: A project maintainer can easily identify all documentation that needs updating when a new feature is added, with no risk of missing related documentation due to poor organization.

**Independent Test**: Have a maintainer successfully identify all documentation that needs updating when a new algorithm is added, with no risk of missing related documentation due to poor organization.

### Implementation for User Story 3

- [ ] T028 [US3] Update all internal links in documentation files to point to new locations per contracts/link-update.md
- [ ] T029 [US3] Verify all section anchors work correctly after file consolidation
- [ ] T030 [US3] Add cross-references between related documentation sections (e.g., algorithms.md ↔ api docs, testing ↔ guides)
- [ ] T031 [US3] Ensure technical terms are consistently defined across all documentation files
- [ ] T032 [US3] Add "Documentation Structure" section to README.md explaining organization pattern for maintainers
- [ ] T033 [US3] Verify all outdated files are removed or clearly marked as archived
- [ ] T034 [US3] Create documentation maintenance guide in docs/reference/maintenance.md explaining update process

**Checkpoint**: At this point, all user stories should be independently functional - maintainers can easily identify and update documentation

---

## Phase 6: User Story 4 - User Understands Test Results and Expectations (Priority: P2)

**Goal**: A developer or user can run tests and understand what the results mean, what they indicate about the system's correctness, and what to expect.

**Independent Test**: Have a user run a test suite and successfully interpret the results, understanding what each test validates and what the expected output format should be.

### Implementation for User Story 4

- [ ] T035 [US4] Add example test outputs section to docs/testing/overview.md showing successful test output format (JSON)
- [ ] T036 [US4] Add example test outputs section to docs/testing/overview.md showing successful test output format (human-readable)
- [ ] T037 [US4] Add interpretation guide to docs/testing/overview.md explaining what test results mean
- [ ] T038 [US4] Add success criteria section to docs/testing/overview.md explaining what indicates a passing test
- [ ] T039 [US4] Add failure examples section to docs/testing/overview.md explaining what indicates a failing test (optional but helpful)
- [ ] T040 [US4] Update tests/README.md with links to test examples in docs/testing/overview.md

**Checkpoint**: At this point, all user stories should be complete - users can understand test results and expectations

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Final verification, cleanup, and cross-cutting improvements

- [ ] T041 [P] Verify all internal links work correctly (no broken links)
- [ ] T042 [P] Verify all technical terms are defined or linked to definitions (100% coverage)
- [ ] T043 [P] Verify documentation structure matches contracts/documentation-structure.md
- [ ] T044 Verify all content preserved during consolidation (no information loss)
- [ ] T045 Verify README.md provides clear navigation to all documentation sections
- [ ] T046 Verify Polkadot ecosystem overview is comprehensive and accurate
- [ ] T047 Run quickstart.md validation checklist to ensure all steps completed
- [ ] T048 Update any external documentation references if needed
- [ ] T049 Final review: ensure repository presents clean, well-structured, context-rich codebase

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3+)**: All depend on Foundational phase completion
  - User stories can then proceed in parallel (if staffed)
  - Or sequentially in priority order (P1 → P2)
- **Polish (Phase 7)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories. Creates entry point and context.
- **User Story 2 (P1)**: Can start after Foundational (Phase 2) - May reference US1 content but should be independently testable. Organizes documentation structure.
- **User Story 3 (P2)**: Can start after User Story 2 - Depends on files being moved/consolidated. Updates links and ensures maintainability.
- **User Story 4 (P2)**: Can start after User Story 2 - Depends on testing documentation being consolidated. Adds test examples.

### Within Each User Story

- File moves/consolidations before link updates
- Content creation before navigation updates
- Core documentation before cross-references
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel (T002, T003)
- All Foundational tasks marked [P] can run in parallel within Phase 2 (T007, T008)
- Once Foundational phase completes, User Stories 1 and 2 can start in parallel (both P1)
- User Stories 3 and 4 can proceed in parallel after User Story 2 completes (both P2)
- All Polish tasks marked [P] can run in parallel (T041, T042, T043)

---

## Parallel Example: User Story 1

```bash
# Launch all content creation tasks for User Story 1 together:
Task: "Create comprehensive Polkadot ecosystem overview in docs/polkadot/ecosystem-overview.md"
Task: "Create technical terms glossary in docs/reference/glossary.md"

# These can run in parallel as they create different files:
Task: "Add Quick Start section to README.md"
Task: "Add Polkadot ecosystem context summary section to README.md"
Task: "Add navigation section to README.md"
```

---

## Parallel Example: User Story 2

```bash
# Launch all file consolidation tasks for User Story 2 together:
Task: "Consolidate TESTING.md and TEST_RUNNING_INSTRUCTIONS.md into docs/testing/overview.md"
Task: "Consolidate RPC_ARCHIVE_NODES.md and RPC_TESTING.md into docs/guides/rpc-usage.md"
Task: "Split API_USAGE.md into docs/api/rest-api.md and docs/api/programmatic-api.md"

# These can run in parallel as they work on different files:
Task: "Move ALGORITHM_EXTENSIBILITY.md to docs/guides/algorithms.md"
Task: "Move PERFORMANCE_BENCHMARKS.md to docs/guides/performance.md"
Task: "Move RFP_COMPLIANCE_ASSESSMENT.md to docs/reference/rfp-compliance.md"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (create docs/ structure, audit files)
2. Complete Phase 2: Foundational (analyze duplicates/redundancies, create mapping)
3. Complete Phase 3: User Story 1 (enhance README, create Polkadot overview, add glossary)
4. **STOP and VALIDATE**: Test User Story 1 independently - can a newcomer understand the project within 5 minutes?
5. Deploy/demo if ready

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready (file audit complete)
2. Add User Story 1 → Test independently → Deploy/Demo (MVP! - Newcomers can understand project)
3. Add User Story 2 → Test independently → Deploy/Demo (Contributors can find documentation)
4. Add User Story 3 → Test independently → Deploy/Demo (Maintainers can update easily)
5. Add User Story 4 → Test independently → Deploy/Demo (Users understand test results)
6. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple team members:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Team Member A: User Story 1 (README enhancements, Polkadot overview)
   - Team Member B: User Story 2 (file consolidation, organization)
3. Once User Story 2 completes:
   - Team Member A: User Story 3 (link updates, maintainability)
   - Team Member B: User Story 4 (test examples)
4. Stories complete and integrate independently

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence
- **Critical**: Update all internal links BEFORE removing old files (FR-012)
- **Critical**: Preserve all essential information during consolidation (FR-015)
- **Critical**: Ensure 100% technical term coverage (SC-002)

---

## Summary

**Total Tasks**: 49 tasks across 7 phases

**Task Count by User Story**:
- Setup (Phase 1): 3 tasks
- Foundational (Phase 2): 6 tasks
- User Story 1 (Phase 3): 8 tasks
- User Story 2 (Phase 4): 10 tasks
- User Story 3 (Phase 5): 7 tasks
- User Story 4 (Phase 6): 6 tasks
- Polish (Phase 7): 9 tasks

**Parallel Opportunities Identified**:
- Setup phase: 2 parallel tasks (T002, T003)
- Foundational phase: 2 parallel tasks (T007, T008)
- User Stories 1 and 2 can run in parallel after Foundational
- User Stories 3 and 4 can run in parallel after User Story 2
- Polish phase: 3 parallel verification tasks (T041, T042, T043)

**Independent Test Criteria**:
- **User Story 1**: Newcomer can answer "What is this tool?", "Why would I use it?", "How do I get started?", "How does this relate to Polkadot?" within 5 minutes
- **User Story 2**: Contributor can find documentation for algorithms, API, testing, RPC usage within 2 clicks from README, no duplicates
- **User Story 3**: Maintainer can identify all documentation needing updates when new algorithm added, no missed files
- **User Story 4**: User can run test suite and interpret results, understanding expected output format

**Suggested MVP Scope**: 
- Phase 1: Setup
- Phase 2: Foundational
- Phase 3: User Story 1 (Newcomer experience)

This delivers a complete newcomer experience with enhanced README, Polkadot context, and glossary - the foundation for all other documentation work.

**Format Validation**: ✅ All tasks follow checklist format: checkbox, Task ID, optional [P] marker, optional [Story] label, description with file path


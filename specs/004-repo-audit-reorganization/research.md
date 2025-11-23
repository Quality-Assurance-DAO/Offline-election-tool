# Research: Repository Audit and Reorganization

**Feature**: Repository Audit and Reorganization  
**Date**: 2025-01-27  
**Phase**: Phase 0 - Outline & Research

## Overview

This document consolidates research findings for the repository audit and reorganization feature. All "NEEDS CLARIFICATION" items from the Technical Context have been resolved through analysis of the existing codebase, documentation structure, and best practices.

## Research Areas

### 1. Documentation File Analysis

**Decision**: Systematic audit of all root-level markdown files to identify duplicates, redundancies, and outdated content.

**Rationale**: The feature spec requires removal of duplicate files (>95% similarity), consolidation of redundant files (>60% overlap), and identification of outdated files (non-existent features OR 18+ months old AND contradicted by current code/docs).

**Findings**:

#### Root-Level Documentation Files Identified:
1. **README.md** - Main entry point (stays at root, needs enhancement)
2. **ALGORITHM_EXTENSIBILITY.md** - Algorithm extensibility guide (candidate for `docs/guides/algorithms.md`)
3. **API_USAGE.md** - Comprehensive API documentation (candidate for `docs/api/` split)
4. **PERFORMANCE_BENCHMARKS.md** - Performance testing guide (candidate for `docs/guides/performance.md`)
5. **RPC_ARCHIVE_NODES.md** - RPC archive node guide (candidate for `docs/guides/rpc-usage.md`)
6. **RPC_TESTING.md** - RPC testing guide (candidate for consolidation with RPC_ARCHIVE_NODES.md)
7. **TESTING.md** - Testing overview (candidate for `docs/testing/overview.md`)
8. **TEST_RUNNING_INSTRUCTIONS.md** - Test instructions (candidate for consolidation with TESTING.md)
9. **RFP_COMPLIANCE_ASSESSMENT.md** - RFP assessment (candidate for `docs/reference/rfp-compliance.md`)

#### Potential Duplicates/Redundancies:
- **TESTING.md** and **TEST_RUNNING_INSTRUCTIONS.md**: Significant overlap (>60%) - both cover test execution. TEST_RUNNING_INSTRUCTIONS.md has more detailed examples. Consolidate into `docs/testing/overview.md` with examples section.
- **RPC_ARCHIVE_NODES.md** and **RPC_TESTING.md**: Overlap on RPC usage. RPC_TESTING.md focuses on testing, RPC_ARCHIVE_NODES.md focuses on archive nodes. Consolidate into `docs/guides/rpc-usage.md` with sections for both.
- **API_USAGE.md**: Large file (900+ lines) covering both REST API and programmatic API. Split into `docs/api/rest-api.md` and `docs/api/programmatic-api.md`.

#### Outdated Files Analysis:
- **RFP_COMPLIANCE_ASSESSMENT.md**: Created 2025-01-27, references features that may have been implemented since. Needs review to ensure accuracy.
- All other files appear current based on content analysis (no references to non-existent features).

**Alternatives Considered**:
- Keeping all files at root: Rejected - violates FR-004 (logical hierarchy requirement)
- Single docs/README.md: Rejected - violates FR-011 (README as entry point requirement)
- Flat docs/ structure: Rejected - violates FR-006 (clear relationships requirement)

### 2. Technical Terms Requiring Definitions

**Decision**: Identify all domain-specific terms used in documentation and ensure definitions are provided inline or via links.

**Rationale**: FR-005 requires every technical term to be defined where it first appears, or linked to a definition. SC-002 requires 100% coverage.

**Findings**:

#### Key Technical Terms Identified:
1. **NPoS (Nominated Proof of Stake)** - Core concept, needs definition in README
2. **Sequential Phragmen** - Algorithm name, needs definition
3. **Parallel Phragmen** - Algorithm name, needs definition
4. **Multi-phase** - Algorithm name, needs definition
5. **Archive Node** - RPC concept, needs definition
6. **Active Set** - Election concept, needs definition
7. **Validator** - Polkadot concept, needs definition
8. **Nominator** - Polkadot concept, needs definition
9. **Stake** - Polkadot concept, needs definition
10. **RPC (Remote Procedure Call)** - Technical term, needs definition
11. **Substrate** - Framework name, needs definition
12. **Phragmen** - Algorithm family name, needs definition
13. **SS58** - Encoding format, needs definition
14. **Bit-for-bit accuracy** - Technical term, needs definition

**Strategy**:
- Define terms inline where first used in each document
- Create glossary/reference section in `docs/reference/glossary.md` for comprehensive definitions
- Link to glossary from README and other key documents
- Ensure definitions are consistent across all documentation

**Alternatives Considered**:
- Definitions only in glossary: Rejected - violates FR-005 (must define where first appears)
- Definitions only inline: Accepted as primary approach, with glossary as supplement

### 3. Polkadot Ecosystem Context

**Decision**: Create comprehensive Polkadot ecosystem overview section explaining validators, nominators, staking, and NPoS elections, plus explicit description of how this tool fits within the ecosystem.

**Rationale**: FR-007 requires comprehensive Polkadot ecosystem overview. FR-008 requires explicit description of tool's role, dependencies, and interactions. SC-007 and SC-008 specify measurable outcomes.

**Findings**:

#### Required Content:
1. **Polkadot Ecosystem Overview**:
   - What is Polkadot (parachain ecosystem)
   - Validators: role, responsibilities, rewards
   - Nominators: role, responsibilities, rewards
   - Staking: how it works, why it matters
   - NPoS Elections: purpose, frequency, process

2. **Tool's Role in Ecosystem**:
   - Offline simulation of on-chain elections
   - Use cases: testing, analysis, what-if scenarios
   - Relationship to on-chain election processes
   - When to use offline vs. on-chain

3. **Dependencies**:
   - Substrate crates: `sp-npos-elections`, `frame-election-provider-support`, `pallet-election-provider-multi-phase`
   - RPC endpoints: interaction with Substrate RPC for data fetching
   - Compatibility: which Substrate versions supported

4. **Interactions**:
   - RPC data fetching from live chains
   - Bit-for-bit accuracy with on-chain elections
   - Support for multiple chains (Polkadot, Kusama, Westend)

**Strategy**:
- Create `docs/polkadot/ecosystem-overview.md` with comprehensive overview
- Add summary section to README.md linking to detailed overview
- Reference ecosystem context in relevant guides (algorithms, RPC usage)
- Ensure all Polkadot-specific terms are defined

**Alternatives Considered**:
- Single paragraph in README: Rejected - insufficient for SC-007 (comprehensive overview required)
- External links only: Rejected - violates FR-007 (must include overview, not just link)
- Separate file only: Accepted - detailed overview in `docs/polkadot/`, summary in README

### 4. Documentation Structure Patterns

**Decision**: Use hybrid pattern: README.md at root as entry point, detailed documentation in `docs/` folder organized by topic.

**Rationale**: Clarification from spec.md specifies hybrid pattern. FR-004 requires logical hierarchy. FR-014 requires structure intuitive for both newcomers and experienced contributors.

**Findings**:

#### Best Practices for Documentation Structure:
1. **Entry Point**: README.md at root provides overview, quick start, and navigation
2. **Topic-Based Organization**: Group related documentation by topic (api/, guides/, testing/, polkadot/, reference/)
3. **Progressive Disclosure**: Start with overview, link to detailed docs
4. **Cross-References**: Clear links between related sections
5. **Searchability**: Logical structure makes information easy to find

#### Proposed Structure:
```
README.md (root) - Entry point with overview, quick start, navigation
docs/
├── api/ - API documentation
│   ├── rest-api.md
│   └── programmatic-api.md
├── guides/ - User guides
│   ├── algorithms.md
│   ├── rpc-usage.md
│   └── performance.md
├── testing/ - Testing documentation
│   ├── overview.md
│   └── examples.md
├── polkadot/ - Polkadot ecosystem context
│   └── ecosystem-overview.md
└── reference/ - Reference documentation
    ├── glossary.md
    └── rfp-compliance.md
```

**Alternatives Considered**:
- Single docs/README.md: Rejected - violates GitHub convention (README at root)
- Flat structure: Rejected - violates FR-006 (clear relationships)
- Deep nesting (>3 levels): Rejected - violates SC-006 (3-click maximum)

### 5. Link Management Strategies

**Decision**: Systematic link audit and update process: identify all internal links, update to new locations, verify external links, remove old files completely.

**Rationale**: FR-012 requires systematic link updates. SC-010 requires 100% link accuracy. Clarification specifies updating all internal links before removing old files.

**Findings**:

#### Link Types Identified:
1. **Internal Links**: Links between documentation files (e.g., `[API_USAGE.md](API_USAGE.md)`)
2. **Section Links**: Links to sections within same file (e.g., `[Quick Start](#quick-start)`)
3. **External Links**: Links to external resources (e.g., Polkadot documentation, GitHub)
4. **Code References**: References to code files (e.g., `src/algorithms/sequential_phragmen.rs`)

#### Link Update Strategy:
1. **Audit Phase**: Identify all internal links in documentation files
2. **Mapping Phase**: Create mapping from old paths to new paths
3. **Update Phase**: Update all links to new locations
4. **Verification Phase**: Verify all links work (no broken links)
5. **Removal Phase**: Remove old files only after links updated

#### Tools/Approach:
- Manual audit of all markdown files
- Use grep/search to find all link patterns (`[text](path)`)
- Create link mapping table
- Update links systematically
- Test links after reorganization

**Alternatives Considered**:
- Redirect files: Rejected - clarification specifies removing old files completely
- Broken links acceptable: Rejected - violates SC-010 (100% link accuracy required)
- Update links after removal: Rejected - violates clarification (update before removal)

### 6. Test Documentation Examples

**Decision**: Add documented example test results showing expected output format and interpretation to test documentation.

**Rationale**: FR-010 requires documented example test results. SC-009 requires at least one example per test documentation file. User Story 4 requires users to understand test results.

**Findings**:

#### Current Test Documentation:
- `tests/README.md`: Exists but may lack example outputs
- `TESTING.md`: Covers test execution but may lack example outputs
- `TEST_RUNNING_INSTRUCTIONS.md`: Detailed instructions but may lack example outputs

#### Required Content:
1. **Example Test Output**: Show what successful test output looks like
2. **Output Format**: Explain JSON structure, human-readable format
3. **Interpretation Guide**: Explain what results mean
4. **Success Criteria**: What indicates a passing test
5. **Failure Examples**: What indicates a failing test (optional but helpful)

**Strategy**:
- Consolidate test documentation into `docs/testing/overview.md`
- Add examples section with sample test outputs
- Include interpretation guide
- Link to test examples from README

**Alternatives Considered**:
- No examples: Rejected - violates FR-010 and SC-009
- Examples only in code comments: Rejected - violates FR-010 (must be in documentation)
- Examples in separate file: Accepted - examples section in testing documentation

## Consolidated Decisions

### Documentation Structure
- **Pattern**: Hybrid (README at root + docs/ by topic)
- **Organization**: Topic-based (`api/`, `guides/`, `testing/`, `polkadot/`, `reference/`)
- **Entry Point**: README.md with overview, quick start, navigation

### File Consolidation
- **TESTING.md** + **TEST_RUNNING_INSTRUCTIONS.md** → `docs/testing/overview.md`
- **RPC_ARCHIVE_NODES.md** + **RPC_TESTING.md** → `docs/guides/rpc-usage.md`
- **API_USAGE.md** → Split into `docs/api/rest-api.md` and `docs/api/programmatic-api.md`
- **ALGORITHM_EXTENSIBILITY.md** → `docs/guides/algorithms.md`
- **PERFORMANCE_BENCHMARKS.md** → `docs/guides/performance.md`
- **RFP_COMPLIANCE_ASSESSMENT.md** → `docs/reference/rfp-compliance.md`

### Technical Terms
- **Strategy**: Define inline where first used + glossary in `docs/reference/glossary.md`
- **Coverage**: 100% of technical terms defined or linked

### Polkadot Context
- **Location**: `docs/polkadot/ecosystem-overview.md` (detailed) + summary in README.md
- **Content**: Validators, nominators, staking, NPoS elections, tool's role, dependencies, interactions

### Link Management
- **Process**: Audit → Map → Update → Verify → Remove
- **Timing**: Update links before removing old files
- **Accuracy**: 100% link accuracy required

### Test Documentation
- **Location**: `docs/testing/overview.md` (consolidated)
- **Content**: Test execution + example outputs + interpretation guide

## Open Questions Resolved

All "NEEDS CLARIFICATION" items from Technical Context have been resolved:

1. ✅ **Documentation file analysis**: Systematic audit approach defined
2. ✅ **Technical terms**: Strategy for definitions established
3. ✅ **Polkadot context**: Content requirements identified
4. ✅ **Documentation structure**: Hybrid pattern confirmed
5. ✅ **Link management**: Process defined
6. ✅ **Test documentation**: Examples strategy established

## Next Steps

Proceed to Phase 1: Design & Contracts
- Generate `data-model.md` for documentation entities
- Generate `contracts/` for documentation structure contracts
- Generate `quickstart.md` for reorganization process
- Update agent context


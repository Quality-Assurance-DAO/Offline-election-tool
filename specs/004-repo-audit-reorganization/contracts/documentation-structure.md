# Contract: Documentation Structure

**Feature**: Repository Audit and Reorganization  
**Date**: 2025-01-27  
**Type**: Structure Contract

## Overview

This contract defines the required documentation structure after reorganization. The structure follows a hybrid pattern: README.md at root as entry point, detailed documentation in `docs/` folder organized by topic.

## Structure Requirements

### Root Level

**README.md** (required)
- **Purpose**: Main entry point for repository
- **Content Requirements**:
  - Project overview (what it does, why it exists)
  - Quick start guide (how to run first election)
  - Polkadot ecosystem context summary (link to detailed overview)
  - Navigation to detailed documentation
  - Links to key guides (API, algorithms, testing, RPC usage)
- **Target Audience**: Newcomers and experienced users
- **Status**: Enhanced (not moved, content updated)

### Documentation Directory (`docs/`)

**Structure**: Topic-based organization with clear hierarchy

#### `docs/api/` - API Documentation

**Purpose**: API usage documentation

**Files**:
- `rest-api.md` - REST API server documentation
  - Starting the server
  - API endpoints
  - Request/response formats
  - Examples for all three algorithms
  - Security considerations
- `programmatic-api.md` - Programmatic library API
  - Library usage examples
  - API reference
  - Integration patterns

**Source**: Split from `API_USAGE.md`

#### `docs/guides/` - User Guides

**Purpose**: Step-by-step guides for common tasks

**Files**:
- `algorithms.md` - Algorithm extensibility guide
  - Current algorithms overview
  - Architecture explanation
  - Adding new algorithms
  - Examples
- `rpc-usage.md` - RPC usage guide
  - Archive nodes explanation
  - RPC endpoint information
  - Historical block queries
  - Testing RPC connections
- `performance.md` - Performance benchmarks
  - Benchmark overview
  - Running benchmarks
  - Interpreting results
  - Large-scale testing

**Sources**:
- `ALGORITHM_EXTENSIBILITY.md` → `algorithms.md`
- `RPC_ARCHIVE_NODES.md` + `RPC_TESTING.md` → `rpc-usage.md`
- `PERFORMANCE_BENCHMARKS.md` → `performance.md`

#### `docs/testing/` - Testing Documentation

**Purpose**: Testing guides and examples

**Files**:
- `overview.md` - Testing overview
  - Test types (unit, integration, contract)
  - Running tests
  - Test structure
  - Example test outputs
  - Interpretation guide
- `examples.md` - Test examples (optional, can be in overview.md)
  - Example test results
  - Expected output formats
  - Success criteria

**Sources**:
- `TESTING.md` + `TEST_RUNNING_INSTRUCTIONS.md` → `overview.md`
- `tests/README.md` → Content merged into `overview.md`

#### `docs/polkadot/` - Polkadot Ecosystem Context

**Purpose**: Polkadot ecosystem overview and tool's role

**Files**:
- `ecosystem-overview.md` - Comprehensive Polkadot overview
  - What is Polkadot
  - Validators: role, responsibilities, rewards
  - Nominators: role, responsibilities, rewards
  - Staking: how it works, why it matters
  - NPoS Elections: purpose, frequency, process
  - Tool's role in ecosystem
  - Dependencies on Substrate crates
  - Interactions with RPC endpoints
  - Relationship to on-chain elections

**Source**: New content (required by FR-007, FR-008)

#### `docs/reference/` - Reference Documentation

**Purpose**: Reference material and compliance information

**Files**:
- `glossary.md` - Technical terms glossary
  - All technical terms defined
  - Cross-references
  - Term categories
- `rfp-compliance.md` - RFP compliance assessment
  - Requirement analysis
  - Compliance status
  - Implementation details

**Sources**:
- New: `glossary.md` (consolidated from all documentation)
- `RFP_COMPLIANCE_ASSESSMENT.md` → `rfp-compliance.md`

## File Mapping

### Files to Move

| Old Path | New Path | Action |
|----------|----------|--------|
| `ALGORITHM_EXTENSIBILITY.md` | `docs/guides/algorithms.md` | Move & rename |
| `API_USAGE.md` | `docs/api/rest-api.md` + `docs/api/programmatic-api.md` | Split & move |
| `PERFORMANCE_BENCHMARKS.md` | `docs/guides/performance.md` | Move & rename |
| `RPC_ARCHIVE_NODES.md` | `docs/guides/rpc-usage.md` | Consolidate & move |
| `RPC_TESTING.md` | `docs/guides/rpc-usage.md` | Consolidate & move |
| `TESTING.md` | `docs/testing/overview.md` | Consolidate & move |
| `TEST_RUNNING_INSTRUCTIONS.md` | `docs/testing/overview.md` | Consolidate & move |
| `RFP_COMPLIANCE_ASSESSMENT.md` | `docs/reference/rfp-compliance.md` | Move & rename |

### Files to Create

| Path | Purpose | Source |
|------|---------|--------|
| `docs/polkadot/ecosystem-overview.md` | Polkadot ecosystem context | New content |
| `docs/reference/glossary.md` | Technical terms glossary | Consolidated from all docs |
| `docs/api/programmatic-api.md` | Programmatic API docs | Split from API_USAGE.md |

### Files to Update

| Path | Changes |
|------|--------|
| `README.md` | Add Polkadot overview summary, update navigation links |
| `tests/README.md` | Update links to point to `docs/testing/overview.md` |

### Files to Remove

After consolidation and link updates:
- `ALGORITHM_EXTENSIBILITY.md`
- `API_USAGE.md`
- `PERFORMANCE_BENCHMARKS.md`
- `RPC_ARCHIVE_NODES.md`
- `RPC_TESTING.md`
- `TESTING.md`
- `TEST_RUNNING_INSTRUCTIONS.md`
- `RFP_COMPLIANCE_ASSESSMENT.md`

## Validation Rules

1. **Structure Compliance**: All files must be in correct locations per structure above
2. **No Orphaned Files**: All files must be linked from README.md or other documentation
3. **Link Integrity**: All internal links must point to new locations
4. **Content Preservation**: No content loss during consolidation
5. **Term Definitions**: All technical terms defined in glossary or inline

## Success Criteria

- ✅ All documentation files in correct locations per structure
- ✅ README.md provides clear navigation to all documentation
- ✅ No broken internal links
- ✅ All technical terms defined or linked
- ✅ Polkadot ecosystem overview present
- ✅ Test documentation includes examples


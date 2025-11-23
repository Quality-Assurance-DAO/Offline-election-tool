# Data Model: Offline NPoS Election Tool

**Date**: 2025-01-27  
**Feature**: 001-offline-npos-election

## Overview

This document defines the core data models for the Offline NPoS Election Tool. All entities, their fields, relationships, validation rules, and state transitions are documented here.

## Core Entities

### 1. ElectionData

Represents the complete state needed to run an election, including all validator candidates, all nominators, their stake amounts, and voting preferences (edges).

**Fields**:
- `candidates: Vec<ValidatorCandidate>` - List of validator candidates
- `nominators: Vec<Nominator>` - List of nominators with their stakes and votes
- `metadata: ElectionMetadata` - Optional metadata about the election data source

**Validation Rules**:
- Must contain at least one validator candidate (FR-025)
- Must contain at least one nominator (FR-025)
- All candidate account IDs must be unique (FR-022)
- All nominator account IDs must be unique (FR-022)
- All voting edges must reference existing candidates

**Relationships**:
- Contains multiple `ValidatorCandidate` entities
- Contains multiple `Nominator` entities
- Each `Nominator` contains multiple `VotingEdge` references to `ValidatorCandidate`

**State Transitions**:
- **Initial**: Created from RPC, JSON file, or synthetic data
- **Modified**: Can have parameters overridden (stakes, edges, active set size)
- **Validated**: After validation passes, ready for election execution
- **Invalid**: If validation fails, cannot proceed to election

---

### 2. ValidatorCandidate

Represents a potential validator in the election, with associated stake and metadata. Can be real (from chain) or synthetic.

**Fields**:
- `account_id: AccountId` - SS58-encoded account identifier (must be unique)
- `stake: Balance` - Total stake amount (can be zero or overridden)
- `metadata: Option<CandidateMetadata>` - Optional metadata (e.g., commission rate, on-chain status)

**Validation Rules**:
- `account_id` must be valid SS58 format
- `account_id` must be unique within the candidate set (FR-022)
- `stake` must be non-negative (can be zero - FR-018)
- `account_id` can reference accounts that don't exist on-chain (FR-004)

**Relationships**:
- Referenced by multiple `VotingEdge` entities (nominators vote for this candidate)
- Part of `ElectionData.candidates` collection

**State Transitions**:
- **Created**: When loaded from RPC, JSON, or created synthetically
- **Modified**: Stake can be overridden (FR-008)
- **Selected**: After election, if chosen as active validator
- **Not Selected**: After election, if not chosen

---

### 3. Nominator

Represents an account that stakes tokens and votes for validator candidates. Includes stake amount and voting preferences (edges to candidates).

**Fields**:
- `account_id: AccountId` - SS58-encoded account identifier (must be unique)
- `stake: Balance` - Total stake amount available for voting
- `targets: Vec<AccountId>` - List of candidate account IDs this nominator votes for
- `metadata: Option<NominatorMetadata>` - Optional metadata

**Validation Rules**:
- `account_id` must be valid SS58 format
- `account_id` must be unique within the nominator set (FR-022)
- `stake` must be non-negative (can be zero - FR-018)
- All `targets` must reference existing candidates in the election data
- `account_id` can reference accounts that don't exist on-chain (FR-004)

**Relationships**:
- Contains multiple `VotingEdge` references (one per target candidate)
- Part of `ElectionData.nominators` collection

**State Transitions**:
- **Created**: When loaded from RPC, JSON, or created synthetically
- **Modified**: Stake and targets can be overridden (FR-007, FR-008)
- **Voting**: After election, stake is distributed according to algorithm

---

### 4. VotingEdge

Represents a nominator's preference to vote for a specific validator candidate, including the weight or stake allocated to that vote.

**Fields**:
- `nominator_id: AccountId` - Account ID of the nominator
- `candidate_id: AccountId` - Account ID of the candidate being voted for
- `weight: Option<Balance>` - Optional explicit weight (if None, uses nominator's stake proportionally)

**Validation Rules**:
- `nominator_id` must exist in the nominator set
- `candidate_id` must exist in the candidate set
- `weight` must be non-negative if specified
- Edge must not exceed nominator's total stake

**Relationships**:
- Links a `Nominator` to a `ValidatorCandidate`
- Part of `Nominator.targets` collection

**State Transitions**:
- **Created**: When nominator votes are loaded or modified
- **Modified**: Can be added, removed, or weight changed (FR-007)
- **Active**: Used during election execution
- **Distributed**: After election, stake allocation determined by algorithm

---

### 5. ElectionConfiguration

Defines how an election should be executed, including which algorithm to use, active set size, any parameter overrides, and data source settings.

**Fields**:
- `algorithm: AlgorithmType` - Election algorithm to use (sequential-phragmen, parallel-phragmen, multi-phase)
- `active_set_size: u32` - Number of validators to select (must be positive)
- `data_source: DataSource` - Source of election data (RPC, JSON file, synthetic)
- `overrides: Option<ElectionOverrides>` - Optional parameter overrides
- `block_number: Option<BlockNumber>` - Optional block number for RPC snapshot (FR-005)

**Validation Rules**:
- `active_set_size` must be positive integer (FR-006)
- `active_set_size` must not exceed number of available candidates (FR-024)
- `algorithm` must be one of the supported algorithms (FR-009)
- If `data_source` is RPC, `block_number` should be specified
- If `data_source` is JSON, file path must be valid
- If `data_source` is synthetic, no external data required

**Relationships**:
- Used to configure `ElectionExecution`
- References `ElectionData` for input

**State Transitions**:
- **Created**: When user specifies election parameters
- **Validated**: After validation passes
- **Ready**: Ready to execute election

---

### 6. ElectionResult

Contains the outcome of an election, including which validators were selected, how stake is distributed among them, and any diagnostic information explaining the results.

**Fields**:
- `selected_validators: Vec<SelectedValidator>` - List of validators selected for the active set
- `stake_distribution: Vec<StakeAllocation>` - How nominator stakes are allocated across validators
- `total_stake: Balance` - Total stake participating in election
- `algorithm_used: AlgorithmType` - Algorithm that produced these results
- `execution_metadata: ExecutionMetadata` - Timing, block number, etc.

**Validation Rules**:
- Number of `selected_validators` must equal `active_set_size` from configuration
- Total stake in `stake_distribution` must equal `total_stake`
- All selected validators must have been in the candidate set
- All stake allocations must reference valid nominator-validator pairs

**Relationships**:
- Contains multiple `SelectedValidator` entities
- Contains multiple `StakeAllocation` entities
- Produced by `ElectionExecution`

**State Transitions**:
- **Generated**: After election algorithm completes successfully
- **Validated**: After result validation passes
- **Final**: Ready for output (JSON, diagnostics, etc.)

---

### 7. SelectedValidator

Represents a validator that was selected in the election, with associated stake information.

**Fields**:
- `account_id: AccountId` - Account ID of the selected validator
- `total_backing_stake: Balance` - Total stake backing this validator
- `nominator_count: u32` - Number of nominators backing this validator
- `rank: Option<u32>` - Optional rank/position in the active set

**Validation Rules**:
- `account_id` must have been in the candidate set
- `total_backing_stake` must be non-negative
- `nominator_count` must be non-negative

**Relationships**:
- Part of `ElectionResult.selected_validators`
- Referenced by multiple `StakeAllocation` entities

---

### 8. StakeAllocation

Represents how a nominator's stake is allocated to a validator.

**Fields**:
- `nominator_id: AccountId` - Account ID of the nominator
- `validator_id: AccountId` - Account ID of the validator receiving stake
- `amount: Balance` - Amount of stake allocated
- `proportion: f64` - Proportion of nominator's total stake (0.0 to 1.0)

**Validation Rules**:
- `nominator_id` must exist in nominator set
- `validator_id` must be in selected validators
- `amount` must be non-negative
- `proportion` must be between 0.0 and 1.0
- Sum of allocations for a nominator should not exceed their total stake

**Relationships**:
- Links a `Nominator` to a `SelectedValidator`
- Part of `ElectionResult.stake_distribution`

---

### 9. Diagnostics

Detailed explanations about election results, including reasoning for validator selection, stake distribution calculations, and algorithm-specific insights.

**Fields**:
- `validator_explanations: Vec<ValidatorExplanation>` - Why each validator was selected or not
- `stake_analysis: StakeAnalysis` - Analysis of stake distribution patterns
- `algorithm_insights: Option<AlgorithmInsights>` - Algorithm-specific diagnostic information
- `warnings: Vec<DiagnosticWarning>` - Any warnings or notable conditions

**Validation Rules**:
- Should explain at least 95% of selected validators (SC-007)
- Should explain why validators were not selected
- Should provide actionable insights

**Relationships**:
- Generated from `ElectionResult`
- Provides detailed analysis of election outcomes

---

### 10. ValidatorExplanation

Explains why a validator was selected or not selected in the election.

**Fields**:
- `account_id: AccountId` - Account ID of the validator
- `selected: bool` - Whether this validator was selected
- `reason: String` - Human-readable explanation
- `key_factors: Vec<String>` - Key factors that influenced selection (e.g., "high total stake", "many nominators")
- `stake_details: Option<StakeDetails>` - Detailed stake information

**Validation Rules**:
- `reason` must be non-empty
- `key_factors` should highlight most important factors

---

### 11. ElectionOverrides

Optional parameter overrides that modify election data before execution.

**Fields**:
- `candidate_stakes: HashMap<AccountId, Balance>` - Override stake for specific candidates
- `nominator_stakes: HashMap<AccountId, Balance>` - Override stake for specific nominators
- `voting_edges: Vec<EdgeModification>` - Add, modify, or remove voting edges
- `active_set_size: Option<u32>` - Override active set size

**Validation Rules**:
- Overridden stakes must be non-negative
- Modified edges must reference existing candidates and nominators
- Active set size override must be positive and not exceed candidate count

**Relationships**:
- Applied to `ElectionData` before execution
- Part of `ElectionConfiguration`

---

### 12. DataSource

Represents the source of election data.

**Enum Variants**:
- `Rpc { url: String, block_number: Option<BlockNumber> }` - Fetch from Substrate RPC
- `JsonFile { path: PathBuf }` - Load from JSON file
- `Synthetic` - Create synthetic data programmatically

**Validation Rules**:
- RPC URL must be valid HTTP/HTTPS URL
- JSON file path must exist and be readable
- Block number must be valid if specified

---

### 13. AlgorithmType

Represents the election algorithm to use.

**Enum Variants**:
- `SequentialPhragmen` - Sequential Phragmen algorithm
- `ParallelPhragmen` - Parallel Phragmen algorithm
- `MultiPhase` - Multi-phase election algorithm

**Validation Rules**:
- Must be one of the supported algorithms (FR-009)

---

## Type Definitions

### AccountId
- Type: `String` (SS58-encoded)
- Format: SS58 encoding standard used by Substrate chains
- Validation: Must be valid SS58 format, can be any length

### Balance
- Type: `u128` (or `String` for JSON serialization to handle large numbers)
- Range: 0 to 2^128 - 1
- Validation: Must be non-negative

### BlockNumber
- Type: `u32` or `u64` (depending on chain)
- Range: Chain-specific
- Validation: Must be valid block number for the chain

---

## Relationships Diagram

```
ElectionConfiguration
    ├── uses ──> ElectionData
    │              ├── contains ──> ValidatorCandidate[]
    │              └── contains ──> Nominator[]
    │                                  └── contains ──> VotingEdge[]
    │                                                     └── references ──> ValidatorCandidate
    │
    └── executes ──> ElectionResult
                          ├── contains ──> SelectedValidator[]
                          ├── contains ──> StakeAllocation[]
                          └── generates ──> Diagnostics
```

---

## Validation Rules Summary

### Input Validation
1. **Unique Account IDs**: All candidate and nominator account IDs must be unique (FR-022)
2. **Non-Negative Stakes**: All stake values must be non-negative (can be zero - FR-018)
3. **Valid References**: All voting edges must reference existing candidates
4. **Minimum Data**: Must have at least one candidate and one nominator (FR-025)
5. **Active Set Size**: Must not exceed number of candidates (FR-024)

### Execution Validation
1. **Algorithm Convergence**: Algorithm must converge and produce valid results (FR-023)
2. **Result Completeness**: Result must contain exactly `active_set_size` validators
3. **Stake Conservation**: Total allocated stake must equal total nominator stake

### Output Validation
1. **Diagnostic Coverage**: Diagnostics must explain at least 95% of selected validators (SC-007)
2. **Deterministic Results**: Same inputs must produce identical outputs (SC-008)

---

## State Transitions

### ElectionData Lifecycle
```
Created → Validated → Ready → Executed → Result Generated
   ↓         ↓
Invalid   Invalid (validation failed)
```

### ElectionConfiguration Lifecycle
```
Created → Validated → Ready → Executing → Completed
   ↓         ↓
Invalid   Invalid (validation failed)
```

### ElectionResult Lifecycle
```
Generated → Validated → Final
   ↓
Invalid (validation failed)
```

---

## Error Types

### ElectionError (Enum)
- `ValidationError { message: String, field: Option<String> }` - Input validation failure
- `RpcError { message: String, url: String }` - RPC connection/query failure
- `AlgorithmError { message: String, algorithm: AlgorithmType }` - Algorithm execution failure
- `InsufficientCandidates { requested: u32, available: u32 }` - Not enough candidates
- `InvalidData { message: String }` - Invalid election data structure
- `FileError { message: String, path: PathBuf }` - File I/O error

---

## Serialization

All entities support JSON serialization via `serde`:
- Use `serde_json` for JSON file I/O
- Use `serde` derive macros for automatic serialization
- Handle large numbers as strings in JSON to avoid precision loss
- Use SCALE codec for Substrate compatibility where needed

---

## Notes

- All account IDs use SS58 encoding for Substrate compatibility
- Balance values are stored as `u128` internally but serialized as strings in JSON
- The data model supports both real on-chain data and fully synthetic data
- Parameter overrides allow modification of data without changing source files
- Diagnostics provide detailed explanations for transparency and debugging



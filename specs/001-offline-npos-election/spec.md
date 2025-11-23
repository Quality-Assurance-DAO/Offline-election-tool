# Feature Specification: Offline NPoS Election Tool

**Feature Branch**: `001-offline-npos-election`  
**Created**: 2025-01-27  
**Status**: Draft  
**Input**: User description: "Build a new Rust-based offline NPoS election tool that exactly mirrors the election logic of any Substrate chain. Refactor the old substrate-debug-kit/offline-election (https://github.com/paritytech/substrate-debug-kit/tree/master/offline-election ) into a modern Rust project with a clean library/CLI architecture, using Substrate's native crates (sp-npos-elections, frame-election-provider-support, pallet-election-provider-multi-phase, parity-scale-codec, and related runtime primitives) to ensure results match on-chain elections bit-for-bit. Implement a modular election engine that supports all election algorithms (sequential phragmen, parallel phragmen, multi-phase), with a trait-based abstraction to swap algorithms at runtime. Add a flexible input layer that can ingest on-chain state via public RPC (no API keys), JSON files, or fully synthetic voters/candidates—including accounts that do not exist or have no bonded stake—with full parameterization such as custom active-set size, custom edges, overrides, and snapshot-at-block functionality. Provide both a robust CLI (using clap) and a fully documented Rust API that exposes election configuration, execution, and results. Include JSON output, an optional REST API server mode, extensive diagnostics (e.g., explain why each validator is selected), deterministic test harnesses, and continuous compatibility with latest Substrate runtimes."

## Clarifications

### Session 2025-01-27

- Q: How should the REST API handle authentication and access control? → A: No authentication required - public API accessible to anyone
- Q: How should the system handle duplicate account identifiers in input data? → A: Reject duplicates with error - require unique account identifiers
- Q: What should happen when an election algorithm fails to converge or produces invalid results? → A: Return error with diagnostics explaining the failure reason
- Q: What should happen when the requested active validator set size exceeds the number of available candidates? → A: Return error indicating insufficient candidates
- Q: What should happen when election data contains zero validators or zero nominators? → A: Return error indicating invalid election data

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Run Election Simulation from On-Chain State (Priority: P1)

A validator operator needs to predict election outcomes before they occur on-chain. They connect to a public Substrate node, specify a block number to snapshot the state, and run an election simulation to see which validators would be selected and how stake would be distributed.

**Why this priority**: This is the core use case - enabling users to simulate elections offline using real blockchain data. Without this, the tool cannot fulfill its primary purpose of mirroring on-chain elections.

**Independent Test**: Can be fully tested by connecting to any public Substrate node, fetching state at a specific block, running an election, and comparing results to the actual on-chain election outcome at that block. Delivers immediate value by enabling election prediction.

**Acceptance Scenarios**:

1. **Given** a public Substrate node URL and a block number, **When** a user requests an election simulation, **Then** the system fetches all validator candidates, nominator stakes, and voting preferences from that block and produces election results
2. **Given** an election simulation request, **When** the system cannot connect to the node, **Then** the system provides a clear error message indicating the connection failure
3. **Given** a block number that doesn't exist, **When** a user requests an election simulation, **Then** the system reports that the block is not available

---

### User Story 2 - Run Election with Custom Parameters (Priority: P1)

A researcher needs to test "what-if" scenarios by modifying election parameters. They load election data from a file or RPC, then override specific parameters like active validator set size, add synthetic candidates with no stake, modify voting edges, or adjust stake amounts to understand how these changes affect election outcomes.

**Why this priority**: This enables advanced use cases like research, testing, and scenario planning. It's essential for understanding election behavior under different conditions and is a key differentiator from simple on-chain queries.

**Independent Test**: Can be fully tested by loading a known election dataset, applying parameter overrides (e.g., changing active set size from 100 to 50), running the election, and verifying that results reflect the changes. Delivers value by enabling experimentation and analysis.

**Acceptance Scenarios**:

1. **Given** election data loaded from any source, **When** a user specifies a custom active validator set size, **Then** the election produces results with exactly that many validators selected
2. **Given** election data, **When** a user adds a synthetic candidate account that doesn't exist on-chain, **Then** the system includes that candidate in the election and produces results including or excluding them based on stake
3. **Given** election data, **When** a user modifies voting edges (nominator-to-validator preferences), **Then** the election results reflect the modified voting patterns
4. **Given** election data, **When** a user sets a candidate's stake to zero or removes them, **Then** the system handles this gracefully and produces valid results

---

### User Story 3 - Compare Different Election Algorithms (Priority: P2)

A developer needs to understand how different election algorithms (sequential phragmen, parallel phragmen, multi-phase) produce different results given the same input data. They run the same election data through multiple algorithms and compare the selected validators and stake distribution.

**Why this priority**: This enables users to understand algorithm differences and choose the appropriate one for their chain. It's valuable for research and chain configuration decisions.

**Independent Test**: Can be fully tested by loading a single election dataset, running it through each available algorithm, and comparing the output results. Delivers value by enabling algorithm comparison and selection.

**Acceptance Scenarios**:

1. **Given** election data, **When** a user runs the same data through sequential phragmen algorithm, **Then** the system produces valid election results
2. **Given** election data, **When** a user runs the same data through parallel phragmen algorithm, **Then** the system produces valid election results that may differ from sequential phragmen
3. **Given** election data, **When** a user runs the same data through multi-phase algorithm, **Then** the system produces valid election results
4. **Given** election results from multiple algorithms, **When** a user compares them, **Then** the system provides clear output showing differences in selected validators and stake distribution

---

### User Story 4 - Get Detailed Election Diagnostics (Priority: P2)

A validator wants to understand why they were or weren't selected in an election. They run an election and receive detailed diagnostics explaining the selection criteria, stake calculations, voting patterns, and reasoning for each validator's inclusion or exclusion.

**Why this priority**: This provides transparency and helps validators optimize their strategies. It's valuable for understanding election mechanics and improving validator performance.

**Independent Test**: Can be fully tested by running any election and verifying that the output includes explanations for validator selection decisions. Delivers value by providing actionable insights.

**Acceptance Scenarios**:

1. **Given** an election result, **When** a user requests diagnostics, **Then** the system explains why each selected validator was chosen (e.g., total stake, voting support, algorithm-specific criteria)
2. **Given** an election result, **When** a user requests diagnostics, **Then** the system explains why specific validators were not selected (e.g., insufficient stake, lack of nominations)
3. **Given** an election result, **When** a user requests diagnostics, **Then** the system provides stake distribution details showing how nominator stakes are allocated across validators

---

### User Story 5 - Use Tool Programmatically via API (Priority: P3)

A developer wants to integrate election simulation into their own application. They use the programmatic API to configure elections, execute them, and retrieve results without using the command-line interface.

**Why this priority**: This enables integration with other tools and automation. While CLI is sufficient for many users, API access enables advanced use cases and tooling.

**Independent Test**: Can be fully tested by writing a simple program that uses the API to load data, configure an election, run it, and retrieve results. Delivers value by enabling automation and integration.

**Acceptance Scenarios**:

1. **Given** the programmatic API, **When** a developer configures an election with data and parameters, **Then** they can execute it and receive results programmatically
2. **Given** the programmatic API, **When** a developer requests election results, **Then** they receive structured data that can be processed by their application
3. **Given** the programmatic API, **When** a developer uses it incorrectly, **Then** the system provides clear error messages indicating what went wrong

---

### User Story 6 - Access Tool via REST API Server (Priority: P3)

A team wants to run election simulations through a web service without installing the tool locally. They start a server, send HTTP requests with election parameters, and receive JSON responses with results.

**Why this priority**: This enables remote access and integration with web applications. It's valuable for teams and services that need centralized election simulation capabilities.

**Independent Test**: Can be fully tested by starting the server, sending HTTP requests with election data, and verifying JSON responses contain valid results. Delivers value by enabling remote access and web integration.

**Acceptance Scenarios**:

1. **Given** the server is running, **When** a user sends a POST request with election configuration, **Then** the server processes the election and returns JSON results
2. **Given** the server is running, **When** a user requests election diagnostics via HTTP, **Then** the server returns detailed diagnostic information in JSON format
3. **Given** an invalid HTTP request, **When** the server receives it, **Then** it returns an appropriate HTTP error response with error details

---

### Edge Cases

- What happens when election data contains zero validators or zero nominators? **Answer**: System returns an error indicating that the election data is invalid, as elections require at least one validator candidate and at least one nominator to produce meaningful results.
- How does the system handle election data with malformed or invalid account addresses?
- What happens when the requested active set size is larger than the number of available candidates? **Answer**: System returns an error indicating that there are insufficient candidates to fill the requested active set size.
- How does the system handle very large election datasets (thousands of validators, millions of nominators)?
- What happens when RPC connection times out or returns partial data?
- How does the system handle election data files with invalid JSON structure?
- What happens when synthetic candidates have negative stake values?
- How does the system handle duplicate candidate or nominator entries in input data? **Answer**: System rejects duplicate account identifiers with a clear error message, requiring unique identifiers for all candidates and nominators.
- What happens when voting edges reference candidates that don't exist in the candidate set?
- How does the system handle election algorithms that fail to converge or produce invalid results? **Answer**: System returns an error with detailed diagnostics explaining why the algorithm failed to converge or why results are invalid, allowing users to understand and address the issue.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST produce election results that exactly match on-chain election results when given identical input data and parameters
- **FR-002**: System MUST support fetching election data (validators, nominators, stakes, votes) from public Substrate nodes via RPC without requiring authentication credentials
- **FR-003**: System MUST support loading election data from JSON files provided by users
- **FR-004**: System MUST support creating fully synthetic election data including candidates and nominators that don't exist on-chain
- **FR-005**: System MUST support specifying a block number to snapshot state when fetching from RPC
- **FR-006**: System MUST support overriding the active validator set size to any positive integer value
- **FR-007**: System MUST support adding, modifying, or removing voting edges (nominator-to-validator preferences) in election data
- **FR-008**: System MUST support overriding stake amounts for candidates and nominators
- **FR-009**: System MUST support multiple election algorithms (sequential phragmen, parallel phragmen, multi-phase) and allow users to select which algorithm to use
- **FR-010**: System MUST allow swapping election algorithms at runtime without changing input data
- **FR-011**: System MUST provide a command-line interface for running elections
- **FR-012**: System MUST provide a programmatic API for running elections from other applications
- **FR-013**: System MUST output election results in JSON format
- **FR-014**: System MUST provide an optional REST API server mode for remote access
- **FR-021**: REST API server MUST operate without authentication requirements - all endpoints are publicly accessible
- **FR-015**: System MUST provide detailed diagnostics explaining why each validator was selected or not selected
- **FR-016**: System MUST support deterministic test harnesses that produce consistent results for the same inputs
- **FR-017**: System MUST maintain compatibility with the latest Substrate runtime versions
- **FR-018**: System MUST handle election data with accounts that have zero or no bonded stake
- **FR-019**: System MUST validate input data and provide clear error messages for invalid inputs
- **FR-020**: System MUST handle RPC connection failures gracefully with informative error messages
- **FR-022**: System MUST reject election data containing duplicate account identifiers (for candidates or nominators) and return an error indicating which accounts are duplicated
- **FR-023**: System MUST return an error with detailed diagnostics when an election algorithm fails to converge or produces invalid results, explaining the failure reason to help users understand and address the issue
- **FR-024**: System MUST return an error when the requested active validator set size exceeds the number of available candidates, clearly indicating the insufficient candidate count
- **FR-025**: System MUST return an error when election data contains zero validator candidates or zero nominators, indicating that the election data is invalid

### Key Entities

- **Election Data**: Represents the complete state needed to run an election, including all validator candidates, all nominators, their stake amounts, and voting preferences (edges). Can be sourced from on-chain state, JSON files, or created synthetically.

- **Election Configuration**: Defines how an election should be executed, including which algorithm to use, active set size, any parameter overrides, and data source settings.

- **Election Result**: Contains the outcome of an election, including which validators were selected, how stake is distributed among them, and any diagnostic information explaining the results.

- **Validator Candidate**: Represents a potential validator in the election, with associated stake and metadata. Can be real (from chain) or synthetic.

- **Nominator**: Represents an account that stakes tokens and votes for validator candidates. Includes stake amount and voting preferences (edges to candidates).

- **Voting Edge**: Represents a nominator's preference to vote for a specific validator candidate, including the weight or stake allocated to that vote.

- **Election Algorithm**: Defines the computational method used to select validators and distribute stake. Different algorithms may produce different results for the same input data.

- **Diagnostics**: Detailed explanations about election results, including reasoning for validator selection, stake distribution calculations, and algorithm-specific insights.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: When given identical input data and parameters as an on-chain election, the system produces results that match the on-chain results with 100% accuracy (same validators selected, same stake distribution)

- **SC-002**: Users can successfully fetch election data from any public Substrate node and run an election simulation in under 30 seconds for typical network conditions

- **SC-003**: Users can load election data from JSON files and run elections without requiring network connectivity

- **SC-004**: Users can create and run elections with fully synthetic data (including non-existent accounts) without errors

- **SC-005**: Users can override election parameters (active set size, edges, stake amounts) and see results reflect those changes immediately

- **SC-006**: Users can run the same election data through different algorithms and receive valid results from each algorithm

- **SC-007**: Election results include diagnostic information that explains validator selection decisions for at least 95% of selected validators

- **SC-008**: The system produces deterministic results - running the same election with the same inputs produces identical outputs

- **SC-009**: The programmatic API allows developers to configure and execute elections programmatically without using the CLI

- **SC-010**: The REST API server responds to election requests within 5 seconds for typical election sizes

- **SC-011**: The system successfully processes election data with up to 1,000 validator candidates and 10,000 nominators without performance degradation

- **SC-012**: The system maintains compatibility with Substrate runtime updates - when a new Substrate version is released, the tool continues to work with chains using that version within 30 days

## Assumptions

- Public Substrate RPC nodes are available and accessible without authentication
- Users have network connectivity when fetching data from RPC (optional for file-based and synthetic data)
- Election data formats follow standard Substrate conventions
- Users understand basic concepts of validator elections and stake distribution
- JSON file formats will be documented and users will follow the documented structure
- Substrate runtime changes that affect election logic will be backward-compatible or the tool will be updated accordingly
- Users may want to test edge cases with invalid or unusual data, and the system should handle these gracefully

## Dependencies

- Access to public Substrate RPC endpoints (for on-chain data fetching)
- Substrate runtime compatibility (for accurate election simulation)
- Standard network protocols for RPC communication

## Out of Scope

- Modifying on-chain election results (this is a read-only simulation tool)
- Real-time monitoring of on-chain elections (focus is on simulation, not live tracking)
- Wallet integration or transaction signing
- Direct blockchain node operation or consensus participation
- Election result visualization or graphical interfaces (focus is on data and diagnostics)

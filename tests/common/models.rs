//! Test data models for fixtures, chain snapshots, benchmarks, and edge cases

use chrono::{DateTime, Utc};
use offline_election::models::{ElectionData, ElectionResult};
use offline_election::types::AlgorithmType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Test fixture metadata
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestFixtureMetadata {
    /// Unique identifier for the test scenario
    pub test_name: String,
    /// Human-readable description of what the test validates
    pub description: String,
    /// Timestamp when fixture was created
    pub created: DateTime<Utc>,
    /// Election algorithm used
    pub algorithm: AlgorithmType,
    /// Test category
    pub category: TestCategory,
    /// Optional tags for test organization
    #[serde(default)]
    pub tags: Vec<String>,
}

/// Test fixture containing input data and expected results
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestFixture {
    /// Metadata about the test scenario
    pub metadata: TestFixtureMetadata,
    /// Input election data
    pub input: ElectionData,
    /// Expected election result (for regression tests)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_result: Option<ElectionResult>,
}

/// Chain snapshot metadata
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChainSnapshotMetadata {
    /// Chain identifier (e.g., "polkadot", "kusama")
    pub chain: String,
    /// Block number at which snapshot was taken
    pub block_number: u64,
    /// Timestamp when snapshot was created
    pub timestamp: DateTime<Utc>,
    /// RPC endpoint URL used to fetch data
    pub rpc_endpoint: String,
    /// List of validator account IDs selected on-chain
    pub expected_validators: Vec<String>,
    /// On-chain stake allocations (nominator -> validator -> amount)
    pub expected_stake_allocations: HashMap<String, HashMap<String, u128>>,
}

/// Chain snapshot containing election data and expected results
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChainSnapshot {
    /// Chain and block information
    pub metadata: ChainSnapshotMetadata,
    /// Election data fetched from chain at specified block
    pub election_data: ElectionData,
    /// Actual on-chain election result for comparison
    pub expected_result: ElectionResult,
}

/// Benchmark metadata
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BenchmarkMetadata {
    /// Unique identifier for the benchmark
    pub benchmark_name: String,
    /// Description of what the benchmark measures
    pub description: String,
    /// Number of validator candidates in dataset
    pub candidate_count: usize,
    /// Number of nominators in dataset
    pub nominator_count: usize,
    /// Election algorithm being benchmarked
    pub algorithm: AlgorithmType,
    /// Target execution time in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_time_ms: Option<u64>,
    /// Target memory usage in MB
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_memory_mb: Option<u64>,
}

/// Benchmark results
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BenchmarkResults {
    /// Total execution time in milliseconds
    pub execution_time_ms: u64,
    /// Peak memory usage in MB
    pub memory_peak_mb: u64,
    /// Final memory usage after execution
    pub memory_final_mb: u64,
    /// Number of iterations run
    pub iterations: usize,
    /// Mean execution time (if multiple iterations)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mean_time_ms: Option<f64>,
    /// Standard deviation of execution times
    #[serde(skip_serializing_if = "Option::is_none")]
    pub std_dev_ms: Option<f64>,
    /// Additional metadata
    #[serde(default)]
    pub metadata: HashMap<String, String>,
}

/// Edge case scenario
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EdgeCaseScenario {
    /// Unique identifier for the scenario
    pub scenario_name: String,
    /// Description of the edge case being tested
    pub description: String,
    /// Input data representing the edge case
    pub input: ElectionData,
    /// Expected system behavior
    pub expected_behavior: ExpectedBehavior,
    /// Tags for categorization
    #[serde(default)]
    pub tags: Vec<String>,
}

/// Expected behavior for an edge case test
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExpectedBehavior {
    /// Whether the election should complete successfully
    pub should_succeed: bool,
    /// Expected result if successful
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_result: Option<ElectionResult>,
    /// Expected error pattern if failure
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_error: Option<String>,
    /// Substrings that must appear in error message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message_contains: Option<Vec<String>>,
}

/// Test category enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestCategory {
    /// Tests for boundary conditions and unusual inputs
    EdgeCase,
    /// Tests measuring execution time and memory usage
    Performance,
    /// Tests ensuring results remain consistent across changes
    Regression,
    /// Tests validating accuracy against on-chain results
    ChainSnapshot,
    /// End-to-end integration tests
    Integration,
}


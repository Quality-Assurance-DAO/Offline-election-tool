//! REST API request/response models

use crate::models::election_data::ElectionData;
use crate::models::election_overrides::ElectionOverrides;
use crate::models::election_result::ElectionResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Election request model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectionRequest {
    /// Election algorithm to use
    pub algorithm: String,
    /// Number of validators to select
    pub active_set_size: u32,
    /// Data source for election data
    pub data_source: DataSource,
    /// Optional parameter overrides
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrides: Option<ElectionOverrides>,
    /// Optional block number for RPC snapshot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_number: Option<u64>,
}

/// Data source for election data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum DataSource {
    /// Fetch data from RPC endpoint
    Rpc {
        /// RPC endpoint URL
        url: String,
        /// Optional block number
        #[serde(skip_serializing_if = "Option::is_none")]
        block_number: Option<u64>,
    },
    /// Use JSON data provided in request
    Json {
        /// Election data as JSON object
        data: ElectionData,
    },
    /// Create synthetic data
    Synthetic {
        /// List of candidates
        candidates: Vec<CandidateInput>,
        /// List of nominators
        nominators: Vec<NominatorInput>,
    },
}

/// Candidate input for synthetic data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandidateInput {
    /// Account ID
    pub account_id: String,
    /// Stake amount (as string to handle large numbers)
    pub stake: String,
}

/// Nominator input for synthetic data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NominatorInput {
    /// Account ID
    pub account_id: String,
    /// Stake amount (as string to handle large numbers)
    pub stake: String,
    /// List of candidate account IDs to vote for
    pub targets: Vec<String>,
}

/// Election response model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectionResponse {
    /// Unique identifier for this election
    pub election_id: String,
    /// Election result
    pub result: ElectionResult,
    /// Execution time in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_time_ms: Option<u64>,
}

/// Error response model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    /// Error type/code
    pub error: String,
    /// Human-readable error message
    pub message: String,
    /// Additional error details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<HashMap<String, serde_json::Value>>,
    /// Field name if validation error
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
}

impl ErrorResponse {
    /// Create a new error response
    pub fn new(error: String, message: String) -> Self {
        Self {
            error,
            message,
            details: None,
            field: None,
        }
    }

    /// Create a validation error response
    pub fn validation_error(message: String, field: Option<String>) -> Self {
        Self {
            error: "VALIDATION_ERROR".to_string(),
            message,
            details: None,
            field,
        }
    }
}



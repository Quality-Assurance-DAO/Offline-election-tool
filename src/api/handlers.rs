//! REST API request handlers

use crate::api::models::{DataSource, ElectionRequest, ElectionResponse, ErrorResponse};
use crate::diagnostics::explainer::DiagnosticsGenerator;
use crate::engine::ElectionEngine;
use crate::error::ElectionError;
use crate::input::rpc::RpcLoader;
use crate::input::synthetic::SyntheticDataBuilder;
use crate::models::election_config::ElectionConfiguration;
use crate::models::election_data::ElectionData;
use crate::types::AlgorithmType;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Stored election data including result and original data for diagnostics
#[derive(Clone)]
struct StoredElection {
    response: ElectionResponse,
    original_data: ElectionData,
}

/// In-memory storage for election results (for demo purposes)
/// In production, this would be replaced with a database
type ElectionStorage = Arc<RwLock<HashMap<String, StoredElection>>>;

/// Handler state containing shared resources
#[derive(Clone)]
pub struct HandlerState {
    /// Storage for election results
    storage: ElectionStorage,
}

impl HandlerState {
    /// Create a new handler state
    pub fn new() -> Self {
        Self {
            storage: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for HandlerState {
    fn default() -> Self {
        Self::new()
    }
}

/// Run an election simulation
pub async fn run_election(
    axum::extract::State(state): axum::extract::State<HandlerState>,
    Json(request): Json<ElectionRequest>,
) -> Result<Json<ElectionResponse>, ApiError> {
    let start_time = std::time::Instant::now();

    // Parse algorithm type
    let algorithm = request.algorithm.parse::<AlgorithmType>()
        .map_err(|e| ApiError::Validation(format!("Invalid algorithm: {}", e)))?;

    // Load election data based on data source
    let election_data = load_election_data(&request.data_source).await
        .map_err(|e| ApiError::Election(e))?;

    // Create election configuration
    let mut config = ElectionConfiguration::new()
        .algorithm(algorithm)
        .active_set_size(request.active_set_size);

    if let Some(block) = request.block_number {
        config = config.block_number(block);
    }

    // Apply overrides if present
    if let Some(ref overrides) = request.overrides {
        config = config.overrides(overrides.clone());
    }

    let config = config.build()
        .map_err(|e| ApiError::Validation(e.to_string()))?;

    // Execute election
    let engine = ElectionEngine::new();
    let result = engine.execute(&config, &election_data)
        .map_err(|e| ApiError::Election(e))?;

    // Calculate execution time
    let execution_time_ms = start_time.elapsed().as_millis() as u64;

    // Create response
    let election_id = Uuid::new_v4().to_string();
    let response = ElectionResponse {
        election_id: election_id.clone(),
        result,
        execution_time_ms: Some(execution_time_ms),
    };

    // Store result with original data for diagnostics generation
    state.storage.write().await.insert(election_id.clone(), StoredElection {
        response: response.clone(),
        original_data: election_data.clone(),
    });

    Ok(Json(response))
}

/// Get election results by ID
pub async fn get_election_results(
    axum::extract::State(state): axum::extract::State<HandlerState>,
    Path(election_id): Path<String>,
) -> Result<Json<ElectionResponse>, ApiError> {
    let storage = state.storage.read().await;
    storage.get(&election_id)
        .map(|stored| stored.response.clone())
        .ok_or_else(|| ApiError::NotFound(format!("Election not found: {}", election_id)))
        .map(Json)
}

/// Get election diagnostics by ID
pub async fn get_election_diagnostics(
    axum::extract::State(state): axum::extract::State<HandlerState>,
    Path(election_id): Path<String>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let storage = state.storage.read().await;
    let stored = storage.get(&election_id)
        .ok_or_else(|| ApiError::NotFound(format!("Election not found: {}", election_id)))?;

    // Generate diagnostics from stored result and original data
    let diagnostics_gen = DiagnosticsGenerator::new();
    let diagnostics = diagnostics_gen.generate(&stored.response.result, &stored.original_data)
        .map_err(|e| ApiError::Internal(format!("Failed to generate diagnostics: {}", e)))?;

    // Convert diagnostics to JSON
    let diagnostics_json = serde_json::to_value(&diagnostics)
        .map_err(|e| ApiError::Internal(format!("Failed to serialize diagnostics: {}", e)))?;

    Ok(Json(diagnostics_json))
}

/// Load election data from the specified data source
async fn load_election_data(data_source: &DataSource) -> Result<ElectionData, ElectionError> {
    match data_source {
        DataSource::Rpc { url, block_number } => {
            let loader = RpcLoader::new(url)?;
            if let Some(block) = block_number {
                loader.load_at_block(*block).await
            } else {
                loader.load_latest().await
            }
        }
        DataSource::Json { data } => {
            // Validate the provided data
            data.validate()?;
            Ok(data.clone())
        }
        DataSource::Synthetic { candidates, nominators } => {
            let mut builder = SyntheticDataBuilder::new();

            // Add candidates
            for candidate in candidates {
                let stake = candidate.stake.parse::<u128>()
                    .map_err(|e| ElectionError::ValidationError {
                        message: format!("Invalid stake value: {}", e),
                        field: Some("candidates.stake".to_string()),
                    })?;
                builder.add_candidate(candidate.account_id.clone(), stake)?;
            }

            // Add nominators
            for nominator in nominators {
                let stake = nominator.stake.parse::<u128>()
                    .map_err(|e| ElectionError::ValidationError {
                        message: format!("Invalid stake value: {}", e),
                        field: Some("nominators.stake".to_string()),
                    })?;
                builder.add_nominator(
                    nominator.account_id.clone(),
                    stake,
                    nominator.targets.clone(),
                )?;
            }

            builder.build()
        }
    }
}

/// API error type
#[derive(Debug)]
pub enum ApiError {
    /// Validation error
    Validation(String),
    /// Election execution error
    Election(ElectionError),
    /// Not found error
    NotFound(String),
    /// Internal server error
    Internal(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_response) = match self {
            ApiError::Validation(msg) => (
                StatusCode::BAD_REQUEST,
                ErrorResponse::validation_error(msg, None),
            ),
            ApiError::Election(e) => {
                let (status, error, message) = match e {
                    ElectionError::ValidationError { message, field: _ } => (
                        StatusCode::BAD_REQUEST,
                        "VALIDATION_ERROR".to_string(),
                        message,
                    ),
                    ElectionError::InsufficientCandidates { requested, available } => (
                        StatusCode::BAD_REQUEST,
                        "INSUFFICIENT_CANDIDATES".to_string(),
                        format!("Requested {} candidates but only {} available", requested, available),
                    ),
                    ElectionError::RpcError { message, .. } => (
                        StatusCode::BAD_GATEWAY,
                        "RPC_ERROR".to_string(),
                        message,
                    ),
                    ElectionError::AlgorithmError { message, .. } => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "ALGORITHM_ERROR".to_string(),
                        message,
                    ),
                    _ => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "ELECTION_ERROR".to_string(),
                        e.to_string(),
                    ),
                };
                (status, ErrorResponse::new(error, message))
            }
            ApiError::NotFound(msg) => (
                StatusCode::NOT_FOUND,
                ErrorResponse::new("NOT_FOUND".to_string(), msg),
            ),
            ApiError::Internal(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse::new("INTERNAL_ERROR".to_string(), msg),
            ),
        };

        (status, Json(error_response)).into_response()
    }
}



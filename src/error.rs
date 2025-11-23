//! Error types for the Offline NPoS Election Tool

use crate::types::AlgorithmType;
use std::path::PathBuf;
use thiserror::Error;

/// Main error type for election operations
#[derive(Debug, Error)]
pub enum ElectionError {
    /// Validation error with optional field name
    #[error("Validation error: {message}")]
    ValidationError {
        message: String,
        field: Option<String>,
    },

    /// RPC connection or query error
    #[error("RPC error: {message} (URL: {url})")]
    RpcError {
        message: String,
        url: String,
    },

    /// Algorithm execution error
    #[error("Algorithm error: {message} (algorithm: {algorithm:?})")]
    AlgorithmError {
        message: String,
        algorithm: AlgorithmType,
    },

    /// Insufficient candidates for requested active set size
    #[error("Insufficient candidates: requested {requested}, available {available}")]
    InsufficientCandidates {
        requested: u32,
        available: u32,
    },

    /// Invalid data structure
    #[error("Invalid data: {message}")]
    InvalidData { message: String },

    /// File I/O error
    #[error("File error: {message} (path: {path:?})")]
    FileError {
        message: String,
        path: PathBuf,
    },
}



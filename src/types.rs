//! Type definitions for algorithm types and data sources

use serde::{Deserialize, Serialize};

/// Election algorithm type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AlgorithmType {
    /// Sequential Phragmen algorithm
    SequentialPhragmen,
    /// Parallel Phragmen algorithm
    ParallelPhragmen,
    /// Multi-phase election algorithm
    MultiPhase,
}

impl std::str::FromStr for AlgorithmType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "sequential-phragmen" | "sequential" => Ok(AlgorithmType::SequentialPhragmen),
            "parallel-phragmen" | "parallel" => Ok(AlgorithmType::ParallelPhragmen),
            "multi-phase" | "multiphase" => Ok(AlgorithmType::MultiPhase),
            _ => Err(format!("Unknown algorithm type: {}", s)),
        }
    }
}

impl std::fmt::Display for AlgorithmType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AlgorithmType::SequentialPhragmen => write!(f, "sequential-phragmen"),
            AlgorithmType::ParallelPhragmen => write!(f, "parallel-phragmen"),
            AlgorithmType::MultiPhase => write!(f, "multi-phase"),
        }
    }
}

/// Data source for election data
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum DataSource {
    /// Fetch data from Substrate RPC endpoint
    Rpc {
        /// RPC endpoint URL
        url: String,
        /// Optional block number to snapshot state
        block_number: Option<u64>,
    },
    /// Load data from JSON file
    JsonFile {
        /// Path to JSON file
        path: std::path::PathBuf,
    },
    /// Create synthetic data programmatically
    Synthetic,
}



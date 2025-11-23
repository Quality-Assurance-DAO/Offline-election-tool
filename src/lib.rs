//! Offline NPoS Election Tool
//!
//! A Rust library for running offline NPoS (Nominated Proof of Stake) election simulations
//! that exactly mirror the election logic of Substrate chains.

pub mod algorithms;
pub mod api;
pub mod cli;
pub mod diagnostics;
pub mod engine;
pub mod error;
pub mod input;
pub mod models;
pub mod types;

// Re-export commonly used types
pub use engine::ElectionEngine;
pub use error::ElectionError;
pub use models::election_config::ElectionConfiguration;
pub use models::election_data::ElectionData;
pub use models::election_result::ElectionResult;
pub use models::election_overrides::ElectionOverrides;
pub use models::nominator::Nominator;
pub use models::validator::ValidatorCandidate;
pub use types::AlgorithmType;
pub use types::DataSource;


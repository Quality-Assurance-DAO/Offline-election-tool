//! Common test utilities module
//! 
//! This module provides shared utilities for all integration tests.
//!
//! Note: Many functions and types here are intentionally unused in specific test files
//! but are provided for use across the entire test suite. Warnings are suppressed
//! for intentionally unused but provided utilities.

#![allow(dead_code)]
#![allow(unused_imports)]

pub mod assertions;
pub mod benchmark_utils;
pub mod data_generator;
pub mod fixture_loader;
pub mod models;
pub mod rpc_retry;
pub mod rpc_utils;

// Re-export all utilities for use in test files
// These may be unused in individual test files but are provided for the entire test suite
pub use assertions::*;
pub use benchmark_utils::*;
pub use data_generator::*;
pub use fixture_loader::*;
pub use models::*;
pub use rpc_retry::*;
pub use rpc_utils::*;


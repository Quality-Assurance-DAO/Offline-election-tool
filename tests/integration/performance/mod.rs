//! Performance integration tests
//!
//! This module contains performance tests for large-scale election scenarios.
//! Tests are marked with #[ignore] by default and should be run with:
//! `cargo test -- --ignored`

mod test_large_scale_1k;
mod test_large_scale_5k;
mod test_large_scale_10k;
mod test_large_nominee_sets;
mod test_max_active_set_large;
mod test_dense_voting;
mod test_sparse_voting;
mod test_memory_leak;
mod test_concurrent_execution;
mod test_polkadot_mainnet;

pub use test_large_scale_1k::*;
pub use test_large_scale_5k::*;
pub use test_large_scale_10k::*;
pub use test_large_nominee_sets::*;
pub use test_max_active_set_large::*;
pub use test_dense_voting::*;
pub use test_sparse_voting::*;
pub use test_memory_leak::*;
pub use test_concurrent_execution::*;
pub use test_polkadot_mainnet::*;


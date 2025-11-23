//! Regression test suite
//!
//! This module contains regression tests that ensure election results remain
//! consistent across code changes. Tests use fixtures with known expected results
//! and fail if results change unexpectedly.

mod test_runner;
mod test_normal_elections;
mod test_edge_case_regression;
mod test_performance_regression;

pub use test_runner::*;
pub use test_normal_elections::*;
pub use test_edge_case_regression::*;
pub use test_performance_regression::*;


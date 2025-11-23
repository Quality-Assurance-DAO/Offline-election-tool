//! Edge case integration tests

mod test_zero_candidates;
mod test_zero_nominators;
mod test_single_candidate;
mod test_single_nominator;
mod test_zero_candidate_stakes;
mod test_max_active_set_size;
mod test_empty_voting_edges;
mod test_duplicate_account_ids;

pub use test_zero_candidates::*;
pub use test_zero_nominators::*;
pub use test_single_candidate::*;
pub use test_single_nominator::*;
pub use test_zero_candidate_stakes::*;
pub use test_max_active_set_size::*;
pub use test_empty_voting_edges::*;
pub use test_duplicate_account_ids::*;


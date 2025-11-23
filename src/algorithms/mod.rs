//! Election algorithm implementations

pub mod trait_def;
pub mod sequential_phragmen;
pub mod parallel_phragmen;
pub mod multi_phase;

pub use trait_def::ElectionAlgorithm;
pub use sequential_phragmen::SequentialPhragmen;
pub use parallel_phragmen::ParallelPhragmen;


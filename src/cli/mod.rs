//! CLI interface for the Offline NPoS Election Tool

pub mod commands;
pub mod output;

pub use commands::{RunCommand, ServerCommand};
pub use output::format_json;



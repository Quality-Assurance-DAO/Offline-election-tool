//! Input data loading from various sources (RPC, JSON files, synthetic data)

pub mod rpc;
pub mod json;
pub mod synthetic;

pub use rpc::RpcLoader;
pub use json::JsonLoader;
pub use synthetic::SyntheticDataBuilder;



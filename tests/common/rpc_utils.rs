//! RPC utilities for chain snapshot fetching

use crate::common::models::ChainSnapshot;
use offline_election::models::ElectionData;
use offline_election::input::rpc::RpcLoader;
use crate::common::rpc_retry::retry_with_backoff;
use chrono::Utc;
use std::path::Path;
use std::time::Duration;

/// Fetch chain snapshot from RPC endpoint
/// 
/// # Arguments
/// * `rpc_endpoint` - RPC endpoint URL
/// * `block_number` - Block number to snapshot
/// 
/// # Returns
/// ChainSnapshot with election data and expected results
pub async fn fetch_chain_snapshot(
    rpc_endpoint: &str,
    block_number: u64,
) -> Result<ChainSnapshot, String> {
    // Load election data from RPC
    let _election_data = ElectionData::from_rpc(rpc_endpoint, Some(block_number))
        .await
        .map_err(|e| format!("Failed to fetch election data from RPC: {}", e))?;
    
    // TODO: Fetch expected results from chain
    // For now, create a placeholder snapshot
    // In a real implementation, this would fetch the actual on-chain election results
    
    Err("Chain snapshot fetching not yet fully implemented".to_string())
}

/// Save chain snapshot to JSON file
pub fn save_chain_snapshot<P: AsRef<Path>>(
    snapshot: &ChainSnapshot,
    path: P,
) -> Result<(), String> {
    let json = serde_json::to_string_pretty(snapshot)
        .map_err(|e| format!("Failed to serialize chain snapshot: {}", e))?;
    
    std::fs::write(path.as_ref(), json)
        .map_err(|e| format!("Failed to write chain snapshot to {:?}: {}", path.as_ref(), e))?;
    
    Ok(())
}

/// Calculate a recent block number within the last 30 days
/// 
/// Polkadot block time is approximately 6 seconds, so:
/// - Blocks per day: ~14,400
/// - Blocks per 30 days: ~432,000
/// 
/// # Arguments
/// * `latest_block` - The latest block number from the chain
/// 
/// # Returns
/// A block number that is approximately 30 days old (or latest_block if it's less than 30 days old)
pub fn calculate_recent_block_number(latest_block: u64) -> u64 {
    // Polkadot block time: ~6 seconds
    // Blocks per day: 86400 / 6 = 14,400
    // Blocks per 30 days: 14,400 * 30 = 432,000
    const BLOCKS_PER_30_DAYS: u64 = 432_000;
    
    if latest_block > BLOCKS_PER_30_DAYS {
        latest_block - BLOCKS_PER_30_DAYS
    } else {
        // If chain is less than 30 days old, use block 1
        1
    }
}

/// Polkadot mainnet snapshot structure for benchmarks
#[derive(Debug, Clone)]
pub struct PolkadotMainnetSnapshot {
    pub election_data: ElectionData,
    pub block_number: u64,
    pub rpc_endpoint: String,
    pub fetch_timestamp: chrono::DateTime<Utc>,
}

/// Fetch Polkadot mainnet snapshot using RPC loader with retry logic
/// 
/// Uses retry_with_backoff from tests/common/rpc_retry.rs with max_attempts: 3,
/// initial_delay: Duration::from_secs(1) for benchmark tests.
/// 
/// # Arguments
/// * `rpc_endpoint` - RPC endpoint URL
/// * `block_number` - Optional block number (None = recent block within last 30 days)
/// 
/// # Returns
/// PolkadotMainnetSnapshot with election_data, block_number, rpc_endpoint, fetch_timestamp
pub async fn fetch_polkadot_mainnet_snapshot(
    rpc_endpoint: &str,
    block_number: Option<u64>,
) -> Result<PolkadotMainnetSnapshot, String> {
    let loader = RpcLoader::new(rpc_endpoint)
        .map_err(|e| format!("Failed to create RPC loader: {}", e))?;
    
    // Determine block number to use
    let target_block = if let Some(block) = block_number {
        block
    } else {
        // Get latest block number and calculate recent block
        let latest_data = retry_with_backoff(
            || async { loader.load_latest().await },
            3,
            Duration::from_secs(1),
        )
        .await
        .map_err(|e| format!("Failed to fetch latest block: {}", e))?;
        
        let latest_block = latest_data.metadata
            .and_then(|m| m.block_number)
            .ok_or_else(|| "Latest block number not available".to_string())?;
        
        calculate_recent_block_number(latest_block)
    };
    
    // Fetch election data at the target block with retry logic
    let election_data = retry_with_backoff(
        || async { loader.load_at_block(target_block).await },
        3,
        Duration::from_secs(1),
    )
    .await
    .map_err(|e| format!("Failed to fetch election data at block {}: {}", target_block, e))?;
    
    Ok(PolkadotMainnetSnapshot {
        election_data,
        block_number: target_block,
        rpc_endpoint: rpc_endpoint.to_string(),
        fetch_timestamp: Utc::now(),
    })
}


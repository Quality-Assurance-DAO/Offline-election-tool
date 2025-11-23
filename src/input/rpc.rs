//! RPC input loader for fetching election data from Substrate RPC endpoints

use crate::error::ElectionError;
use crate::models::election_data::{ElectionData, ElectionMetadata};
use crate::models::nominator::Nominator;
use crate::models::validator::ValidatorCandidate;
use jsonrpsee::core::client::ClientT;
use jsonrpsee::http_client::{HttpClient, HttpClientBuilder};
use serde_json::Value;

/// RPC loader for fetching election data from Substrate nodes
pub struct RpcLoader {
    client: HttpClient,
    url: String,
}

impl RpcLoader {
    /// Create a new RPC loader
    pub fn new(url: impl Into<String>) -> Result<Self, ElectionError> {
        let url_str = url.into();
        let client = HttpClientBuilder::default()
            .build(&url_str)
            .map_err(|e| ElectionError::RpcError {
                message: format!("Failed to create RPC client: {}", e),
                url: url_str.clone(),
            })?;

        Ok(Self {
            client,
            url: url_str,
        })
    }

    /// Load election data at a specific block number
    pub async fn load_at_block(&self, block_number: u64) -> Result<ElectionData, ElectionError> {
        // Fetch block hash first
        let block_hash = self.get_block_hash(block_number).await?;

        // Fetch validator candidates
        let candidates = self.fetch_validators(&block_hash).await?;

        // Fetch nominators and their votes
        let nominators = self.fetch_nominators(&block_hash).await?;

        Ok(ElectionData {
            candidates,
            nominators,
            metadata: Some(ElectionMetadata {
                block_number: Some(block_number),
                chain: None,
            }),
        })
    }

    /// Get block hash for a given block number
    async fn get_block_hash(&self, block_number: u64) -> Result<String, ElectionError> {
        let response: Value = self
            .client
            .request(
                "chain_getBlockHash",
                (format!("0x{:x}", block_number),),
            )
            .await
            .map_err(|e| ElectionError::RpcError {
                message: format!("Failed to get block hash: {}", e),
                url: self.url.clone(),
            })?;

        let hash = response.as_str().ok_or_else(|| ElectionError::RpcError {
            message: "Invalid block hash response".to_string(),
            url: self.url.clone(),
        })?;

        Ok(hash.to_string())
    }

    /// Fetch validator candidates from chain
    async fn fetch_validators(&self, _block_hash: &str) -> Result<Vec<ValidatorCandidate>, ElectionError> {
        // Implementation: Query chain state for validators
        // This typically involves querying storage keys like:
        // - Session::Validators()
        // - Staking::Validators()
        // - Staking::ErasStakers()
        
        // For now, return an error indicating this needs implementation
        // The actual implementation would use state_call or storage queries
        Err(ElectionError::RpcError {
            message: "Validator fetching not yet fully implemented - requires chain-specific storage queries".to_string(),
            url: self.url.clone(),
        })
    }

    /// Fetch nominators and their votes from chain
    async fn fetch_nominators(&self, _block_hash: &str) -> Result<Vec<Nominator>, ElectionError> {
        // Implementation: Query chain state for nominators
        // This typically involves querying storage keys like:
        // - Staking::Nominators()
        // - Staking::Ledger()
        
        // For now, return an error indicating this needs implementation
        Err(ElectionError::RpcError {
            message: "Nominator fetching not yet fully implemented - requires chain-specific storage queries".to_string(),
            url: self.url.clone(),
        })
    }
}

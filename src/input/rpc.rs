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

    /// Load election data from the latest block
    pub async fn load_latest(&self) -> Result<ElectionData, ElectionError> {
        // Get latest block hash (None = latest)
        let block_hash = self.get_block_hash(0).await?;
        
        // Fetch validator candidates
        let candidates = self.fetch_validators(&block_hash).await?;

        // Fetch nominators and their votes
        let nominators = self.fetch_nominators(&block_hash).await?;

        // Get latest block number
        let latest_block = self.get_latest_block_number().await?;

        Ok(ElectionData {
            candidates,
            nominators,
            metadata: Some(ElectionMetadata {
                block_number: Some(latest_block),
                chain: None,
            }),
        })
    }

    /// Get the latest block number
    async fn get_latest_block_number(&self) -> Result<u64, ElectionError> {
        let response: Value = self
            .client
            .request("chain_getHeader", Vec::<String>::new())
            .await
            .map_err(|e| ElectionError::RpcError {
                message: format!("Failed to get latest header: {}", e),
                url: self.url.clone(),
            })?;

        let number = response
            .get("number")
            .and_then(|n| n.as_str())
            .ok_or_else(|| ElectionError::RpcError {
                message: "Invalid header response".to_string(),
                url: self.url.clone(),
            })?;

        // Parse hex number
        let number = number.trim_start_matches("0x");
        u64::from_str_radix(number, 16).map_err(|e| ElectionError::RpcError {
            message: format!("Failed to parse block number: {}", e),
            url: self.url.clone(),
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
    async fn fetch_validators(&self, block_hash: &str) -> Result<Vec<ValidatorCandidate>, ElectionError> {
        // Query Session::Validators() storage
        // Storage key: TwoX128("Session") + TwoX128("Validators")
        // For now, we'll use state_queryStorageAt with the storage key
        // Note: This is a simplified approach - production code would properly encode storage keys
        
        // Try to use state_call to get validators
        // First, try using staking_Validators query
        let storage_key = self.encode_storage_key("Staking", "Validators")?;
        
        let response: Value = self
            .client
            .request(
                "state_getStorage",
                (storage_key, block_hash),
            )
            .await
            .map_err(|e| ElectionError::RpcError {
                message: format!("Failed to query validators storage: {}", e),
                url: self.url.clone(),
            })?;

        // If we got data, decode it
        // Otherwise, try alternative approach using state_queryStorageAt
        if response.is_null() {
            // Try querying all validators using state_queryStorageAt
            return self.fetch_validators_alternative(block_hash).await;
        }

        // Decode the storage value
        // This is a simplified decoder - actual implementation would use proper SCALE decoding
        self.decode_validators_from_storage(&response, block_hash).await
    }

    /// Alternative method to fetch validators
    async fn fetch_validators_alternative(&self, block_hash: &str) -> Result<Vec<ValidatorCandidate>, ElectionError> {
        // Use state_queryStorageAt to get all entries
        // For now, return a helpful error message
        Err(ElectionError::RpcError {
            message: format!(
                "Could not fetch validators. This may require chain-specific storage key encoding.\n\
                Try using --input-file with JSON data instead, or check if the RPC endpoint supports \
                state_queryStorageAt for Staking::Validators storage key.\n\
                Block hash: {}", block_hash
            ),
            url: self.url.clone(),
        })
    }

    /// Decode validators from storage value
    async fn decode_validators_from_storage(&self, _storage_value: &Value, block_hash: &str) -> Result<Vec<ValidatorCandidate>, ElectionError> {
        // This is a placeholder - actual implementation would decode SCALE-encoded data
        // For now, return error suggesting to use JSON input
        Err(ElectionError::RpcError {
            message: format!(
                "Storage decoding not yet fully implemented. The RPC endpoint is reachable (block hash: {}),\n\
                but decoding Substrate storage requires proper SCALE codec implementation.\n\
                For now, please use --input-file with JSON data, or implement storage decoding.\n\
                Storage keys needed:\n\
                - Staking::Validators() - for validator list\n\
                - Staking::Nominators() - for nominator targets\n\
                - Staking::Ledger() - for nominator stakes",
                block_hash
            ),
            url: self.url.clone(),
        })
    }

    /// Encode a storage key (simplified - production would use proper TwoX128 hashing)
    fn encode_storage_key(&self, _pallet: &str, _storage_item: &str) -> Result<String, ElectionError> {
        // This is a placeholder - proper implementation would use sp_core::twox_128
        // For now, return a basic hex string
        // In production, this should use: twox_128(pallet) + twox_128(storage_item)
        // Using a known storage key prefix for Staking::Validators as example
        // Actual key would be: twox128("Staking") + twox128("Validators")
        Ok("0x5f3e4907f716ac89b6347d15ececedca8bde0a0ea8864605e3b68ed9cb2d2da313".to_string())
    }

    /// Fetch nominators and their votes from chain
    async fn fetch_nominators(&self, block_hash: &str) -> Result<Vec<Nominator>, ElectionError> {
        // Similar to validators, this needs proper storage key encoding and SCALE decoding
        // For now, return a helpful error
        Err(ElectionError::RpcError {
            message: format!(
                "Nominator fetching requires storage decoding implementation.\n\
                Block hash: {}\n\
                Storage keys needed:\n\
                - Staking::Nominators() - for nominator targets\n\
                - Staking::Ledger() - for nominator stakes\n\
                Please use --input-file with JSON data for now.",
                block_hash
            ),
            url: self.url.clone(),
        })
    }
}

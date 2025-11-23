//! RPC input loader for fetching election data from Substrate RPC endpoints

use crate::error::ElectionError;
use crate::models::election_data::{ElectionData, ElectionMetadata};
use crate::models::nominator::Nominator;
use crate::models::validator::ValidatorCandidate;
use jsonrpsee::core::client::ClientT;
use jsonrpsee::http_client::{HttpClient, HttpClientBuilder};
use serde_json::Value;
use std::hash::Hasher;
use twox_hash::XxHash64;

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
        // Try Session::Validators() first (active validator set)
        // Storage key: TwoX128("Session") + TwoX128("Validators")
        let session_key = self.encode_storage_key("Session", "Validators")?;
        
        let response: Value = self
            .client
            .request(
                "state_getStorage",
                (session_key.clone(), block_hash),
            )
            .await
            .map_err(|e| ElectionError::RpcError {
                message: format!("Failed to query Session::Validators storage: {}", e),
                url: self.url.clone(),
            })?;

        // If Session::Validators returns data, decode it
        if !response.is_null() {
            return self.decode_validators_from_storage(&response, block_hash).await;
        }

        // If Session::Validators is null, try Staking::Validators
        // Note: Staking::Validators might not exist in all chains, but Session::Validators should
        let staking_key = self.encode_storage_key("Staking", "Validators")?;
        
        let response: Value = self
            .client
            .request(
                "state_getStorage",
                (staking_key.clone(), block_hash),
            )
            .await
            .map_err(|e| ElectionError::RpcError {
                message: format!("Failed to query Staking::Validators storage: {}", e),
                url: self.url.clone(),
            })?;

        if !response.is_null() {
            return self.decode_validators_from_storage(&response, block_hash).await;
        }

        // If both are null, try using state_queryStorageAt with prefix
        self.fetch_validators_with_prefix(block_hash).await
    }

    /// Fetch validators using state_queryStorageAt with storage prefix
    async fn fetch_validators_with_prefix(&self, block_hash: &str) -> Result<Vec<ValidatorCandidate>, ElectionError> {
        // Try using state_queryStorageAt (if available) or state_queryStorage
        // Format: state_queryStorageAt([keys], at_block)
        let session_prefix = self.encode_storage_key("Session", "Validators")?;
        
        // Try state_queryStorageAt first
        let response: Result<Value, _> = self
            .client
            .request(
                "state_queryStorageAt",
                (vec![session_prefix.clone()], block_hash),
            )
            .await;

        if let Ok(value) = response {
            // Parse the response - state_queryStorageAt returns array of StorageChangeSet
            if let Some(entries) = value.as_array() {
                for entry in entries {
                    if let Some(changes) = entry.get("changes").and_then(|c| c.as_array()) {
                        for change in changes {
                            if let Some(change_arr) = change.as_array() {
                                if change_arr.len() == 2 {
                                    if let Some(value) = change_arr.get(1) {
                                        if let Some(value_str) = value.as_str() {
                                            if !value_str.is_empty() {
                                                return self.decode_validators_from_storage(value, block_hash).await;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // If state_queryStorageAt doesn't work, return error with helpful message
        Err(ElectionError::RpcError {
            message: format!(
                "Could not fetch validators from chain storage.\n\
                Tried Session::Validators() and Staking::Validators() storage keys.\n\
                Both returned null, which might indicate:\n\
                1. The storage keys are incorrect for this chain\n\
                2. The block hash is invalid\n\
                3. The RPC endpoint doesn't support these storage queries\n\
                Block hash: {}\n\
                Try using --input-file with JSON data instead.",
                block_hash
            ),
            url: self.url.clone(),
        })
    }

    /// Decode validators from storage value (SCALE-encoded Vec<AccountId>)
    async fn decode_validators_from_storage(&self, storage_value: &Value, block_hash: &str) -> Result<Vec<ValidatorCandidate>, ElectionError> {
        // Get the hex string from the storage value
        let hex_str = storage_value.as_str().ok_or_else(|| ElectionError::RpcError {
            message: "Storage value is not a string".to_string(),
            url: self.url.clone(),
        })?;

        // Remove 0x prefix if present
        let hex_str = hex_str.trim_start_matches("0x");
        
        // Decode hex to bytes
        let bytes = hex::decode(hex_str).map_err(|e| ElectionError::RpcError {
            message: format!("Failed to decode hex: {}", e),
            url: self.url.clone(),
        })?;

        // Decode SCALE-encoded Vec<AccountId>
        // AccountId in Polkadot is 32 bytes, and Vec<T> is encoded as length (compact) + items
        let mut validators = Vec::new();
        
        // Try to decode as Vec<[u8; 32]> (AccountId32)
        let mut offset = 0;
        
        // Decode compact length
        if bytes.is_empty() {
            return Ok(validators);
        }

        // Simple SCALE decoding: first byte(s) encode the length
        let (len, len_bytes) = self.decode_compact_u32(&bytes[offset..])?;
        offset += len_bytes;

        // Each AccountId is 32 bytes
        let account_id_size = 32;
        let expected_size = offset + (len as usize * account_id_size);
        
        if bytes.len() < expected_size {
            return Err(ElectionError::RpcError {
                message: format!(
                    "Invalid storage data length. Expected at least {} bytes, got {}",
                    expected_size,
                    bytes.len()
                ),
                url: self.url.clone(),
            });
        }

        // Decode each AccountId
        for i in 0..len {
            let start = offset + (i as usize * account_id_size);
            let end = start + account_id_size;
            let account_id_bytes = &bytes[start..end];
            
            // Convert to SS58 address (Polkadot uses SS58 encoding)
            // For now, we'll use hex representation, but ideally we'd convert to SS58
            let account_id_hex = format!("0x{}", hex::encode(account_id_bytes));
            
            // Create validator candidate with zero stake initially
            // Stake will need to be fetched separately from Staking::Ledger
            validators.push(ValidatorCandidate::new(account_id_hex, 0));
        }

        if validators.is_empty() {
        Err(ElectionError::RpcError {
            message: format!(
                    "No validators found in storage. Block hash: {}",
                block_hash
            ),
            url: self.url.clone(),
        })
        } else {
            Ok(validators)
        }
    }

    /// Decode compact u32 from SCALE encoding
    fn decode_compact_u32(&self, data: &[u8]) -> Result<(u32, usize), ElectionError> {
        if data.is_empty() {
            return Err(ElectionError::RpcError {
                message: "Empty data for compact decoding".to_string(),
                url: self.url.clone(),
            });
        }

        let first_byte = data[0];
        let mode = first_byte & 0b11;

        match mode {
            0b00 => {
                // Single byte mode: upper 6 bits are the value
                Ok(((first_byte >> 2) as u32, 1))
            }
            0b01 => {
                // Two byte mode: upper 6 bits + next byte
                if data.len() < 2 {
                    return Err(ElectionError::RpcError {
                        message: "Insufficient data for two-byte compact".to_string(),
                        url: self.url.clone(),
                    });
                }
                let value = ((first_byte >> 2) as u32) | ((data[1] as u32) << 6);
                Ok((value, 2))
            }
            0b10 => {
                // Four byte mode: upper 6 bits + next 3 bytes
                if data.len() < 4 {
                    return Err(ElectionError::RpcError {
                        message: "Insufficient data for four-byte compact".to_string(),
                        url: self.url.clone(),
                    });
                }
                let value = ((first_byte >> 2) as u32)
                    | ((data[1] as u32) << 6)
                    | ((data[2] as u32) << 14)
                    | ((data[3] as u32) << 22);
                Ok((value, 4))
            }
            _ => {
                // Multi-byte mode: lower 6 bits indicate number of following bytes
                let len = (first_byte >> 2) as usize;
                if data.len() < 1 + len {
                    return Err(ElectionError::RpcError {
                        message: format!("Insufficient data for {}-byte compact", len),
                        url: self.url.clone(),
                    });
                }
                // Read little-endian u32 from following bytes
                let mut value = 0u32;
                for i in 0..len.min(4) {
                    value |= (data[1 + i] as u32) << (i * 8);
                }
                Ok((value, 1 + len))
            }
        }
    }

    /// Encode a storage key using Substrate's TwoX128 hashing (twox_128)
    fn encode_storage_key(&self, pallet: &str, storage_item: &str) -> Result<String, ElectionError> {
        let mut key_bytes = Vec::with_capacity(32);
        key_bytes.extend_from_slice(&twox_128_hash(pallet.as_bytes()));
        key_bytes.extend_from_slice(&twox_128_hash(storage_item.as_bytes()));

        Ok(format!("0x{}", hex::encode(key_bytes)))
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

/// Re-implementation of Substrate's twox_128 hashing combinator.
/// Uses two XxHash64 computations with seeds 0 and 1 and concatenates the outputs.
fn twox_128_hash(data: &[u8]) -> [u8; 16] {
    let mut hasher0 = XxHash64::with_seed(0);
    hasher0.write(data);
    let mut hasher1 = XxHash64::with_seed(1);
    hasher1.write(data);

    let mut result = [0u8; 16];
    result[..8].copy_from_slice(&hasher0.finish().to_le_bytes());
    result[8..].copy_from_slice(&hasher1.finish().to_le_bytes());
    result
}

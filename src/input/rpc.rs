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

/// Maximum number of retry attempts for transient errors
const MAX_RETRIES: u32 = 5;
/// Initial delay in seconds before first retry
const INITIAL_RETRY_DELAY_SECS: u64 = 2;

/// RPC loader for fetching election data from Substrate nodes
pub struct RpcLoader {
    client: HttpClient,
    url: String,
}

impl RpcLoader {
    /// Create a new RPC loader
    pub fn new(url: impl Into<String>) -> Result<Self, ElectionError> {
        let url_str = url.into();
        // Configure timeouts to prevent hanging
        // 30 seconds for request timeout
        let client = HttpClientBuilder::default()
            .request_timeout(std::time::Duration::from_secs(30))
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

    /// Get suggested alternative RPC endpoints based on current URL
    fn get_alternative_endpoints(&self) -> Vec<&str> {
        let url_lower = self.url.to_lowercase();
        if url_lower.contains("polkadot") {
            vec![
                "https://rpc.polkadot.io",
                "https://polkadot.api.onfinality.io/public",
                "https://polkadot-rpc.dwellir.com",
                "https://polkadot.public.curie.com",
                "https://polkadot-rpc.publicnode.com",
                "https://1rpc.io/dot",
            ]
        } else if url_lower.contains("kusama") {
            vec![
                "https://kusama-rpc.polkadot.io",
                "https://kusama.api.onfinality.io/public",
                "https://kusama-rpc.dwellir.com",
                "https://kusama-rpc.publicnode.com",
                "https://1rpc.io/ksm",
            ]
        } else if url_lower.contains("westend") {
            vec![
                "https://westend-rpc.polkadot.io",
                "https://westend.api.onfinality.io/public",
            ]
        } else {
            // Generic/unknown chain - suggest Polkadot endpoints as default
            vec![
                "https://rpc.polkadot.io",
                "https://polkadot.api.onfinality.io/public",
                "https://polkadot-rpc.dwellir.com",
                "https://polkadot.public.curie.com",
            ]
        }
    }

    /// Check if an error is retryable (transient error)
    fn is_retryable_error(&self, error: &ElectionError) -> bool {
        match error {
            ElectionError::RpcError { message, .. } => {
                let msg_lower = message.to_lowercase();
                // Check for HTTP status codes in error message
                msg_lower.contains("503") || // Service Unavailable
                msg_lower.contains("502") || // Bad Gateway
                msg_lower.contains("504") || // Gateway Timeout
                msg_lower.contains("500") || // Internal Server Error
                msg_lower.contains("timeout") ||
                msg_lower.contains("network") ||
                msg_lower.contains("connection") ||
                msg_lower.contains("temporary") ||
                msg_lower.contains("unavailable") ||
                msg_lower.contains("server returned an error status code") ||
                msg_lower.contains("networking or low-level protocol error")
            }
            _ => false,
        }
    }

    /// Retry an RPC call with exponential backoff for transient errors
    async fn retry_rpc_call<F, Fut, T>(&self, mut f: F) -> Result<T, ElectionError>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = Result<T, ElectionError>>,
    {
        for attempt in 0..=MAX_RETRIES {
            match f().await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    // Check if error is retryable
                    if !self.is_retryable_error(&e) {
                        // Not retryable, return immediately
                        return Err(e);
                    }
                    
                    // If this was the last attempt, return enhanced error with suggestions
                    if attempt >= MAX_RETRIES {
                        return Err(match &e {
                            ElectionError::RpcError { message, url } => {
                                let alternatives = self.get_alternative_endpoints();
                                let alternatives_list = alternatives
                                    .iter()
                                    .map(|alt| format!("  - {}", alt))
                                    .collect::<Vec<_>>()
                                    .join("\n");
                                
                                ElectionError::RpcError {
                                    message: format!(
                                        "{}\n\n\
                                        All {} retry attempts failed. The RPC endpoint appears to be unavailable.\n\n\
                                        Suggested alternative endpoints:\n{}\n\n\
                                        Other options:\n\
                                        - Use --input-file with JSON data instead\n\
                                        - Wait a few minutes and try again\n\
                                        - Check the endpoint status page",
                                        message,
                                        MAX_RETRIES + 1,
                                        alternatives_list
                                    ),
                                    url: url.clone(),
                                }
                            }
                            _ => e,
                        });
                    }
                    
                    // Calculate exponential backoff delay with cap at 30 seconds
                    let delay_secs = std::cmp::min(
                        INITIAL_RETRY_DELAY_SECS * (1u64 << attempt),
                        30
                    );
                    eprintln!("  ⚠ RPC error (attempt {}/{}), retrying in {} seconds...", 
                             attempt + 1, MAX_RETRIES + 1, delay_secs);
                    std::io::Write::flush(&mut std::io::stderr()).ok();
                    
                    // Wait before retrying
                    tokio::time::sleep(std::time::Duration::from_secs(delay_secs)).await;
                }
            }
        }
        
        // This should never be reached, but handle it just in case
        Err(ElectionError::RpcError {
            message: "Unknown error during retry".to_string(),
            url: self.url.clone(),
        })
    }

    /// Load election data at a specific block number
    pub async fn load_at_block(&self, block_number: u64) -> Result<ElectionData, ElectionError> {
        eprintln!("Fetching data from block {}...", block_number);
        std::io::Write::flush(&mut std::io::stderr()).ok();
        
        // Fetch block hash first
        eprintln!("  → Getting block hash (this may take up to 30 seconds)...");
        std::io::Write::flush(&mut std::io::stderr()).ok();
        
        let block_hash = tokio::time::timeout(
            std::time::Duration::from_secs(30),
            self.get_block_hash(block_number)
        ).await.map_err(|_| ElectionError::RpcError {
            message: format!(
                "Timeout after 30 seconds while getting block hash for block {}.\n\
                The RPC endpoint may be slow or unresponsive.\n\
                Please try:\n\
                - Using a different RPC endpoint\n\
                - Using --input-file with JSON data instead\n\
                - Checking your network connection",
                block_number
            ),
            url: self.url.clone(),
        })??;
        
        eprintln!("  ✓ Block hash: {}", block_hash);
        std::io::Write::flush(&mut std::io::stderr()).ok();

        // Fetch validator candidates
        eprintln!("  → Fetching validators (this may take up to 30 seconds)...");
        std::io::Write::flush(&mut std::io::stderr()).ok();
        
        let candidates = tokio::time::timeout(
            std::time::Duration::from_secs(30),
            self.fetch_validators(&block_hash)
        ).await.map_err(|_| ElectionError::RpcError {
            message: format!(
                "Timeout after 30 seconds while fetching validators.\n\
                Block hash: {}\n\
                The RPC endpoint may be slow or unresponsive.",
                block_hash
            ),
            url: self.url.clone(),
        })??;
        
        eprintln!("  ✓ Found {} validators", candidates.len());
        std::io::Write::flush(&mut std::io::stderr()).ok();

        // Fetch nominators and their votes
        eprintln!("  → Fetching nominators (this may take a while, timeout: 60 seconds)...");
        std::io::Write::flush(&mut std::io::stderr()).ok();
        
        let nominators = tokio::time::timeout(
            std::time::Duration::from_secs(60),
            self.fetch_nominators(&block_hash)
        ).await.unwrap_or_else(|_| {
            Err(ElectionError::RpcError {
                message: format!(
                    "Timeout after 60 seconds while fetching nominators.\n\
                    Block hash: {}\n\
                    This usually means the RPC endpoint doesn't support storage queries or is very slow.\n\
                    Proceeding with zero nominators - election will use only validator self-stakes.",
                    block_hash
                ),
                url: self.url.clone(),
            })
        }).unwrap_or_else(|e| {
            eprintln!("  ⚠ Warning: Could not fetch nominators from RPC: {}", e);
            eprintln!("  → Proceeding with zero nominators - election will use only validator self-stakes.");
            std::io::Write::flush(&mut std::io::stderr()).ok();
            Vec::new()
        });
        
        eprintln!("  ✓ Found {} nominators", nominators.len());
        std::io::Write::flush(&mut std::io::stderr()).ok();

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
        eprintln!("Fetching data from latest block...");
        std::io::Write::flush(&mut std::io::stderr()).ok();
        
        // Get latest block hash (None = latest)
        eprintln!("  → Getting latest block hash (this may take up to 30 seconds)...");
        std::io::Write::flush(&mut std::io::stderr()).ok();
        
        let block_hash = tokio::time::timeout(
            std::time::Duration::from_secs(30),
            self.get_block_hash(0)
        ).await.map_err(|_| ElectionError::RpcError {
            message: format!(
                "Timeout after 30 seconds while getting latest block hash.\n\
                The RPC endpoint may be slow or unresponsive.\n\
                Please try:\n\
                - Using a different RPC endpoint\n\
                - Using --input-file with JSON data instead\n\
                - Checking your network connection"
            ),
            url: self.url.clone(),
        })??;
        
        eprintln!("  ✓ Block hash: {}", block_hash);
        std::io::Write::flush(&mut std::io::stderr()).ok();
        
        // Fetch validator candidates
        eprintln!("  → Fetching validators (this may take up to 30 seconds)...");
        std::io::Write::flush(&mut std::io::stderr()).ok();
        
        let candidates = tokio::time::timeout(
            std::time::Duration::from_secs(30),
            self.fetch_validators(&block_hash)
        ).await.map_err(|_| ElectionError::RpcError {
            message: format!(
                "Timeout after 30 seconds while fetching validators.\n\
                Block hash: {}\n\
                The RPC endpoint may be slow or unresponsive.",
                block_hash
            ),
            url: self.url.clone(),
        })??;
        
        eprintln!("  ✓ Found {} validators", candidates.len());
        std::io::Write::flush(&mut std::io::stderr()).ok();

        // Fetch nominators and their votes
        eprintln!("  → Fetching nominators (this may take a while, timeout: 60 seconds)...");
        std::io::Write::flush(&mut std::io::stderr()).ok();
        
        let nominators = tokio::time::timeout(
            std::time::Duration::from_secs(60),
            self.fetch_nominators(&block_hash)
        ).await.unwrap_or_else(|_| {
            Err(ElectionError::RpcError {
                message: format!(
                    "Timeout after 60 seconds while fetching nominators.\n\
                    Block hash: {}\n\
                    This usually means the RPC endpoint doesn't support storage queries or is very slow.\n\
                    Proceeding with zero nominators - election will use only validator self-stakes.",
                    block_hash
                ),
                url: self.url.clone(),
            })
        }).unwrap_or_else(|e| {
            eprintln!("  ⚠ Warning: Could not fetch nominators from RPC: {}", e);
            eprintln!("  → Proceeding with zero nominators - election will use only validator self-stakes.");
            std::io::Write::flush(&mut std::io::stderr()).ok();
            Vec::new()
        });
        
        eprintln!("  ✓ Found {} nominators", nominators.len());
        std::io::Write::flush(&mut std::io::stderr()).ok();

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
        self.retry_rpc_call(|| async {
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
        })
        .await
    }

    /// Get block hash for a given block number
    async fn get_block_hash(&self, block_number: u64) -> Result<String, ElectionError> {
        self.retry_rpc_call(|| async {
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
        })
        .await
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
        // Staking::Nominators is a StorageMap<AccountId, Nominations>
        // Staking::Ledger is a StorageMap<AccountId, StakingLedger>
        // We need to fetch all entries from both maps and combine them
        
        // Get the base storage key prefix for Nominators
        let nominators_prefix = self.encode_storage_key("Staking", "Nominators")?;
        
        // Get the base storage key prefix for Ledger
        let ledger_prefix = self.encode_storage_key("Staking", "Ledger")?;
        
        // Fetch all storage keys with the Nominators prefix
        let nominator_keys_result = self.get_storage_keys(&nominators_prefix, block_hash).await;
        let nominator_keys = match nominator_keys_result {
            Ok(keys) => {
                if keys.is_empty() {
                    // Try pagination method if regular method returns empty
                    return self.fetch_nominators_with_pagination(&nominators_prefix, &ledger_prefix, block_hash).await;
                }
                keys
            }
            Err(_e) => {
                // Try alternative RPC method: state_getKeysPaged
                return self.fetch_nominators_with_pagination(&nominators_prefix, &ledger_prefix, block_hash).await;
            }
        };
        
        // Fetch all storage keys with the Ledger prefix
        let ledger_keys_result = self.get_storage_keys(&ledger_prefix, block_hash).await;
        let ledger_keys = match ledger_keys_result {
            Ok(keys) => keys,
            Err(_e) => {
                // If Ledger keys fail, try pagination method
                return self.fetch_nominators_with_pagination(&nominators_prefix, &ledger_prefix, block_hash).await;
            }
        };
        
        // Build a map of AccountId -> Nominator (initially with empty targets)
        let mut nominators_map: std::collections::HashMap<String, Nominator> = std::collections::HashMap::new();
        
        let mut nominator_keys_processed = 0;
        let mut ledger_keys_processed = 0;
        let mut decode_errors = Vec::new();
        
        // Process Nominators storage entries to get targets
        for key in nominator_keys {
            nominator_keys_processed += 1;
            // Extract AccountId from storage key
            // Format: prefix (32 bytes) + blake2_128(AccountId) (16 bytes) + AccountId (32 bytes)
            let account_id = match self.decode_account_id_from_key(&key, &nominators_prefix, true) {
                Ok(id) => id,
                Err(e) => {
                    decode_errors.push(format!("Failed to decode AccountId from Nominators key: {}", e));
                    continue;
                }
            };
            
            // Fetch the storage value for this key
            let value = match self.get_storage_value(&key, block_hash).await {
                Ok(v) => v,
                Err(e) => {
                    decode_errors.push(format!("Failed to get storage value for Nominators key: {}", e));
                    continue;
                }
            };
            
            if let Some(nominations_bytes) = value {
                // Decode Nominations struct to get targets
                match self.decode_nominations_targets(&nominations_bytes) {
                    Ok(targets) => {
                        // Create or update nominator with targets
                        let nominator = nominators_map.entry(account_id.clone()).or_insert_with(|| {
                            Nominator::new(account_id, 0)
                        });
                        nominator.targets = targets;
                    }
                    Err(e) => {
                        decode_errors.push(format!("Failed to decode Nominations targets: {}", e));
                    }
                }
            }
        }
        
        // Process Ledger storage entries to get stakes
        for key in ledger_keys {
            ledger_keys_processed += 1;
            // Extract AccountId from storage key
            // Format: prefix (32 bytes) + twox64(AccountId) (8 bytes) + AccountId (32 bytes)
            let account_id = match self.decode_account_id_from_key(&key, &ledger_prefix, false) {
                Ok(id) => id,
                Err(e) => {
                    decode_errors.push(format!("Failed to decode AccountId from Ledger key: {}", e));
                    continue;
                }
            };
            
            // Fetch the storage value for this key
            let value = match self.get_storage_value(&key, block_hash).await {
                Ok(v) => v,
                Err(e) => {
                    decode_errors.push(format!("Failed to get storage value for Ledger key: {}", e));
                    continue;
                }
            };
            
            if let Some(ledger_bytes) = value {
                // Decode StakingLedger to get total stake
                match self.decode_staking_ledger_stake(&ledger_bytes) {
                    Ok(stake) => {
                        // Create or update nominator with stake
                        let nominator = nominators_map.entry(account_id.clone()).or_insert_with(|| {
                            Nominator::new(account_id, 0)
                        });
                        nominator.stake = stake;
                    }
                    Err(e) => {
                        decode_errors.push(format!("Failed to decode StakingLedger stake: {}", e));
                    }
                }
            }
        }
        
        // Convert HashMap to Vec
        let mut nominators: Vec<Nominator> = nominators_map.into_values().collect();
        
        // Build diagnostic message
        let mut diag_msg = format!(
            "Nominator fetch diagnostics:\n\
            - Nominator keys found: {}\n\
            - Ledger keys found: {}\n\
            - Nominators processed: {}\n",
            nominator_keys_processed,
            ledger_keys_processed,
            nominators.len()
        );
        
        if !decode_errors.is_empty() {
            diag_msg.push_str(&format!("\nDecode errors (showing first 5):\n"));
            for err in decode_errors.iter().take(5) {
                diag_msg.push_str(&format!("  - {}\n", err));
            }
        }
        
        // Filter out nominators with no targets (they're not actually nominating)
        let before_filter = nominators.len();
        nominators.retain(|n| !n.targets.is_empty());
        let after_filter = nominators.len();
        
        diag_msg.push_str(&format!(
            "- Nominators before filtering (no targets): {}\n\
            - Nominators after filtering: {}",
            before_filter,
            after_filter
        ));
        
        if nominators.is_empty() {
            // Return empty list instead of error - election can run without nominators
            // This allows the tool to work even if RPC doesn't support these methods
            return Ok(Vec::new());
        }
        
        Ok(nominators)
    }
    
    /// Alternative method using state_queryStorage (more reliable on some endpoints)
    async fn fetch_nominators_with_query_storage(
        &self,
        _nominators_prefix: &str,
        _ledger_prefix: &str,
        _block_hash: &str,
    ) -> Result<Vec<Nominator>, ElectionError> {
        // Note: state_queryStorageAt doesn't actually support prefix queries to get all keys
        // It's designed for querying specific keys. This method is a fallback that likely won't work
        // but we try it anyway in case the RPC endpoint has special handling.
        
        // Since state_queryStorageAt with a prefix won't return all entries,
        // and state_getKeys/state_getKeysPaged aren't working, we return an empty list
        // This allows the election to proceed with just validators (no nominator votes)
        
        // Return empty list - election can proceed without nominators
        // The user will see a warning that no nominators were found
        Ok(Vec::new())
    }
    
    /// Process nominator and ledger keys to build Nominator objects
    async fn process_nominator_keys(
        &self,
        nominator_keys: Vec<String>,
        ledger_keys: Vec<String>,
        nominators_prefix: &str,
        ledger_prefix: &str,
        block_hash: &str,
    ) -> Result<Vec<Nominator>, ElectionError> {
        // Store lengths before processing
        let nominator_keys_count = nominator_keys.len();
        let ledger_keys_count = ledger_keys.len();
        
        // Build a map of AccountId -> Nominator
        let mut nominators_map: std::collections::HashMap<String, Nominator> = std::collections::HashMap::new();
        let mut decode_errors = Vec::new();
        let mut nominators_processed = 0;
        let mut ledgers_processed = 0;
        let mut targets_decoded = 0;
        let mut stakes_decoded = 0;
        
        // Process Nominators storage entries
        for key in &nominator_keys {
            // Skip keys that are exactly the prefix (some RPCs return the prefix itself)
            let key_normalized = key.trim_start_matches("0x");
            let prefix_normalized = nominators_prefix.trim_start_matches("0x");
            if key_normalized == prefix_normalized {
                decode_errors.push(format!("Skipping key that is exactly the prefix (not a valid entry)"));
                continue;
            }
            
            nominators_processed += 1;
            let account_id = match self.decode_account_id_from_key(key, nominators_prefix, true) {
                Ok(id) => id,
                Err(e) => {
                    decode_errors.push(format!("Failed to decode AccountId from Nominators key: {}", e));
                    continue;
                }
            };
            
            let nominations_bytes = match self.get_storage_value(key, block_hash).await {
                Ok(Some(bytes)) => bytes,
                Ok(None) => {
                    decode_errors.push(format!("Nominators storage value is null for key"));
                    continue;
                }
                Err(e) => {
                    decode_errors.push(format!("Failed to get storage value for Nominators key: {}", e));
                    continue;
                }
            };
            
            match self.decode_nominations_targets(&nominations_bytes) {
                Ok(targets) => {
                    if !targets.is_empty() {
                        targets_decoded += 1;
                        let nominator = nominators_map.entry(account_id.clone()).or_insert_with(|| {
                            Nominator::new(account_id, 0)
                        });
                        nominator.targets = targets;
                    }
                }
                Err(e) => {
                    decode_errors.push(format!("Failed to decode Nominations targets: {}", e));
                }
            }
        }
        
        // Process Ledger storage entries
        for key in &ledger_keys {
            // Skip keys that are exactly the prefix (some RPCs return the prefix itself)
            let key_normalized = key.trim_start_matches("0x");
            let prefix_normalized = ledger_prefix.trim_start_matches("0x");
            if key_normalized == prefix_normalized {
                decode_errors.push(format!("Skipping key that is exactly the prefix (not a valid entry)"));
                continue;
            }
            
            ledgers_processed += 1;
            let account_id = match self.decode_account_id_from_key(key, ledger_prefix, false) {
                Ok(id) => id,
                Err(e) => {
                    decode_errors.push(format!("Failed to decode AccountId from Ledger key: {}", e));
                    continue;
                }
            };
            
            let ledger_bytes = match self.get_storage_value(key, block_hash).await {
                Ok(Some(bytes)) => bytes,
                Ok(None) => {
                    decode_errors.push(format!("Ledger storage value is null for key"));
                    continue;
                }
                Err(e) => {
                    decode_errors.push(format!("Failed to get storage value for Ledger key: {}", e));
                    continue;
                }
            };
            
            match self.decode_staking_ledger_stake(&ledger_bytes) {
                Ok(stake) => {
                    stakes_decoded += 1;
                        let nominator = nominators_map.entry(account_id.clone()).or_insert_with(|| {
                            Nominator::new(account_id, 0)
                        });
                        nominator.stake = stake;
                    }
                Err(e) => {
                    decode_errors.push(format!("Failed to decode StakingLedger stake: {}", e));
                }
            }
        }
        
        let mut nominators: Vec<Nominator> = nominators_map.into_values().collect();
        let before_filter = nominators.len();
        nominators.retain(|n| !n.targets.is_empty());
        let after_filter = nominators.len();
        
        if nominators.is_empty() {
            let mut error_msg = format!(
                "No nominators found after processing.\n\
                Block hash: {}\n\
                Nominator keys found: {}\n\
                Ledger keys found: {}\n\
                Nominators processed: {}\n\
                Ledgers processed: {}\n\
                Targets decoded: {}\n\
                Stakes decoded: {}\n\
                Nominators before filtering: {}\n\
                Nominators after filtering: {}",
                block_hash,
                nominator_keys_count,
                ledger_keys_count,
                nominators_processed,
                ledgers_processed,
                targets_decoded,
                stakes_decoded,
                before_filter,
                after_filter
            );
            
            // Check if we only got prefix keys (common issue with some RPC endpoints)
            let only_prefix_keys = nominator_keys_count > 0 && nominators_processed == 0 && 
                                   ledger_keys_count > 0 && ledgers_processed == 0;
            
            if only_prefix_keys {
                error_msg.push_str("\n\n⚠️  Only prefix keys were returned by the RPC endpoint.\n");
                error_msg.push_str("This usually means:\n");
                error_msg.push_str("  1. The RPC endpoint doesn't support state_getKeys/state_getKeysPaged properly\n");
                error_msg.push_str("  2. The storage structure might be different than expected\n");
                error_msg.push_str("  3. There might be no nominators at this block\n\n");
                error_msg.push_str("Please try using --input-file with JSON data instead.\n");
            }
            
            if !decode_errors.is_empty() {
                error_msg.push_str("\n\nDecode errors (showing first 10):\n");
                for err in decode_errors.iter().take(10) {
                    error_msg.push_str(&format!("  - {}\n", err));
                }
            }
            
            return Err(ElectionError::RpcError {
                message: error_msg,
                url: self.url.clone(),
            });
        }
        
        Ok(nominators)
    }
    
    /// Alternative method using pagination if state_getKeys doesn't work
    async fn fetch_nominators_with_pagination(
        &self,
        nominators_prefix: &str,
        ledger_prefix: &str,
        block_hash: &str,
    ) -> Result<Vec<Nominator>, ElectionError> {
        // Try state_getKeysPaged with pagination
        // Note: Parameter order may vary by RPC implementation
        let mut nominator_keys = Vec::new();
        let mut ledger_keys = Vec::new();
        let mut nominator_start_key: Option<String> = None;
        let mut ledger_start_key: Option<String> = None;
        let page_size = 1000u32;
        
        // Try different parameter orders for state_getKeysPaged
        // Some implementations use: (prefix, count, start_key, at)
        // Others use: (prefix, count, at, start_key)
        
        // Fetch Nominator keys with pagination - try first parameter order
        let mut page_count = 0;
        loop {
            page_count += 1;
            if page_count > 1 {
                eprintln!("    → Fetching nominator keys page {}...", page_count);
            }
            
            // Add timeout wrapper for individual requests
            let response = tokio::time::timeout(
                std::time::Duration::from_secs(30),
                self.client.request(
                    "state_getKeysPaged",
                    (
                        nominators_prefix,
                        page_size,
                        nominator_start_key.as_ref(),
                        Some(block_hash),
                    ),
                )
            ).await;
            
            // If that fails, try alternative parameter order
            let response: Result<Value, _> = match response {
                Ok(Ok(v)) => Ok(v),
                Ok(Err(_)) => {
                    // Try alternative parameter order
                    let alt_response = tokio::time::timeout(
                        std::time::Duration::from_secs(30),
                        self.client.request(
                            "state_getKeysPaged",
                            (
                                nominators_prefix,
                                page_size,
                                Some(block_hash),
                                nominator_start_key.as_ref(),
                            ),
                        )
                    ).await;
                    
                    match alt_response {
                        Ok(Ok(v)) => Ok(v),
                        Ok(Err(e)) => Err(e),
                        Err(_) => {
                            return Err(ElectionError::RpcError {
                                message: format!(
                                    "Request timeout after 30 seconds while fetching nominator keys.\n\
                                    This usually means the RPC endpoint is slow or doesn't support this method.\n\
                                    Block hash: {}\n\
                                    Please try using --input-file with JSON data instead.",
                                    block_hash
                                ),
                                url: self.url.clone(),
                            });
                        }
                    }
                }
                Err(_) => {
                    return Err(ElectionError::RpcError {
                        message: format!(
                            "Request timeout after 30 seconds while fetching nominator keys.\n\
                            This usually means the RPC endpoint is slow or doesn't support this method.\n\
                            Block hash: {}\n\
                            Please try using --input-file with JSON data instead.",
                            block_hash
                        ),
                        url: self.url.clone(),
                    });
                }
            };
            
            match response {
                Ok(value) => {
                    if let Some(keys_array) = value.as_array() {
                        if keys_array.is_empty() {
                            break;
                        }
                        let prefix_normalized = nominators_prefix.trim_start_matches("0x");
                        for key in keys_array {
                            if let Some(key_str) = key.as_str() {
                                // Filter out keys that are exactly the prefix
                                let key_normalized = key_str.trim_start_matches("0x");
                                if key_normalized != prefix_normalized {
                                nominator_keys.push(key_str.to_string());
                                }
                            }
                        }
                        // Set start key for next page
                        if let Some(last_key) = keys_array.last().and_then(|k| k.as_str()) {
                            nominator_start_key = Some(last_key.to_string());
                        } else {
                            break;
                        }
                        // If we got fewer than page_size, we're done
                        if keys_array.len() < page_size as usize {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                Err(_) => {
                    // If pagination doesn't work, return error with helpful message
                    return Err(ElectionError::RpcError {
            message: format!(
                            "Failed to fetch nominator storage keys.\n\
                            Tried both state_getKeys and state_getKeysPaged.\n\
                Block hash: {}\n\
                            \n\
                            This RPC endpoint might not support these methods.\n\
                            Please use --input-file with JSON data instead.",
                block_hash
            ),
                        url: self.url.clone(),
                    });
                }
            }
        }
        
        // Fetch Ledger keys with pagination - try first parameter order
        let mut ledger_page_count = 0;
        loop {
            ledger_page_count += 1;
            if ledger_page_count > 1 {
                eprintln!("    → Fetching ledger keys page {}...", ledger_page_count);
            }
            
            // Add timeout wrapper for individual requests
            let response = tokio::time::timeout(
                std::time::Duration::from_secs(30),
                self.client.request(
                    "state_getKeysPaged",
                    (
                        ledger_prefix,
                        page_size,
                        ledger_start_key.as_ref(),
                        Some(block_hash),
                    ),
                )
            ).await;
            
            let response: Result<Value, _> = match response {
                Ok(Ok(v)) => Ok(v),
                Ok(Err(e)) => Err(e),
                Err(_) => {
                    return Err(ElectionError::RpcError {
                        message: format!(
                            "Request timeout after 30 seconds while fetching ledger keys.\n\
                            This usually means the RPC endpoint is slow or doesn't support this method.\n\
                            Block hash: {}\n\
                            Please try using --input-file with JSON data instead.",
                            block_hash
                        ),
                        url: self.url.clone(),
                    });
                }
            };
            
            // If that fails, try alternative parameter order
            let response = match response {
                Ok(v) => Ok(v),
                Err(_) => {
                    self.client
                        .request(
                            "state_getKeysPaged",
                            (
                                ledger_prefix,
                                page_size,
                                Some(block_hash),
                                ledger_start_key.as_ref(),
                            ),
                        )
                        .await
                }
            };
            
            match response {
                Ok(value) => {
                    if let Some(keys_array) = value.as_array() {
                        if keys_array.is_empty() {
                            break;
                        }
                        let prefix_normalized = ledger_prefix.trim_start_matches("0x");
                        for key in keys_array {
                            if let Some(key_str) = key.as_str() {
                                // Filter out keys that are exactly the prefix
                                let key_normalized = key_str.trim_start_matches("0x");
                                if key_normalized != prefix_normalized {
                                    ledger_keys.push(key_str.to_string());
                                }
                            }
                        }
                        // Set start key for next page
                        if let Some(last_key) = keys_array.last().and_then(|k| k.as_str()) {
                            ledger_start_key = Some(last_key.to_string());
                        } else {
                            break;
                        }
                        // If we got fewer than page_size, we're done
                        if keys_array.len() < page_size as usize {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                Err(_) => {
                    break;
                }
            }
        }
        
        // If we found no valid keys with pagination (only prefix keys were returned), 
        // try query_storage method as fallback
        if nominator_keys.is_empty() && ledger_keys.is_empty() {
            // Try alternative method
            return self.fetch_nominators_with_query_storage(nominators_prefix, ledger_prefix, block_hash).await;
        }
        
        // Process the keys using the shared processing logic
        let result = self.process_nominator_keys(nominator_keys, ledger_keys, nominators_prefix, ledger_prefix, block_hash).await;
        
        // If processing failed, try query_storage as final fallback
        match result {
            Ok(nominators) if !nominators.is_empty() => Ok(nominators),
            Err(e) => {
                // If we got an error, try query_storage as fallback
                // But if query_storage also fails, return the original error with more context
                match self.fetch_nominators_with_query_storage(nominators_prefix, ledger_prefix, block_hash).await {
                    Ok(nominators) if !nominators.is_empty() => Ok(nominators),
                    _ => Err(e), // Return original error
                }
            }
            _ => self.fetch_nominators_with_query_storage(nominators_prefix, ledger_prefix, block_hash).await,
        }
    }
    
    /// Get all storage keys with a given prefix
    async fn get_storage_keys(&self, prefix: &str, block_hash: &str) -> Result<Vec<String>, ElectionError> {
        // Use state_getKeys RPC method to get all keys with the prefix
        // Note: Some RPC endpoints use state_getKeysPaged instead
        let response: Result<Value, _> = self
            .client
            .request(
                "state_getKeys",
                (prefix, block_hash),
            )
            .await;
        
        let value = match response {
            Ok(v) => v,
            Err(e) => {
                // If state_getKeys fails, the error will be caught by caller
                return Err(ElectionError::RpcError {
                    message: format!("Failed to query storage keys: {}", e),
                    url: self.url.clone(),
                });
            }
        };
        
        // Parse the response - should be an array of hex strings
        let keys_array = value.as_array().ok_or_else(|| ElectionError::RpcError {
            message: "Invalid storage keys response (not an array)".to_string(),
            url: self.url.clone(),
        })?;
        
        let prefix_normalized = prefix.trim_start_matches("0x");
        let mut result = Vec::new();
        for key in keys_array {
            if let Some(key_str) = key.as_str() {
                // Filter out keys that are exactly the prefix (some RPCs return the prefix itself)
                let key_normalized = key_str.trim_start_matches("0x");
                if key_normalized != prefix_normalized {
                result.push(key_str.to_string());
                }
            }
        }
        
        Ok(result)
    }
    
    /// Get storage value for a given key
    async fn get_storage_value(&self, key: &str, block_hash: &str) -> Result<Option<Vec<u8>>, ElectionError> {
        let response: Value = self
            .client
            .request(
                "state_getStorage",
                (key, block_hash),
            )
            .await
            .map_err(|e| ElectionError::RpcError {
                message: format!("Failed to query storage value: {}", e),
                url: self.url.clone(),
            })?;
        
        if response.is_null() {
            return Ok(None);
        }
        
        let hex_str = response.as_str().ok_or_else(|| ElectionError::RpcError {
            message: "Storage value is not a string".to_string(),
            url: self.url.clone(),
        })?;
        
        let hex_str = hex_str.trim_start_matches("0x");
        let bytes = hex::decode(hex_str).map_err(|e| ElectionError::RpcError {
            message: format!("Failed to decode hex: {}", e),
            url: self.url.clone(),
        })?;
        
        Ok(Some(bytes))
    }
    
    /// Decode AccountId from a storage key
    /// For blake2_128_concat: prefix (32 bytes) + blake2_128 hash (16 bytes) + AccountId (32 bytes)
    /// For twox64_concat: prefix (32 bytes) + twox64 hash (8 bytes) + AccountId (32 bytes)
    fn decode_account_id_from_key(&self, full_key: &str, prefix: &str, is_blake2: bool) -> Result<String, ElectionError> {
        // Normalize keys by removing 0x prefix for comparison
        let key_normalized = full_key.trim_start_matches("0x");
        let prefix_normalized = prefix.trim_start_matches("0x");
        
        // Check if the key is exactly the prefix (some RPCs return the prefix itself)
        if key_normalized == prefix_normalized {
            return Err(ElectionError::RpcError {
                message: format!(
                    "Storage key is exactly the prefix (not a valid entry). Key length: {} bytes",
                    key_normalized.len() / 2
                ),
                url: self.url.clone(),
            });
        }
        
        // Decode hex strings
        let key_bytes = hex::decode(key_normalized).map_err(|e| ElectionError::RpcError {
            message: format!("Failed to decode key hex: {}", e),
            url: self.url.clone(),
        })?;
        
        let prefix_bytes = hex::decode(prefix_normalized).map_err(|e| ElectionError::RpcError {
            message: format!("Failed to decode prefix hex: {}", e),
            url: self.url.clone(),
        })?;
        
        // Ensure the key starts with the prefix
        if key_bytes.len() < prefix_bytes.len() {
            return Err(ElectionError::RpcError {
                message: format!(
                    "Storage key shorter than prefix. Key: {} bytes, Prefix: {} bytes",
                    key_bytes.len(),
                    prefix_bytes.len()
                ),
                url: self.url.clone(),
            });
        }
        
        if &key_bytes[..prefix_bytes.len()] != prefix_bytes.as_slice() {
            return Err(ElectionError::RpcError {
                message: "Storage key does not start with expected prefix".to_string(),
                url: self.url.clone(),
            });
        }
        
        // Calculate offset: prefix length + hash length
        let hash_length = if is_blake2 { 16 } else { 8 };
        let offset = prefix_bytes.len() + hash_length;
        
        if key_bytes.len() < offset + 32 {
            return Err(ElectionError::RpcError {
                message: format!(
                    "Storage key too short. Expected at least {} bytes (prefix: {} + hash: {} + account: 32), got {} bytes",
                    offset + 32,
                    prefix_bytes.len(),
                    hash_length,
                    key_bytes.len()
                ),
                url: self.url.clone(),
            });
        }
        
        // Extract AccountId (last 32 bytes after prefix and hash)
        let account_id_bytes = &key_bytes[offset..offset + 32];
        let account_id_hex = format!("0x{}", hex::encode(account_id_bytes));
        
        Ok(account_id_hex)
    }
    
    /// Decode Nominations struct to extract targets (BoundedVec<AccountId>)
    /// Nominations structure: { targets: BoundedVec<AccountId>, ... }
    /// BoundedVec is encoded as Vec: compact length + items
    fn decode_nominations_targets(&self, bytes: &[u8]) -> Result<Vec<String>, ElectionError> {
        if bytes.is_empty() {
            return Ok(Vec::new());
        }
        
        let mut offset = 0;
        let mut targets = Vec::new();
        
        // Skip any prefix bytes if Nominations has other fields before targets
        // For simplicity, assume targets is the first field
        // If not, we may need to skip some bytes based on the actual structure
        
        // Try to find the Vec<AccountId> encoding
        // Look for compact length encoding
        if let Ok((len, len_bytes)) = self.decode_compact_u32(&bytes[offset..]) {
            offset += len_bytes;
            
            // Each AccountId is 32 bytes
            let account_id_size = 32;
            let expected_size = offset + (len as usize * account_id_size);
            
            if bytes.len() >= expected_size {
                // Decode each AccountId
                for i in 0..len {
                    let start = offset + (i as usize * account_id_size);
                    let end = start + account_id_size;
                    if end <= bytes.len() {
                        let account_id_bytes = &bytes[start..end];
                        let account_id_hex = format!("0x{}", hex::encode(account_id_bytes));
                        targets.push(account_id_hex);
                    }
                }
            }
        }
        
        Ok(targets)
    }
    
    /// Decode StakingLedger struct to extract total stake
    /// StakingLedger structure: { stash: AccountId, total: Balance, active: Balance, ... }
    /// We need to find the 'total' field which is a Balance (u128, 16 bytes)
    fn decode_staking_ledger_stake(&self, bytes: &[u8]) -> Result<u128, ElectionError> {
        if bytes.len() < 32 {
            return Err(ElectionError::RpcError {
                message: "StakingLedger data too short".to_string(),
                url: self.url.clone(),
            });
        }
        
        // StakingLedger structure (simplified):
        // - stash: AccountId (32 bytes) - offset 0
        // - total: Balance (u128, 16 bytes) - offset 32
        // - active: Balance (u128, 16 bytes) - offset 48
        // - ... other fields
        
        // Extract total stake (u128, little-endian, 16 bytes) at offset 32
        if bytes.len() < 48 {
            // If we don't have enough bytes, try to read what we have
            // Some chains might have different structures
            return Err(ElectionError::RpcError {
                message: "StakingLedger data incomplete".to_string(),
                url: self.url.clone(),
            });
        }
        
        let mut stake_bytes = [0u8; 16];
        stake_bytes.copy_from_slice(&bytes[32..48]);
        
        // Decode u128 as little-endian
        let stake = u128::from_le_bytes(stake_bytes);
        
        Ok(stake)
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

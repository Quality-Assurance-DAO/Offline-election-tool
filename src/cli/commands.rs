//! CLI commands implementation

use crate::error::ElectionError;
use crate::engine::ElectionEngine;
use crate::models::election_config::ElectionConfiguration;
use crate::models::election_data::ElectionData;
use crate::types::AlgorithmType;
use clap::Parser;
use std::path::PathBuf;

/// Run command for executing elections
#[derive(Parser)]
#[command(name = "run")]
#[command(about = "Run an election simulation")]
pub struct RunCommand {
    /// Election algorithm to use (sequential-phragmen, parallel-phragmen, multi-phase)
    #[arg(long)]
    pub algorithm: String,

    /// Number of validators to select
    #[arg(long)]
    pub active_set_size: u32,

    /// RPC URL for fetching on-chain data
    #[arg(long, conflicts_with_all = ["input_file", "synthetic"])]
    pub rpc_url: Option<String>,

    /// Block number for RPC snapshot
    #[arg(long, requires = "rpc_url")]
    pub block_number: Option<u64>,

    /// Input file path (JSON format)
    #[arg(long, conflicts_with_all = ["rpc_url", "synthetic"])]
    pub input_file: Option<PathBuf>,

    /// Use synthetic data (requires additional flags)
    #[arg(long, conflicts_with_all = ["rpc_url", "input_file"])]
    pub synthetic: bool,

    /// Include detailed diagnostics in output
    #[arg(long)]
    pub diagnostics: bool,

    /// Output file path (default: stdout)
    #[arg(long)]
    pub output_file: Option<PathBuf>,

    /// Output format: json or human-readable
    #[arg(long, default_value = "json")]
    pub format: String,
}

impl RunCommand {
    /// Execute the run command
    pub async fn execute(&self) -> Result<(), ElectionError> {
        // Load election data
        let election_data = self.load_data().await?;

        // Parse algorithm type
        let algorithm = self.algorithm.parse::<AlgorithmType>()
            .map_err(|e| ElectionError::ValidationError {
                message: format!("Invalid algorithm: {}", e),
                field: Some("algorithm".to_string()),
            })?;

        // Create election configuration
        let mut config = ElectionConfiguration::new()
            .algorithm(algorithm)
            .active_set_size(self.active_set_size);

        if let Some(block) = self.block_number {
            config = config.block_number(block);
        }

        let config = config.build()?;

        // Execute election
        let engine = ElectionEngine::new();
        let result = engine.execute(&config, &election_data)?;

        // Output results
        self.output_result(&result)?;

        Ok(())
    }

    /// Load election data from the specified source
    async fn load_data(&self) -> Result<ElectionData, ElectionError> {
        if let Some(ref rpc_url) = self.rpc_url {
            // Load from RPC
            let loader = crate::input::rpc::RpcLoader::new(rpc_url)?;
            let block_number = self.block_number.unwrap_or_else(|| {
                // If no block number specified, use latest (None = latest)
                0 // We'll handle this in the RPC loader
            });
            
            if block_number == 0 {
                // Get latest block
                loader.load_latest().await
            } else {
                loader.load_at_block(block_number).await
            }
        } else if let Some(ref input_file) = self.input_file {
            // Load from JSON file
            let json_loader = crate::input::json::JsonLoader::new();
            json_loader.load_from_file(input_file.clone())
        } else if self.synthetic {
            // Create synthetic data
            Err(ElectionError::InvalidData {
                message: "Synthetic data creation not yet implemented".to_string(),
            })
        } else {
            Err(ElectionError::ValidationError {
                message: "Must specify one of: --rpc-url, --input-file, or --synthetic".to_string(),
                field: None,
            })
        }
    }

    /// Output election results
    fn output_result(&self, result: &crate::models::election_result::ElectionResult) -> Result<(), ElectionError> {
        let output = if self.format == "human-readable" {
            self.format_human_readable(result)?
        } else {
            result.to_json()?
        };

        if let Some(ref output_file) = self.output_file {
            std::fs::write(output_file, output).map_err(|e| ElectionError::FileError {
                message: format!("Failed to write output file: {}", e),
                path: output_file.clone(),
            })?;
        } else {
            println!("{}", output);
        }

        Ok(())
    }

    /// Format result as human-readable text
    fn format_human_readable(&self, result: &crate::models::election_result::ElectionResult) -> Result<String, ElectionError> {
        let mut output = String::new();
        output.push_str("Election Results\n");
        output.push_str("================\n");
        output.push_str(&format!("Algorithm: {:?}\n", result.algorithm_used));
        output.push_str(&format!("Total Stake: {}\n", result.total_stake));
        output.push_str(&format!("Selected Validators: {}\n\n", result.selected_validators.len()));

        output.push_str("Selected Validators:\n");
        for (idx, validator) in result.selected_validators.iter().take(10).enumerate() {
            output.push_str(&format!(
                "{}. {} - Stake: {}, Nominators: {}\n",
                idx + 1,
                validator.account_id,
                validator.total_backing_stake,
                validator.nominator_count
            ));
        }

        if result.selected_validators.len() > 10 {
            output.push_str(&format!("... and {} more\n", result.selected_validators.len() - 10));
        }

        Ok(output)
    }
}

use ethers::types::Address;
use serde::{Deserialize, Serialize};
use crate::utils::error::{BridgeError, BridgeResult};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BridgeConfig {
    pub ethereum_rpc: String,
    pub base_rpc: String,
    pub bridge_contract_ethereum: Address,
    pub bridge_contract_base: Address,
    pub validator_private_key: String,
    pub ethereum_chain_id: u64,
    pub base_chain_id: u64,
    pub confirmation_blocks: u64,
}

impl BridgeConfig {
    pub fn from_env() -> BridgeResult<Self> {
        dotenv::dotenv().ok();

        Ok(Self {
            ethereum_rpc: std::env::var("ETHEREUM_RPC")
                .map_err(|e| BridgeError::ConfigError(format!("ETHEREUM_RPC not set: {}", e)))?,
            base_rpc: std::env::var("BASE_RPC")
                .map_err(|e| BridgeError::ConfigError(format!("BASE_RPC not set: {}", e)))?,
            bridge_contract_ethereum: std::env::var("BRIDGE_CONTRACT_ETHEREUM")
                .map_err(|e| BridgeError::ConfigError(format!("BRIDGE_CONTRACT_ETHEREUM not set: {}", e)))?
                .parse()
                .map_err(|e| BridgeError::ConfigError(format!("Invalid Ethereum contract address: {}", e)))?,
            bridge_contract_base: std::env::var("BRIDGE_CONTRACT_BASE")
                .map_err(|e| BridgeError::ConfigError(format!("BRIDGE_CONTRACT_BASE not set: {}", e)))?
                .parse()
                .map_err(|e| BridgeError::ConfigError(format!("Invalid Base contract address: {}", e)))?,
            validator_private_key: std::env::var("VALIDATOR_PRIVATE_KEY")
                .map_err(|e| BridgeError::ConfigError(format!("VALIDATOR_PRIVATE_KEY not set: {}", e)))?,
            ethereum_chain_id: std::env::var("ETHEREUM_CHAIN_ID")
                .unwrap_or_else(|_| "1".to_string())
                .parse()
                .map_err(|e| BridgeError::ConfigError(format!("Invalid Ethereum chain ID: {}", e)))?,
            base_chain_id: std::env::var("BASE_CHAIN_ID")
                .unwrap_or_else(|_| "8453".to_string())
                .parse()
                .map_err(|e| BridgeError::ConfigError(format!("Invalid Base chain ID: {}", e)))?,
            confirmation_blocks: std::env::var("CONFIRMATION_BLOCKS")
                .unwrap_or_else(|_| "12".to_string())
                .parse()
                .map_err(|e| BridgeError::ConfigError(format!("Invalid confirmation blocks: {}", e)))?,
        })
    }
} 
use ethers::types::{Address, U256, H256};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CrossChainMessage {
    pub source_chain_id: U256,
    pub destination_chain_id: U256,
    pub sender: Address,
    pub recipient: Address,
    pub amount: U256,
    pub nonce: U256,
    pub message_id: H256,
    pub data: Vec<u8>,
    pub signature: Vec<u8>,
}

#[derive(Debug, Clone, Copy)]
pub enum ChainType {
    Ethereum,
    Base,
}

#[derive(Debug, Clone)]
pub struct MessageStatus {
    pub message_id: H256,
    pub processed: bool,
    pub confirmed_block: Option<U256>,
    pub execution_status: ExecutionStatus,
}

#[derive(Debug, Clone)]
pub enum ExecutionStatus {
    Pending,
    Confirmed,
    Failed(String),
    Executed,
}

#[derive(Debug, Clone)]
pub struct BridgeEvent {
    pub chain_type: ChainType,
    pub message: CrossChainMessage,
    pub block_number: U256,
    pub transaction_hash: H256,
} 
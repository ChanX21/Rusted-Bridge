use crate::{
    config::BridgeConfig,
    utils::error::{BridgeError, BridgeResult},
    bridge::types::{CrossChainMessage, ChainType, MessageStatus, ExecutionStatus, BridgeEvent},
};
use ethers::{
    prelude::*,
    providers::{Provider, Ws},
    types::{Log, Filter, H256, H160, U256},
};
use std::sync::Arc;
use tracing::{info, error, warn};

pub struct Bridge {
    eth_client: Arc<Provider<Ws>>,
    base_client: Arc<Provider<Ws>>,
    config: BridgeConfig,
}

impl Bridge {
    pub async fn new(config: BridgeConfig) -> BridgeResult<Self> {
        let eth_client = Arc::new(
            Provider::<Ws>::connect(&config.ethereum_rpc)
                .await
                .map_err(|e| BridgeError::ProviderError(e.to_string()))?
        );
        let base_client = Arc::new(
            Provider::<Ws>::connect(&config.base_rpc)
                .await
                .map_err(|e| BridgeError::ProviderError(e.to_string()))?
        );

        Ok(Bridge {
            eth_client,
            base_client,
            config,
        })
    }

    pub async fn start(&self) -> BridgeResult<()> {
        info!("Starting bridge monitoring...");
        
        tokio::join!(
            self.monitor_ethereum_events(),
            self.monitor_base_events(),
        );

        Ok(())
    }

    async fn monitor_ethereum_events(&self) -> BridgeResult<()> {
        info!("Starting Ethereum event monitoring");
        
        let filter = Filter::new()
            .address(self.config.bridge_contract_ethereum)
            .event("CrossChainTransfer(address,address,uint256,uint256,bytes)");

        let mut stream = self.eth_client.subscribe_logs(&filter)
            .await
            .map_err(|e| BridgeError::ChainCommunicationError(e.to_string()))?;

        while let Some(log) = stream.next().await {
            match self.process_ethereum_event(log).await {
                Ok(_) => info!("Successfully processed Ethereum event"),
                Err(e) => error!("Failed to process Ethereum event: {}", e),
            }
        }

        Ok(())
    }

    async fn monitor_base_events(&self) -> BridgeResult<()> {
        info!("Starting Base event monitoring");
        
        let filter = Filter::new()
            .address(self.config.bridge_contract_base)
            .event("CrossChainTransfer(address,address,uint256,uint256,bytes)");

        let mut stream = self.base_client.subscribe_logs(&filter)
            .await
            .map_err(|e| BridgeError::ChainCommunicationError(e.to_string()))?;

        while let Some(log) = stream.next().await {
            match self.process_base_event(log).await {
                Ok(_) => info!("Successfully processed Base event"),
                Err(e) => error!("Failed to process Base event: {}", e),
            }
        }

        Ok(())
    }

    async fn process_ethereum_event(&self, log: Log) -> BridgeResult<()> {
        let event = self.parse_event(log, ChainType::Ethereum)?;
        
        // Verify the event has enough confirmations
        self.verify_confirmations(&event).await?;
        
        // Verify the message
        self.verify_message(&event.message)?;
        
        // Relay to Base
        self.relay_to_base(event).await
    }

    async fn process_base_event(&self, log: Log) -> BridgeResult<()> {
        let event = self.parse_event(log, ChainType::Base)?;
        
        // Verify the event has enough confirmations
        self.verify_confirmations(&event).await?;
        
        // Verify the message
        self.verify_message(&event.message)?;
        
        // Relay to Ethereum
        self.relay_to_ethereum(event).await
    }

    fn verify_message(&self, message: &CrossChainMessage) -> BridgeResult<()> {
        // Verify chain IDs
        if message.source_chain_id.as_u64() != self.config.ethereum_chain_id &&
           message.source_chain_id.as_u64() != self.config.base_chain_id {
            return Err(BridgeError::MessageVerificationError(
                "Invalid source chain ID".to_string()
            ));
        }

        if message.destination_chain_id.as_u64() != self.config.ethereum_chain_id &&
           message.destination_chain_id.as_u64() != self.config.base_chain_id {
            return Err(BridgeError::MessageVerificationError(
                "Invalid destination chain ID".to_string()
            ));
        }

        // Add more verification logic here
        Ok(())
    }

    async fn verify_confirmations(&self, event: &BridgeEvent) -> BridgeResult<()> {
        let current_block = match event.chain_type {
            ChainType::Ethereum => self.eth_client.get_block_number().await,
            ChainType::Base => self.base_client.get_block_number().await,
        }.map_err(|e| BridgeError::ChainCommunicationError(e.to_string()))?;

        if current_block.as_u64() - event.block_number.as_u64() < self.config.confirmation_blocks {
            return Err(BridgeError::MessageVerificationError(
                "Not enough confirmations".to_string()
            ));
        }

        Ok(())
    }

    async fn relay_to_base(&self, event: BridgeEvent) -> BridgeResult<()> {
        info!("Relaying message to Base: {:?}", event.message.message_id);
        // Implement relay logic to Base
        Ok(())
    }

    async fn relay_to_ethereum(&self, event: BridgeEvent) -> BridgeResult<()> {
        info!("Relaying message to Ethereum: {:?}", event.message.message_id);
        // Implement relay logic to Ethereum
        Ok(())
    }

    fn parse_event(&self, log: Log, chain_type: ChainType) -> BridgeResult<BridgeEvent> {
        // Get source and destination chain IDs based on chain type
        let (source_chain_id, destination_chain_id) = match chain_type {
            ChainType::Ethereum => (
                U256::from(self.config.ethereum_chain_id),
                U256::from(self.config.base_chain_id)
            ),
            ChainType::Base => (
                U256::from(self.config.base_chain_id),
                U256::from(self.config.ethereum_chain_id)
            ),
        };

        // Parse the event data from the log
        let event = BridgeEvent {
            chain_type,
            message: CrossChainMessage {
                source_chain_id,
                destination_chain_id,
                sender: H160::from_slice(&log.topics.get(1)
                    .ok_or_else(|| BridgeError::MessageVerificationError("Missing sender".to_string()))?
                    .as_fixed_bytes()[12..]),
                recipient: H160::from_slice(&log.topics.get(2)
                    .ok_or_else(|| BridgeError::MessageVerificationError("Missing recipient".to_string()))?
                    .as_fixed_bytes()[12..]),
                amount: U256::from_big_endian(&log.data[..32]),
                nonce: U256::from_big_endian(&log.data[32..64]),
                message_id: H256::from_slice(&log.data[64..96]),
                data: log.data[96..].to_vec(),
                signature: vec![], // Signature will be added during relay
            },
            block_number: U256::from(log.block_number
                .ok_or_else(|| BridgeError::MessageVerificationError("Missing block number".to_string()))?
                .as_u64()),
            transaction_hash: log.transaction_hash
                .ok_or_else(|| BridgeError::MessageVerificationError("Missing transaction hash".to_string()))?,
        };

        Ok(event)
    }
} 
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BridgeError {
    #[error("Provider error: {0}")]
    ProviderError(String),

    #[error("Message verification failed: {0}")]
    MessageVerificationError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Chain communication error: {0}")]
    ChainCommunicationError(String),

    #[error("Invalid signature: {0}")]
    SignatureError(String),

    #[error("Contract interaction error: {0}")]
    ContractError(String),

    #[error("Database error: {0}")]
    DatabaseError(String),
}

pub type BridgeResult<T> = Result<T, BridgeError>; 
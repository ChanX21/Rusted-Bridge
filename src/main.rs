mod bridge;
mod config;
mod utils;

use bridge::Bridge;
use config::BridgeConfig;
use tracing::{info, error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    info!("Starting RustedBridge...");

    // Load configuration from environment
    let config = BridgeConfig::from_env()?;
    
    // Initialize bridge
    let bridge = Bridge::new(config).await?;

    info!("Bridge initialized successfully");

    // Start the bridge
    bridge.start().await?;

    Ok(())
} 
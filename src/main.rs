mod rpc_manager;
mod engine;
mod trading;

use rpc_manager::RpcManager;
use engine::monitor::PumpMonitor;
use std::sync::Arc;
use log::{info, error};
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    env_logger::init();
    info!("Starting Saha Sniper Bot on Oracle VM...");

    // 1. Setup RPC Manager from .env
    let rpc_urls_raw = env::var("RPC_URLS")
        .unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string());
    let rpc_urls: Vec<String> = rpc_urls_raw.split(',').map(|s| s.to_string()).collect();
    
    info!("Loaded {} RPC endpoints for rotation", rpc_urls.len());
    let rpc_manager = Arc::new(RpcManager::new(rpc_urls));

    // 2. Setup WSS URLs from .env
    let wss_urls_raw = env::var("WSS_URLS")
        .unwrap_or_else(|_| "wss://pumpportal.fun/api/data".to_string());
    let wss_urls: Vec<String> = wss_urls_raw.split(',').map(|s| s.to_string()).collect();

    // 3. Initialize Engine & Monitor
    let monitor = PumpMonitor::new(wss_urls, rpc_manager.clone());

    info!("Bot is active. Monitoring for new token launches...");

    // 4. Run Monitor loop (needs to be mutable now for rotation)
    let mut monitor = monitor;
    if let Err(e) = monitor.listen().await {
        error!("Monitor crashed: {}. Consider restarting.", e);
    }

    Ok(())
}

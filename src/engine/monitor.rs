use futures_util::{StreamExt, SinkExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;
use log::{info, error};
use std::sync::Arc;
use crate::rpc_manager::RpcManager;

pub struct PumpMonitor {
    wss_urls: Vec<String>,
    current_wss_index: usize,
    rpc_manager: Arc<RpcManager>,
}

impl PumpMonitor {
    pub fn new(wss_urls: Vec<String>, rpc_manager: Arc<RpcManager>) -> Self {
        Self { 
            wss_urls, 
            current_wss_index: 0,
            rpc_manager 
        }
    }

    pub async fn listen(&mut self) -> anyhow::Result<()> {
        loop {
            let url_str = &self.wss_urls[self.current_wss_index];
            let url = Url::parse(url_str)?;
            
            info!("Attempting connection to WSS: {}", url_str);

            match connect_async(url).await {
                Ok((ws_stream, _)) => {
                    info!("Successfully connected to WSS");
                    let (mut _write, mut read) = ws_stream.split();

                    while let Some(msg) = read.next().await {
                        match msg {
                            Ok(Message::Text(text)) => {
                                self.process_message(text).await;
                            }
                            Ok(_) => {}
                            Err(e) => {
                                error!("WSS Read Error: {}. Rotating...", e);
                                break; // Break inner loop to rotate
                            }
                        }
                    }
                }
                Err(e) => {
                    error!("WSS Connection Error: {}. Rotating...", e);
                }
            }

            // Rotate to next WSS URL on failure
            self.current_wss_index = (self.current_wss_index + 1) % self.wss_urls.len();
            tokio::time::sleep(Duration::from_secs(2)).await;
        }
    }

    async fn process_message(&self, text: String) {
        // Logic to parse new token creation
        // If new token:
        // 1. Get current slot
        // 2. Schedule buy for slot + 2 (2nd block)
        // 3. Run Intelligence Scoring
    }
}

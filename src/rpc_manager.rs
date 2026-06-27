use solana_client::rpc_client::RpcClient;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
use log::{info, warn};

pub struct RpcManager {
    clients: Vec<Arc<RpcClient>>,
    current_index: AtomicUsize,
    urls: Vec<String>,
}

impl RpcManager {
    pub fn new(urls: Vec<String>) -> Self {
        let clients = urls.iter()
            .map(|url| {
                Arc::new(RpcClient::new_with_timeout(
                    url.to_string(),
                    Duration::from_secs(10),
                ))
            })
            .collect();

        Self {
            clients,
            current_index: AtomicUsize::new(0),
            urls,
        }
    }

    pub fn get_client(&self) -> Arc<RpcClient> {
        let index = self.current_index.load(Ordering::Relaxed);
        self.clients[index].clone()
    }

    pub fn rotate(&self) {
        let old_index = self.current_index.load(Ordering::Relaxed);
        let new_index = (old_index + 1) % self.clients.len();
        self.current_index.store(new_index, Ordering::Relaxed);
        warn!("Rotating RPC client to: {}", self.urls[new_index]);
    }

    /// Execute an RPC call with automatic rotation on rate limit (429)
    pub async fn execute<F, T, E>(&self, f: F) -> Result<T, anyhow::Error>
    where
        F: Fn(Arc<RpcClient>) -> Result<T, E>,
        E: std::fmt::Display,
    {
        let mut attempts = 0;
        let max_attempts = self.clients.len();

        while attempts < max_attempts {
            let client = self.get_client();
            match f(client) {
                Ok(result) => return Ok(result),
                Err(e) => {
                    let err_str = e.to_string();
                    if err_str.contains("429") || err_str.contains("Too Many Requests") {
                        warn!("Rate limit hit on RPC. Rotating...");
                        self.rotate();
                        attempts += 1;
                        tokio::time::sleep(Duration::from_millis(500)).await;
                    } else {
                        return Err(anyhow::anyhow!("RPC Error: {}", err_str));
                    }
                }
            }
        }
        Err(anyhow::anyhow!("Failed after rotating through all RPCs"))
    }
}

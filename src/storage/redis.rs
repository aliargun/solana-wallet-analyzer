use redis::{Client, Commands, Connection};
use crate::types::WalletMetrics;
use serde_json;
use std::time::Duration;
use tracing::{info, error};

pub struct RedisStorage {
    client: Client,
}

const RANKINGS_KEY: &str = "wallet_rankings";
const TOP_WALLETS_KEY: &str = "top_wallets";
const METRICS_EXPIRY: u64 = 3600; // 1 hour

impl RedisStorage {
    pub fn new(redis_url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let client = redis::Client::open(redis_url)?;
        Ok(Self { client })
    }

    pub async fn store_metrics(
        &self,
        metrics: &WalletMetrics,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut conn = self.client.get_connection()?;
        
        // Store full metrics as JSON
        let metrics_json = serde_json::to_string(metrics)?;
        let key = format!("wallet:{}", metrics.address);
        conn.set_ex(&key, metrics_json, METRICS_EXPIRY as usize)?;

        // Update rankings
        conn.zadd(
            RANKINGS_KEY,
            metrics.address.clone(),
            metrics.total_profit_loss,
        )?;

        Ok(())
    }

    pub async fn store_top_wallets(
        &self,
        wallets: &[WalletMetrics],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut conn = self.client.get_connection()?;
        
        // Store the complete list of top wallets
        let wallets_json = serde_json::to_string(wallets)?;
        conn.set_ex(TOP_WALLETS_KEY, wallets_json, METRICS_EXPIRY as usize)?;

        // Update individual wallet rankings
        let mut pipe = redis::pipe();
        pipe.atomic();

        // Clear existing rankings
        pipe.del(RANKINGS_KEY);

        // Add new rankings
        for (idx, wallet) in wallets.iter().enumerate() {
            pipe.zadd(RANKINGS_KEY, &wallet.address, -(idx as i64));
        }

        pipe.execute(&mut conn);
        Ok(())
    }

    pub async fn get_top_wallets(
        &self,
        limit: usize,
    ) -> Result<Vec<WalletMetrics>, Box<dyn std::error::Error>> {
        let mut conn = self.client.get_connection()?;
        
        // First try to get the cached top wallets list
        if let Ok(data) = conn.get::<_, String>(TOP_WALLETS_KEY) {
            if let Ok(wallets) = serde_json::from_str(&data) {
                return Ok(wallets);
            }
        }

        // Fallback to reconstructing from individual metrics
        let addresses: Vec<String> = conn.zrevrange(RANKINGS_KEY, 0, (limit - 1) as isize)?;
        
        let mut metrics = Vec::with_capacity(addresses.len());
        for addr in addresses {
            let key = format!("wallet:{}", addr);
            if let Ok(data) = conn.get::<_, String>(&key) {
                if let Ok(wallet_metrics) = serde_json::from_str(&data) {
                    metrics.push(wallet_metrics);
                }
            }
        }

        Ok(metrics)
    }

    pub async fn get_wallet_metrics(
        &self,
        address: &str,
    ) -> Result<Option<WalletMetrics>, Box<dyn std::error::Error>> {
        let mut conn = self.client.get_connection()?;
        let key = format!("wallet:{}", address);

        if let Ok(data) = conn.get::<_, String>(&key) {
            if let Ok(metrics) = serde_json::from_str(&data) {
                return Ok(Some(metrics));
            }
        }

        Ok(None)
    }
}
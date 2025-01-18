use redis::{Client, Commands, Connection};
use crate::types::WalletMetrics;
use serde_json;
use std::time::Duration;

pub struct RedisStorage {
    client: Client,
}

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
        conn.set_ex(&key, metrics_json, 3600)?; // 1 hour expiry

        // Update rankings
        conn.zadd(
            "wallet_rankings",
            metrics.address.clone(),
            metrics.total_profit_loss,
        )?;

        Ok(())
    }

    pub async fn get_top_wallets(
        &self,
        limit: usize,
    ) -> Result<Vec<WalletMetrics>, Box<dyn std::error::Error>> {
        let mut conn = self.client.get_connection()?;
        
        // Get top addresses by score
        let addresses: Vec<String> = conn.zrevrange("wallet_rankings", 0, (limit - 1) as isize)?;
        
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
}
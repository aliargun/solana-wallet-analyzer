use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WalletMetrics {
    pub address: String,
    pub total_profit_loss: f64,
    pub win_rate: f64,
    pub avg_trade_size: f64,
    pub trade_count: u64,
    pub last_updated: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TradeInfo {
    pub wallet_address: String,
    pub timestamp: i64,
    pub amount: f64,
    pub profit_loss: f64,
    pub transaction_hash: String,
}
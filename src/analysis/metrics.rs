use crate::types::{TradeInfo, WalletMetrics};
use chrono::{DateTime, Utc};

pub struct MetricsCalculator;

impl MetricsCalculator {
    pub fn new() -> Self {
        Self
    }

    pub fn calculate_metrics(
        &self,
        trades: &[TradeInfo],
    ) -> Result<WalletMetrics, Box<dyn std::error::Error>> {
        if trades.is_empty() {
            return Err("No trades found".into());
        }

        let wallet_address = trades[0].wallet_address.clone();
        let total_trades = trades.len() as u64;
        
        let total_profit_loss: f64 = trades.iter()
            .map(|t| t.profit_loss)
            .sum();

        let profitable_trades = trades.iter()
            .filter(|t| t.profit_loss > 0.0)
            .count();

        let win_rate = (profitable_trades as f64) / (total_trades as f64) * 100.0;

        let avg_trade_size = trades.iter()
            .map(|t| t.amount)
            .sum::<f64>() / (total_trades as f64);

        Ok(WalletMetrics {
            address: wallet_address,
            total_profit_loss,
            win_rate,
            avg_trade_size,
            trade_count: total_trades,
            last_updated: Utc::now().timestamp(),
        })
    }
}
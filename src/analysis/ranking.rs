use crate::types::WalletMetrics;

pub struct WalletRanker;

impl WalletRanker {
    pub fn new() -> Self {
        Self
    }

    pub fn rank_wallets(
        &self,
        metrics: &[WalletMetrics],
    ) -> Vec<WalletMetrics> {
        let mut ranked = metrics.to_vec();
        ranked.sort_by(|a, b| {
            // Primary sort by profit/loss
            let profit_cmp = b.total_profit_loss.partial_cmp(&a.total_profit_loss).unwrap();
            if profit_cmp != std::cmp::Ordering::Equal {
                return profit_cmp;
            }
            
            // Secondary sort by win rate
            let winrate_cmp = b.win_rate.partial_cmp(&a.win_rate).unwrap();
            if winrate_cmp != std::cmp::Ordering::Equal {
                return winrate_cmp;
            }
            
            // Finally sort by trade count
            b.trade_count.cmp(&a.trade_count)
        });
        ranked
    }
}
use serde::Serialize;
use std::collections::HashMap;
use crate::types::WalletMetrics;

#[derive(Serialize)]
pub struct DashboardData {
    pub top_wallets: Vec<WalletMetrics>,
    pub performance_summary: PerformanceSummary,
    pub metrics_distribution: MetricsDistribution,
}

#[derive(Serialize)]
pub struct PerformanceSummary {
    pub total_wallets_analyzed: usize,
    pub average_profit_loss: f64,
    pub average_win_rate: f64,
    pub total_trade_volume: f64,
}

#[derive(Serialize)]
pub struct MetricsDistribution {
    pub profit_loss_ranges: HashMap<String, usize>,
    pub win_rate_ranges: HashMap<String, usize>,
    pub trade_size_ranges: HashMap<String, usize>,
}

pub fn generate_dashboard_data(wallets: &[WalletMetrics]) -> DashboardData {
    let total_wallets = wallets.len();
    let mut total_profit_loss = 0.0;
    let mut total_win_rate = 0.0;
    let mut total_volume = 0.0;

    // Calculate summary statistics
    for wallet in wallets {
        total_profit_loss += wallet.total_profit_loss;
        total_win_rate += wallet.win_rate;
        total_volume += wallet.avg_trade_size * wallet.trade_count as f64;
    }

    let summary = PerformanceSummary {
        total_wallets_analyzed: total_wallets,
        average_profit_loss: if total_wallets > 0 { total_profit_loss / total_wallets as f64 } else { 0.0 },
        average_win_rate: if total_wallets > 0 { total_win_rate / total_wallets as f64 } else { 0.0 },
        total_trade_volume: total_volume,
    };

    // Calculate distributions
    let mut profit_loss_ranges = HashMap::new();
    let mut win_rate_ranges = HashMap::new();
    let mut trade_size_ranges = HashMap::new();

    for wallet in wallets {
        // Profit/Loss ranges
        let pl_range = match wallet.total_profit_loss {
            x if x < 0.0 => "Loss",
            x if x < 100.0 => "0-100",
            x if x < 1000.0 => "100-1000",
            _ => ">1000",
        };
        *profit_loss_ranges.entry(pl_range.to_string()).or_insert(0) += 1;

        // Win rate ranges
        let wr_range = match wallet.win_rate {
            x if x < 40.0 => "<40%",
            x if x < 50.0 => "40-50%",
            x if x < 60.0 => "50-60%",
            _ => ">60%",
        };
        *win_rate_ranges.entry(wr_range.to_string()).or_insert(0) += 1;

        // Trade size ranges
        let ts_range = match wallet.avg_trade_size {
            x if x < 100.0 => "<100",
            x if x < 1000.0 => "100-1000",
            x if x < 10000.0 => "1000-10000",
            _ => ">10000",
        };
        *trade_size_ranges.entry(ts_range.to_string()).or_insert(0) += 1;
    }

    let distribution = MetricsDistribution {
        profit_loss_ranges,
        win_rate_ranges,
        trade_size_ranges,
    };

    DashboardData {
        top_wallets: wallets.to_vec(),
        performance_summary: summary,
        metrics_distribution: distribution,
    }
}
use colored::*;
use crate::visualization::{DashboardData, PerformanceSummary, MetricsDistribution};
use crate::types::WalletMetrics;

pub fn display_dashboard(data: &DashboardData) {
    println!("{}", "=== Solana Wallet Performance Dashboard ===".bold());
    println!();

    display_summary(&data.performance_summary);
    println!();

    display_top_wallets(&data.top_wallets);
    println!();

    display_distributions(&data.metrics_distribution);
}

fn display_summary(summary: &PerformanceSummary) {
    println!("{}", "Performance Summary".bold().underline());
    println!("Total Wallets Analyzed: {}", summary.total_wallets_analyzed);
    println!("Average Profit/Loss: {:.2} SOL", summary.average_profit_loss);
    println!("Average Win Rate: {:.2}%", summary.average_win_rate);
    println!("Total Trade Volume: {:.2} SOL", summary.total_trade_volume);
}

fn display_top_wallets(wallets: &[WalletMetrics]) {
    println!("{}", "Top Performing Wallets".bold().underline());
    println!("{:<44} {:>12} {:>10} {:>12}", "Wallet", "Profit/Loss", "Win Rate", "Trade Count");
    println!("{}", "=".repeat(80));

    for wallet in wallets.iter().take(10) {
        println!(
            "{:<44} {:>12.2} {:>9.1}% {:>12}",
            wallet.address,
            wallet.total_profit_loss,
            wallet.win_rate,
            wallet.trade_count
        );
    }
}

fn display_distributions(dist: &MetricsDistribution) {
    println!("{}", "Metric Distributions".bold().underline());
    
    println!("
Profit/Loss Distribution:");
    for (range, count) in &dist.profit_loss_ranges {
        println!("{:<10}: {}", range, "█".repeat(*count));
    }

    println!("
Win Rate Distribution:");
    for (range, count) in &dist.win_rate_ranges {
        println!("{:<10}: {}", range, "█".repeat(*count));
    }

    println!("
Trade Size Distribution:");
    for (range, count) in &dist.trade_size_ranges {
        println!("{:<10}: {}", range, "█".repeat(*count));
    }
}
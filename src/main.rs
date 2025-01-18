use clap::Parser;
use tokio::time::{sleep, Duration};
use tracing::{info, warn, error, Level};
use tracing_subscriber::FmtSubscriber;
use rayon::prelude::*;
use std::sync::Arc;

mod ingestion;
mod analysis;
mod storage;
mod types;
mod error;
mod visualization;

use ingestion::client::SolanaClient;
use analysis::{metrics::MetricsCalculator, ranking::WalletRanker};
use storage::redis::RedisStorage;
use types::{TradeInfo, WalletMetrics};
use error::{AnalyzerError, Result};
use visualization::{self, cli::display_dashboard, generate_dashboard_data};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(long, env = "SOLANA_RPC_URL")]
    rpc_url: String,

    #[arg(long, env = "REDIS_URL", default_value = "redis://127.0.0.1/")]
    redis_url: String,

    #[arg(long, default_value = "1000")]
    batch_size: u64,

    #[arg(long, default_value = "5")]
    update_interval: u64,

    #[arg(long, default_value = "info")]
    log_level: String,

    #[arg(long)]
    no_dashboard: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    // Initialize logging
    let log_level = match args.log_level.to_lowercase().as_str() {
        "debug" => Level::DEBUG,
        "info" => Level::INFO,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        _ => Level::INFO,
    };

    let subscriber = FmtSubscriber::builder()
        .with_max_level(log_level)
        .with_target(false)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .pretty()
        .build();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set tracing subscriber");
    
    info!("Starting Solana Wallet Performance Analyzer");
    
    // Initialize components
    let storage = Arc::new(RedisStorage::new(&args.redis_url)
        .map_err(|e| AnalyzerError::StorageError(format!("Failed to initialize Redis: {}", e)))?);    
    let client = Arc::new(SolanaClient::new(&args.rpc_url)
        .map_err(|e| AnalyzerError::SolanaClientError(format!("Failed to initialize Solana client: {}", e)))?);    
    let metrics_calculator = Arc::new(MetricsCalculator::new());
    let wallet_ranker = Arc::new(WalletRanker::new());
    
    info!("Initialization complete");
    
    // Main processing loop
    loop {
        match process_batch(
            Arc::clone(&client),
            Arc::clone(&storage),
            Arc::clone(&metrics_calculator),
            Arc::clone(&wallet_ranker),
            args.batch_size,
            !args.no_dashboard,
        ).await {
            Ok(processed) => {
                info!("Successfully processed {} transactions", processed);
            }
            Err(e) => {
                error!("Error processing batch: {}", e);
                sleep(Duration::from_secs(1)).await;
            }
        }

        sleep(Duration::from_secs(args.update_interval)).await;
    }
}

async fn process_batch(
    client: Arc<SolanaClient>,
    storage: Arc<RedisStorage>,
    metrics_calculator: Arc<MetricsCalculator>,
    wallet_ranker: Arc<WalletRanker>,
    batch_size: u64,
    show_dashboard: bool,
) -> Result<usize> {
    let transactions = client.get_recent_transactions().await
        .map_err(|e| AnalyzerError::SolanaClientError(format!("Failed to fetch transactions: {}", e)))?;
    
    let processed_count = transactions.len();
    info!("Fetched {} transactions", processed_count);

    // Process transactions in parallel
    let trade_infos: Vec<_> = transactions.par_iter()
        .filter_map(|tx| client.extract_trade_info(tx))
        .collect();

    // Group by wallet (in parallel)
    let mut wallet_trades = std::collections::HashMap::new();
    trade_infos.into_par_iter().for_each(|trade| {
        wallet_trades
            .entry(trade.wallet_address.clone())
            .or_insert_with(Vec::new)
            .push(trade);
    });

    info!("Found trades for {} unique wallets", wallet_trades.len());

    // Calculate metrics in parallel
    let all_metrics: Vec<_> = wallet_trades.par_iter()
        .filter_map(|(_, trades)| {
            match metrics_calculator.calculate_metrics(trades) {
                Ok(metrics) => Some(metrics),
                Err(e) => {
                    error!("Failed to calculate metrics: {}", e);
                    None
                }
            }
        })
        .collect();

    // Store metrics in parallel batches
    let metrics_chunks = all_metrics.chunks(100);
    for chunk in metrics_chunks {
        let futures: Vec<_> = chunk.iter()
            .map(|metrics| storage.store_metrics(metrics))
            .collect();
        
        futures::future::join_all(futures).await;
    }

    // Rank wallets and update top performers
    let ranked_wallets = wallet_ranker.rank_wallets(&all_metrics);
    let top_wallets = ranked_wallets.into_iter().take(100).collect::<Vec<_>>();
    
    if let Err(e) = storage.store_top_wallets(&top_wallets).await {
        error!("Failed to store top wallets: {}", e);
    }

    // Generate and display dashboard if enabled
    if show_dashboard {
        let dashboard_data = generate_dashboard_data(&top_wallets);
        display_dashboard(&dashboard_data);
    }

    Ok(processed_count)
}
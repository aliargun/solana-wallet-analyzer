use clap::Parser;
use tokio::time::{sleep, Duration};
use tracing::{info, warn, error, Level};
use tracing_subscriber::FmtSubscriber;

mod ingestion;
mod analysis;
mod storage;
mod types;
mod error;

use ingestion::client::SolanaClient;
use analysis::{metrics::MetricsCalculator, ranking::WalletRanker};
use storage::redis::RedisStorage;
use types::{TradeInfo, WalletMetrics};
use error::{AnalyzerError, Result};

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
    let storage = RedisStorage::new(&args.redis_url)
        .map_err(|e| AnalyzerError::StorageError(format!("Failed to initialize Redis: {}", e)))?;
    
    let client = SolanaClient::new(&args.rpc_url)
        .map_err(|e| AnalyzerError::SolanaClientError(format!("Failed to initialize Solana client: {}", e)))?;
    
    let metrics_calculator = MetricsCalculator::new();
    let wallet_ranker = WalletRanker::new();
    
    info!("Initialization complete");
    
    // Main processing loop
    loop {
        match process_batch(
            &client,
            &storage,
            &metrics_calculator,
            &wallet_ranker,
            args.batch_size
        ).await {
            Ok(processed) => {
                info!("Successfully processed {} transactions", processed);
            }
            Err(e) => {
                error!("Error processing batch: {}", e);
                // Short delay before retry on error
                sleep(Duration::from_secs(1)).await;
            }
        }

        sleep(Duration::from_secs(args.update_interval)).await;
    }
}

async fn process_batch(
    client: &SolanaClient,
    storage: &RedisStorage,
    metrics_calculator: &MetricsCalculator,
    wallet_ranker: &WalletRanker,
    batch_size: u64,
) -> Result<usize> {
    let transactions = client.get_recent_transactions().await
        .map_err(|e| AnalyzerError::SolanaClientError(format!("Failed to fetch transactions: {}", e)))?;
    
    let processed_count = transactions.len();
    info!("Fetched {} transactions", processed_count);

    let mut wallet_trades = std::collections::HashMap::new();
    
    // Process transactions
    for tx in transactions {
        if let Some(trade_info) = client.extract_trade_info(&tx) {
            wallet_trades
                .entry(trade_info.wallet_address.clone())
                .or_insert_with(Vec::new)
                .push(trade_info);
        }
    }

    info!("Found trades for {} unique wallets", wallet_trades.len());

    // Calculate and store metrics
    let mut all_metrics = Vec::new();
    for (wallet, trades) in wallet_trades {
        match metrics_calculator.calculate_metrics(&trades) {
            Ok(metrics) => {
                if let Err(e) = storage.store_metrics(&metrics).await {
                    error!("Failed to store metrics for wallet {}: {}", wallet, e);
                    continue;
                }
                all_metrics.push(metrics);
            }
            Err(e) => {
                error!("Failed to calculate metrics for wallet {}: {}", wallet, e);
                continue;
            }
        }
    }

    // Update rankings
    if !all_metrics.is_empty() {
        let ranked_wallets = wallet_ranker.rank_wallets(&all_metrics);
        let top_wallets = ranked_wallets.into_iter().take(100).collect::<Vec<_>>();
        
        storage.store_top_wallets(&top_wallets).await
            .map_err(|e| AnalyzerError::StorageError(format!("Failed to store top wallets: {}", e)))?;
            
        info!("Updated rankings for {} wallets", top_wallets.len());
    }

    Ok(processed_count)
}
use clap::Parser;
use tracing::{info, warn};

mod ingestion;
mod analysis;
mod storage;
mod types;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(long, env = "SOLANA_RPC_URL")]
    rpc_url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    
    let args = Args::parse();
    info!("Starting Solana Wallet Performance Analyzer");
    
    // Initialize components
    let _storage = storage::init_redis().await?;
    let _client = ingestion::init_solana_client(&args.rpc_url)?;
    
    info!("Initialization complete");
    
    // TODO: Implement main processing loop
    
    Ok(())
}
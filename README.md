# Solana Wallet Performance Analyzer

A proof-of-concept tool designed to analyze and identify top-performing wallets on the Solana network. This tool processes real-time transaction data to calculate performance metrics and rank wallets based on their trading success.

## Features

- Real-time Solana transaction data ingestion
- Wallet performance analysis and ranking
- Key metrics calculation:
  - Total profit/loss
  - Win rate
  - Average trade size
  - Trading frequency
- High-performance data processing using Rust
- Redis-based caching for quick data access

## Requirements

- Rust 1.70+
- Redis 6.0+
- Solana RPC endpoint (mainnet or testnet)

## Installation

1. Install Rust:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Install Redis:
   ```bash
   # Ubuntu/Debian
   sudo apt-get install redis-server

   # macOS
   brew install redis
   ```

3. Clone the repository:
   ```bash
   git clone https://github.com/aliargun/solana-wallet-analyzer.git
   cd solana-wallet-analyzer
   ```

4. Build the project:
   ```bash
   cargo build --release
   ```

## Configuration

Create a `.env` file in the project root:

```env
SOLANA_RPC_URL=https://api.mainnet-beta.solana.com
REDIS_URL=redis://127.0.0.1:6379
LOG_LEVEL=info
```

## Usage

1. Start the analyzer:
   ```bash
   ./target/release/solana-wallet-analyzer --rpc-url <SOLANA_RPC_URL>
   ```

2. View real-time metrics:
   ```bash
   # Top performing wallets
   redis-cli ZRANGE wallet_rankings 0 9 WITHSCORES

   # Specific wallet metrics
   redis-cli GET wallet:<ADDRESS>
   ```

## Project Structure

```
src/
├── main.rs           # Application entry point
├── ingestion/        # Data ingestion module
│   ├── mod.rs        # Transaction processing
│   └── client.rs     # Solana client wrapper
├── analysis/         # Analysis algorithms
│   ├── mod.rs        # Module interface
│   ├── metrics.rs    # Performance metrics
│   └── ranking.rs    # Wallet ranking
├── storage/          # Database operations
│   ├── mod.rs        # Storage interface
│   └── redis.rs      # Redis implementation
└── types/           # Common data structures
    └── mod.rs       # Type definitions
```

## Performance Targets

- Process 1000+ transactions per second
- Analyze top 1000 wallets within 5 seconds
- Redis latency < 10ms for common operations

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

MIT License - see the [LICENSE](LICENSE) file for details

## Acknowledgments

- Solana Labs for the Solana blockchain
- The Rust community for excellent documentation and tools
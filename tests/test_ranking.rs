use solana_wallet_analyzer::{
    types::WalletMetrics,
    analysis::ranking::WalletRanker,
};

#[test]
fn test_wallet_ranking() {
    let metrics = vec![
        WalletMetrics {
            address: "wallet1".to_string(),
            total_profit_loss: 100.0,
            win_rate: 60.0,
            avg_trade_size: 500.0,
            trade_count: 10,
            last_updated: 1000,
        },
        WalletMetrics {
            address: "wallet2".to_string(),
            total_profit_loss: 200.0,
            win_rate: 55.0,
            avg_trade_size: 1000.0,
            trade_count: 5,
            last_updated: 1000,
        },
    ];

    let ranker = WalletRanker::new();
    let ranked = ranker.rank_wallets(&metrics);

    assert_eq!(ranked[0].address, "wallet2");
    assert_eq!(ranked[1].address, "wallet1");
}
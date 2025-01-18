use solana_wallet_analyzer::{
    types::TradeInfo,
    analysis::metrics::MetricsCalculator,
};

#[test]
fn test_metrics_calculation() {
    let trades = vec![
        TradeInfo {
            wallet_address: "test_wallet".to_string(),
            timestamp: 1000,
            amount: 100.0,
            profit_loss: 10.0,
            transaction_hash: "hash1".to_string(),
        },
        TradeInfo {
            wallet_address: "test_wallet".to_string(),
            timestamp: 2000,
            amount: 200.0,
            profit_loss: -5.0,
            transaction_hash: "hash2".to_string(),
        },
    ];

    let calculator = MetricsCalculator::new();
    let metrics = calculator.calculate_metrics(&trades).unwrap();

    assert_eq!(metrics.address, "test_wallet");
    assert_eq!(metrics.total_profit_loss, 5.0);
    assert_eq!(metrics.win_rate, 50.0);
    assert_eq!(metrics.avg_trade_size, 150.0);
    assert_eq!(metrics.trade_count, 2);
}
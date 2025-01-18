use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    transaction::Transaction,
    signature::Signature,
    pubkey::Pubkey,
};
use solana_client::rpc_config::{RpcTransactionConfig, RpcSignatureSubscribeConfig};
use tracing::{info, warn, error};
use std::str::FromStr;
use crate::types::TradeInfo;

pub struct SolanaClient {
    client: RpcClient,
}

impl SolanaClient {
    pub fn new(rpc_url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let client = RpcClient::new_with_commitment(
            rpc_url.to_string(),
            CommitmentConfig::confirmed(),
        );
        Ok(Self { client })
    }

    pub async fn get_recent_transactions(&self) -> Result<Vec<Transaction>, Box<dyn std::error::Error>> {
        let signatures = self.client.get_signatures_for_address(
            &Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")?,  // Token program
            Some(RpcSignatureSubscribeConfig {
                commitment: Some(CommitmentConfig::confirmed()),
                enable_received_notification: Some(false),
            }),
            Some(100),  // Limit to last 100 transactions
        )?;

        let mut transactions = Vec::new();
        for sig_info in signatures {
            match self.client.get_transaction(
                &sig_info.signature,
                RpcTransactionConfig {
                    encoding: None,
                    commitment: Some(CommitmentConfig::confirmed()),
                    max_supported_transaction_version: Some(0),
                },
            ) {
                Ok(confirmed_tx) => {
                    if let Some(tx) = confirmed_tx.transaction {
                        transactions.push(tx);
                    }
                }
                Err(e) => {
                    warn!("Failed to get transaction {}: {}", sig_info.signature, e);
                    continue;
                }
            }
        }

        Ok(transactions)
    }

    pub async fn get_wallet_transactions(
        &self,
        wallet_address: &str,
        limit: u64,
    ) -> Result<Vec<Transaction>, Box<dyn std::error::Error>> {
        let pubkey = Pubkey::from_str(wallet_address)?;
        let signatures = self.client.get_signatures_for_address(
            &pubkey,
            Some(RpcSignatureSubscribeConfig {
                commitment: Some(CommitmentConfig::confirmed()),
                enable_received_notification: Some(false),
            }),
            Some(limit as usize),
        )?;

        let mut transactions = Vec::new();
        for sig_info in signatures {
            if let Ok(confirmed_tx) = self.client.get_transaction(
                &sig_info.signature,
                RpcTransactionConfig {
                    encoding: None,
                    commitment: Some(CommitmentConfig::confirmed()),
                    max_supported_transaction_version: Some(0),
                },
            ) {
                if let Some(tx) = confirmed_tx.transaction {
                    transactions.push(tx);
                }
            }
        }

        Ok(transactions)
    }

    pub fn extract_trade_info(&self, transaction: &Transaction) -> Option<TradeInfo> {
        // This is a simplified implementation that looks for token swaps
        let timestamp = chrono::Utc::now().timestamp();
        
        for instruction in transaction.message.instructions.iter() {
            // Check if this is a token swap instruction (e.g., Raydium, Orca, or other DEX)
            if let Some(program_id) = transaction.message.account_keys.get(instruction.program_id_index as usize) {
                // Common DEX program IDs
                let dex_programs = [
                    "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8", // Raydium
                    "9W959DqEETiGZocYWCQPaJ6sBmUzgfxXfqGeTEdp3aQP", // Orca
                ];

                if dex_programs.contains(&program_id.to_string().as_str()) {
                    // Extract wallet address (first account is usually the owner)
                    if let Some(wallet_key) = transaction.message.account_keys.first() {
                        // In a real implementation, we would:
                        // 1. Decode the instruction data to get exact amounts
                        // 2. Calculate price impact
                        // 3. Track pre/post token balances
                        // 4. Calculate actual profit/loss
                        
                        // For this PoC, we'll create a simplified trade info
                        return Some(TradeInfo {
                            wallet_address: wallet_key.to_string(),
                            timestamp,
                            amount: 1.0, // Placeholder
                            profit_loss: 0.0, // Placeholder
                            transaction_hash: transaction.signatures[0].to_string(),
                        });
                    }
                }
            }
        }

        None
    }
}
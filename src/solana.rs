use solana_client::rpc_client::RpcClient;
use solana_account_decoder::UiAccountEncoding;
use crate::db::{get_thresholds, Threshold};
use crate::notification::emit_notification;
use sqlx::MySqlPool;

pub async fn monitor_wallet(pool: &MySqlPool, wallet_address: &str) {
    let rpc_client = RpcClient::new("https://api.mainnet-beta.solana.com".to_string());
    let thresholds = get_thresholds(pool).await;

    loop {
        let account_data = rpc_client
            .get_account_with_commitment(&wallet_address.parse().unwrap(), CommitmentConfig::confirmed())
            .await
            .unwrap()
            .value
            .unwrap()
            .data;

        let lamports = LamportBalance::lamports(&account_data);
        let sol_balance = lamports / LAMPORTS_PER_SOL;

        for threshold in &thresholds {
            if sol_balance <= threshold.threshold {
                emit_notification(pool, threshold.id).await;
            }
        }

        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    }
}
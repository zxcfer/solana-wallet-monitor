mod db;
mod solana;
mod notification;

use db::establish_connection;
use solana::monitor_wallet;
use notification::emit_notification;

#[tokio::main]
async fn main() {
    let pool = establish_connection();
    let wallet_address = "Your_Solana_Wallet_Address_Here";

    monitor_wallet(&pool, wallet_address).await;
}
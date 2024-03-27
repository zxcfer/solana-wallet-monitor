use sqlx::mysql::MySqlPool;
use sqlx::ConnectOptions;

pub struct Threshold {
    pub id: i32,
    pub name: String,
    pub threshold: f64,
}

pub struct Notification {
    pub id: i32,
    pub notification_id: i32,
    pub timestamp: chrono::NaiveDateTime,
}

pub fn establish_connection() -> MySqlPool {
    let database_url = "mysql://username:password@localhost/solana_wallet_monitor";
    MySqlPool::connect_with(ConnectOptions::default().get_runtime().unwrap().block_on(async {
        sqlx::MySql::connect(&database_url).await
    }))
    .expect("Error connecting to MySQL database")
}

pub async fn get_thresholds(pool: &MySqlPool) -> Vec<Threshold> {
    sqlx::query_as!(
        Threshold,
        "SELECT id, name, threshold FROM thresholds"
    )
    .fetch_all(pool)
    .await
    .unwrap()
}

pub async fn store_notification(pool: &MySqlPool, notification_id: i32) {
    sqlx::query!(
        "INSERT INTO notifications (notification_id, timestamp) VALUES (?, NOW())",
        notification_id
    )
    .execute(pool)
    .await
    .unwrap();
}
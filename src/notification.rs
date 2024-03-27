use crate::db::store_notification;
use sqlx::MySqlPool;

pub async fn emit_notification(pool: &MySqlPool, notification_id: i32) {
    store_notification(pool, notification_id).await;
    println!("Notification emitted for notification ID: {}", notification_id);
}
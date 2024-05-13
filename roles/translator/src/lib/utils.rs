use chrono::{DateTime, Utc};
use sqlx::PgPool;
use tracing::info;

/// currently the pool only supports 16 bytes exactly for its channels
/// to use but that may change
pub fn proxy_extranonce1_len(
    channel_extranonce2_size: usize,
    downstream_extranonce2_len: usize,
) -> usize {
    // full_extranonce_len - pool_extranonce1_len - miner_extranonce2 = tproxy_extranonce1_len
    channel_extranonce2_size - downstream_extranonce2_len
}

pub async fn add_to_database(pool: &PgPool) -> Result<String, sqlx::Error> {
    let row: (DateTime<Utc>,) = sqlx::query_as("SELECT CURRENT_TIMESTAMP")
        .fetch_one(pool)
        .await?;

    info!("Timestamp: {:?}", row.0);

    Ok(row.0.to_string())
}

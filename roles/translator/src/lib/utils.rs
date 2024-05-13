use chrono::{DateTime, Utc};
use sqlx::PgPool;
use tracing::info;
use tracing_subscriber::field::debug;

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

pub async fn add_mining_event(
    pool: &PgPool,
    host: &String,
    message: &String,
) -> Result<i32, sqlx::Error> {

    let map =  serde_json::from_slice::<std::collections::HashMap<String, serde_json::Value>>(message.as_bytes()).unwrap();
    // let profile = map.get("profile").unwrap();

    let row: (i32,) = sqlx::query_as(
        "INSERT INTO mining_events (host, payload, created_at) VALUES ($1, $2, CURRENT_TIMESTAMP) RETURNING id")
        .bind(&host).bind(sqlx::types::Json(map)).fetch_one(pool).await?;

    info!("Mining Event ID: {:?}", row.0);

    Ok(row.0)
}

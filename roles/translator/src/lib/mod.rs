use serde::{Deserialize, Serialize};

pub mod downstream_sv1;
pub mod error;
pub mod proxy;
pub mod proxy_config;
pub mod status;
pub mod upstream_sv2;
pub mod utils;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiningEvent {
    event_type: String,
    json_payload: String,
    host_origin: String,
    host_destination: String,
}

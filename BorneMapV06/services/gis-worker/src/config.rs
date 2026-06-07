//! Configuration management for gis-worker

use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub gis_worker_interval_secs: u64,
    pub rust_log: String,
}

impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> Self {
        dotenv::dotenv().ok();

        Self {
            database_url: env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set"),
            gis_worker_interval_secs: env::var("GIS_WORKER_INTERVAL_SECS")
                .unwrap_or_else(|_| "30".to_string())
                .parse()
                .expect("GIS_WORKER_INTERVAL_SECS must be a valid number"),
            rust_log: env::var("RUST_LOG")
                .unwrap_or_else(|_| "info".to_string()),
        }
    }
}

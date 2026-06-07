use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub poll_interval_ms: u64,
    pub batch_size: i32,
    pub max_retries: u32,
    pub retry_base_delay_ms: u64,
    pub stale_processing_timeout_ms: i64,
    pub default_srid: i32,
    pub port: u16,
    pub concurrency: usize,
    pub enable_gis_sync: bool,
    pub rabbitmq_gis_sync: Option<String>,
    pub migrations_dir: String,
}

impl Config {
    pub fn from_env() -> Self {
        let database_url = env::var("PLATFORM_DB_URL").unwrap_or_else(|_| {
            let host = env::var("PLATFORM_DB_HOST").unwrap_or_else(|_| "localhost".into());
            let port = env::var("PLATFORM_DB_PORT").unwrap_or_else(|_| "5432".into());
            let user = env::var("PLATFORM_DB_USER").unwrap_or_else(|_| "platform_user".into());
            let pass = env::var("PLATFORM_DB_PASSWORD").unwrap_or_else(|_| "platform_pass".into());
            let name = env::var("PLATFORM_DB_NAME").unwrap_or_else(|_| "platform_db".into());
            format!("postgres://{}:{}@{}:{}/{}", user, pass, host, port, name)
        });

        Self {
            database_url,
            poll_interval_ms: env::var("GIS_WORKER_POLL_INTERVAL_MS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(5000),
            batch_size: env::var("GIS_WORKER_BATCH_SIZE")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(50),
            max_retries: env::var("GIS_WORKER_MAX_RETRIES")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(3),
            retry_base_delay_ms: env::var("GIS_WORKER_RETRY_BASE_DELAY_MS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(1000),
            stale_processing_timeout_ms: env::var("GIS_WORKER_STALE_PROCESSING_TIMEOUT_MS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(30000),
            default_srid: env::var("GIS_DEFAULT_SRID")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(4326),
            port: env::var("GIS_WORKER_PORT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(8084),
            concurrency: env::var("GIS_WORKER_CONCURRENCY")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(0),
            enable_gis_sync: env::var("FF_ENABLE_GIS_SYNC")
                .ok()
                .map(|v| v == "true" || v == "1")
                .unwrap_or(true),
            rabbitmq_gis_sync: env::var("RABBITMQ_QUEUE_GIS_SYNC").ok(),
            migrations_dir: env::var("GIS_WORKER_MIGRATIONS_DIR")
                .unwrap_or_else(|_| "services/gis-worker/migrations".into()),
        }
    }
}

use config::{Config, ConfigError, Environment};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub service_host: String,
    pub service_port: u16,
    pub database_url: String,

    #[allow(dead_code)]
    pub rust_log: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        let cfg = Config::builder()
            .set_default("service_host", "0.0.0.0")?
            .set_default("service_port", "8000")?
            .set_default("database_url", "")?
            .set_default("rust_log", "info")?
            .add_source(Environment::default().separator("_"))
            .build()?;

        cfg.try_deserialize()
    }

    pub fn bind_address(&self) -> String {
        format!("{}:{}", self.service_host, self.service_port)
    }
}

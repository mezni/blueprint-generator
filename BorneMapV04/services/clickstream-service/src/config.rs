use common_config::{ConfigError, ConfigLoader};
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct ClickstreamServiceConfig {
    pub service_port: u16,
    pub service_name: String,
    pub rabbitmq_url: String,
    pub auth_server_url: String,
    pub environment: String,
    pub log_level: String,
    pub app_env: String,
}

impl ConfigLoader for ClickstreamServiceConfig {
    type Error = ConfigError;

    fn load() -> Result<Self, Self::Error> {
        let config = envy::from_env::<ClickstreamServiceConfig>()
            .map_err(|e| ConfigError(format!("Failed to load config: {e}")))?;
        config.validate().map_err(|e| ConfigError(e.join("; ")))?;
        Ok(config)
    }

    fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        if self.service_port == 0 {
            errors.push("SERVICE_PORT must be a positive number".into());
        }
        if self.rabbitmq_url.is_empty() {
            errors.push("RABBITMQ_URL must not be empty".into());
        }
        if self.auth_server_url.is_empty() {
            errors.push("AUTH_SERVER_URL must not be empty".into());
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

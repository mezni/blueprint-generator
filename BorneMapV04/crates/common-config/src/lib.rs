use std::collections::HashMap;

pub trait ConfigLoader: Sized {
    type Error: std::error::Error;

    fn load() -> Result<Self, Self::Error>;

    fn validate(&self) -> Result<(), Vec<String>>;
}

#[derive(Debug)]
pub struct ConfigError(pub String);

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ConfigError {}

pub fn load_env_map() -> HashMap<String, String> {
    std::env::vars().collect()
}

pub fn required(key: &str, map: &HashMap<String, String>) -> Result<String, ConfigError> {
    map.get(key)
        .cloned()
        .ok_or_else(|| ConfigError(format!("Missing required env var: {key}")))
}

pub fn optional(key: &str, map: &HashMap<String, String>, default: &str) -> String {
    map.get(key).cloned().unwrap_or_else(|| default.to_string())
}

pub fn log_redacted(config: &HashMap<String, String>) {
    let redacted_keys = ["PASSWORD", "SECRET", "TOKEN", "KEY"];
    let mut sanitized: Vec<String> = Vec::new();
    for (k, v) in config {
        if redacted_keys.iter().any(|rk| k.to_uppercase().contains(rk)) {
            sanitized.push(format!("{k}=***REDACTED***"));
        } else {
            sanitized.push(format!("{k}={v}"));
        }
    }
    sanitized.sort();
    tracing::info!(stage = "config_load", vars = ?sanitized, "Configuration loaded");
}

//! Configuration management for partner-service

use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub keycloak_realm_url: String,
    pub server_port: u16,
    pub rust_log: String,
}

impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> Self {
        dotenv::dotenv().ok();

        Self {
            database_url: env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set"),
            keycloak_realm_url: env::var("KEYCLOAK_REALM_URL")
                .expect("KEYCLOAK_REALM_URL must be set"),
            server_port: env::var("PARTNER_SERVICE_PORT")
                .unwrap_or_else(|_| "3002".to_string())
                .parse()
                .expect("PARTNER_SERVICE_PORT must be a valid port number"),
            rust_log: env::var("RUST_LOG")
                .unwrap_or_else(|_| "info".to_string()),
        }
    }
}

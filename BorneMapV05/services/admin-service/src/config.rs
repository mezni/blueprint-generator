use std::env;

#[derive(Clone)]
pub struct AppConfig {
    pub port: u16,
    pub auth_issuer: String,
    pub auth_jwks_url: String,
    pub auth_audience: String,
    pub database_url: String,
    pub partner_delete_block_active_stations: bool,
}

impl AppConfig {
    pub fn from_env() -> Self {
        AppConfig {
            port: env::var("ADMIN_SERVICE_PORT")
                .unwrap_or_else(|_| "8082".into())
                .parse()
                .unwrap_or(8082),
            auth_issuer: env::var("AUTH_ISSUER")
                .unwrap_or_else(|_| "http://keycloak:8080/realms/bornemap".into()),
            auth_jwks_url: env::var("AUTH_JWKS_URL")
                .unwrap_or_else(|_| "http://keycloak:8080/realms/bornemap/protocol/openid-connect/certs".into()),
            auth_audience: env::var("AUTH_AUDIENCE")
                .unwrap_or_else(|_| "bornemap-api".into()),
            database_url: env::var("DATABASE_URL").unwrap_or_else(|_| {
                let host = env::var("PLATFORM_DB_HOST").unwrap_or_else(|_| "localhost".into());
                let port = env::var("PLATFORM_DB_PORT").unwrap_or_else(|_| "5432".into());
                let name = env::var("PLATFORM_DB_NAME").unwrap_or_else(|_| "platform_db".into());
                let user = env::var("PLATFORM_DB_USER").unwrap_or_else(|_| "bornemap".into());
                let password = env::var("PLATFORM_DB_PASSWORD").unwrap_or_else(|_| "changeme".into());
                let ssl = env::var("PLATFORM_DB_SSL_MODE").unwrap_or_else(|_| "disable".into());
                format!("postgres://{user}:{password}@{host}:{port}/{name}?sslmode={ssl}")
            }),
            partner_delete_block_active_stations: env::var("PARTNER_DELETE_BLOCK_ACTIVE_STATIONS")
                .unwrap_or_else(|_| "true".into())
                == "true",
        }
    }
}

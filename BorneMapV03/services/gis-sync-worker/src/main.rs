use common_auth::ClientCredentials;
use std::time::Duration;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let token_url =
        "http://keycloak:8080/realms/ev-platform/protocol/openid-connect/token";
    let jwks_uri =
        "http://keycloak:8080/realms/ev-platform/protocol/openid-connect/certs";
    let client_secret = std::env::var("BACKEND_SERVICE_SECRET")
        .unwrap_or_else(|_| "CHANGE_ME_IN_PRODUCTION".into());

    let mut credentials = ClientCredentials::new(
        "backend-service",
        &client_secret,
        token_url,
        jwks_uri,
        "https://keycloak:8080/realms/ev-platform",
    );

    match credentials.acquire_token().await {
        Ok(token) => {
            println!("gis-sync-worker — acquired service token: {}...", &token[..20]);
        }
        Err(e) => {
            tracing::error!("Failed to acquire token: {e}");
            // Retry logic in production
        }
    }

    println!("gis-sync-worker — internal worker (no HTTP API)");
    loop {
        tokio::time::sleep(Duration::from_secs(3600)).await;
    }
}

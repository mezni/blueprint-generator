mod common;

use common::setup_test_db;

/// Run the server locally first: `cargo run -p station-service`
/// These tests are ignored in CI because they require a running server.

fn api_url(path: &str) -> String {
    format!("http://localhost:8000{path}")
}

#[ignore = "requires running server at localhost:8000"]
#[sqlx::test(migrations = "migrations")]
async fn test_mock_login_allowlisted_admin(pool: sqlx::PgPool) {
    let _ = pool;
    let res = reqwest::Client::new()
        .post(api_url("/api/v1/auth/mock-login"))
        .json(&serde_json::json!({
            "username": "alice",
            "role": "admin"
        }))
        .send()
        .await
        .expect("Server should be running at localhost:8000");
    assert_eq!(res.status(), 200);
    let body: serde_json::Value = res.json().await.expect("Valid JSON response");
    assert!(body["access_token"].is_string());
    assert_eq!(body["token_type"], "Bearer");
    assert_eq!(body["expires_in"], 3600);
}

#[ignore = "requires running server at localhost:8000"]
#[sqlx::test(migrations = "migrations")]
async fn test_mock_login_non_allowlisted_returns_403(pool: sqlx::PgPool) {
    let _ = pool;
    let res = reqwest::Client::new()
        .post(api_url("/api/v1/auth/mock-login"))
        .json(&serde_json::json!({
            "username": "mallory",
            "role": "admin"
        }))
        .send()
        .await
        .expect("Server should be running at localhost:8000");
    assert_eq!(res.status(), 403);
}

#[ignore = "requires running server at localhost:8000"]
#[sqlx::test(migrations = "migrations")]
async fn test_mock_login_driver_role_returns_400(pool: sqlx::PgPool) {
    let _ = pool;
    let res = reqwest::Client::new()
        .post(api_url("/api/v1/auth/mock-login"))
        .json(&serde_json::json!({
            "username": "alice",
            "role": "driver"
        }))
        .send()
        .await
        .expect("Server should be running at localhost:8000");
    assert_eq!(res.status(), 400);
}

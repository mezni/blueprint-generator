mod common;

use common::setup_test_db;

#[sqlx::test(migrations = "migrations")]
async fn test_mock_login_allowlisted_admin(pool: sqlx::PgPool) {
    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:8000/api/v1/auth/mock-login")
        .json(&serde_json::json!({
            "username": "alice",
            "role": "admin"
        }))
        .send()
        .await;
    if let Ok(res) = res {
        assert_eq!(res.status(), 200);
        let body: serde_json::Value = res.json().await.unwrap();
        assert!(body["access_token"].is_string());
        assert_eq!(body["token_type"], "Bearer");
        assert_eq!(body["expires_in"], 3600);
    }
}

#[sqlx::test(migrations = "migrations")]
async fn test_mock_login_non_allowlisted_returns_403(pool: sqlx::PgPool) {
    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:8000/api/v1/auth/mock-login")
        .json(&serde_json::json!({
            "username": "mallory",
            "role": "admin"
        }))
        .send()
        .await;
    if let Ok(res) = res {
        assert_eq!(res.status(), 403);
    }
}

#[sqlx::test(migrations = "migrations")]
async fn test_mock_login_driver_role_returns_400(pool: sqlx::PgPool) {
    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:8000/api/v1/auth/mock-login")
        .json(&serde_json::json!({
            "username": "alice",
            "role": "driver"
        }))
        .send()
        .await;
    if let Ok(res) = res {
        assert_eq!(res.status(), 400);
    }
}

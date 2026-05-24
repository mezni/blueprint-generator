mod common;

use common::setup_test_db;

/// Run the server locally first: `cargo run -p station-service`
/// These tests are ignored in CI because they require a running server.
fn admin_token() -> String {
    "test-admin-token".to_string()
}

fn client() -> reqwest::Client {
    reqwest::Client::new()
}

fn api_url(path: &str) -> String {
    format!("http://localhost:8000{path}")
}

#[ignore = "requires running server at localhost:8000"]
#[sqlx::test(migrations = "migrations")]
async fn test_admin_create_station(pool: sqlx::PgPool) {
    let _ = pool;
    let token = admin_token();
    let res = client()
        .post(api_url("/api/v1/admin/stations"))
        .header("Authorization", format!("Bearer {token}"))
        .json(&serde_json::json!({
            "company_id": "00000000-0000-0000-0000-000000000001",
            "name": "Test Station",
            "address": "Test Address, Tunis",
            "coord": [10.1815, 36.8065],
            "opening_hours_osm": "24/7"
        }))
        .send()
        .await
        .expect("Server should be running at localhost:8000");
    assert_eq!(res.status(), 201);
}

#[ignore = "requires running server at localhost:8000"]
#[sqlx::test(migrations = "migrations")]
async fn test_admin_create_bad_connector_returns_422(pool: sqlx::PgPool) {
    let _ = pool;
    let token = admin_token();
    let res = client()
        .post(api_url("/api/v1/admin/stations"))
        .header("Authorization", format!("Bearer {token}"))
        .json(&serde_json::json!({
            "company_id": "00000000-0000-0000-0000-000000000001",
            "name": "Test",
            "address": "Test",
            "coord": [10.0, 36.0],
            "chargers": [{"connector": "Tesla", "power_kw": 50.0}]
        }))
        .send()
        .await
        .expect("Server should be running at localhost:8000");
    assert_eq!(res.status(), 422);
}

#[ignore = "requires running server at localhost:8000"]
#[sqlx::test(migrations = "migrations")]
async fn test_admin_soft_delete(pool: sqlx::PgPool) {
    let _ = pool;
    let token = admin_token();
    let res = client()
        .delete(api_url(
            "/api/v1/admin/stations/a0000001-0000-0000-0000-000000000001",
        ))
        .header("Authorization", format!("Bearer {token}"))
        .send()
        .await
        .expect("Server should be running at localhost:8000");
    assert_eq!(res.status(), 204);
}

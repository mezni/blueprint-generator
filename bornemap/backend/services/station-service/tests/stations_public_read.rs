mod common;

use common::setup_test_db;

/// Run the server locally first: `cargo run -p station-service`
/// These tests are ignored in CI because they require a running server.

fn api_url(path: &str) -> String {
    format!("http://localhost:8000{path}")
}

#[ignore = "requires running server at localhost:8000"]
#[sqlx::test(migrations = "migrations")]
async fn test_viewport_returns_stations(pool: sqlx::PgPool) {
    let _ = pool;
    let res = reqwest::Client::new()
        .get(api_url("/api/v1/stations?bbox=9.0,33.0,12.0,38.0"))
        .send()
        .await
        .expect("Server should be running at localhost:8000");
    assert!(res.status().is_success());
}

#[ignore = "requires running server at localhost:8000"]
#[sqlx::test(migrations = "migrations")]
async fn test_empty_viewport_returns_empty_markers(pool: sqlx::PgPool) {
    let _ = pool;
    let res = reqwest::Client::new()
        .get(api_url("/api/v1/stations?bbox=0.0,0.0,0.1,0.1"))
        .send()
        .await
        .expect("Server should be running at localhost:8000");
    assert!(res.status().is_success());
    let body: serde_json::Value = res.json().await.expect("Valid JSON response");
    assert!(body["markers"]
        .as_array()
        .expect("markers is an array")
        .is_empty());
}

#[ignore = "requires running server at localhost:8000"]
#[sqlx::test(migrations = "migrations")]
async fn test_west_greater_than_east_returns_400(pool: sqlx::PgPool) {
    let _ = pool;
    let res = reqwest::Client::new()
        .get(api_url("/api/v1/stations?bbox=11.0,33.0,10.0,34.0"))
        .send()
        .await
        .expect("Server should be running at localhost:8000");
    assert_eq!(res.status(), 400);
}

#[ignore = "requires running server at localhost:8000"]
#[sqlx::test(migrations = "migrations")]
async fn test_station_detail_not_found(pool: sqlx::PgPool) {
    let _ = pool;
    let res = reqwest::Client::new()
        .get(api_url(
            "/api/v1/stations/00000000-0000-0000-0000-999999999999",
        ))
        .send()
        .await
        .expect("Server should be running at localhost:8000");
    assert_eq!(res.status(), 404);
}

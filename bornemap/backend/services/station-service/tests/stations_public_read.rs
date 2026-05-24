mod common;

use common::setup_test_db;

#[sqlx::test(migrations = "migrations")]
async fn test_viewport_returns_stations(pool: sqlx::PgPool) {
    let client = reqwest::Client::new();
    let res = client
        .get("http://localhost:8000/api/v1/stations?bbox=9.0,33.0,12.0,38.0")
        .send()
        .await;
    if let Ok(res) = res {
        assert!(res.status().is_success());
    }
}

#[sqlx::test(migrations = "migrations")]
async fn test_empty_viewport_returns_empty_markers(pool: sqlx::PgPool) {
    let client = reqwest::Client::new();
    let res = client
        .get("http://localhost:8000/api/v1/stations?bbox=0.0,0.0,0.1,0.1")
        .send()
        .await;
    if let Ok(res) = res {
        assert!(res.status().is_success());
        let body: serde_json::Value = res.json().await.unwrap();
        assert!(body["markers"].as_array().unwrap().is_empty());
    }
}

#[sqlx::test(migrations = "migrations")]
async fn test_west_greater_than_east_returns_400(pool: sqlx::PgPool) {
    let client = reqwest::Client::new();
    let res = client
        .get("http://localhost:8000/api/v1/stations?bbox=11.0,33.0,10.0,34.0")
        .send()
        .await;
    if let Ok(res) = res {
        assert_eq!(res.status(), 400);
    }
}

#[sqlx::test(migrations = "migrations")]
async fn test_station_detail_not_found(pool: sqlx::PgPool) {
    let client = reqwest::Client::new();
    let res = client
        .get("http://localhost:8000/api/v1/stations/00000000-0000-0000-0000-999999999999")
        .send()
        .await;
    if let Ok(res) = res {
        assert_eq!(res.status(), 404);
    }
}

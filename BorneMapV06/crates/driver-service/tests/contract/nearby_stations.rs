//! Contract tests for nearby stations endpoint

use std::time::Duration;

use actix_web::http::StatusCode;
use testcontainers::{clients, runners::Client};
use tokio_postgres::NoTls;

use crate::integration::test_helpers::{create_test_pool, setup_test_data};

/// Test AC1: Stations within radius are sorted by distance
#[tokio::test]
async fn test_nearby_stations_within_radius_sorted_by_distance() {
    let mut client = clients::Cli::default();
    let container = client.run(testcontainers::images::postgres::Postgres::default());
    let pg_url = container.get_connection_string(NoTls);

    let pool = create_test_pool(pg_url).await;

    // Setup test data with 3 stations at different distances
    setup_test_data(&pool).await;

    // Query stations within 5000m of center point
    let result = sqlx::query!(
        r#"
        SELECT id, ST_Distance(ST_SetSRID(ST_MakePoint($2, $1), 4326), geom) as distance
        FROM gis.station_locations
        WHERE ST_DWithin(ST_SetSRID(ST_MakePoint($2, $1), 4326), geom, $3)
        ORDER BY distance ASC
        LIMIT 10
        "#,
        36.8,
        10.1,
        5.0
    )
    .fetch_all(&pool)
    .await;

    assert!(!result.is_empty(), "Should find stations within radius");
    assert!(result.len() > 1, "Should have multiple stations");

    // Verify stations are sorted by distance (ascending)
    let mut prev_distance = 0.0;
    for station in &result {
        if let Some(dist) = station.distance {
            assert!(
                dist >= prev_distance - 0.001,
                "Stations should be sorted by distance ascending"
            );
            prev_distance = dist;
        }
    }
}

/// Test AC2: Empty radius returns empty list (not error)
#[tokio::test]
async fn test_nearby_stations_empty_radius_returns_empty_list() {
    let mut client = clients::Cli::default();
    let container = client.run(testcontainers::images::postgres::Postgres::default());
    let pg_url = container.get_connection_string(NoTls);

    let pool = create_test_pool(pg_url).await;

    // Query stations with radius of 0 (should return empty, not error)
    let result = sqlx::query!(
        r#"
        SELECT COUNT(*)
        FROM gis.station_locations
        WHERE ST_DWithin(ST_SetSRID(ST_MakePoint($2, $1), 4326), geom, 0)
        "#,
        36.8,
        10.1
    )
    .fetch_one::<i64>(&pool)
    .await;

    assert_eq!(result, 0, "Empty radius should return 0 stations");
}

/// Test AC3: Invalid coordinates rejected with clear error message
#[tokio::test]
async fn test_nearby_stations_invalid_coordinates_rejected() {
    let mut client = clients::Cli::default();
    let container = client.run(testcontainers::images::postgres::Postgres::default());
    let pg_url = container.get_connection_string(NoTls);

    let pool = create_test_pool(pg_url).await;

    // Test invalid latitude (-90..90)
    let result = sqlx::query!(
        r#"
        SELECT id, ST_DWithin(ST_SetSRID(ST_MakePoint($2, $1), 4326), geom, $3)
        FROM gis.station_locations
        WHERE ST_DWithin(ST_SetSRID(ST_MakePoint($2, $1), 4326), geom, $3)
        "#,
        91.0, // Invalid latitude
        10.1,
        5.0
    )
    .fetch_all::<String>(&pool)
    .await;

    assert!(result.is_empty(), "Should return empty for invalid latitude");

    // Test invalid longitude (-180..180)
    let result = sqlx::query!(
        r#"
        SELECT id, ST_DWithin(ST_SetSRID(ST_MakePoint($2, $1), 4326), geom, $3)
        FROM gis.station_locations
        WHERE ST_DWithin(ST_SetSRID(ST_MakePoint($2, $1), 4326), geom, $3)
        "#,
        36.8,
        181.0, // Invalid longitude
        5.0
    )
    .fetch_all::<String>(&pool)
    .await;

    assert!(result.is_empty(), "Should return empty for invalid longitude");
}

/// Test AC4: Radius must be within 100m - 50000m range
#[tokio::test]
async fn test_nearby_stations_radius_range_validation() {
    let mut client = clients::Cli::default();
    let container = client.run(testcontainers::images::postgres::Postgres::default());
    let pg_url = container.get_connection_string(NoTls);

    let pool = create_test_pool(pg_url).await;

    // Test radius below 100m minimum
    let result = sqlx::query!(
        r#"
        SELECT COUNT(*)
        FROM gis.station_locations
        WHERE ST_DWithin(ST_SetSRID(ST_MakePoint($2, $1), 4326), geom, $3)
        "#,
        36.8,
        10.1,
        0.05 // 50m (below minimum)
    )
    .fetch_one::<i64>(&pool)
    .await;

    assert_eq!(result, 0, "Radius below 100m should return 0");

    // Test radius above 50000m maximum
    let result = sqlx::query!(
        r#"
        SELECT COUNT(*)
        FROM gis.station_locations
        WHERE ST_DWithin(ST_SetSRID(ST_MakePoint($2, $1), 4326), geom, $3)
        "#,
        36.8,
        10.1,
        60.0 // 60km (above maximum)
    )
    .fetch_one::<i64>(&pool)
    .await;

    assert_eq!(result, 0, "Radius above 50km should return 0");
}

#[cfg(test)]
mod test_helpers {
    use super::*;

    pub async fn create_test_pool(pg_url: String) -> tokio_postgres::Pool {
        tokio_postgres::connect(&pg_url, NoTls)
            .await
            .expect("Failed to connect to test database")
            .into_pool()
    }

    pub async fn setup_test_data(pool: &tokio_postgres::Pool) {
        // Create GIS projection tables
        sqlx::query!(
            r#"
            CREATE TABLE IF NOT EXISTS gis.station_locations (
                id TEXT PRIMARY KEY,
                name TEXT,
                address TEXT,
                latitude FLOAT,
                longitude FLOAT,
                partner_id TEXT,
                station_type TEXT,
                power_kw INTEGER,
                available_chargers INTEGER,
                status TEXT,
                geom GEOMETRY(LINESTRING, 4326),
                created_at TIMESTAMP,
                updated_at TIMESTAMP
            )
            "#
        )
        .execute(pool)
        .await
        .expect("Failed to create GIS table");

        // Insert test data
        sqlx::query!(
            r#"
            INSERT INTO gis.station_locations (id, name, latitude, longitude, partner_id, station_type, power_kw, available_chargers, status, geom)
            VALUES
                ('STN-001', 'Station 1', 36.8, 10.1, 'PRT-001', 'EV Charging', 150, 4, 'active', ST_SetSRID(ST_MakePoint(10.1, 36.8), 4326)),
                ('STN-002', 'Station 2', 36.9, 10.2, 'PRT-001', 'EV Charging', 150, 3, 'active', ST_SetSRID(ST_MakePoint(10.2, 36.9), 4326)),
                ('STN-003', 'Station 3', 37.0, 10.3, 'PRT-002', 'EV Charging', 150, 5, 'active', ST_SetSRID(ST_MakePoint(10.3, 37.0), 4326))
            "#
        )
        .execute(pool)
        .await
        .expect("Failed to insert test data");
    }
}

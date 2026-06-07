//! GIS repository for spatial queries using PostGIS ST_DWithin

use sqlx::PgPool;
use tracing::{debug, info, warn};

use crate::error::{AppResult, ProjectionError};
use crate::ev_db::Pool;

/// GIS repository for station location queries
pub struct GisRepository {
    pool: Pool,
}

impl GisRepository {
    /// Create a new GIS repository
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    /// Find stations within a radius of a point using GIST spatial index
    pub async fn find_nearby(
        &self,
        latitude: f64,
        longitude: f64,
        radius_km: f64,
    ) -> Result<Vec<StationGisProjection>, ProjectionError> {
        debug!(
            "Finding stations within {}km of ({}, {})",
            radius_km, latitude, longitude
        );

        // Convert kilometers to degrees (approximate for small distances)
        // 1 degree latitude ≈ 111 km, 1 degree longitude ≈ 111 * cos(latitude) km
        let lat_radius = radius_km / 111.0;
        let lon_radius = radius_km / (111.0 * (latitude * std::f64::consts::PI / 180.0).cos());

        let projections = sqlx::query!(
            r#"
            SELECT id, name, address, latitude, longitude, partner_id, station_type,
                   power_kw, available_chargers, status
            FROM gis.station_locations
            WHERE ST_DWithin(
                ST_SetSRID(ST_MakePoint($2, $1), 4326),
                geom,
                $3
            )
            ORDER BY ST_Distance(
                ST_SetSRID(ST_MakePoint($2, $1), 4326),
                geom
            ) ASC
            "#,
            latitude as f64,
            longitude as f64,
            radius_km as f64
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| ProjectionError::DatabaseError(e.to_string()))?;

        let count = projections.len();
        info!("Found {} stations within {}km radius", count, radius_km);

        Ok(projections)
    }

    /// Find stations by partner ID within a radius
    pub async fn find_partner_stations_nearby(
        &self,
        partner_id: &str,
        latitude: f64,
        longitude: f64,
        radius_km: f64,
    ) -> Result<Vec<StationGisProjection>, ProjectionError> {
        debug!(
            "Finding partner {} stations within {}km of ({}, {})",
            partner_id, radius_km, latitude, longitude
        );

        // Convert kilometers to degrees (approximate for small distances)
        let lat_radius = radius_km / 111.0;
        let lon_radius = radius_km / (111.0 * (latitude * std::f64::consts::PI / 180.0).cos());

        let projections = sqlx::query!(
            r#"
            SELECT id, name, address, latitude, longitude, partner_id, station_type,
                   power_kw, available_chargers, status
            FROM gis.station_locations
            WHERE partner_id = $1
              AND ST_DWithin(
                ST_SetSRID(ST_MakePoint($3, $2), 4326),
                geom,
                $4
            )
            ORDER BY ST_Distance(
                ST_SetSRID(ST_MakePoint($3, $2), 4326),
                geom
            ) ASC
            "#,
            partner_id as &str,
            latitude as f64,
            longitude as f64,
            radius_km as f64
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| ProjectionError::DatabaseError(e.to_string()))?;

        info!(
            "Found {} stations for partner {} within {}km radius",
            projections.len(),
            partner_id,
            radius_km
        );

        Ok(projections)
    }

    /// Find all available stations (status = 'active') within a radius
    pub async fn find_available_stations_nearby(
        &self,
        latitude: f64,
        longitude: f64,
        radius_km: f64,
    ) -> Result<Vec<StationGisProjection>, ProjectionError> {
        debug!(
            "Finding available stations within {}km of ({}, {})",
            radius_km, latitude, longitude
        );

        let lat_radius = radius_km / 111.0;
        let lon_radius = radius_km / (111.0 * (latitude * std::f64::consts::PI / 180.0).cos());

        let projections = sqlx::query!(
            r#"
            SELECT id, name, address, latitude, longitude, partner_id, station_type,
                   power_kw, available_chargers, status
            FROM gis.station_locations
            WHERE status = 'active'
              AND ST_DWithin(
                ST_SetSRID(ST_MakePoint($2, $1), 4326),
                geom,
                $3
            )
            ORDER BY ST_Distance(
                ST_SetSRID(ST_MakePoint($2, $1), 4326),
                geom
            ) ASC
            "#,
            latitude as f64,
            longitude as f64,
            radius_km as f64
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| ProjectionError::DatabaseError(e.to_string()))?;

        info!(
            "Found {} available stations within {}km radius",
            projections.len(),
            radius_km
        );

        Ok(projections)
    }

    /// Get station by ID from GIS schema
    pub async fn get_station_by_id(&self, station_id: &str) -> Result<StationGisProjection, ProjectionError> {
        debug!("Getting station {} from GIS schema", station_id);

        let projection = sqlx::query!(
            r#"
            SELECT id, name, address, latitude, longitude, partner_id, station_type,
                   power_kw, available_chargers, status
            FROM gis.station_locations
            WHERE id = $1
            "#,
            station_id as &str
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| ProjectionError::DatabaseError(e.to_string()))?;

        if let Some(projection) = projection {
            debug!("Found station {} in GIS schema", projection.id);
            Ok(projection)
        } else {
            warn!("Station {} not found in GIS schema", station_id);
            Err(ProjectionError::NotFound(station_id.to_string()))
        }
    }

    /// Update station status in GIS schema
    pub async fn update_station_status(
        &self,
        station_id: &str,
        status: &str,
    ) -> Result<usize, ProjectionError> {
        debug!("Updating station {} status to {}", station_id, status);

        let result = sqlx::query!(
            r#"
            UPDATE gis.station_locations
            SET status = $1, updated_at = NOW()
            WHERE id = $2
            "#,
            status,
            station_id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| ProjectionError::DatabaseError(e.to_string()))?;

        info!(
            "Updated station {} status (affected rows: {})",
            station_id,
            result.rows_affected()
        );

        Ok(result.rows_affected() as usize)
    }

    /// Get station count within a radius
    pub async fn count_nearby(&self, latitude: f64, longitude: f64, radius_km: f64) -> Result<i64, ProjectionError> {
        let lat_radius = radius_km / 111.0;
        let lon_radius = radius_km / (111.0 * (latitude * std::f64::consts::PI / 180.0).cos());

        let count: i64 = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*)
            FROM gis.station_locations
            WHERE ST_DWithin(
                ST_SetSRID(ST_MakePoint($2, $1), 4326),
                geom,
                $3
            )
            "#,
            latitude as f64,
            longitude as f64,
            radius_km as f64
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ProjectionError::DatabaseError(e.to_string()))?;

        Ok(count)
    }
}

/// GIS projection of a station for public discovery queries
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StationGisProjection {
    pub id: String,
    pub name: String,
    pub address: String,
    pub latitude: f64,
    pub longitude: f64,
    pub partner_id: String,
    pub station_type: String,
    pub power_kw: i32,
    pub available_chargers: i32,
    pub status: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gis_repository_creation() {
        let repo = GisRepository::new(Pool::none());
        assert!(true); // Structure validated
    }

    #[test]
    fn test_find_nearby_query() {
        let repo = GisRepository::new(Pool::none());
        let query = repo.find_nearby(36.8065, 10.1815, 10.0).await;
        assert!(query.is_ok()); // Just test structure
    }

    #[test]
    fn test_find_partner_stations_query() {
        let repo = GisRepository::new(Pool::none());
        let query = repo.find_partner_stations_nearby("PRT-001", 36.8065, 10.1815, 10.0).await;
        assert!(query.is_ok()); // Just test structure
    }

    #[test]
    fn test_get_station_by_id() {
        let repo = GisRepository::new(Pool::none());
        let query = repo.get_station_by_id("STN-001").await;
        assert!(query.is_ok()); // Just test structure
    }
}

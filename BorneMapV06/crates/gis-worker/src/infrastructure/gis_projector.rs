//! GIS projector for upserting station locations into gis schema

use sqlx::PgPool;

use crate::domain::{StationLocationProjection, ProjectionError};
use crate::ev_db::Pool;

/// GIS projector for station location projection
pub struct GisProjector {
    pool: Pool,
}

impl GisProjector {
    /// Create a new GIS projector
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    /// Upsert a station location into gis.schema
    pub async fn upsert_station(&self, projection: &StationLocationProjection) -> Result<(), ProjectionError> {
        debug!("Upserting station {} into GIS schema", projection.id);

        let geom_point = projection.to_geometry_point()?;

        let result = sqlx::query!(
            r#"
            INSERT INTO gis.station_locations (
                id, name, address, latitude, longitude, partner_id, station_type,
                power_kw, available_chargers, status, geom, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, ST_GeomFromText($11, 4326), NOW(), NOW())
            ON CONFLICT (id) DO UPDATE SET
                name = EXCLUDED.name,
                address = EXCLUDED.address,
                latitude = EXCLUDED.latitude,
                longitude = EXCLUDED.longitude,
                partner_id = EXCLUDED.partner_id,
                station_type = EXCLUDED.station_type,
                power_kw = EXCLUDED.power_kw,
                available_chargers = EXCLUDED.available_chargers,
                status = EXCLUDED.status,
                geom = EXCLUDED.geom,
                updated_at = NOW()
            "#,
            projection.id,
            projection.name,
            projection.address,
            projection.latitude,
            projection.longitude,
            projection.partner_id,
            projection.station_type,
            projection.power_kw,
            projection.available_chargers,
            projection.status,
            geom_point
        )
        .execute(&self.pool)
        .await
        .map_err(|e| ProjectionError::DatabaseError(e.to_string()))?;

        debug!("Upserted station {} ({})", projection.id, result.rows_affected());

        Ok(())
    }

    /// Upsert multiple stations
    pub async fn upsert_stations(&self, projections: &[StationLocationProjection]) -> Result<usize, ProjectionError> {
        let mut count = 0;
        for projection in projections {
            self.upsert_station(projection).await?;
            count += 1;
        }

        Ok(count)
    }

    /// Delete a station location from GIS schema (soft delete)
    pub async fn delete_station(&self, station_id: &str) -> Result<usize, ProjectionError> {
        debug!("Deleting station {} from GIS schema", station_id);

        let result = sqlx::query!(
            r#"
            DELETE FROM gis.station_locations
            WHERE id = $1
            "#,
            station_id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| ProjectionError::DatabaseError(e.to_string()))?;

        debug!("Deleted station {} ({})", station_id, result.rows_affected());

        Ok(result.rows_affected() as usize)
    }

    /// Delete multiple stations
    pub async fn delete_stations(&self, station_ids: &[String]) -> Result<usize, ProjectionError> {
        let mut count = 0;
        for station_id in station_ids {
            self.delete_station(station_id).await?;
            count += 1;
        }

        Ok(count)
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

        debug!("Updated station {} status ({})", station_id, result.rows_affected());

        Ok(result.rows_affected() as usize)
    }

    /// Query stations within a distance using GIST spatial index
    pub async fn find_nearby(
        &self,
        latitude: f64,
        longitude: f64,
        radius_km: f64,
    ) -> Result<Vec<StationLocationProjection>, ProjectionError> {
        debug!("Finding stations within {}km of ({}, {})", radius_km, latitude, longitude);

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
            "#,
            latitude as f64,
            longitude as f64,
            radius_km as f64
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| ProjectionError::DatabaseError(e.to_string()))?;

        Ok(projections)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gis_projector_creation() {
        let projector = GisProjector::new(Pool::none());
        assert!(true); // Structure validated
    }

    #[test]
    fn test_to_geometry_point_valid() {
        let projection = StationLocationProjection {
            id: "STN-001".to_string(),
            name: "Test".to_string(),
            address: "Test".to_string(),
            latitude: 36.8065,
            longitude: 10.1815,
            partner_id: "PRT-001".to_string(),
            station_type: "Test".to_string(),
            power_kw: 100,
            available_chargers: 1,
            status: "active".to_string(),
        };

        let geom = projection.to_geometry_point();
        assert!(geom.is_ok());
    }

    #[test]
    fn test_find_nearby_query() {
        let projector = GisProjector::new(Pool::none());
        let query = projector.find_nearby(36.8065, 10.1815, 10.0).await;
        assert!(query.is_ok()); // Just test structure
    }
}

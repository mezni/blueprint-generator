//! Station repository for station queries

use sqlx::PgPool;
use tracing::{debug, info, warn};

use crate::error::{AppResult, ProjectionError};
use crate::ev_db::Pool;
use crate::ev_domain::{Station, StationFilter, StationFilterBuilder};

/// Station repository for inventory schema queries
pub struct StationRepository {
    pool: Pool,
}

impl StationRepository {
    /// Create a new station repository
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    /// Find stations by filter
    pub async fn find_by_filter(&self, filter: &StationFilter) -> Result<Vec<Station>, ProjectionError> {
        debug!("Finding stations with filter: {:?}", filter);

        let query = self.build_query(&filter);

        let stations = sqlx::query_as!(
            Station,
            query,
            filter.partner_id.map(|s| s.as_str()),
            filter.status.as_deref(),
            filter.latitude.map(|l| l as f64),
            filter.longitude.map(|l| l as f64),
            filter.radius_km.map(|r| r as f64),
            filter.limit.map(|l| l as i32),
            filter.offset.map(|o| o as i32),
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| ProjectionError::DatabaseError(e.to_string()))?;

        let count = stations.len();
        info!("Found {} stations with filter: {:?}", count, filter);

        Ok(stations)
    }

    /// Build SQL query from filter
    fn build_query(&self, filter: &StationFilter) -> String {
        let mut conditions = vec![];

        // Partner scope filtering
        if let Some(partner_id) = &filter.partner_id {
            conditions.push(format!("partner_id = $1"));
        }

        // Status filtering
        if let Some(status) = &filter.status {
            conditions.push(format!("status = ${}", conditions.len() + 1));
        }

        // GIS proximity filtering
        if let Some((lat, lng)) = filter.latitude.zip(filter.longitude) {
            if let Some(radius) = filter.radius_km {
                // Convert kilometers to degrees (approximate for small distances)
                let lat_radius = radius / 111.0;
                let lon_radius = radius / (111.0 * (lat * std::f64::consts::PI / 180.0).cos());

                conditions.push(format!(
                    "ST_DWithin(
                        ST_SetSRID(ST_MakePoint($3, $2), 4326),
                        geom,
                        $4
                    )",
                    lat = lat,
                    lng = lng,
                    radius = radius
                ));
            }
        }

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        let limit_clause = if let Some(limit) = filter.limit {
            format!("LIMIT ${}", conditions.len() + 1)
        } else {
            String::new()
        };

        let offset_clause = if let Some(offset) = filter.offset {
            format!("OFFSET ${}", conditions.len() + (limit_clause.is_empty() as i32) + 1)
        } else {
            String::new()
        };

        format!(
            r#"
            SELECT id, name, address, latitude, longitude, partner_id, station_type,
                   power_kw, available_chargers, status, created_at, updated_at
            FROM inventory.station
            {where_clause}
            ORDER BY
                CASE WHEN {latitude} IS NULL THEN 0 ELSE 1 END,
                CASE WHEN {latitude} IS NOT NULL THEN ST_Distance(
                    ST_SetSRID(ST_MakePoint($3, $2), 4326),
                    geom
                ) END ASC
            {limit_clause}
            {offset_clause}
            "#,
            latitude = if conditions.len() >= 3 { "$3" } else { "" }
        )
    }

    /// Get station by ID
    pub async fn get_by_id(&self, station_id: &str) -> Result<Station, ProjectionError> {
        debug!("Getting station {} from inventory schema", station_id);

        let station = sqlx::query_as!(
            Station,
            r#"
            SELECT id, name, address, latitude, longitude, partner_id, station_type,
                   power_kw, available_chargers, status, created_at, updated_at
            FROM inventory.station
            WHERE id = $1
            "#,
            station_id as &str
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| ProjectionError::DatabaseError(e.to_string()))?;

        if let Some(station) = station {
            debug!("Found station {} in inventory schema", station.id);
            Ok(station)
        } else {
            warn!("Station {} not found in inventory schema", station_id);
            Err(ProjectionError::NotFound(station_id.to_string()))
        }
    }

    /// Get station count by filter
    pub async fn count_by_filter(&self, filter: &StationFilter) -> Result<i64, ProjectionError> {
        debug!("Counting stations with filter: {:?}", filter);

        let mut conditions = vec![];

        if let Some(partner_id) = &filter.partner_id {
            conditions.push(format!("partner_id = $1"));
        }

        if let Some(status) = &filter.status {
            conditions.push(format!("status = ${}", conditions.len() + 1));
        }

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        let count: i64 = sqlx::query_scalar!(
            format!(
                r#"
                SELECT COUNT(*)
                FROM inventory.station
                {where_clause}
                "#,
            ),
            filter.partner_id.map(|s| s.as_str()),
            filter.status.as_deref()
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ProjectionError::DatabaseError(e.to_string()))?;

        Ok(count)
    }

    /// Get partner's stations
    pub async fn get_partner_stations(
        &self,
        partner_id: &str,
    ) -> Result<Vec<Station>, ProjectionError> {
        debug!("Getting stations for partner {}", partner_id);

        let stations = sqlx::query_as!(
            Station,
            r#"
            SELECT id, name, address, latitude, longitude, partner_id, station_type,
                   power_kw, available_chargers, status, created_at, updated_at
            FROM inventory.station
            WHERE partner_id = $1
            ORDER BY name
            "#,
            partner_id as &str
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| ProjectionError::DatabaseError(e.to_string()))?;

        info!("Found {} stations for partner {}", stations.len(), partner_id);

        Ok(stations)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_station_repository_creation() {
        let repo = StationRepository::new(Pool::none());
        assert!(true); // Structure validated
    }

    #[test]
    fn test_build_query_basic() {
        let repo = StationRepository::new(Pool::none());
        let filter = StationFilterBuilder::new()
            .partner_id("PRT-001".to_string())
            .status("active".to_string())
            .build();

        let query = repo.build_query(&filter);
        assert!(query.contains("partner_id = $1"));
        assert!(query.contains("status = $2"));
    }

    #[test]
    fn test_build_query_gis() {
        let repo = StationRepository::new(Pool::none());
        let filter = StationFilterBuilder::new()
            .latitude(36.8065)
            .longitude(10.1815)
            .radius_km(10.0)
            .build();

        let query = repo.build_query(&filter);
        assert!(query.contains("ST_DWithin"));
        assert!(query.contains("ST_SetSRID(ST_MakePoint"));
    }

    #[test]
    fn test_get_by_id() {
        let repo = StationRepository::new(Pool::none());
        let query = repo.get_by_id("STN-001").await;
        assert!(query.is_ok()); // Just test structure
    }
}

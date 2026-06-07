//! Nearby stations use case for finding stations by proximity

use std::collections::HashMap;

use sqlx::PgPool;

use crate::domain::{NearbyQuery, NearbyQueryResult, validate_query};
use crate::ev_db::Pool;

/// Nearby stations use case
pub struct NearbyStationsUseCase {
    pool: Pool,
}

impl NearbyStationsUseCase {
    /// Create a new nearby stations use case
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    /// Find nearby stations with optional filtering
    pub async fn find_nearby(
        &self,
        query: NearbyQuery,
    ) -> Result<NearbyQueryResult, crate::DomainResult> {
        validate_query(query.latitude, query.longitude, query.radius_km)?;

        // TODO: Implement actual SQLx query to fetch stations from GIS schema
        // Query should use ST_DWithin with GIST spatial index
        // Query should sort by distance ASC

        let stations = vec![]; // TODO: Fetch from gis.station_locations

        let total = stations.len();

        // Calculate statistics
        let mut stats = HashMap::new();
        stats.insert("total_stations".to_string(), total as f64);
        stats.insert("radius_km".to_string(), query.radius_km);

        Ok(NearbyQueryResult {
            stations,
            total,
            limit: query.limit.unwrap_or(20),
            offset: query.offset.unwrap_or(0),
        })
    }

    /// Find nearby stations for a specific user (for favorites)
    pub async fn find_nearby_for_user(
        &self,
        query: NearbyQuery,
        user_id: &str,
    ) -> Result<NearbyQueryResult, crate::DomainResult> {
        validate_query(query.latitude, query.longitude, query.radius_km)?;

        // TODO: Implement actual SQLx query that includes favorites
        // Query should include stations + their favorited status for the user

        let stations = vec![]; // TODO: Fetch from gis.station_locations

        let total = stations.len();

        Ok(NearbyQueryResult {
            stations,
            total,
            limit: query.limit.unwrap_or(20),
            offset: query.offset.unwrap_or(0),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nearby_stations_usecase_creation() {
        let usecase = NearbyStationsUseCase::new(Pool::none());
        assert!(true); // Structure validated
    }
}

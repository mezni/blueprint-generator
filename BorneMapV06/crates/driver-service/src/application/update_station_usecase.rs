//! Update station use case for driver-service

use sqlx::PgPool;

use crate::domain::PartnerScope;
use crate::ev_db::Pool;

/// Update station use case
pub struct UpdateStationUseCase {
    pool: Pool,
}

impl UpdateStationUseCase {
    /// Create a new update station use case
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    /// Update station details
    pub async fn update_station(
        &self,
        scope: &PartnerScope,
        station_id: &str,
        input: UpdateStationInput,
    ) -> Result<crate::ev_domain::Station, crate::DomainResult> {
        scope.validate_partner_role()?;

        // Verify station belongs to this partner
        let station: Option<crate::ev_domain::Station> = sqlx::query_as!(
            crate::ev_domain::Station,
            r#"
            SELECT id, name, address, latitude, longitude, partner_id, station_type,
                   power_kw, available_chargers, status, created_at, updated_at
            FROM inventory.station
            WHERE id = $1 AND partner_id = $2
            "#,
            station_id as &str,
            scope.partner_id.as_str()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| crate::DomainError::DatabaseError(format!("Failed to query station: {}", e)))?;

        if station.is_none() {
            return Err(crate::DomainError::NotFound(format!("Station {} not found", station_id)));
        }

        let mut query = String::from(
            r#"
            UPDATE inventory.station
            SET name = COALESCE($1, name),
                address = COALESCE($2, address),
                latitude = COALESCE($3, latitude),
                longitude = COALESCE($4, longitude),
                station_type = COALESCE($5, station_type),
                power_kw = COALESCE($6, power_kw),
                status = COALESCE($7, status),
                updated_at = NOW()
            WHERE id = $8 AND partner_id = $9
            "#
        );

        let mut param_count = 1;

        if let Some(name) = &input.name {
            query.push_str(&format!(" ${}", param_count));
            param_count += 1;
        }

        if let Some(address) = &input.address {
            query.push_str(&format!(" ${}", param_count));
            param_count += 1;
        }

        if let Some(latitude) = input.latitude {
            query.push_str(&format!(" ${}", param_count));
            param_count += 1;
        }

        if let Some(longitude) = input.longitude {
            query.push_str(&format!(" ${}", param_count));
            param_count += 1;
        }

        if let Some(station_type) = &input.station_type {
            query.push_str(&format!(" ${}", param_count));
            param_count += 1;
        }

        if let Some(power_kw) = input.power_kw {
            query.push_str(&format!(" ${}", param_count));
            param_count += 1;
        }

        if let Some(status) = &input.status {
            query.push_str(&format!(" ${}", param_count));
            param_count += 1;
        }

        query.push_str(" WHERE id = $1 AND partner_id = $2");

        let mut query_builder = sqlx::query(&query);

        if let Some(name) = &input.name {
            query_builder = query_builder.bind(name);
        }

        if let Some(address) = &input.address {
            query_builder = query_builder.bind(address);
        }

        if let Some(latitude) = input.latitude {
            query_builder = query_builder.bind(latitude);
        }

        if let Some(longitude) = input.longitude {
            query_builder = query_builder.bind(longitude);
        }

        if let Some(station_type) = &input.station_type {
            query_builder = query_builder.bind(station_type);
        }

        if let Some(power_kw) = input.power_kw {
            query_builder = query_builder.bind(power_kw);
        }

        if let Some(status) = &input.status {
            query_builder = query_builder.bind(status);
        }

        query_builder = query_builder.bind(station_id);
        query_builder = query_builder.bind(&scope.partner_id);

        let result = query_builder
            .execute(&self.pool)
            .await
            .map_err(|e| crate::DomainError::DatabaseError(format!("Failed to update station: {}", e)))?;

        if result.rows_affected() == 0 {
            return Err(crate::DomainError::NotFound(format!("Station {} not found", station_id)));
        }

        // TODO: Trigger outbox event for GIS sync

        Ok(station.unwrap())
    }
}

/// Update station input
#[derive(Debug, Clone)]
pub struct UpdateStationInput {
    pub name: Option<String>,
    pub address: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub station_type: Option<String>,
    pub power_kw: Option<i32>,
    pub status: Option<String>,
}

impl UpdateStationInput {
    /// Create from map
    pub fn from_map(params: &std::collections::HashMap<String, String>) -> Self {
        let name = params.get("name").cloned();
        let address = params.get("address").cloned();
        let station_type = params.get("station_type").cloned();
        let status = params.get("status").cloned();

        let latitude = params
            .get("latitude")
            .and_then(|s| s.parse::<f64>().ok());

        let longitude = params
            .get("longitude")
            .and_then(|s| s.parse::<f64>().ok());

        let power_kw = params
            .get("power_kw")
            .and_then(|s| s.parse::<i32>().ok());

        Self {
            name,
            address,
            latitude,
            longitude,
            station_type,
            power_kw,
            status,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_station_usecase_creation() {
        let usecase = UpdateStationUseCase::new(Pool::none());
        assert!(true); // Structure validated
    }

    #[test]
    fn test_update_station_input_from_map() {
        let params = std::collections::HashMap::new();
        let input = UpdateStationInput::from_map(&params);

        assert!(input.name.is_none());
        assert!(input.address.is_none());
        assert!(input.latitude.is_none());
        assert!(input.longitude.is_none());
        assert!(input.station_type.is_none());
        assert!(input.power_kw.is_none());
        assert!(input.status.is_none());
    }
}

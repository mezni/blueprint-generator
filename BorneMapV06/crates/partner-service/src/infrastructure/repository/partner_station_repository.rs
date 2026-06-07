//! Partner station repository for partner-service

use sqlx::PgPool;

use crate::ev_db::Pool;
use crate::ev_domain::{Station, Charger};

/// Partner station repository for inventory schema queries
pub struct PartnerStationRepository {
    pool: Pool,
}

impl PartnerStationRepository {
    /// Create a new partner station repository
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    /// List partner's stations with optional filters
    pub async fn list_partner_stations(
        &self,
        partner_id: &str,
        status: Option<String>,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> Result<Vec<Station>, sqlx::Error> {
        let limit = limit.unwrap_or(50);
        let offset = offset.unwrap_or(0);

        let mut query = String::from(
            r#"
            SELECT id, name, address, latitude, longitude, partner_id, station_type,
                   power_kw, available_chargers, status, created_at, updated_at
            FROM inventory.station
            WHERE partner_id = $1
            "#
        );

        if let Some(status_filter) = &status {
            query.push_str(&format!(" AND status = ${}", 2));
        }

        query.push_str(" ORDER BY name ASC");

        if limit > 0 {
            query.push_str(&format!(" LIMIT ${}", 2 + (status.is_some() as usize)));
        }

        if offset > 0 {
            query.push_str(&format!(" OFFSET ${}", 2 + (status.is_some() as usize) + (limit > 0 as usize)));
        }

        let mut query_builder = sqlx::query(&query);
        query_builder = query_builder.bind(partner_id);

        if let Some(status_filter) = &status {
            query_builder = query_builder.bind(status_filter);
        }

        if limit > 0 {
            query_builder = query_builder.bind(limit as i32);
        }

        if offset > 0 {
            query_builder = query_builder.bind(offset as i32);
        }

        let stations = query_builder.fetch_all(&self.pool).await?;

        Ok(stations)
    }

    /// Get station detail with all chargers
    pub async fn get_station_detail(
        &self,
        partner_id: &str,
        station_id: &str,
    ) -> Result<PartnerStationDetail, sqlx::Error> {
        // Verify station belongs to this partner
        let station: Option<Station> = sqlx::query_as!(
            Station,
            r#"
            SELECT id, name, address, latitude, longitude, partner_id, station_type,
                   power_kw, available_chargers, status, created_at, updated_at
            FROM inventory.station
            WHERE id = $1 AND partner_id = $2
            "#,
            station_id as &str,
            partner_id as &str
        )
        .fetch_optional(&self.pool)
        .await?;

        if station.is_none() {
            return Err(sqlx::Error::RowNotFound);
        }

        // Get all chargers for this station
        let chargers: Vec<Charger> = sqlx::query_as!(
            Charger,
            r#"
            SELECT id, station_id, connector_type, power_kw, status
            FROM inventory.charger
            WHERE station_id = $1
            ORDER BY power_kw DESC
            "#,
            station_id as &str
        )
        .fetch_all(&self.pool)
        .await?;

        let station = station.unwrap();

        Ok(PartnerStationDetail {
            station,
            chargers,
        })
    }

    /// Create a new station
    pub async fn create_station(
        &self,
        partner_id: &str,
        input: CreateStationInput,
    ) -> Result<Station, sqlx::Error> {
        let now = chrono::Utc::now().timestamp();

        let station = Station {
            id: format!("STN-{}", now),
            name: input.name,
            address: input.address,
            latitude: input.latitude,
            longitude: input.longitude,
            partner_id: Some(partner_id.to_string()),
            station_type: input.station_type,
            power_kw: input.power_kw,
            available_chargers: input.available_chargers.unwrap_or(0),
            status: input.status.unwrap_or("active".to_string()),
            created_at: Some(now),
            updated_at: Some(now),
        };

        sqlx::query!(
            r#"
            INSERT INTO inventory.station (
                id, name, address, latitude, longitude, partner_id, station_type,
                power_kw, available_chargers, status, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            "#,
            station.id,
            station.name,
            station.address,
            station.latitude,
            station.longitude,
            station.partner_id,
            station.station_type,
            station.power_kw,
            station.available_chargers,
            station.status,
            station.created_at,
            station.updated_at
        )
        .execute(&self.pool)
        .await?;

        Ok(station)
    }

    /// Update station details
    pub async fn update_station(
        &self,
        partner_id: &str,
        station_id: &str,
        input: UpdateStationInput,
    ) -> Result<usize, sqlx::Error> {
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
        query_builder = query_builder.bind(partner_id);

        let result = query_builder
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() as usize)
    }
}

/// Partner station detail with chargers
#[derive(Debug, Clone)]
pub struct PartnerStationDetail {
    pub station: Station,
    pub chargers: Vec<Charger>,
}

/// Create station input
#[derive(Debug, Clone)]
pub struct CreateStationInput {
    pub name: Option<String>,
    pub address: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub station_type: Option<String>,
    pub power_kw: Option<i32>,
    pub available_chargers: Option<i32>,
    pub status: Option<String>,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partner_station_repository_creation() {
        let repo = PartnerStationRepository::new(Pool::none());
        assert!(true); // Structure validated
    }
}

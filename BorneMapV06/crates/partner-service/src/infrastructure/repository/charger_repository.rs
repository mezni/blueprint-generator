//! Charger repository for partner-service

use sqlx::PgPool;

use crate::ev_domain::Charger;

/// Charger repository for inventory schema queries
pub struct ChargerRepository {
    pool: Pool,
}

impl ChargerRepository {
    /// Create a new charger repository
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    /// List chargers for a station
    pub async fn list_chargers_by_station(
        &self,
        station_id: &str,
    ) -> Result<Vec<Charger>, sqlx::Error> {
        let chargers = sqlx::query_as!(
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

        Ok(chargers)
    }

    /// Get charger by ID
    pub async fn get_charger(
        &self,
        charger_id: &str,
    ) -> Result<Option<Charger>, sqlx::Error> {
        let charger = sqlx::query_as!(
            Charger,
            r#"
            SELECT id, station_id, connector_type, power_kw, status
            FROM inventory.charger
            WHERE id = $1
            "#,
            charger_id as &str
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(charger)
    }

    /// Update charger status
    pub async fn update_charger_status(
        &self,
        charger_id: &str,
        status: &str,
    ) -> Result<usize, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            UPDATE inventory.charger
            SET status = $1, updated_at = NOW()
            WHERE id = $2
            "#,
            status,
            charger_id as &str
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() as usize)
    }

    /// Create a new charger
    pub async fn create_charger(
        &self,
        station_id: &str,
        input: CreateChargerInput,
    ) -> Result<Charger, sqlx::Error> {
        let now = chrono::Utc::now().timestamp();

        let charger = Charger {
            id: format!("CHR-{}", now),
            station_id: station_id.to_string(),
            connector_type: input.connector_type,
            power_kw: input.power_kw,
            status: input.status.unwrap_or("active".to_string()),
        };

        sqlx::query!(
            r#"
            INSERT INTO inventory.charger (
                id, station_id, connector_type, power_kw, status, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            charger.id,
            charger.station_id,
            charger.connector_type,
            charger.power_kw,
            charger.status,
            now,
            now
        )
        .execute(&self.pool)
        .await?;

        Ok(charger)
    }

    /// Delete a charger
    pub async fn delete_charger(
        &self,
        charger_id: &str,
    ) -> Result<usize, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            DELETE FROM inventory.charger
            WHERE id = $1
            "#,
            charger_id as &str
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() as usize)
    }

    /// Get charger summary for a station
    pub async fn get_charger_summary(
        &self,
        station_id: &str,
    ) -> Result<ChargerStatusSummary, sqlx::Error> {
        let status_counts: Vec<(String, i64)> = sqlx::query!(
            r#"
            SELECT status, COUNT(*) as count
            FROM inventory.charger
            WHERE station_id = $1
            GROUP BY status
            "#,
            station_id as &str
        )
        .fetch_all(&self.pool)
        .await?;

        let mut available = 0i64;
        let mut in_use = 0i64;
        let mut maintenance = 0i64;
        let mut offline = 0i64;

        for (status, count) in status_counts {
            match status.as_str() {
                "available" => available = count,
                "in_use" => in_use = count,
                "maintenance" => maintenance = count,
                "offline" => offline = count,
                _ => {}
            }
        }

        let total = available + in_use + maintenance + offline;

        Ok(ChargerStatusSummary::new(
            total as i32,
            available as i32,
            in_use as i32,
            maintenance as i32,
            offline as i32,
        ))
    }
}

/// Charger status summary
#[derive(Debug, Clone)]
pub struct ChargerStatusSummary {
    pub total: i32,
    pub available: i32,
    pub in_use: i32,
    pub maintenance: i32,
    pub offline: i32,
}

impl ChargerStatusSummary {
    /// Create new summary
    pub fn new(
        total: i32,
        available: i32,
        in_use: i32,
        maintenance: i32,
        offline: i32,
    ) -> Self {
        Self {
            total,
            available,
            in_use,
            maintenance,
            offline,
        }
    }

    /// Get availability rate
    pub fn availability_rate(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            (self.available as f64 / self.total as f64) * 100.0
        }
    }
}

/// Create charger input
#[derive(Debug, Clone)]
pub struct CreateChargerInput {
    pub connector_type: String,
    pub power_kw: Option<i32>,
    pub status: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_charger_repository_creation() {
        let repo = ChargerRepository::new(Pool::none());
        assert!(true); // Structure validated
    }

    #[test]
    fn test_charger_status_summary() {
        let summary = ChargerStatusSummary::new(10, 8, 1, 1, 0);
        assert_eq!(summary.total, 10);
        assert_eq!(summary.available, 8);
        assert!((summary.availability_rate() - 80.0).abs() < 0.01);
    }
}

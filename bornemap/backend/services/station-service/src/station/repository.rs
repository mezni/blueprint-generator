use crate::station::models::*;
use common_utils::error::{db_err, DomainError};
use sqlx::PgPool;

type StationRow = (
    uuid::Uuid,
    uuid::Uuid,
    String,
    String,
    f64,
    f64,
    bool,
    bool,
    Option<String>,
    chrono::DateTime<chrono::Utc>,
    chrono::DateTime<chrono::Utc>,
);

pub struct StationRepository;

impl StationRepository {
    pub async fn list_by_viewport(
        pool: &PgPool,
        bbox: &BboxQuery,
    ) -> Result<Vec<StationMarker>, DomainError> {
        let rows: Vec<(uuid::Uuid, String, f64, f64, bool, bool)> = sqlx::query_as(
            r#"
            SELECT
                s.id,
                s.name,
                ST_X(s.location::geometry) AS lng,
                ST_Y(s.location::geometry) AS lat,
                s.is_active,
                s.under_maintenance
            FROM station_domain.stations s
            WHERE s.deleted_at IS NULL
              AND ST_DWithin(
                    s.location,
                    ST_MakeEnvelope($1, $2, $3, $4, 4326)::geography,
                    0
                  )
            ORDER BY s.id
            LIMIT 5000
            "#,
        )
        .bind(bbox.west)
        .bind(bbox.south)
        .bind(bbox.east)
        .bind(bbox.north)
        .fetch_all(pool)
        .await
        .map_err(db_err)?;

        let markers = rows
            .into_iter()
            .map(
                |(id, name, lng, lat, is_active, under_maintenance)| StationMarker {
                    id: id.to_string(),
                    name,
                    coord: [lng, lat],
                    is_active,
                    under_maintenance,
                },
            )
            .collect();

        Ok(markers)
    }

    pub async fn get_by_id(pool: &PgPool, id: &str) -> Result<StationDetail, DomainError> {
        let station_id = uuid::Uuid::parse_str(id)
            .map_err(|e| DomainError::Validation(format!("Invalid station ID: {e}")))?;

        let row: Option<StationRow> = sqlx::query_as(
            r#"
            SELECT
                s.id,
                s.company_id,
                s.name,
                s.address,
                ST_X(s.location::geometry) AS lng,
                ST_Y(s.location::geometry) AS lat,
                s.is_active,
                s.under_maintenance,
                s.opening_hours_osm,
                s.created_at,
                s.updated_at
            FROM station_domain.stations s
            WHERE s.id = $1
              AND s.deleted_at IS NULL
            "#,
        )
        .bind(station_id)
        .fetch_optional(pool)
        .await
        .map_err(db_err)?;

        let row = row.ok_or_else(|| DomainError::NotFound("Station not found".into()))?;

        let company_row: (String,) = sqlx::query_as(
            "SELECT name FROM station_domain.companies WHERE id = $1 AND deleted_at IS NULL",
        )
        .bind(row.1)
        .fetch_one(pool)
        .await
        .map_err(|_| DomainError::NotFound("Company not found".into()))?;

        let chargers = Self::get_chargers(pool, &station_id).await?;

        Ok(StationDetail {
            id: row.0.to_string(),
            company: Company {
                id: row.1.to_string(),
                name: company_row.0,
            },
            name: row.2,
            address: row.3,
            coord: [row.4, row.5],
            is_active: row.6,
            under_maintenance: row.7,
            opening_hours_osm: row.8,
            chargers,
            created_at: row.9.to_rfc3339(),
            updated_at: row.10.to_rfc3339(),
        })
    }

    async fn get_chargers(
        pool: &PgPool,
        station_id: &uuid::Uuid,
    ) -> Result<Vec<Charger>, DomainError> {
        let rows: Vec<(uuid::Uuid, String, f64, bool)> = sqlx::query_as(
            r#"
            SELECT
                ch.id,
                ch.connector,
                ch.power_kw::float8,
                ch.is_active
            FROM station_domain.chargers ch
            WHERE ch.station_id = $1
              AND ch.deleted_at IS NULL
            ORDER BY ch.id
            "#,
        )
        .bind(station_id)
        .fetch_all(pool)
        .await
        .map_err(db_err)?;

        Ok(rows
            .into_iter()
            .map(|(id, connector, power_kw, is_active)| Charger {
                id: id.to_string(),
                connector,
                power_kw,
                is_active,
            })
            .collect())
    }

    pub async fn admin_create(
        pool: &PgPool,
        input: &AdminStationCreate,
        station_id: uuid::Uuid,
    ) -> Result<String, DomainError> {
        let company_uuid = uuid::Uuid::parse_str(&input.company_id)
            .map_err(|e| DomainError::Validation(format!("Invalid company_id: {e}")))?;

        sqlx::query(
            r#"
            INSERT INTO station_domain.stations
                (id, company_id, name, address, location, is_active, under_maintenance, opening_hours_osm, is_test)
            VALUES
                ($1, $2, $3, $4, ST_MakePoint($5, $6)::geography, $7, $8, $9, FALSE)
            "#,
        )
        .bind(station_id)
        .bind(company_uuid)
        .bind(&input.name)
        .bind(&input.address)
        .bind(input.coord[0])
        .bind(input.coord[1])
        .bind(input.is_active)
        .bind(input.under_maintenance)
        .bind(&input.opening_hours_osm)
        .execute(pool)
        .await
        .map_err(db_err)?;

        if let Some(chargers) = &input.chargers {
            for charger in chargers {
                let charger_id = uuid::Uuid::new_v4();
                sqlx::query(
                    r#"
                    INSERT INTO station_domain.chargers
                        (id, station_id, connector, power_kw, is_active, is_test)
                    VALUES
                        ($1, $2, $3, $4, $5, FALSE)
                    "#,
                )
                .bind(charger_id)
                .bind(station_id)
                .bind(&charger.connector)
                .bind(charger.power_kw)
                .bind(charger.is_active)
                .execute(pool)
                .await
                .map_err(db_err)?;
            }
        }

        Ok(station_id.to_string())
    }

    pub async fn admin_patch(
        pool: &PgPool,
        id: &str,
        current: &StationDetail,
        patch: &AdminStationPatch,
    ) -> Result<bool, DomainError> {
        let station_id = uuid::Uuid::parse_str(id)
            .map_err(|e| DomainError::Validation(format!("Invalid station ID: {e}")))?;

        let name = patch.name.as_deref().unwrap_or(&current.name);
        let address = patch.address.as_deref().unwrap_or(&current.address);
        let coord = patch.coord.unwrap_or(current.coord);
        let is_active = patch.is_active.unwrap_or(current.is_active);
        let under_maintenance = patch.under_maintenance.unwrap_or(current.under_maintenance);
        let opening_hours_osm = patch
            .opening_hours_osm
            .as_ref()
            .or(current.opening_hours_osm.as_ref());

        let result = sqlx::query(
            r#"
            UPDATE station_domain.stations
            SET name = $2,
                address = $3,
                location = ST_MakePoint($4, $5)::geography,
                is_active = $6,
                under_maintenance = $7,
                opening_hours_osm = $8,
                updated_at = NOW()
            WHERE id = $1
              AND deleted_at IS NULL
            "#,
        )
        .bind(station_id)
        .bind(name)
        .bind(address)
        .bind(coord[0])
        .bind(coord[1])
        .bind(is_active)
        .bind(under_maintenance)
        .bind(opening_hours_osm)
        .execute(pool)
        .await
        .map_err(db_err)?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn admin_soft_delete(pool: &PgPool, id: &str) -> Result<bool, DomainError> {
        let station_id = uuid::Uuid::parse_str(id)
            .map_err(|e| DomainError::Validation(format!("Invalid station ID: {e}")))?;

        let result = sqlx::query(
            r#"
            UPDATE station_domain.stations
            SET deleted_at = NOW(),
                updated_at = NOW()
            WHERE id = $1
              AND deleted_at IS NULL
            "#,
        )
        .bind(station_id)
        .execute(pool)
        .await
        .map_err(db_err)?;

        if result.rows_affected() == 0 {
            return Err(DomainError::NotFound(
                "Station not found or already deleted".into(),
            ));
        }
        Ok(true)
    }

    pub async fn admin_list(
        pool: &PgPool,
        limit: i64,
        cursor: Option<&str>,
        include_deleted: bool,
        include_test: bool,
    ) -> Result<(Vec<StationDetail>, Option<String>), DomainError> {
        let cursor_id = match cursor {
            Some(c) => Some(
                uuid::Uuid::parse_str(c)
                    .map_err(|e| DomainError::Validation(format!("Invalid cursor: {e}")))?,
            ),
            None => None,
        };

        let rows: Vec<StationRow> = sqlx::query_as(
            r#"
            SELECT
                s.id,
                s.company_id,
                s.name,
                s.address,
                ST_X(s.location::geometry) AS lng,
                ST_Y(s.location::geometry) AS lat,
                s.is_active,
                s.under_maintenance,
                s.opening_hours_osm,
                s.created_at,
                s.updated_at
            FROM station_domain.stations s
            WHERE ($1::boolean OR s.deleted_at IS NULL)
              AND ($2::boolean OR s.is_test = FALSE)
              AND ($3::uuid IS NULL OR s.id > $3)
            ORDER BY s.id
            LIMIT $4
            "#,
        )
        .bind(include_deleted)
        .bind(include_test)
        .bind(cursor_id)
        .bind(limit + 1)
        .fetch_all(pool)
        .await
        .map_err(db_err)?;

        let has_more = rows.len() > limit as usize;
        let rows: Vec<_> = rows.into_iter().take(limit as usize).collect();

        let mut items = Vec::with_capacity(rows.len());
        for row in &rows {
            let company_row: Option<(String,)> = sqlx::query_as(
                "SELECT name FROM station_domain.companies WHERE id = $1 AND deleted_at IS NULL",
            )
            .bind(row.1)
            .fetch_optional(pool)
            .await
            .map_err(db_err)?;

            let chargers = Self::get_chargers(pool, &row.0).await.unwrap_or_default();

            items.push(StationDetail {
                id: row.0.to_string(),
                company: Company {
                    id: row.1.to_string(),
                    name: company_row.map(|r| r.0).unwrap_or_default(),
                },
                name: row.2.clone(),
                address: row.3.clone(),
                coord: [row.4, row.5],
                is_active: row.6,
                under_maintenance: row.7,
                opening_hours_osm: row.8.clone(),
                chargers,
                created_at: row.9.to_rfc3339(),
                updated_at: row.10.to_rfc3339(),
            });
        }

        let next_cursor = if has_more {
            items.last().map(|i| i.id.clone())
        } else {
            None
        };

        Ok((items, next_cursor))
    }
}

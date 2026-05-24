use sqlx::PgPool;
use common_utils::error::DomainError;
use super::filters;
use super::models::*;
use super::repository::StationRepository;

const VALID_CONNECTORS: &[&str] = &["Type2", "CCS", "CHAdeMO", "Type2_Tethered"];

pub struct StationService;

impl StationService {
    pub async fn list_by_viewport(
        pool: &PgPool,
        bbox_query: &str,
    ) -> Result<StationListResponse, DomainError> {
        let viewport = filters::parse_bbox(bbox_query)?;
        let markers = StationRepository::list_by_viewport(pool, &viewport).await?;
        let truncated = markers.len() >= 5000;
        Ok(StationListResponse {
            viewport,
            markers,
            truncated,
        })
    }

    pub async fn get_by_id(
        pool: &PgPool,
        id: &str,
    ) -> Result<StationDetail, DomainError> {
        StationRepository::get_by_id(pool, id).await
    }

    pub async fn admin_create(
        pool: &PgPool,
        input: &AdminStationCreate,
    ) -> Result<StationDetail, DomainError> {
        Self::validate_coord(input.coord)?;
        if let Some(chargers) = &input.chargers {
            for charger in chargers {
                Self::validate_connector(&charger.connector)?;
                Self::validate_power_kw(charger.power_kw)?;
            }
        }
        if let Some(ref hrs) = input.opening_hours_osm {
            Self::validate_opening_hours(hrs)?;
        }

        let station_id = uuid::Uuid::new_v4();
        StationRepository::admin_create(pool, input, station_id).await?;
        StationRepository::get_by_id(pool, &station_id.to_string()).await
    }

    pub async fn admin_patch(
        pool: &PgPool,
        id: &str,
        patch: &AdminStationPatch,
    ) -> Result<StationDetail, DomainError> {
        if let Some(coord) = patch.coord {
            Self::validate_coord(coord)?;
        }
        if let Some(ref hrs) = patch.opening_hours_osm {
            Self::validate_opening_hours(hrs)?;
        }

        let current = StationRepository::get_by_id(pool, id).await?;
        StationRepository::admin_patch(pool, id, &current, patch).await?;
        StationRepository::get_by_id(pool, id).await
    }

    pub async fn admin_soft_delete(
        pool: &PgPool,
        id: &str,
    ) -> Result<(), DomainError> {
        StationRepository::admin_soft_delete(pool, id).await?;
        Ok(())
    }

    pub async fn admin_list(
        pool: &PgPool,
        limit: i64,
        cursor: Option<&str>,
        include_deleted: bool,
        include_test: bool,
    ) -> Result<AdminStationListResponse, DomainError> {
        let (items, next_cursor) =
            StationRepository::admin_list(pool, limit, cursor, include_deleted, include_test)
                .await?;
        Ok(AdminStationListResponse {
            items,
            next_cursor,
        })
    }

    fn validate_coord(coord: [f64; 2]) -> Result<(), DomainError> {
        let lng = coord[0];
        let lat = coord[1];
        if lng < -180.0 || lng > 180.0 {
            return Err(DomainError::Validation(
                "Longitude must be between -180 and 180".into(),
            ));
        }
        if lat < -90.0 || lat > 90.0 {
            return Err(DomainError::Validation(
                "Latitude must be between -90 and 90".into(),
            ));
        }
        Ok(())
    }

    fn validate_connector(connector: &str) -> Result<(), DomainError> {
        if !VALID_CONNECTORS.contains(&connector) {
            return Err(DomainError::Validation(format!(
                "Invalid connector type: '{connector}'. Must be one of: Type2, CCS, CHAdeMO, Type2_Tethered"
            )));
        }
        Ok(())
    }

    fn validate_power_kw(power_kw: f64) -> Result<(), DomainError> {
        if power_kw <= 0.0 || power_kw > 600.0 {
            return Err(DomainError::Validation(
                "power_kw must be between 0 (exclusive) and 600".into(),
            ));
        }
        Ok(())
    }

    fn validate_opening_hours(hrs: &str) -> Result<(), DomainError> {
        if hrs.is_empty() {
            return Ok(());
        }
        match hrs.parse::<opening_hours::OpeningHours>() {
            Ok(_) => Ok(()),
            Err(e) => Err(DomainError::Validation(format!(
                "Invalid opening_hours_osm: {e}"
            ))),
        }
    }
}

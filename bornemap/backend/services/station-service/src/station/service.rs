use super::filters;
use super::models::*;
use super::repository::StationRepository;
use common_utils::error::DomainError;
use sqlx::PgPool;

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

    pub async fn get_by_id(pool: &PgPool, id: &str) -> Result<StationDetail, DomainError> {
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

    pub async fn admin_soft_delete(pool: &PgPool, id: &str) -> Result<(), DomainError> {
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
        Ok(AdminStationListResponse { items, next_cursor })
    }

    fn validate_coord(coord: [f64; 2]) -> Result<(), DomainError> {
        let lng = coord[0];
        let lat = coord[1];
        if !(-180.0..=180.0).contains(&lng) {
            return Err(DomainError::Validation(
                "Longitude must be between -180 and 180".into(),
            ));
        }
        if !(-90.0..=90.0).contains(&lat) {
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

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    // --- validate_coord ---

    #[test]
    fn valid_coord_ok() {
        StationService::validate_coord([10.0, 36.0]).unwrap();
        StationService::validate_coord([-180.0, -90.0]).unwrap();
        StationService::validate_coord([180.0, 90.0]).unwrap();
        StationService::validate_coord([0.0, 0.0]).unwrap();
    }

    #[test]
    fn coord_lng_out_of_range_errs() {
        let err = StationService::validate_coord([181.0, 0.0]).unwrap_err();
        assert!(matches!(err, DomainError::Validation(_)));

        let err = StationService::validate_coord([-181.0, 0.0]).unwrap_err();
        assert!(matches!(err, DomainError::Validation(_)));
    }

    #[test]
    fn coord_lat_out_of_range_errs() {
        let err = StationService::validate_coord([0.0, 91.0]).unwrap_err();
        assert!(matches!(err, DomainError::Validation(_)));

        let err = StationService::validate_coord([0.0, -91.0]).unwrap_err();
        assert!(matches!(err, DomainError::Validation(_)));
    }

    // --- validate_connector ---

    #[test]
    fn valid_connectors_ok() {
        for c in &["Type2", "CCS", "CHAdeMO", "Type2_Tethered"] {
            StationService::validate_connector(c).unwrap();
        }
    }

    #[test]
    fn invalid_connector_errs() {
        let err = StationService::validate_connector("Tesla").unwrap_err();
        assert!(matches!(err, DomainError::Validation(_)));
    }

    #[test]
    fn empty_connector_errs() {
        let err = StationService::validate_connector("").unwrap_err();
        assert!(matches!(err, DomainError::Validation(_)));
    }

    // --- validate_power_kw ---

    #[test]
    fn valid_power_ok() {
        StationService::validate_power_kw(1.0).unwrap();
        StationService::validate_power_kw(50.0).unwrap();
        StationService::validate_power_kw(600.0).unwrap();
    }

    #[test]
    fn zero_power_errs() {
        let err = StationService::validate_power_kw(0.0).unwrap_err();
        assert!(matches!(err, DomainError::Validation(_)));
    }

    #[test]
    fn negative_power_errs() {
        let err = StationService::validate_power_kw(-1.0).unwrap_err();
        assert!(matches!(err, DomainError::Validation(_)));
    }

    #[test]
    fn power_too_high_errs() {
        let err = StationService::validate_power_kw(600.001).unwrap_err();
        assert!(matches!(err, DomainError::Validation(_)));
    }

    // --- validate_opening_hours ---

    #[test]
    fn empty_opening_hours_ok() {
        StationService::validate_opening_hours("").unwrap();
    }

    #[test]
    fn valid_opening_hours_ok() {
        StationService::validate_opening_hours("24/7").unwrap();
        StationService::validate_opening_hours("Mo-Fr 08:00-18:00").unwrap();
    }

    #[test]
    fn invalid_opening_hours_errs() {
        let err = StationService::validate_opening_hours("not-a-valid-spec").unwrap_err();
        assert!(matches!(err, DomainError::Validation(_)));
    }
}

use chrono::{DateTime, Utc};
use common_types::{StationAvailabilityStatus, StationStatus};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::error::ServiceError;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Station {
    pub station_id: String,
    pub partner_id: String,
    pub name: String,
    pub address: Option<String>,
    pub latitude: f64,
    pub longitude: f64,
    pub status: StationStatus,
    pub availability_status: StationAvailabilityStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StationCreate {
    pub name: String,
    pub address: Option<String>,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StationUpdate {
    pub name: Option<String>,
    pub address: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub status: Option<StationStatus>,
    pub availability_status: Option<StationAvailabilityStatus>,
}

pub fn validate_coordinates(lat: f64, lng: f64) -> Result<(), ServiceError> {
    if lat < -90.0 || lat > 90.0 || lng < -180.0 || lng > 180.0 {
        return Err(ServiceError::invalid_coordinates());
    }
    Ok(())
}

pub fn validate_status_transition(
    from: StationStatus,
    to: StationStatus,
) -> Result<(), ServiceError> {
    let allowed = match (from, to) {
        (StationStatus::Draft, StationStatus::Active) => true,
        (StationStatus::Active, StationStatus::Inactive) => true,
        (StationStatus::Active, StationStatus::Maintenance) => true,
        (StationStatus::Inactive, StationStatus::Active) => true,
        (StationStatus::Inactive, StationStatus::Maintenance) => true,
        (StationStatus::Maintenance, StationStatus::Active) => true,
        (f, t) if f == t => true,
        _ => false,
    };
    if !allowed {
        return Err(ServiceError::invalid_state_transition(
            from.as_str(),
            to.as_str(),
        ));
    }
    Ok(())
}

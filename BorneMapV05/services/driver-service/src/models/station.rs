use common_types::StationAvailabilityStatus;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoPoint {
    pub lat: f64,
    pub lng: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewSummary {
    pub average_rating: Option<f64>,
    pub total_reviews: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargerTypeInfo {
    pub connector_type: String,
    pub power_kw: Option<f64>,
    pub status: String,
}

/// Lightweight station representation for list/search results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StationListItem {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub latitude: f64,
    pub longitude: f64,
    pub city: Option<String>,
    pub country: Option<String>,
    pub distance_km: Option<f64>,
    pub geom: Option<GeoPoint>,
    pub charger_types: Vec<ChargerTypeInfo>,
    pub availability: Option<StationAvailabilityStatus>,
    pub review_summary: Option<ReviewSummary>,
}

/// Full station detail including chargers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StationDetail {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub latitude: f64,
    pub longitude: f64,
    pub city: Option<String>,
    pub country: Option<String>,
    pub distance_km: Option<f64>,
    pub geom: Option<GeoPoint>,
    pub chargers: Vec<super::charger::Charger>,
    pub charger_types: Vec<ChargerTypeInfo>,
    pub availability: Option<StationAvailabilityStatus>,
    pub review_summary: Option<ReviewSummary>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct StationListQuery {
    pub lat: Option<f64>,
    pub lng: Option<f64>,
    pub radius_km: Option<f64>,
    pub bbox: Option<String>,
    pub connector_type: Option<String>,
    pub availability: Option<String>,
    pub page: Option<i32>,
    pub size: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct StationSearchQuery {
    pub q: Option<String>,
    pub city: Option<String>,
    pub connector_type: Option<String>,
    pub availability: Option<String>,
    pub page: Option<i32>,
    pub size: Option<i32>,
}

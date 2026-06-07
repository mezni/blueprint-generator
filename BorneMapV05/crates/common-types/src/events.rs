use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum EventName {
    #[serde(rename = "page.viewed")]
    PageViewed,
    #[serde(rename = "map.loaded")]
    MapLoaded,
    #[serde(rename = "map.viewport_changed")]
    MapViewportChanged,
    #[serde(rename = "search.performed")]
    SearchPerformed,
    #[serde(rename = "stations.nearby.viewed")]
    StationsNearbyViewed,
    #[serde(rename = "filter.applied")]
    FilterApplied,
    #[serde(rename = "station.marker_clicked")]
    StationMarkerClicked,
    #[serde(rename = "station.opened")]
    StationOpened,
    #[serde(rename = "charger.opened")]
    ChargerOpened,
    #[serde(rename = "favorite_station.added")]
    FavoriteStationAdded,
    #[serde(rename = "favorite_station.removed")]
    FavoriteStationRemoved,
    #[serde(rename = "review.submitted")]
    ReviewSubmitted,
    #[serde(rename = "review.updated")]
    ReviewUpdated,
    #[serde(rename = "auth.started")]
    AuthStarted,
    #[serde(rename = "auth.succeeded")]
    AuthSucceeded,
    #[serde(rename = "auth.failed")]
    AuthFailed,
    #[serde(rename = "partner_station.created")]
    PartnerStationCreated,
    #[serde(rename = "partner_station.updated")]
    PartnerStationUpdated,
    #[serde(rename = "partner_availability.updated")]
    PartnerAvailabilityUpdated,
    #[serde(rename = "admin_station.created")]
    AdminStationCreated,
    #[serde(rename = "admin_review.moderated")]
    AdminReviewModerated,
    #[serde(rename = "search.failed")]
    SearchFailed,
    #[serde(rename = "station.load_failed")]
    StationLoadFailed,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Channel {
    #[serde(rename = "driver_web")]
    DriverWeb,
    #[serde(rename = "driver_mobile")]
    DriverMobile,
    #[serde(rename = "partner_dashboard")]
    PartnerDashboard,
    #[serde(rename = "admin_dashboard")]
    AdminDashboard,
}

impl Channel {
    pub fn as_str(&self) -> &'static str {
        match self {
            Channel::DriverWeb => "driver_web",
            Channel::DriverMobile => "driver_mobile",
            Channel::PartnerDashboard => "partner_dashboard",
            Channel::AdminDashboard => "admin_dashboard",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ActorRole {
    #[serde(rename = "registered_driver")]
    RegisteredDriver,
    #[serde(rename = "partner")]
    Partner,
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "anonymous")]
    Anonymous,
}

pub const EVENT_VERSION: i32 = 1;
pub const SCHEMA_NAMESPACE: &str = "clickstream";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventEnvelope {
    pub event_id: String,
    pub event_version: i32,
    pub schema_namespace: String,
    pub event_name: EventName,
    pub occurred_at: String,
    pub ingested_at: String,
    pub channel: Channel,
    pub session_id: String,
    pub correlation_id: Option<String>,
    pub anonymous_id: Option<String>,
    pub user_id: Option<String>,
    pub actor_role: Option<ActorRole>,
    pub path: Option<String>,
    pub payload: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,
}

impl EventEnvelope {
    pub fn new(
        event_id: String,
        event_name: EventName,
        occurred_at: String,
        ingested_at: String,
        channel: Channel,
        session_id: String,
    ) -> Self {
        Self {
            event_id,
            event_version: EVENT_VERSION,
            schema_namespace: SCHEMA_NAMESPACE.to_string(),
            event_name,
            occurred_at,
            ingested_at,
            channel,
            session_id,
            correlation_id: None,
            anonymous_id: None,
            user_id: None,
            actor_role: None,
            path: None,
            payload: None,
            metadata: None,
        }
    }
}

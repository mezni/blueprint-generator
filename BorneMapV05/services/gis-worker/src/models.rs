use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct GisQueueEntry {
    pub id: String,
    pub entity_type: String,
    pub entity_id: String,
    pub operation: String,
    pub payload: Option<serde_json::Value>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub processed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StationGeometry {
    pub id: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsmRoad {
    pub osm_id: i64,
    pub name: Option<String>,
    pub highway: Option<String>,
    pub geom: Option<serde_json::Value>,
    pub tags: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsmAdminBoundary {
    pub osm_id: i64,
    pub name: Option<String>,
    pub admin_level: Option<i32>,
    pub geom: Option<serde_json::Value>,
    pub tags: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsmPoi {
    pub osm_id: i64,
    pub name: Option<String>,
    pub amenity: Option<String>,
    pub geom: Option<serde_json::Value>,
    pub tags: Option<serde_json::Value>,
}

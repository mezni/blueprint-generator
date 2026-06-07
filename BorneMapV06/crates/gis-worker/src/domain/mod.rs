//! Domain layer for gis-worker

pub mod gis_projection;
pub mod outbox_event;

pub use gis_projection::{StationLocationProjection, ProjectionError};
pub use outbox_event::{OutboxEvent, EventType, EventReader};

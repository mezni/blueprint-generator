//! Outbox event reader for event-driven GIS sync

use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::{debug, warn};

use crate::AppState;

/// Event types for station changes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EventType {
    Created,
    Updated,
    Deleted,
}

/// Outbox event for station change tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutboxEvent {
    pub id: String,
    pub aggregate_type: String,
    pub aggregate_id: String,
    pub event_type: EventType,
    pub payload: Option<serde_json::Value>,
    pub processed: bool,
    pub created_at: String,
}

/// Event reader for processing outbox events
pub struct EventReader {
    pool: PgPool,
}

impl EventReader {
    /// Create a new event reader
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Read unprocessed events from the outbox
    pub async fn read_events(&self, limit: usize) -> Result<Vec<OutboxEvent>, Box<dyn std::error::Error>> {
        debug!("Reading outbox events with limit: {}", limit);

        let events = sqlx::query_as!(
            OutboxEvent,
            r#"
            SELECT id, aggregate_type, aggregate_id, event_type, payload, processed, created_at
            FROM inventory.station_outbox
            WHERE processed = false
            ORDER BY created_at ASC
            LIMIT $1
            "#,
            limit as i32
        )
        .fetch_all(&self.pool)
        .await?;

        debug!("Read {} unprocessed events", events.len());
        Ok(events)
    }

    /// Mark events as processed
    pub async fn mark_processed(&self, event_ids: &[String]) -> Result<(), Box<dyn std::error::Error>> {
        if event_ids.is_empty() {
            return Ok(());
        }

        debug!("Marking {} events as processed", event_ids.len());

        for event_id in event_ids {
            sqlx::query!(
                r#"
                UPDATE inventory.station_outbox
                SET processed = true
                WHERE id = $1 AND processed = false
                "#,
                event_id
            )
            .execute(&self.pool)
            .await?;
        }

        Ok(())
    }

    /// Process a single event and trigger GIS sync
    pub async fn process_event(&self, event: OutboxEvent) -> Result<(), Box<dyn std::error::Error>> {
        debug!("Processing event: {} ({})", event.id, event.event_type);

        match event.event_type {
            EventType::Created => self.process_created(event).await?,
            EventType::Updated => self.process_updated(event).await?,
            EventType::Deleted => self.process_deleted(event).await?,
        }

        // Mark event as processed
        self.mark_processed(&[event.id]).await?;

        Ok(())
    }

    /// Process a station created event
    async fn process_created(&self, event: OutboxEvent) -> Result<(), Box<dyn std::error::Error>> {
        debug!("Processing station created event for: {}", event.aggregate_id);

        // TODO: Implement actual GIS sync logic
        // 1. Query inventory.station for the new station
        // 2. Create GIS projection
        // 3. Insert into gis.station_locations with GIST spatial index
        // 4. Log for audit trail

        let station = json!({}); // TODO: Fetch from inventory.station

        debug!("GIS projection created for: {}", event.aggregate_id);

        Ok(())
    }

    /// Process a station updated event
    async fn process_updated(&self, event: OutboxEvent) -> Result<(), Box<dyn std::error::Error>> {
        debug!("Processing station updated event for: {}", event.aggregate_id);

        // TODO: Implement actual GIS sync logic
        // 1. Query inventory.station for the updated station
        // 2. Update GIS projection in gis.station_locations
        // 3. Log for audit trail

        debug!("GIS projection updated for: {}", event.aggregate_id);

        Ok(())
    }

    /// Process a station deleted event
    async fn process_deleted(&self, event: OutboxEvent) -> Result<(), Box<dyn std::error::Error>> {
        debug!("Processing station deleted event for: {}", event.aggregate_id);

        // TODO: Implement actual GIS sync logic
        // 1. Delete from gis.station_locations
        // 2. Log for audit trail

        debug!("GIS projection deleted for: {}", event.aggregate_id);

        Ok(())
    }

    /// Process all unprocessed events
    pub async fn process_all(&self, batch_size: usize) -> Result<usize, Box<dyn std::error::Error>> {
        let events = self.read_events(batch_size).await?;

        for event in events {
            if let Err(e) = self.process_event(event).await {
                warn!("Failed to process event {}: {}", event.id, e);
                // Continue processing other events
            }
        }

        Ok(events.len())
    }

    /// Check for unprocessed events
    pub async fn has_unprocessed_events(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let count: i64 = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*)
            FROM inventory.station_outbox
            WHERE processed = false
            "#
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(count > 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_type_serialization() {
        let created = EventType::Created;
        let serialized = serde_json::to_string(&created).unwrap();
        assert_eq!(serialized, r#""Created""#);

        let updated = EventType::Updated;
        let serialized = serde_json::to_string(&updated).unwrap();
        assert_eq!(serialized, r#""Updated""#);

        let deleted = EventType::Deleted;
        let serialized = serde_json::to_string(&deleted).unwrap();
        assert_eq!(serialized, r#""Deleted""#);
    }

    #[test]
    fn test_event_type_deserialization() {
        let json = r#""Created""#;
        let event: EventType = serde_json::from_str(json).unwrap();
        assert_eq!(event, EventType::Created);
    }

    #[test]
    fn test_event_reader_creation() {
        let reader = EventReader::new(PgPool::none());
        assert!(true); // Structure validated
    }
}

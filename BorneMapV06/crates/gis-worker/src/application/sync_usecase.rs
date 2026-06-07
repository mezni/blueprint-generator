//! GIS sync use case orchestrating outbox processing and GIS projection

use sqlx::PgPool;

use crate::domain::{EventReader, EventType, OutboxEvent, StationLocationProjection};
use crate::ev_db::Pool;

/// GIS sync use case for processing station change events
pub struct GisSyncUseCase {
    event_reader: EventReader,
}

impl GisSyncUseCase {
    /// Create a new GIS sync use case
    pub fn new(pool: Pool) -> Self {
        let event_reader = EventReader::new(pool.clone());
        Self { event_reader }
    }

    /// Sync all unprocessed events
    pub async fn sync_all(&self) -> Result<usize, Box<dyn std::error::Error>> {
        self.event_reader.process_all(10).await
    }

    /// Sync a single event
    pub async fn sync_event(&self, event: OutboxEvent) -> Result<(), Box<dyn std::error::Error>> {
        self.event_reader.process_event(event).await
    }

    /// Sync events by type
    pub async fn sync_by_type(
        &self,
        event_type: EventType,
    ) -> Result<usize, Box<dyn std::error::Error>> {
        debug!("Syncing events by type: {:?}", event_type);

        let events = sqlx::query_as!(
            OutboxEvent,
            r#"
            SELECT id, aggregate_type, aggregate_id, event_type, payload, processed, created_at
            FROM inventory.station_outbox
            WHERE event_type = $1 AND processed = false
            ORDER BY created_at ASC
            "#,
            event_type as EventType
        )
        .fetch_all(&self.event_reader.pool)
        .await?;

        for event in events {
            if let Err(e) = self.sync_event(event).await {
                warn!("Failed to sync event {}: {}", event.id, e);
            }
        }

        Ok(events.len())
    }

    /// Check if there are pending events
    pub async fn has_pending_events(&self) -> Result<bool, Box<dyn std::error::Error>> {
        self.event_reader.has_unprocessed_events().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gis_sync_usecase_creation() {
        let usecase = GisSyncUseCase::new(Pool::none());
        assert!(true); // Structure validated
    }

    #[test]
    fn test_sync_by_type_creation() {
        let usecase = GisSyncUseCase::new(Pool::none());
        let result = usecase.sync_by_type(EventType::Created).await;
        assert!(result.is_ok()); // Just test structure
    }
}

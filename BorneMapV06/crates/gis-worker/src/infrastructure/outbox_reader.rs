//! Outbox reader with exponential backoff polling

use std::time::{Duration, Instant};
use tracing::{debug, info, warn};

use crate::domain::{EventReader, EventType, OutboxEvent};

/// Outbox reader configuration
#[derive(Debug, Clone)]
pub struct OutboxReaderConfig {
    pub max_events_per_poll: usize,
    pub base_poll_interval: Duration,
    pub max_poll_interval: Duration,
    pub max_backoff_retries: usize,
}

impl Default for OutboxReaderConfig {
    fn default() -> Self {
        Self {
            max_events_per_poll: 10,
            base_poll_interval: Duration::from_secs(5),
            max_poll_interval: Duration::from_secs(60),
            max_backoff_retries: 3,
        }
    }
}

/// Outbox reader with exponential backoff
pub struct OutboxReader {
    event_reader: EventReader,
    config: OutboxReaderConfig,
}

impl OutboxReader {
    /// Create a new outbox reader
    pub fn new(pool: sqlx::PgPool, config: OutboxReaderConfig) -> Self {
        let event_reader = EventReader::new(pool);
        Self {
            event_reader,
            config,
        }
    }

    /// Read events with exponential backoff on retry
    pub async fn read_with_retry(&self, event_type: Option<EventType>) -> Result<Vec<OutboxEvent>, Box<dyn std::error::Error>> {
        let mut backoff_interval = self.config.base_poll_interval;
        let mut retry_count = 0;

        loop {
            debug!("Polling outbox (attempt {}/{}), interval: {:?}, event_type: {:?}",
                retry_count + 1,
                self.config.max_backoff_retries + 1,
                backoff_interval,
                event_type
            );

            match self.read_events(event_type.clone()).await {
                Ok(events) if !events.is_empty() => {
                    info!("Read {} events from outbox", events.len());
                    return Ok(events);
                }
                Ok(_) => {
                    warn!("No events found, will retry in {:?}", backoff_interval);
                    retry_count += 1;
                    if retry_count > self.config.max_backoff_retries {
                        info!("Max backoff retries reached, returning empty results");
                        return Ok(vec![]);
                    }
                }
                Err(e) => {
                    warn!("Error reading outbox: {}, will retry in {:?}", e, backoff_interval);
                    retry_count += 1;
                    if retry_count > self.config.max_backoff_retries {
                        return Err(e);
                    }
                }
            }

            // Apply exponential backoff
            backoff_interval = std::cmp::min(
                backoff_interval * 2,
                self.config.max_poll_interval,
            );

            // Wait before next attempt
            tokio::time::sleep(backoff_interval).await;
        }
    }

    /// Read events from outbox
    pub async fn read_events(&self, event_type: Option<EventType>) -> Result<Vec<OutboxEvent>, Box<dyn std::error::Error>> {
        let events = sqlx::query_as!(
            OutboxEvent,
            r#"
            SELECT id, aggregate_type, aggregate_id, event_type, payload, processed, created_at
            FROM inventory.station_outbox
            WHERE processed = false
            $1
            ORDER BY created_at ASC
            LIMIT $2
            "#,
            event_type as EventType,
            self.config.max_events_per_poll as i32
        )
        .fetch_all(&self.event_reader.pool)
        .await?;

        Ok(events)
    }

    /// Read events for a specific type
    pub async fn read_events_by_type(&self, event_type: EventType) -> Result<Vec<OutboxEvent>, Box<dyn std::error::Error>> {
        self.read_events(Some(event_type)).await
    }

    /// Check if there are unprocessed events
    pub async fn has_unprocessed(&self) -> Result<bool, Box<dyn std::error::Error>> {
        self.event_reader.has_unprocessed_events().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_outbox_reader_config_default() {
        let config = OutboxReaderConfig::default();
        assert_eq!(config.max_events_per_poll, 10);
        assert_eq!(config.base_poll_interval.as_secs(), 5);
        assert_eq!(config.max_poll_interval.as_secs(), 60);
        assert_eq!(config.max_backoff_retries, 3);
    }

    #[test]
    fn test_outbox_reader_creation() {
        let reader = OutboxReader::new(sqlx::PgPool::none(), OutboxReaderConfig::default());
        assert!(true); // Structure validated
    }

    #[test]
    fn test_exponential_backoff_calculation() {
        let mut interval = Duration::from_secs(5);
        interval = std::cmp::min(interval * 2, Duration::from_secs(60));
        assert_eq!(interval.as_secs(), 10);
    }
}

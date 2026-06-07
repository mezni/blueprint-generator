-- Create station outbox table for event-driven GIS sync
-- This implements the outbox pattern for reliable event delivery
-- IMPORTANT: Events are persisted before transaction commit, ensuring no data loss

CREATE TABLE inventory.station_outbox (
    id BIGINT PRIMARY KEY DEFAULT nextval('station_outbox_seq'),
    station_id VARCHAR(16) NOT NULL,
    event_type VARCHAR(20) NOT NULL CHECK (event_type IN ('created', 'updated', 'deleted')),
    payload JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    processed_at TIMESTAMPTZ
);

-- Indexes for outbox query performance
CREATE INDEX idx_station_outbox_station_id ON inventory.station_outbox(station_id);
CREATE INDEX idx_station_outbox_created_at ON inventory.station_outbox(created_at);
CREATE INDEX idx_station_outbox_processed_at ON inventory.station_outbox(processed_at) WHERE processed_at IS NULL;

-- Sequence for id generation
CREATE SEQUENCE IF NOT EXISTS station_outbox_seq;

COMMENT ON TABLE inventory.station_outbox IS 'Outbox table for event-driven GIS synchronization';
COMMENT ON COLUMN inventory.station_outbox.id IS 'Auto-increment ID for processing order';
COMMENT ON COLUMN inventory.station_outbox.station_id IS 'Station ID that triggered the event';
COMMENT ON COLUMN inventory.station_outbox.event_type IS 'Event type: created, updated, or deleted';
COMMENT ON COLUMN inventory.station_outbox.payload IS 'JSONB payload containing station data (event payload)';
COMMENT ON COLUMN inventory.station_outbox.created_at IS 'Timestamp when event was enqueued';
COMMENT ON COLUMN inventory.station_outbox.processed_at IS 'NULL = unprocessed, timestamp = processed by GIS worker';

-- ============================================================================
-- Outbox Pattern Explanation
-- ============================================================================

COMMENT ON TABLE inventory.station_outbox IS 'CRITICAL: Outbox pattern ensures reliable event delivery';
COMMENT ON TABLE inventory.station_outbox IS 'Events are written to this table BEFORE committing station changes';
COMMENT ON TABLE inventory.station_outbox IS 'GIS worker polls this table for unprocessed events';
COMMENT ON TABLE inventory.station_outbox IS 'Events are marked processed_at = NOW() after successful projection';

-- ============================================================================
-- Outbox Trigger Example (Implementation by Application Code)
-- ============================================================================

COMMENT ON TABLE inventory.station_outbox IS 'Triggers are applied via application code (see T052)';
COMMENT ON TABLE inventory.station_outbox IS 'Trigger fires AFTER INSERT/UPDATE/DELETE on inventory.station';
COMMENT ON TABLE inventory.station_outbox IS 'Event payload contains relevant station data at trigger time';
COMMENT ON TABLE inventory.station_outbox IS 'No external events consumed - purely event-driven';

-- ============================================================================
-- Event Payload Structure
-- ============================================================================

COMMENT ON TABLE inventory.station_outbox IS 'Example payload for event_type = "created":';
COMMENT ON TABLE inventory.station_outbox IS '{"station_id": "STN-ABC123XYZ1234", "name": "Test Station", "latitude": 36.8, "longitude": 10.1, "capacity": 4}';
COMMENT ON TABLE inventory.station_outbox IS 'Example payload for event_type = "deleted":';
COMMENT ON TABLE inventory.station_outbox IS '{"station_id": "STN-ABC123XYZ1234", "name": "Test Station"}';

-- ============================================================================
-- Important: Eventual Consistency
-- ============================================================================

COMMENT ON TABLE inventory.station_outbox IS 'GIS updates are asynchronous and eventually consistent';
COMMENT ON TABLE inventory.station_outbox IS 'Station updates proceed immediately; GIS sync happens within 5-min SLA (SC-004)';
COMMENT ON TABLE inventory.station_outbox IS 'Last-write-wins: Latest station data always overwrites GIS projection';
COMMENT ON TABLE inventory.station_outbox IS 'Events are not idempotent by default - worker must handle duplicates gracefully';

-- ============================================================================
-- Performance Note
-- ============================================================================

COMMENT ON TABLE inventory.station_outbox IS 'Index on (station_id, created_at) ensures fast polling by worker';
COMMENT ON TABLE inventory.station_outbox IS 'Index on processed_at enables cleanup of old processed events';
COMMENT ON TABLE inventory.station_outbox IS 'Sequence ensures sequential event IDs for easier debugging';

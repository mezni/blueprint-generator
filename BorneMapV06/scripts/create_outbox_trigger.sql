-- Database Trigger for Station Outbox Events
-- Creates a trigger that automatically logs station changes to the outbox table

-- ============================================================================
-- Create the outbox trigger function
-- ============================================================================

CREATE OR REPLACE FUNCTION inventory.station_outbox_trigger()
RETURNS TRIGGER AS $$
BEGIN
    -- Insert a new outbox event for INSERT, UPDATE, or DELETE
    INSERT INTO inventory.station_outbox (
        aggregate_type,
        aggregate_id,
        event_type,
        payload,
        processed
    ) VALUES (
        'station',
        NEW.id,
        CASE
            WHEN TG_OP = 'INSERT' THEN 'Created'
            WHEN TG_OP = 'UPDATE' THEN 'Updated'
            WHEN TG_OP = 'DELETE' THEN 'Deleted'
            ELSE 'Unknown'
        END,
        ROW_TO_JSON(NEW),
        FALSE
    );

    RETURN NEW;
EXCEPTION
    WHEN OTHERS THEN
        -- Log errors but don't prevent the main operation
        RAISE WARNING 'Outbox trigger error for station %: %', COALESCE(NEW.id, OLD.id), SQLERRM;
        RETURN COALESCE(NEW, OLD);
END;
$$ LANGUAGE plpgsql;

-- ============================================================================
-- Create the trigger on inventory.station
-- ============================================================================

DROP TRIGGER IF EXISTS station_outbox_trigger ON inventory.station;
CREATE TRIGGER station_outbox_trigger
    BEFORE INSERT OR UPDATE OR DELETE ON inventory.station
    FOR EACH ROW
    EXECUTE FUNCTION inventory.station_outbox_trigger();

-- ============================================================================
-- Create the outbox table if it doesn't exist
-- ============================================================================

CREATE TABLE IF NOT EXISTS inventory.station_outbox (
    id BIGSERIAL PRIMARY KEY,
    aggregate_type TEXT NOT NULL,
    aggregate_id TEXT NOT NULL,
    event_type TEXT NOT NULL,
    payload JSONB,
    processed BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS station_outbox_processed_idx ON inventory.station_outbox(processed);
CREATE INDEX IF NOT EXISTS station_outbox_aggregate_idx ON inventory.station_outbox(aggregate_type, aggregate_id);

-- ============================================================================
-- Verify trigger creation
-- ============================================================================

SELECT
    t.tgname as trigger_name,
    t.tgenabled as trigger_enabled,
    p.proname as function_name,
    CASE
        WHEN t.tgtype & 8 = 8 THEN 'INSERT'
        WHEN t.tgtype & 4 = 4 THEN 'UPDATE'
        WHEN t.tgtype & 2 = 2 THEN 'DELETE'
        WHEN t.tgtype & 1 = 1 THEN 'TRUNCATE'
        ELSE 'UNKNOWN'
    END as event_type,
    c.relname as table_name
FROM pg_trigger t
JOIN pg_proc p ON t.tgfoid = p.oid
JOIN pg_class c ON t.tgrelid = c.oid
WHERE t.tgname = 'station_outbox_trigger'
  AND c.relname = 'station';

SELECT 'Outbox trigger created successfully' as message;

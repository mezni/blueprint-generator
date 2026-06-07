CREATE OR REPLACE FUNCTION inventory.trg_station_geom_fn()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.latitude IS NOT NULL AND NEW.longitude IS NOT NULL THEN
        NEW.geom := ST_SetSRID(ST_MakePoint(NEW.longitude, NEW.latitude), 4326);
    ELSE
        NEW.geom := NULL;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trg_station_geom ON inventory.station;
CREATE TRIGGER trg_station_geom
    BEFORE INSERT OR UPDATE OF latitude, longitude ON inventory.station
    FOR EACH ROW
    EXECUTE FUNCTION inventory.trg_station_geom_fn();

CREATE OR REPLACE FUNCTION inventory.trg_partner_delete_guard_fn()
RETURNS TRIGGER AS $$
BEGIN
    IF OLD.deleted_at IS NULL AND NEW.deleted_at IS NOT NULL THEN
        IF EXISTS (
            SELECT 1 FROM inventory.station
            WHERE partner_id = NEW.id
              AND is_live = true
              AND deleted_at IS NULL
        ) THEN
            RAISE EXCEPTION 'ACTIVE_STATIONS_EXIST'
                USING HINT = 'Cannot soft-delete partner with active stations';
        END IF;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trg_partner_delete_guard ON inventory.partner;
CREATE TRIGGER trg_partner_delete_guard
    BEFORE UPDATE ON inventory.partner
    FOR EACH ROW
    EXECUTE FUNCTION inventory.trg_partner_delete_guard_fn();

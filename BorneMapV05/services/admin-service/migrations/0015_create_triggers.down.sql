DROP TRIGGER IF EXISTS trg_station_geom ON inventory.station;
DROP FUNCTION IF EXISTS inventory.trg_station_geom_fn();

DROP TRIGGER IF EXISTS trg_partner_delete_guard ON inventory.partner;
DROP FUNCTION IF EXISTS inventory.trg_partner_delete_guard_fn();

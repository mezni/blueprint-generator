CREATE OR REPLACE VIEW inventory.visible_stations AS
SELECT *
FROM inventory.station
WHERE is_live = true
  AND deleted_at IS NULL
  AND status = 'active'
  AND is_public = true;

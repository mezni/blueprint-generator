-- Seed 10,000 visible stations across Tunisia for performance benchmarking
-- Requires PostGIS. Run against platform_db.
--
-- Usage:
--   psql -d platform_db -f seed-10000-stations.sql
--
-- Benchmark after seeding:
--   EXPLAIN ANALYZE SELECT s.id, s.name, s.latitude, s.longitude,
--     ROUND((ST_Distance(s.geom, ST_SetSRID(ST_MakePoint(10.1815, 36.8065), 4326)::geography) / 1000.0)::numeric, 2)::float8 AS distance_km
--   FROM inventory.station s
--   WHERE s.is_live = true AND s.deleted_at IS NULL AND s.status = 'active' AND s.is_public = true
--     AND ST_DWithin(s.geom, ST_SetSRID(ST_MakePoint(10.1815, 36.8065), 4326)::geography, 50000)
--   ORDER BY distance_km ASC NULLS LAST LIMIT 20 OFFSET 0;
--
-- Search benchmark:
--   EXPLAIN ANALYZE SELECT s.id, s.name FROM inventory.station s
--   WHERE s.is_live = true AND s.deleted_at IS NULL AND s.status = 'active' AND s.is_public = true
--     AND (s.name ILIKE '%Tunis%' OR s.city ILIKE '%Tunis%' OR COALESCE(s.description, '') ILIKE '%Tunis%')
--   ORDER BY s.name ASC LIMIT 20 OFFSET 0;

-- Ensure a seed partner exists
INSERT INTO inventory.partner (id, name, status, created_at, updated_at, created_by, updated_by)
VALUES ('PRT-SEED-00000', 'Seed Data Partner', 'active', NOW(), NOW(), 'seed', 'seed')
ON CONFLICT (id) DO NOTHING;

-- Generate 10,000 visible stations with random coordinates in Tunisia
INSERT INTO inventory.station (id, partner_id, name, description, latitude, longitude, geom, status, is_live, is_public, city, country, created_at, updated_at, created_by, updated_by)
SELECT
  'STN-SEED-' || LPAD(i::text, 6, '0'),
  'PRT-SEED-00000',
  CASE (random() * 19)::int
    WHEN 0 THEN 'Tunis Centre Charge'
    WHEN 1 THEN 'Sfax Sud Station'
    WHEN 2 THEN 'Sousse Plage EV'
    WHEN 3 THEN 'Ettadhamen Power'
    WHEN 4 THEN 'Kairouan Ouest'
    WHEN 5 THEN 'Bizerte Nord'
    WHEN 6 THEN 'Gabes Ville'
    WHEN 7 THEN 'Sakiet Ezzit'
    WHEN 8 THEN 'Ariana Park'
    WHEN 9 THEN 'La Marsa Plage'
    WHEN 10 THEN 'Nabeul Corniche'
    WHEN 11 THEN 'Hammamet Centre'
    WHEN 12 THEN 'Monastir Marina'
    WHEN 13 THEN 'Menzel Bourguiba'
    WHEN 14 THEN 'Medenine Sud'
    WHEN 15 THEN 'Beja Ouest'
    WHEN 16 THEN 'El Kef Centre'
    WHEN 17 THEN 'Tozeur Oasis'
    WHEN 18 THEN 'Tataouine Ville'
    ELSE 'Djerba Midoun'
  END || ' #' || i,
  CASE (random() * 4)::int
    WHEN 0 THEN 'Station de recharge rapide'
    WHEN 1 THEN 'Borne de recharge standard'
    WHEN 2 THEN 'Superchargeur urbain'
    WHEN 3 THEN 'Station de recharge'
    ELSE 'Point de charge'
  END,
  30.0 + random() * 7.0,
  7.5 + random() * 3.5,
  ST_SetSRID(ST_MakePoint(7.5 + random() * 3.5, 30.0 + random() * 7.0), 4326),
  'active',
  true,
  true,
  CASE (random() * 9)::int
    WHEN 0 THEN 'Tunis'
    WHEN 1 THEN 'Sfax'
    WHEN 2 THEN 'Sousse'
    WHEN 3 THEN 'Ettadhamen'
    WHEN 4 THEN 'Kairouan'
    WHEN 5 THEN 'Bizerte'
    WHEN 6 THEN 'Gabes'
    WHEN 7 THEN 'Ariana'
    WHEN 8 THEN 'Nabeul'
    ELSE 'Hammamet'
  END,
  'Tunisia',
  NOW(),
  NOW(),
  'seed',
  'seed'
FROM generate_series(1, 10000) AS i;

-- Create a GIST index if not already present (should already exist from migration 0003)
CREATE INDEX IF NOT EXISTS idx_station_geom_seed ON inventory.station USING GIST (geom);

-- Analyze tables for query planner
ANALYZE inventory.station;
ANALYZE inventory.charger;

-- Report count
SELECT COUNT(*) AS total_visible_stations FROM inventory.station
WHERE is_live = true AND deleted_at IS NULL AND status = 'active' AND is_public = true;

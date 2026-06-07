-- EXPLAIN ANALYZE for driver-service spatial queries
-- Run these against platform_db AFTER seeding 10,000 stations.
--
-- Usage:
--   psql -d platform_db -f explain-analyze-spatial.sql
--
-- Expected: Index Scan using idx_station_geom (GIST) + idx_station_live_public (btree)
-- Red flag: Seq Scan on inventory.station = GIST index not being used

-- 1. Radius query (near Tunis, 10km radius)
EXPLAIN ANALYZE
SELECT s.id, s.name, s.latitude, s.longitude,
  ROUND((ST_Distance(s.geom, ST_SetSRID(ST_MakePoint(10.1815, 36.8065), 4326)::geography) / 1000.0)::numeric, 2)::float8 AS distance_km
FROM inventory.station s
WHERE s.is_live = true AND s.deleted_at IS NULL AND s.status = 'active' AND s.is_public = true
  AND ST_DWithin(s.geom, ST_SetSRID(ST_MakePoint(10.1815, 36.8065), 4326)::geography, 10000)
ORDER BY distance_km ASC NULLS LAST
LIMIT 20 OFFSET 0;

-- 2. Radius query (wide, 50km max radius)
EXPLAIN ANALYZE
SELECT s.id, s.name, s.latitude, s.longitude,
  ROUND((ST_Distance(s.geom, ST_SetSRID(ST_MakePoint(10.1815, 36.8065), 4326)::geography) / 1000.0)::numeric, 2)::float8 AS distance_km
FROM inventory.station s
WHERE s.is_live = true AND s.deleted_at IS NULL AND s.status = 'active' AND s.is_public = true
  AND ST_DWithin(s.geom, ST_SetSRID(ST_MakePoint(10.1815, 36.8065), 4326)::geography, 50000)
ORDER BY distance_km ASC NULLS LAST
LIMIT 20 OFFSET 0;

-- 3. Count query with spatial filter
EXPLAIN ANALYZE
SELECT COUNT(*)
FROM inventory.station s
WHERE s.is_live = true AND s.deleted_at IS NULL AND s.status = 'active' AND s.is_public = true
  AND ST_DWithin(s.geom, ST_SetSRID(ST_MakePoint(10.1815, 36.8065), 4326)::geography, 10000);

-- 4. Search query (ILIKE on name/city/description)
EXPLAIN ANALYZE
SELECT s.id, s.name
FROM inventory.station s
WHERE s.is_live = true AND s.deleted_at IS NULL AND s.status = 'active' AND s.is_public = true
  AND (s.name ILIKE '%Tunis%' OR s.city ILIKE '%Tunis%' OR COALESCE(s.description, '') ILIKE '%Tunis%')
ORDER BY s.name ASC
LIMIT 20 OFFSET 0;

-- 5. Station detail (PK lookup)
EXPLAIN ANALYZE
SELECT s.id, s.name, s.description, s.latitude, s.longitude, s.city, s.country,
  s.created_at, s.updated_at
FROM inventory.station s
WHERE s.id = 'STN-SEED-000001'
  AND s.is_live = true AND s.deleted_at IS NULL AND s.status = 'active' AND s.is_public = true;

-- 6. Distance calculation for detail endpoint
EXPLAIN ANALYZE
SELECT ROUND((ST_Distance(s.geom,
  ST_SetSRID(ST_MakePoint(10.1815, 36.8065), 4326)::geography) / 1000.0)::numeric, 2)::float8
FROM inventory.station s
WHERE s.id = 'STN-SEED-000001';

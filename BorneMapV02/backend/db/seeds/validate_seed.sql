-- Validate partner ID patterns
SELECT 'partner_pattern' AS check_name,
       COUNT(*) AS total,
       COUNT(*) FILTER (WHERE id ~ '^prt-[a-f0-9]{8}$') AS valid,
       COUNT(*) FILTER (WHERE id !~ '^prt-[a-f0-9]{8}$') AS invalid
FROM partners;

-- Validate station ID patterns
SELECT 'station_pattern' AS check_name,
       COUNT(*) AS total,
       COUNT(*) FILTER (WHERE id ~ '^stn-[a-f0-9]{8}$') AS valid,
       COUNT(*) FILTER (WHERE id !~ '^stn-[a-f0-9]{8}$') AS invalid
FROM stations;

-- Validate charger ID patterns
SELECT 'charger_pattern' AS check_name,
       COUNT(*) AS total,
       COUNT(*) FILTER (WHERE id ~ '^chg-[a-f0-9]{8}$') AS valid,
       COUNT(*) FILTER (WHERE id !~ '^chg-[a-f0-9]{8}$') AS invalid
FROM chargers;

-- Count partners
SELECT 'partner_count' AS check_name, COUNT(*) AS value FROM partners;

-- Count stations
SELECT 'station_count' AS check_name, COUNT(*) AS value FROM stations;

-- Count chargers
SELECT 'charger_count' AS check_name, COUNT(*) AS value FROM chargers;

-- Station coordinate ranges
SELECT 'lat_range' AS check_name,
       MIN(ST_Y(geom::geometry)) AS min_val,
       MAX(ST_Y(geom::geometry)) AS max_val
FROM stations;

SELECT 'lng_range' AS check_name,
       MIN(ST_X(geom::geometry)) AS min_val,
       MAX(ST_X(geom::geometry)) AS max_val
FROM stations;

-- FK integrity: stations with missing partner
SELECT 'station_orphans' AS check_name, COUNT(*) AS value
FROM stations s LEFT JOIN partners p ON p.id = s.partner_id
WHERE p.id IS NULL;

-- FK integrity: chargers with missing station
SELECT 'charger_orphans' AS check_name, COUNT(*) AS value
FROM chargers c LEFT JOIN stations s ON s.id = c.station_id
WHERE s.id IS NULL;

-- Station-to-charger ratio
SELECT 'avg_chargers_per_station' AS check_name,
       ROUND(AVG(cnt), 2) AS value
FROM (SELECT station_id, COUNT(*) AS cnt FROM chargers GROUP BY station_id) sub;

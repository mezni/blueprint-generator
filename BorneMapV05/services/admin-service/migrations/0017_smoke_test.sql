-- Sprint 4 Smoke Test: verifies all migrations are correct
-- Run with: psql "$PLATFORM_DB_URL" -f 0017_smoke_test.sql
-- Expected: all assertions pass, no errors

-- 1. Verify schemas exist
DO $$ BEGIN
    ASSERT EXISTS (SELECT 1 FROM information_schema.schemata WHERE schema_name = 'inventory'), 'inventory schema missing';
    ASSERT EXISTS (SELECT 1 FROM information_schema.schemata WHERE schema_name = 'users'), 'users schema missing';
    ASSERT EXISTS (SELECT 1 FROM information_schema.schemata WHERE schema_name = 'gis'), 'gis schema missing';
END $$;

-- 2. Verify PostGIS enabled
DO $$ BEGIN
    ASSERT EXISTS (SELECT 1 FROM pg_extension WHERE extname = 'postgis'), 'PostGIS extension not enabled';
END $$;

-- 3. Verify inventory tables exist
DO $$ BEGIN
    ASSERT EXISTS (SELECT 1 FROM information_schema.tables WHERE table_schema = 'inventory' AND table_name = 'partner'), 'inventory.partner missing';
    ASSERT EXISTS (SELECT 1 FROM information_schema.tables WHERE table_schema = 'inventory' AND table_name = 'station'), 'inventory.station missing';
    ASSERT EXISTS (SELECT 1 FROM information_schema.tables WHERE table_schema = 'inventory' AND table_name = 'charger'), 'inventory.charger missing';
    ASSERT EXISTS (SELECT 1 FROM information_schema.tables WHERE table_schema = 'inventory' AND table_name = 'station_availability'), 'inventory.station_availability missing';
END $$;

-- 4. Verify users tables exist
DO $$ BEGIN
    ASSERT EXISTS (SELECT 1 FROM information_schema.tables WHERE table_schema = 'users' AND table_name = 'user_account'), 'users.user_account missing';
    ASSERT EXISTS (SELECT 1 FROM information_schema.tables WHERE table_schema = 'users' AND table_name = 'user_profile'), 'users.user_profile missing';
    ASSERT EXISTS (SELECT 1 FROM information_schema.tables WHERE table_schema = 'users' AND table_name = 'partner_membership'), 'users.partner_membership missing';
    ASSERT EXISTS (SELECT 1 FROM information_schema.tables WHERE table_schema = 'users' AND table_name = 'favorite_station'), 'users.favorite_station missing';
    ASSERT EXISTS (SELECT 1 FROM information_schema.tables WHERE table_schema = 'users' AND table_name = 'station_review'), 'users.station_review missing';
END $$;

-- 5. Verify gis.sync_queue exists
DO $$ BEGIN
    ASSERT EXISTS (SELECT 1 FROM information_schema.tables WHERE table_schema = 'gis' AND table_name = 'sync_queue'), 'gis.sync_queue missing';
END $$;

-- 6. Verify visible_stations view exists
DO $$ BEGIN
    ASSERT EXISTS (SELECT 1 FROM information_schema.views WHERE table_schema = 'inventory' AND table_name = 'visible_stations'), 'inventory.visible_stations view missing';
END $$;

-- 7. Verify GIST index on station.geom
DO $$ BEGIN
    ASSERT EXISTS (
        SELECT 1 FROM pg_indexes
        WHERE schemaname = 'inventory' AND tablename = 'station' AND indexdef LIKE '%GIST%geom%'
    ), 'GIST index on station.geom missing';
END $$;

-- 8. Verify geom trigger auto-populates
DO $$
DECLARE
    test_geom GEOGRAPHY;
BEGIN
    INSERT INTO inventory.station (id, partner_id, name, latitude, longitude, status, is_live, is_public, created_by, updated_by)
    VALUES ('STN-SMOKE-TEST', (SELECT id FROM inventory.partner LIMIT 1), 'Smoke Test Station', 36.8065, 10.1815, 'active', true, true, 'SMOKE', 'SMOKE')
    ON CONFLICT (id) DO NOTHING;

    SELECT geom INTO test_geom FROM inventory.station WHERE id = 'STN-SMOKE-TEST';
    ASSERT test_geom IS NOT NULL, 'geom trigger did not auto-populate';
    ASSERT ST_SRID(test_geom) = 4326, 'geom SRID is not 4326';

    DELETE FROM inventory.station WHERE id = 'STN-SMOKE-TEST';
END $$;

-- 9. Verify partner delete guard trigger
DO $$
BEGIN
    INSERT INTO inventory.partner (id, name, type, status, created_by, updated_by)
    VALUES ('PRT-SMOKE-GUARD', 'Guard Test Partner', 'business', 'active', 'SMOKE', 'SMOKE')
    ON CONFLICT (id) DO NOTHING;

    INSERT INTO inventory.station (id, partner_id, name, latitude, longitude, status, is_live, is_public, created_by, updated_by)
    VALUES ('STN-SMOKE-GUARD', 'PRT-SMOKE-GUARD', 'Guard Test Station', 36.8, 10.1, 'active', true, true, 'SMOKE', 'SMOKE')
    ON CONFLICT (id) DO NOTHING;

    BEGIN
        UPDATE inventory.partner SET deleted_at = NOW() WHERE id = 'PRT-SMOKE-GUARD';
        ASSERT FALSE, 'Partner delete guard did NOT block deletion with active stations';
    EXCEPTION WHEN OTHERS THEN
        ASSERT SQLERRM LIKE '%ACTIVE_STATIONS_EXIST%', 'Unexpected error: ' || SQLERRM;
    END;

    DELETE FROM inventory.station WHERE id = 'STN-SMOKE-GUARD';
    DELETE FROM inventory.partner WHERE id = 'PRT-SMOKE-GUARD';
END $$;

-- 10. Verify visible_stations view filters correctly
DO $$
DECLARE
    vis_count INTEGER;
    total_live_public_active INTEGER;
BEGIN
    SELECT COUNT(*) INTO vis_count FROM inventory.visible_stations;
    SELECT COUNT(*) INTO total_live_public_active
    FROM inventory.station
    WHERE is_live = true AND deleted_at IS NULL AND status = 'active' AND is_public = true;

    ASSERT vis_count = total_live_public_active, 'visible_stations count mismatch';
END $$;

-- 11. Verify CHECK constraints reject invalid data
DO $$
BEGIN
    BEGIN
        INSERT INTO inventory.partner (id, name, type, status, created_by, updated_by)
        VALUES ('PRT-BAD-TYPE', 'Bad', 'invalid_type', 'active', 'SMOKE', 'SMOKE');
        ASSERT FALSE, 'partner.type CHECK did not reject invalid value';
    EXCEPTION WHEN OTHERS THEN
        ASSERT SQLERRM LIKE '%violates check constraint%', 'Unexpected error for bad partner type: ' || SQLERRM;
    END;

    BEGIN
        INSERT INTO inventory.station (id, partner_id, name, latitude, longitude, status, is_live, is_public, created_by, updated_by)
        VALUES ('STN-BAD-LAT', (SELECT id FROM inventory.partner LIMIT 1), 'Bad', 999.0, 10.0, 'active', true, true, 'SMOKE', 'SMOKE');
        ASSERT FALSE, 'station.latitude CHECK did not reject out-of-range value';
    EXCEPTION WHEN OTHERS THEN
        ASSERT SQLERRM LIKE '%violates check constraint%', 'Unexpected error for bad latitude: ' || SQLERRM;
    END;

    BEGIN
        INSERT INTO gis.sync_queue (id, entity_type, entity_id, operation, status)
        VALUES ('SQ-BAD-OP', 'station', 'fake', 'invalid_op', 'pending');
        ASSERT FALSE, 'gis.sync_queue.operation CHECK did not reject invalid value';
    EXCEPTION WHEN OTHERS THEN
        ASSERT SQLERRM LIKE '%violates check constraint%', 'Unexpected error for bad operation: ' || SQLERRM;
    END;
END $$;

-- 12. Verify FK constraint rejects orphan station
DO $$
BEGIN
    BEGIN
        INSERT INTO inventory.station (id, partner_id, name, latitude, longitude, status, is_live, is_public, created_by, updated_by)
        VALUES ('STN-ORPHAN', 'PRT-NONEXISTENT', 'Orphan', 36.8, 10.1, 'active', true, true, 'SMOKE', 'SMOKE');
        ASSERT FALSE, 'FK constraint did not reject orphan station';
    EXCEPTION WHEN OTHERS THEN
        ASSERT SQLERRM LIKE '%violates foreign key constraint%', 'Unexpected error for FK violation: ' || SQLERRM;
    END;
END $$;

-- 13. Verify bbox spatial query uses GIST index
DO $$
DECLARE
    plan_text TEXT;
BEGIN
    plan_text := (
        SELECT query_plan FROM (
            EXPLAIN SELECT * FROM inventory.station
            WHERE geom && ST_MakeEnvelope(9.0, 36.0, 11.0, 37.5, 4326)
        ) t
    );
    ASSERT plan_text LIKE '%Index Scan%', 'Spatial query does not use index scan. Plan: ' || plan_text;
    ASSERT plan_text LIKE '%GIST%' OR plan_text LIKE '%gist%', 'Spatial query does not use GIST index. Plan: ' || plan_text;
END $$;

SELECT 'SMOKE TEST PASSED' AS result;

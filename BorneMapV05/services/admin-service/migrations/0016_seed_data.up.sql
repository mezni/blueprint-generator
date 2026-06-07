CREATE OR REPLACE FUNCTION public.generate_ulid()
RETURNS TEXT AS $$
DECLARE
    timestamp_bytes BYTEA := E'\\000\\000\\000\\000\\000\\000';
    random_bytes    BYTEA := E'\\000\\000\\000\\000\\000\\000\\000\\000\\000\\000';
    ulid_text       TEXT;
    encoding        TEXT := '0123456789ABCDEFGHJKMNPQRSTVWXYZ';
    i               INTEGER;
    ts_ms           BIGINT;
BEGIN
    ts_ms := EXTRACT(EPOCH FROM CLOCK_TIMESTAMP()) * 1000;

    FOR i IN REVERSE 5..0 LOOP
        timestamp_bytes := SET_BYTE(timestamp_bytes, i, (ts_ms % 256)::INT);
        ts_ms := ts_ms >> 8;
    END LOOP;

    random_bytes := gen_random_bytes(10);

    ulid_text := '';
    FOR i IN 0..5 LOOP
        ulid_text := ulid_text || SUBSTRING(encoding FROM (GET_BYTE(timestamp_bytes, i) >> 5 + 1)::INT + 1 FOR 1);
        ulid_text := ulid_text || SUBSTRING(encoding FROM ((GET_BYTE(timestamp_bytes, i) & 31) * 2 + (CASE WHEN i < 5 THEN GET_BYTE(timestamp_bytes, i+1) >> 7 ELSE 0 END) + 1)::INT + 1 FOR 1);
    END LOOP;

    FOR i IN 0..9 LOOP
        ulid_text := ulid_text || SUBSTRING(encoding FROM (GET_BYTE(random_bytes, i) >> 3 + 1)::INT + 1 FOR 1);
        IF i < 9 THEN
            ulid_text := ulid_text || SUBSTRING(encoding FROM ((GET_BYTE(random_bytes, i) & 7) * 4 + (GET_BYTE(random_bytes, i+1) >> 6) + 1)::INT + 1 FOR 1);
        ELSE
            ulid_text := ulid_text || SUBSTRING(encoding FROM ((GET_BYTE(random_bytes, i) & 7) * 4 + 1)::INT + 1 FOR 1);
        END IF;
    END LOOP;

    RETURN ulid_text;
END;
$$ LANGUAGE plpgsql VOLATILE;

INSERT INTO inventory.partner (id, name, type, status, created_by, updated_by)
VALUES
    ('PRT-' || public.generate_ulid(), 'Tunisia Charging Co.', 'business', 'active', 'SEED', 'SEED'),
    ('PRT-' || public.generate_ulid(), 'Sfax Power Solutions', 'business', 'active', 'SEED', 'SEED'),
    ('PRT-' || public.generate_ulid(), 'Private Charger Hub', 'private', 'active', 'SEED', 'SEED')
ON CONFLICT (id) DO NOTHING;

WITH p1 AS (SELECT id AS p1_id FROM inventory.partner WHERE name = 'Tunisia Charging Co.' LIMIT 1),
     p2 AS (SELECT id AS p2_id FROM inventory.partner WHERE name = 'Sfax Power Solutions' LIMIT 1),
     p3 AS (SELECT id AS p3_id FROM inventory.partner WHERE name = 'Private Charger Hub' LIMIT 1)
INSERT INTO inventory.station (id, partner_id, name, description, latitude, longitude, status, is_live, is_public, city, country, created_by, updated_by)
SELECT 'STN-' || public.generate_ulid(), p1_id, 'Tunis Central Station', 'Main hub in downtown Tunis', 36.8065, 10.1815, 'active', true, true, 'Tunis', 'Tunisia', 'SEED', 'SEED' FROM p1
UNION ALL
SELECT 'STN-' || public.generate_ulid(), p1_id, 'Tunis Airport Charger', 'Near Tunis-Carthage Airport', 36.8510, 10.2272, 'active', true, true, 'Tunis', 'Tunisia', 'SEED', 'SEED' FROM p1
UNION ALL
SELECT 'STN-' || public.generate_ulid(), p1_id, 'La Marsa Quick Charge', 'Coastal fast charger', 36.8783, 10.3247, 'active', true, true, 'La Marsa', 'Tunisia', 'SEED', 'SEED' FROM p1
UNION ALL
SELECT 'STN-' || public.generate_ulid(), p1_id, 'Carthage Heritage Station', 'Near ancient Carthage', 36.8569, 10.3325, 'active', true, true, 'Carthage', 'Tunisia', 'SEED', 'SEED' FROM p1
UNION ALL
SELECT 'STN-' || public.generate_ulid(), p2_id, 'Sfax City Center', 'Downtown Sfax charging', 34.7398, 10.7600, 'active', true, true, 'Sfax', 'Tunisia', 'SEED', 'SEED' FROM p2
UNION ALL
SELECT 'STN-' || public.generate_ulid(), p2_id, 'Sfax Industrial Zone', 'Industrial area charger', 34.7500, 10.7100, 'active', true, true, 'Sfax', 'Tunisia', 'SEED', 'SEED' FROM p2
UNION ALL
SELECT 'STN-' || public.generate_ulid(), p2_id, 'Sousse Marina Charger', 'Marina district station', 35.8254, 10.6369, 'active', true, true, 'Sousse', 'Tunisia', 'SEED', 'SEED' FROM p2
UNION ALL
SELECT 'STN-' || public.generate_ulid(), p2_id, 'Sousse Medina Station', 'Old town charging point', 35.8319, 10.5970, 'active', true, true, 'Sousse', 'Tunisia', 'SEED', 'SEED' FROM p2
UNION ALL
SELECT 'STN-' || public.generate_ulid(), p3_id, 'Hammamet Private', 'Private residential charger', 36.4000, 10.6100, 'active', true, false, 'Hammamet', 'Tunisia', 'SEED', 'SEED' FROM p3
UNION ALL
SELECT 'STN-' || public.generate_ulid(), p1_id, 'Bizerte Port Station', 'Port area charger', 37.2746, 9.8739, 'inactive', false, true, 'Bizerte', 'Tunisia', 'SEED', 'SEED' FROM p1
ON CONFLICT (id) DO NOTHING;

INSERT INTO inventory.charger (id, station_id, type, power_kw, status, created_by, updated_by)
SELECT 'CHG-' || public.generate_ulid(), s.id, 'CCS', 50.0, 'available', 'SEED', 'SEED'
FROM inventory.station s WHERE s.name IN ('Tunis Central Station', 'Sfax City Center') LIMIT 2
ON CONFLICT (id) DO NOTHING;

INSERT INTO inventory.charger (id, station_id, type, power_kw, status, created_by, updated_by)
SELECT 'CHG-' || public.generate_ulid(), s.id, 'Type2', 22.0, 'available', 'SEED', 'SEED'
FROM inventory.station s WHERE s.name IN ('Tunis Central Station', 'Tunis Airport Charger', 'La Marsa Quick Charge', 'Carthage Heritage Station', 'Sfax City Center', 'Sfax Industrial Zone', 'Sousse Marina Charger') LIMIT 7
ON CONFLICT (id) DO NOTHING;

INSERT INTO inventory.charger (id, station_id, type, power_kw, status, created_by, updated_by)
SELECT 'CHG-' || public.generate_ulid(), s.id, 'CHAdeMO', 62.5, 'available', 'SEED', 'SEED'
FROM inventory.station s WHERE s.name IN ('Tunis Airport Charger', 'Sousse Medina Station', 'Sousse Marina Charger', 'Hammamet Private', 'Bizerte Port Station', 'Carthage Heritage Station') LIMIT 6
ON CONFLICT (id) DO NOTHING;

INSERT INTO users.user_account (id, keycloak_user_id, email, status)
VALUES
    ('USR-' || public.generate_ulid(), 'kc-driver-001', 'ahmed@example.tn', 'active'),
    ('USR-' || public.generate_ulid(), 'kc-driver-002', 'fatma@example.tn', 'active'),
    ('USR-' || public.generate_ulid(), 'kc-driver-003', 'youssef@example.tn', 'active'),
    ('USR-' || public.generate_ulid(), 'kc-partner-001', 'partner1@example.tn', 'active'),
    ('USR-' || public.generate_ulid(), 'kc-admin-001', 'admin@example.tn', 'active')
ON CONFLICT (keycloak_user_id) DO NOTHING;

INSERT INTO users.partner_membership (user_id, partner_id, role)
SELECT ua.id, p.id, 'owner'
FROM users.user_account ua, inventory.partner p
WHERE ua.keycloak_user_id = 'kc-partner-001' AND p.name = 'Tunisia Charging Co.'
ON CONFLICT (user_id) DO NOTHING;

INSERT INTO users.favorite_station (user_id, station_id)
SELECT ua.id, s.id
FROM users.user_account ua, inventory.station s
WHERE ua.keycloak_user_id = 'kc-driver-001' AND s.name IN ('Tunis Central Station', 'La Marsa Quick Charge', 'Sousse Marina Charger')
ON CONFLICT (user_id, station_id) DO NOTHING;

INSERT INTO users.favorite_station (user_id, station_id)
SELECT ua.id, s.id
FROM users.user_account ua, inventory.station s
WHERE ua.keycloak_user_id = 'kc-driver-002' AND s.name IN ('Sfax City Center', 'Tunis Airport Charger')
ON CONFLICT (user_id, station_id) DO NOTHING;

INSERT INTO users.station_review (id, user_id, station_id, rating, comment, status)
SELECT 'REV-' || public.generate_ulid(), ua.id, s.id, 5, 'Excellent fast charging!', 'published'
FROM users.user_account ua, inventory.station s
WHERE ua.keycloak_user_id = 'kc-driver-001' AND s.name = 'Tunis Central Station'
ON CONFLICT (id) DO NOTHING;

INSERT INTO users.station_review (id, user_id, station_id, rating, comment, status)
SELECT 'REV-' || public.generate_ulid(), ua.id, s.id, 4, 'Good location, sometimes busy', 'published'
FROM users.user_account ua, inventory.station s
WHERE ua.keycloak_user_id = 'kc-driver-001' AND s.name = 'Tunis Airport Charger'
ON CONFLICT (id) DO NOTHING;

INSERT INTO users.station_review (id, user_id, station_id, rating, comment, status)
SELECT 'REV-' || public.generate_ulid(), ua.id, s.id, 3, 'Average experience', 'published'
FROM users.user_account ua, inventory.station s
WHERE ua.keycloak_user_id = 'kc-driver-002' AND s.name = 'Sfax City Center'
ON CONFLICT (id) DO NOTHING;

INSERT INTO users.station_review (id, user_id, station_id, rating, comment, status)
SELECT 'REV-' || public.generate_ulid(), ua.id, s.id, 5, 'Very convenient', 'published'
FROM users.user_account ua, inventory.station s
WHERE ua.keycloak_user_id = 'kc-driver-002' AND s.name = 'Tunis Central Station'
ON CONFLICT (id) DO NOTHING;

INSERT INTO users.station_review (id, user_id, station_id, rating, comment, status)
SELECT 'REV-' || public.generate_ulid(), ua.id, s.id, 2, 'Charger was offline', 'flagged'
FROM users.user_account ua, inventory.station s
WHERE ua.keycloak_user_id = 'kc-driver-003' AND s.name = 'Sousse Marina Charger'
ON CONFLICT (id) DO NOTHING;

DROP FUNCTION IF EXISTS public.generate_ulid();

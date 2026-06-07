-- Test Data Seed Script for BorneMap
-- Creates sample data for manual testing and validation

-- Use the inventory schema
SET search_path TO inventory;

-- ============================================================================
-- Partners
-- ============================================================================

INSERT INTO partner (id, name, email, status, created_at, updated_at) VALUES
('PRT-001', 'AutoMotive Tunis', 'partner1@automotive.tn', 'active', NOW(), NOW()),
('PRT-002', 'GreenCharge Ltd', 'partner2@greencharge.io', 'active', NOW(), NOW()),
('PRT-003', 'UrbanEV Solutions', 'partner3@urbanev.com', 'active', NOW(), NOW());

-- ============================================================================
-- Stations
-- ============================================================================

INSERT INTO station (id, partner_id, name, address, latitude, longitude, capacity, status, created_at, updated_at) VALUES
('STN-001', 'PRT-001', 'Tunis City Center', '18.9, 9.5, Tunis, Tunisia', 36.8065, 10.1815, 4, 'active', NOW(), NOW()),
('STN-002', 'PRT-001', 'Sidi Bou Said Station', '18.4, 9.7, Sidi Bou Said, Tunisia', 36.8768, 10.3282, 2, 'active', NOW(), NOW()),
('STN-003', 'PRT-002', 'Sfax Tech Park', '10.7769, 10.7603, Sfax, Tunisia', 34.7406, 10.7603, 6, 'active', NOW(), NOW()),
('STN-004', 'PRT-003', 'Monastir Beach', '35.7787, 10.8236, Monastir, Tunisia', 35.7787, 10.8236, 3, 'active', NOW(), NOW()),
('STN-005', 'PRT-002', 'Gabes Industrial Zone', '33.8810, 10.0920, Gabes, Tunisia', 33.8810, 10.0920, 8, 'active', NOW(), NOW()),
('STN-006', 'PRT-001', 'Bizerte Port', '37.2748, 9.8739, Bizerte, Tunisia', 37.2748, 9.8739, 5, 'active', NOW(), NOW()),
('STN-007', 'PRT-003', 'Ariana Mall', '36.8859, 10.1631, Ariana, Tunisia', 36.8859, 10.1631, 4, 'active', NOW(), NOW()),
('STN-008', 'PRT-002', 'Kairouan Historic Center', '35.6781, 10.0963, Kairouan, Tunisia', 35.6781, 10.0963, 3, 'active', NOW(), NOW()),
('STN-009', 'PRT-001', 'Djerba Resort', '33.7837, 10.8569, Djerba, Tunisia', 33.7837, 10.8569, 6, 'active', NOW(), NOW()),
('STN-010', 'PRT-003', 'Gafsa Mining Zone', '34.4254, 8.7880, Gafsa, Tunisia', 34.4254, 8.7880, 4, 'active', NOW(), NOW());

-- ============================================================================
-- Chargers per station
-- ============================================================================

INSERT INTO charger (id, station_id, type, connector_type, power_kw, status, created_at, updated_at) VALUES
('CHR-001', 'STN-001', 'DC Fast', 'CCS2', 150, 'active', NOW(), NOW()),
('CHR-002', 'STN-001', 'AC Slow', 'Type 2', 22, 'active', NOW(), NOW()),
('CHR-003', 'STN-001', 'AC Slow', 'Type 2', 22, 'maintenance', NOW(), NOW()),
('CHR-004', 'STN-002', 'DC Fast', 'CCS2', 100, 'active', NOW(), NOW()),
('CHR-005', 'STN-002', 'AC Slow', 'Type 2', 22, 'active', NOW(), NOW()),
('CHR-006', 'STN-003', 'DC Fast', 'CCS2', 120, 'active', NOW(), NOW()),
('CHR-007', 'STN-003', 'DC Fast', 'CCS2', 120, 'active', NOW(), NOW()),
('CHR-008', 'STN-003', 'AC Slow', 'Type 2', 22, 'active', NOW(), NOW()),
('CHR-009', 'STN-004', 'DC Fast', 'CCS2', 80, 'active', NOW(), NOW()),
('CHR-010', 'STN-005', 'DC Fast', 'CCS2', 150, 'active', NOW(), NOW()),
('CHR-011', 'STN-005', 'DC Fast', 'CCS2', 150, 'active', NOW(), NOW()),
('CHR-012', 'STN-005', 'AC Slow', 'Type 2', 22, 'active', NOW(), NOW()),
('CHR-013', 'STN-006', 'DC Fast', 'CCS2', 100, 'active', NOW(), NOW()),
('CHR-014', 'STN-007', 'DC Fast', 'CCS2', 150, 'active', NOW(), NOW()),
('CHR-015', 'STN-008', 'AC Slow', 'Type 2', 22, 'active', NOW(), NOW()),
('CHR-016', 'STN-009', 'DC Fast', 'CCS2', 120, 'active', NOW(), NOW()),
('CHR-017', 'STN-009', 'AC Slow', 'Type 2', 22, 'active', NOW(), NOW()),
('CHR-018', 'STN-010', 'DC Fast', 'CCS2', 80, 'active', NOW(), NOW());

-- ============================================================================
-- Users
-- ============================================================================

INSERT INTO "user" (id, email, name, role, partner_id, status, created_at) VALUES
('USR-001', 'user1@example.com', 'User One', 'user', NULL, 'active', NOW()),
('USR-002', 'user2@example.com', 'User Two', 'user', NULL, 'active', NOW()),
('USR-003', 'partner1@example.com', 'Partner User', 'partner', 'PRT-001', 'active', NOW()),
('USR-004', 'partner2@example.com', 'Partner Admin', 'partner', 'PRT-002', 'active', NOW());

-- ============================================================================
-- Favorites
-- ============================================================================

INSERT INTO favorite (id, user_id, station_id, created_at) VALUES
('FVT-001', 'USR-001', 'STN-001', NOW()),
('FVT-002', 'USR-001', 'STN-003', NOW()),
('FVT-003', 'USR-002', 'STN-002', NOW()),
('FVT-004', 'USR-003', 'STN-003', NOW()),
('FVT-005', 'USR-004', 'STN-005', NOW()),
('FVT-006', 'USR-003', 'STN-007', NOW()),
('FVT-007', 'USR-004', 'STN-009', NOW());

-- ============================================================================
-- Reviews
-- ============================================================================

INSERT INTO review (id, user_id, station_id, rating, comment, status, created_at, updated_at) VALUES
('RVW-001', 'USR-001', 'STN-001', 5, 'Excellent location, great chargers!', 'approved', NOW(), NOW()),
('RVW-002', 'USR-002', 'STN-003', 4, 'Good power, fast charging', 'approved', NOW(), NOW()),
('RVW-003', 'USR-003', 'STN-005', 5, 'Perfect for my daily commute', 'approved', NOW(), NOW()),
('RVW-004', 'USR-001', 'STN-007', 3, 'Decent but could be cleaner', 'approved', NOW(), NOW()),
('RVW-005', 'USR-004', 'STN-009', 4, 'Great beach access with charging!', 'approved', NOW(), NOW()),
('RVW-006', 'USR-002', 'STN-010', 2, 'Limited hours, mostly broken', 'approved', NOW(), NOW()),
('RVW-007', 'USR-003', 'STN-002', 5, 'Charming station in Sidi Bou Said', 'approved', NOW(), NOW());

-- ============================================================================
-- Verifications
-- ============================================================================

SELECT 'Partners: ' || COUNT(*) FROM partner;
SELECT 'Stations: ' || COUNT(*) FROM station;
SELECT 'Chargers: ' || COUNT(*) FROM charger;
SELECT 'Users: ' || COUNT(*) FROM "user";
SELECT 'Favorites: ' || COUNT(*) FROM favorite;
SELECT 'Reviews: ' || COUNT(*) FROM review;

SELECT 'Station count by partner:' FROM (
    SELECT partner_id, COUNT(*) as station_count
    FROM station
    GROUP BY partner_id
) sub;

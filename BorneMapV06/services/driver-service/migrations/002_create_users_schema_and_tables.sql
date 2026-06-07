-- Create users schema for user profiles and interactions
-- This migration creates tables for User, Favorite, and Review entities

-- ============================================================================
-- User Table
-- ============================================================================

CREATE TABLE users.user (
    id VARCHAR(16) PRIMARY KEY NOT NULL,
    keycloak_id UUID NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL,
    name VARCHAR(255),
    role VARCHAR(20) NOT NULL CHECK (role IN ('registered_driver', 'partner', 'admin')),
    partner_id VARCHAR(16),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ
);

-- Indexes for user table
CREATE INDEX idx_user_keycloak_id ON users.user(keycloak_id);
CREATE INDEX idx_user_role ON users.user(role);
CREATE INDEX idx_user_partner_id ON users.user(partner_id) WHERE partner_id IS NOT NULL;
CREATE INDEX idx_user_deleted_at ON users.user(deleted_at);

COMMENT ON TABLE users.user IS 'Authenticated user (driver or partner)';
COMMENT ON COLUMN users.user.id IS 'NanoID with USR-* prefix (e.g., USR-ABC123XYZ1234)';
COMMENT ON COLUMN users.user.deleted_at IS 'Soft delete marker - non-null when user is inactive (cannot login)';

-- ============================================================================
-- Favorite Table
-- ============================================================================

CREATE TABLE users.favorite (
    id VARCHAR(16) PRIMARY KEY NOT NULL,
    user_id VARCHAR(16) NOT NULL REFERENCES users.user(id) ON DELETE CASCADE,
    station_id VARCHAR(16) NOT NULL REFERENCES inventory.station(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, station_id)
);

-- Indexes for favorite table
CREATE INDEX idx_favorite_user_id ON users.favorite(user_id);
CREATE INDEX idx_favorite_station_id ON users.favorite(station_id);

COMMENT ON TABLE users.favorite IS 'User''s saved charging station';
COMMENT ON COLUMN users.favorite.id IS 'NanoID with FAV-* prefix (e.g., FAV-ABC123XYZ1234)';
COMMENT ON COLUMN users.favorite.user_id IS 'Foreign key to user.user.id';
COMMENT ON COLUMN users.favorite.station_id IS 'Foreign key to inventory.station.id';
COMMENT ON COLUMN users.favorite.created_at IS 'Timestamp when user added this favorite';

-- ============================================================================
-- Review Table
-- ============================================================================

CREATE TABLE users.review (
    id VARCHAR(16) PRIMARY KEY NOT NULL,
    user_id VARCHAR(16) NOT NULL REFERENCES users.user(id) ON DELETE CASCADE,
    station_id VARCHAR(16) NOT NULL REFERENCES inventory.station(id) ON DELETE CASCADE,
    rating SMALLINT NOT NULL CHECK (rating >= 1 AND rating <= 5),
    comment TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ,
    UNIQUE(user_id, station_id)
);

-- Indexes for review table
CREATE INDEX idx_review_user_id ON users.review(user_id);
CREATE INDEX idx_review_station_id ON users.review(station_id);
CREATE INDEX idx_review_created_at ON users.review(created_at);

COMMENT ON TABLE users.review IS 'User''s rating and comment on a Station';
COMMENT ON COLUMN users.review.id IS 'NanoID with REV-* prefix (e.g., REV-ABC123XYZ1234)';
COMMENT ON COLUMN users.review.rating IS 'Star rating (1-5)';
COMMENT ON COLUMN users.review.deleted_at IS 'Soft delete marker - user can remove review (review history preserved)';
COMMENT ON COLUMN users.review.user_id IS 'Foreign key to user.user.id';
COMMENT ON COLUMN users.review.station_id IS 'Foreign key to inventory.station.id';

-- ============================================================================
-- Row-Level Security for User Data
-- ============================================================================

COMMENT ON TABLE users.user IS 'Authenticated users (registered drivers and partners)';
COMMENT ON TABLE users.favorite IS 'User favorites - hard delete (ephemeral user preference)';
COMMENT ON TABLE users.review IS 'User reviews - soft delete for audit trail';

-- Note: Partner scope enforcement at API layer (see tasks.md)
-- Partner users are strictly scoped to one organization via JWT claims

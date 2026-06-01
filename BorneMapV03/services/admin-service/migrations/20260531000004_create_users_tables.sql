-- Users schema tables: user_account, favorite

CREATE TABLE users.user_account (
    id VARCHAR(26) PRIMARY KEY,
    keycloak_id VARCHAR(255) NOT NULL UNIQUE,
    email VARCHAR(255),
    display_name VARCHAR(255),
    status VARCHAR(32) NOT NULL DEFAULT 'active',
    last_login_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ,

    CONSTRAINT chk_user_status CHECK (status IN ('active', 'suspended', 'orphaned')),
    CONSTRAINT chk_user_deleted CHECK (deleted_at IS NULL OR deleted_at > created_at)
);

CREATE INDEX idx_user_keycloak ON users.user_account (keycloak_id);
CREATE INDEX idx_user_status ON users.user_account (status);

CREATE TABLE users.favorite (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id VARCHAR(26) NOT NULL REFERENCES users.user_account(id),
    station_id VARCHAR(26) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT uq_favorite_user_station UNIQUE (user_id, station_id)
);

CREATE INDEX idx_favorite_user ON users.favorite (user_id);
CREATE INDEX idx_favorite_station ON users.favorite (station_id);

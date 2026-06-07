CREATE TABLE IF NOT EXISTS users.user_account (
    id                TEXT        NOT NULL PRIMARY KEY,
    keycloak_user_id  TEXT        NOT NULL UNIQUE,
    email             TEXT        NULL,
    status            TEXT        NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'disabled')),
    created_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_login_at     TIMESTAMPTZ NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_user_account_keycloak_user_id ON users.user_account (keycloak_user_id);

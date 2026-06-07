CREATE TABLE IF NOT EXISTS inventory.partner (
    id          TEXT        NOT NULL PRIMARY KEY,
    name        TEXT        NOT NULL,
    type        TEXT        NOT NULL CHECK (type IN ('business', 'private')),
    status      TEXT        NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'suspended')),
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by  TEXT        NOT NULL DEFAULT '',
    updated_by  TEXT        NOT NULL DEFAULT '',
    deleted_at  TIMESTAMPTZ NULL
);

CREATE INDEX IF NOT EXISTS idx_partner_id     ON inventory.partner (id);
CREATE INDEX IF NOT EXISTS idx_partner_status ON inventory.partner (status);

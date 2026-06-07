CREATE TABLE IF NOT EXISTS users.partner_membership (
    user_id    TEXT NOT NULL PRIMARY KEY REFERENCES users.user_account(id) ON DELETE CASCADE,
    partner_id TEXT NOT NULL REFERENCES inventory.partner(id),
    role       TEXT NOT NULL CHECK (role IN ('owner', 'manager', 'operator', 'viewer')),
    CONSTRAINT uq_partner_membership_user_id UNIQUE (user_id)
);

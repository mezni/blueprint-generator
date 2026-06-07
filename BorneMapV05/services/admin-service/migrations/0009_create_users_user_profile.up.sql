CREATE TABLE IF NOT EXISTS users.user_profile (
    user_id             TEXT  NOT NULL PRIMARY KEY REFERENCES users.user_account(id) ON DELETE CASCADE,
    display_name        TEXT  NULL,
    avatar_url          TEXT  NULL,
    preferred_language  TEXT  NULL,
    preferences         JSONB NULL
);

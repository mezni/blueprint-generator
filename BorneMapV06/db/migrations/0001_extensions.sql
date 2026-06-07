-- Migration 0001: Enable PostgreSQL Extensions
-- Enables PostGIS for geospatial queries, uuid-ossp for UUID generation,
-- and pgcrypto for encryption/hashing utilities.

CREATE EXTENSION IF NOT EXISTS postgis WITH SCHEMA public;
CREATE EXTENSION IF NOT EXISTS "uuid-ossp" WITH SCHEMA public;
CREATE EXTENSION IF NOT EXISTS pgcrypto WITH SCHEMA public;

-- Verify extensions are installed
DO $$
BEGIN
    ASSERT (SELECT COUNT(*) FROM pg_extension) >= 3, 'Extensions not properly installed';
END $$;

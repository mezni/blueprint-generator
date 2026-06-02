#!/usr/bin/env bash
set -e

psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" <<-EOSQL
    CREATE USER keycloak WITH PASSWORD 'change-me';
    CREATE DATABASE keycloak_db OWNER keycloak;
    CREATE DATABASE users_db;
    CREATE DATABASE inventory_db;
    CREATE DATABASE analytics_db;

    GRANT ALL PRIVILEGES ON DATABASE keycloak_db TO keycloak;
EOSQL

-- Create service roles for schema ownership enforcement
CREATE ROLE admin_service WITH LOGIN PASSWORD 'admin_service_password';
CREATE ROLE driver_service WITH LOGIN PASSWORD 'driver_service_password';
CREATE ROLE gis_worker WITH LOGIN PASSWORD 'gis_worker_password';
CREATE ROLE clickstream_service WITH LOGIN PASSWORD 'clickstream_service_password';

-- Revoke all public access to schemas
REVOKE ALL ON SCHEMA inventory, users, gis, analytics FROM PUBLIC;

-- Grant admin_service: owns inventory, writes users, reads gis/analytics
GRANT ALL ON SCHEMA inventory TO admin_service;
GRANT USAGE ON SCHEMA users TO admin_service;
GRANT USAGE ON SCHEMA gis TO admin_service;
GRANT USAGE ON SCHEMA analytics TO admin_service;

-- Grant driver_service: owns users, reads inventory/gis
GRANT ALL ON SCHEMA users TO driver_service;
GRANT USAGE ON SCHEMA inventory TO driver_service;
GRANT USAGE ON SCHEMA gis TO driver_service;

-- Grant gis_worker: owns gis, reads inventory
GRANT ALL ON SCHEMA gis TO gis_worker;
GRANT USAGE ON SCHEMA inventory TO gis_worker;

-- Grant clickstream_service: owns analytics
GRANT ALL ON SCHEMA analytics TO clickstream_service;

-- Grant borne (superuser) full access for migration runner
GRANT ALL ON SCHEMA inventory, users, gis, analytics TO borne;

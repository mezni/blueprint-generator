-- Grant table-level privileges per schema-ownership contract

-- Admin Service: owns inventory, writes users, reads gis/analytics
GRANT ALL ON ALL TABLES IN SCHEMA inventory TO admin_service;
GRANT ALL ON ALL SEQUENCES IN SCHEMA inventory TO admin_service;
GRANT INSERT, UPDATE, SELECT ON ALL TABLES IN SCHEMA users TO admin_service;
GRANT SELECT ON ALL TABLES IN SCHEMA gis TO admin_service;
GRANT SELECT ON ALL TABLES IN SCHEMA analytics TO admin_service;

-- Driver Service: owns users, reads inventory/gis
GRANT ALL ON ALL TABLES IN SCHEMA users TO driver_service;
GRANT ALL ON ALL SEQUENCES IN SCHEMA users TO driver_service;
GRANT SELECT ON inventory.station, inventory.charger, inventory.review TO driver_service;
GRANT SELECT ON ALL TABLES IN SCHEMA gis TO driver_service;

-- GIS Worker: owns gis, reads inventory.station
GRANT ALL ON ALL TABLES IN SCHEMA gis TO gis_worker;
GRANT ALL ON ALL SEQUENCES IN SCHEMA gis TO gis_worker;
GRANT SELECT ON inventory.station TO gis_worker;

-- Clickstream Service: owns analytics
GRANT ALL ON ALL TABLES IN SCHEMA analytics TO clickstream_service;
GRANT ALL ON ALL SEQUENCES IN SCHEMA analytics TO clickstream_service;

-- Grant borne (superuser) full access for migration runner
GRANT ALL ON ALL TABLES IN SCHEMA inventory, users, gis, analytics TO borne;
GRANT ALL ON ALL SEQUENCES IN SCHEMA inventory, users, gis, analytics TO borne;

-- Set default privileges for future tables
ALTER DEFAULT PRIVILEGES IN SCHEMA inventory GRANT ALL ON TABLES TO admin_service;
ALTER DEFAULT PRIVILEGES IN SCHEMA inventory GRANT ALL ON SEQUENCES TO admin_service;
ALTER DEFAULT PRIVILEGES IN SCHEMA users GRANT ALL ON TABLES TO driver_service;
ALTER DEFAULT PRIVILEGES IN SCHEMA users GRANT INSERT, UPDATE, SELECT ON TABLES TO admin_service;
ALTER DEFAULT PRIVILEGES IN SCHEMA gis GRANT ALL ON TABLES TO gis_worker;
ALTER DEFAULT PRIVILEGES IN SCHEMA gis GRANT ALL ON SEQUENCES TO gis_worker;
ALTER DEFAULT PRIVILEGES IN SCHEMA analytics GRANT ALL ON TABLES TO clickstream_service;
ALTER DEFAULT PRIVILEGES IN SCHEMA analytics GRANT ALL ON SEQUENCES TO clickstream_service;

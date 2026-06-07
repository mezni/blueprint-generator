//! GIS Worker — Async worker for GIS synchronization
//!
//! This worker reads from the inventory.station_outbox table and projects
//! station changes to gis.station_locations asynchronously.
//!
//! Features:
//! - Non-blocking GIS sync (station updates proceed regardless of sync status)
//! - Outbox pattern for guaranteed event delivery
//! - Last-write-wins conflict resolution
//! - Graceful shutdown on SIGTERM

pub mod config;

pub use config::Config;

mod config;
mod db;
mod error;
mod geometry;
mod health;
mod models;
mod rabbitmq;
mod retry;
mod worker;

use axum::routing::get;
use axum::Router;
use config::Config;
use sqlx::PgPool;
use std::net::SocketAddr;
use tokio::signal;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .json()
        .init();

    let config = Config::from_env();

    if !config.enable_gis_sync {
        info!("FF_ENABLE_GIS_SYNC=false — gis-worker exiting immediately");
        return;
    }

    let pool: PgPool = db::init_pool(&config.database_url, 5)
        .await
        .expect("Failed to initialize database pool");

    db::run_migrations(&pool, &config.migrations_dir)
        .await
        .expect("Failed to run database migrations");

    if let Some(queue) = &config.rabbitmq_gis_sync {
        rabbitmq::check_rabbitmq_config(queue);
    }

    let app = Router::new()
        .route("/health", get(health::health))
        .with_state(pool.clone());

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    info!("gis-worker listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind address");

    let (tx, rx) = tokio::sync::oneshot::channel::<()>();
    let worker_pool = pool.clone();
    let worker_config = config.clone();

    tokio::spawn(async move {
        worker::run(worker_config, worker_pool, rx).await;
    });

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("Server error");

    let _ = tx.send(());
    info!("gis-worker shutdown complete");
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("shutdown signal received, starting graceful shutdown");
}

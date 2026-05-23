#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use actix_web::{web, App, HttpServer, HttpResponse};
use sqlx::postgres::PgPoolOptions;
use station_service::config::AppConfig;
use station_service::observability;
use tracing_actix_web::TracingLogger;
use utoipa::OpenApi;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = AppConfig::from_env().unwrap_or_else(|e| {
        eprintln!("Configuration error: {e}");
        std::process::exit(1);
    });

    observability::logging::init_tracing();

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
        .unwrap_or_else(|e| {
            eprintln!("Database connection failed: {e}");
            std::process::exit(1);
        });

    let bind_addr = config.bind_addr.clone();
    let admin_usernames = config.mock_admin_usernames.clone();
    let jwt_secret = config.mock_jwt_secret.clone();
    let hide_test_rows = config.hide_test_rows;

    HttpServer::new(move || {
        let openapi = station_service::ApiDoc::openapi();

        App::new()
            .wrap(TracingLogger::default())
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(admin_usernames.clone()))
            .app_data(web::Data::new(jwt_secret.clone()))
            .app_data(web::Data::new(hide_test_rows))
            .service(
                web::scope("/api/v1")
                    .configure(station_service::auth::handlers::configure)
                    .configure(station_service::station::handlers::configure),
            )
            .route("/health/live", web::get().to(health_live))
            .route("/health/ready", web::get().to(health_ready))
            .route("/metrics", web::get().to(metrics))
            .service(utoipa_swagger_ui::SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi))
    })
    .bind(&bind_addr)?
    .run()
    .await
}

async fn health_live() -> HttpResponse {
    HttpResponse::Ok().body("OK")
}

async fn health_ready(pool: web::Data<sqlx::PgPool>) -> HttpResponse {
    match sqlx::query("SELECT 1").execute(pool.get_ref()).await {
        Ok(_) => HttpResponse::Ok().body("READY"),
        Err(_) => HttpResponse::ServiceUnavailable().body("NOT READY"),
    }
}

async fn metrics() -> HttpResponse {
    let body = prometheus::TextEncoder::new()
        .encode_to_string(&prometheus::default_registry().gather())
        .unwrap_or_default();
    HttpResponse::Ok()
        .content_type("text/plain")
        .body(body)
}

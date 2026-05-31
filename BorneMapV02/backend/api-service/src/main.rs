use actix_web::{web, App, HttpServer, middleware::Logger};
use actix_cors::Cors;
use lapin::{Connection, ConnectionProperties};
use mongodb::Client as MongoClient;
use sqlx::PgPool;
use std::sync::Mutex;

mod domains;

pub struct AppState {
    pub db: PgPool,
    pub amqp_channel: lapin::Channel,
    pub mongo_db: mongodb::Database,
    pub filter_store: Mutex<std::collections::HashMap<String, domains::filters::TimestampedFilters>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://borne:borne@localhost:5432/borne_map".to_string());
    let rabbit_uri = std::env::var("RABBITMQ_URI")
        .unwrap_or_else(|_| "amqp://guest:guest@127.0.0.1:5672/%2f".to_string());
    let mongo_uri = std::env::var("MONGO_URI")
        .unwrap_or_else(|_| "mongodb://admin:secret_password_change_me@127.0.0.1:27017".to_string());

    let pool = infra::join_database_pool(&database_url)
        .await
        .expect("Failed to connect to database");

    let amqp_conn = Connection::connect(&rabbit_uri, ConnectionProperties::default())
        .await
        .expect("Failed to connect to RabbitMQ");
    let amqp_channel = amqp_conn.create_channel().await
        .expect("Failed to create RabbitMQ channel");

    let mongo_client = MongoClient::with_uri_str(&mongo_uri)
        .await
        .expect("Failed to connect to MongoDB");
    let mongo_db = mongo_client.database("bornemap_analytics");

    let filter_store = Mutex::new(std::collections::HashMap::new());

    let state = web::Data::new(AppState { db: pool, amqp_channel, mongo_db, filter_store });

    let host = "0.0.0.0";
    let port = 8080;
    log::info!("api-service online on http://{}:{}", host, port);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(state.clone())
            .wrap(Logger::default())
            .route("/health", web::get().to(domains::locate::routes::health))
            .service(
                web::scope("/api/v1")
                    .configure(domains::locate::init_routes)
                    .configure(domains::analytics::init_routes)
                    .configure(domains::filters::init_routes),
            )
    })
    .bind((host, port))?
    .run()
    .await
}

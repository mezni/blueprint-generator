use lapin::{options::*, types::FieldTable, Connection, ConnectionProperties};
use mongodb::{Client, Database};

mod consumer;

pub struct AnalyticsAppState {
    pub mongo_db: Database,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    let rabbit_uri = std::env::var("RABBITMQ_URI")
        .unwrap_or_else(|_| "amqp://guest:guest@127.0.0.1:5672/%2f".to_string());
    let mongo_uri = std::env::var("MONGO_URI")
        .unwrap_or_else(|_| "mongodb://admin:secret_password_change_me@127.0.0.1:27017".to_string());

    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async {
        let rabbit_conn = Connection::connect(&rabbit_uri, ConnectionProperties::default()).await?;
        let channel = rabbit_conn.create_channel().await?;

        let _queue = channel
            .queue_declare(
                "analytics.connections",
                QueueDeclareOptions {
                    durable: true,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await?;

        let mongo_client = Client::with_uri_str(&mongo_uri).await?;
        let mongo_db = mongo_client.database("bornemap_analytics");
        let state = AnalyticsAppState { mongo_db };

        log::info!("analytics-service online");

        consumer::start(channel, state).await;

        Ok::<_, Box<dyn std::error::Error>>(())
    })
}

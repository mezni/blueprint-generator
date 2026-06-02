pub const SERVICE_NAME: &str = env!("CARGO_PKG_NAME");
pub const SERVICE_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn init_tracing(service_name: &str, json: bool) {
    use tracing_subscriber::prelude::*;

    if json {
        let fmt_layer = tracing_subscriber::fmt::layer()
            .json()
            .with_target(true)
            .with_thread_ids(true)
            .with_thread_names(false)
            .with_file(false)
            .with_line_number(true);
        let filter_layer = tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));

        tracing_subscriber::registry()
            .with(filter_layer)
            .with(fmt_layer)
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_env_filter(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
            )
            .init();
    }

    tracing::info!(service_name, version = SERVICE_VERSION, "Service started");
}

pub fn init_default(service_name: &str) {
    let use_json = std::env::var("LOG_FORMAT").map(|v| v == "json").unwrap_or(false);
    init_tracing(service_name, use_json);
}

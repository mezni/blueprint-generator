#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use utoipa::OpenApi;

fn main() {
    let spec = station_service::ApiDoc::openapi();
    let json = serde_json::to_string_pretty(&spec).unwrap();
    println!("{json}");
}

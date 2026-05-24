#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

pub mod auth;
pub mod companies;
pub mod config;
pub mod error_adapter;
pub mod observability;
pub mod station;

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        station::handlers::stations_list_by_viewport,
        station::handlers::stations_get_by_id,
        station::handlers::admin_stations_list,
        station::handlers::admin_stations_create,
        station::handlers::admin_stations_patch,
        station::handlers::admin_stations_soft_delete,
        auth::handlers::auth_mock_login,
    ),
    components(
        schemas(
            station::models::StationMarker,
            station::models::StationDetail,
            station::models::StationListResponse,
            station::models::AdminStationListResponse,
            station::models::AdminStationCreate,
            station::models::AdminStationPatch,
            station::models::Charger,
            station::models::Company,
            station::models::BboxQuery,
            auth::models::MockLoginRequest,
            auth::models::MockLoginResponse,
        )
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearerAuth",
                utoipa::openapi::security::SecurityScheme::Http(
                    utoipa::openapi::security::Http::new(
                        utoipa::openapi::security::HttpAuthScheme::Bearer,
                    ),
                ),
            );
        }
    }
}

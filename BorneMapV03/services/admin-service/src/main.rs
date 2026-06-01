use axum::{
    extract::{FromRequestParts, Request, State},
    http::{request::Parts, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Json, Response},
    routing::get,
    Router,
};
use common_auth::{AuthErrorResponse, AuthMiddleware, JwtValidator};
use std::sync::Arc;

struct AppState {
    auth: AuthMiddleware,
}

#[derive(Clone)]
struct AuthCtx(Option<common_auth::UserContext>);

impl<S: Sync + Send> FromRequestParts<S> for AuthCtx {
    type Rejection = (StatusCode, Json<AuthErrorResponse>);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let user = parts.extensions.get::<common_auth::UserContext>().cloned();
        Ok(AuthCtx(user))
    }
}

async fn auth_mw(
    State(state): State<Arc<AppState>>,
    request: Request,
    next: Next,
) -> Response {
    let path = request.uri().path().to_string();
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok());

    match state.auth.authenticate_request(&path, auth_header).await {
        Ok(ctx) => {
            let mut req = request;
            if let Some(user) = ctx {
                req.extensions_mut().insert(user);
            }
            next.run(req).await
        }
        Err(err) => {
            let code = if err.error_code == "UNAUTHORIZED" || err.error_code == "TOKEN_EXPIRED" {
                StatusCode::UNAUTHORIZED
            } else if err.error_code == "FORBIDDEN" {
                StatusCode::FORBIDDEN
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            };
            (code, Json(err)).into_response()
        }
    }
}

async fn health() -> &'static str {
    "OK"
}

async fn stations_handler(auth: AuthCtx) -> Response {
    match auth.0 {
        Some(ref user) if user.has_role("admin") => {
            Json(serde_json::json!({ "stations": [], "user_id": user.user_id })).into_response()
        }
        Some(_) => (
            StatusCode::FORBIDDEN,
            Json(serde_json::json!({
                "error_code": "FORBIDDEN",
                "message": "Admin role required"
            })),
        ).into_response(),
        None => (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({
                "error_code": "UNAUTHORIZED",
                "message": "Authentication required"
            })),
        ).into_response(),
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://borne:devpassword@localhost:5432/borne_map".to_string());

    tracing::info!("Connecting to database...");
    let pool = common_db::create_pool(Some(&database_url))
        .await
        .expect("Failed to create database connection pool");

    common_db::check_connection(&pool)
        .await
        .expect("Database health check failed");
    tracing::info!("Database connection established");

    tracing::info!("Running database migrations...");
    common_db::run_migrations(&pool)
        .await
        .expect("Database migration failed");
    tracing::info!("Migrations applied successfully");

    let validator = JwtValidator::new(
        "http://keycloak:8080/realms/ev-platform/protocol/openid-connect/certs".into(),
        "https://keycloak:8080/realms/ev-platform".into(),
        "backend-service".into(),
    );

    if let Err(e) = validator.refresh_jwks().await {
        tracing::warn!("Initial JWKS fetch failed (will retry on first request): {e}");
    }

    let auth = AuthMiddleware::new(validator);

    let state = Arc::new(AppState { auth });

    let app = Router::new()
        .route("/api/v1/admin/stations", get(stations_handler))
        .route("/health", get(health))
        .route_layer(middleware::from_fn_with_state(state.clone(), auth_mw))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    tracing::info!("admin-service — listening at /api/v1/admin/*");
    axum::serve(listener, app).await.unwrap();
}

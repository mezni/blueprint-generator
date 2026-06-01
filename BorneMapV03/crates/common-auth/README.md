# common-auth

Shared authentication and authorization middleware for BorneMap backend services.

## Usage

### JWT Validation

```rust
use common_auth::JwtValidator;

let validator = JwtValidator::new(
    "http://keycloak:8080/realms/ev-platform/protocol/openid-connect/certs".into(),
    "https://keycloak:8080/realms/ev-platform".into(),
    "backend-service".into(),
);

validator.refresh_jwks().await?;
let validated = validator.validate_token(token).await?;
```

### Auth Middleware

```rust
use common_auth::{AuthMiddleware, JwtValidator};

let validator = JwtValidator::new(/* ... */);
let middleware = AuthMiddleware::new(validator);

let ctx = middleware.authenticate_request(path, auth_header).await?;
```

### Client Credentials (Service-to-Service)

```rust
use common_auth::ClientCredentials;

let mut credentials = ClientCredentials::new(
    "backend-service",
    &std::env::var("BACKEND_SERVICE_SECRET").unwrap(),
    "http://keycloak:8080/realms/ev-platform/protocol/openid-connect/token",
    "http://keycloak:8080/realms/ev-platform/protocol/openid-connect/certs",
    "https://keycloak:8080/realms/ev-platform",
);

let token = credentials.acquire_token().await?;
// Use token for authenticated API calls to other services
```

### Public Endpoints

The following paths skip authentication:
- `GET /health`
- `GET /ready`
- `/auth/*`

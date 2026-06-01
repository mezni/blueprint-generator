pub mod audit;
pub mod client;
pub mod error;
pub mod middleware;
pub mod validator;

pub use audit::AuditProducer;
pub use client::ClientCredentials;
pub use error::{AuditEvent, AuditEventType, AuthError, AuthErrorResponse};
pub use middleware::{AuthMiddleware, UserContext};
pub use validator::{JwtValidator, ValidatedToken};

pub async fn validate_token(
    validator: &JwtValidator,
    token: &str,
) -> Result<ValidatedToken, AuthError> {
    validator.validate_token(token).await
}

pub fn extract_roles(validated: &ValidatedToken) -> Vec<String> {
    validated.roles.clone()
}

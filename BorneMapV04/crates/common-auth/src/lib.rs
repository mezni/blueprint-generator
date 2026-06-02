pub mod claims;
pub mod config;
pub mod error;
pub mod jwt;
pub mod middleware;

pub use claims::{AuthContext, Role};
pub use config::AuthConfig;
pub use error::AuthError;
pub use jwt::JwtValidator;
pub use middleware::{auth_middleware, require_any_role, require_role};

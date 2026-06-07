pub mod errors;
pub mod guards;
pub mod jwt;
pub mod provisioning;

use common_types::Role;
use serde::{Deserialize, Serialize};

pub use errors::AuthError;
pub use guards::{
    auth_middleware, extract_current_user, optional_auth_middleware, require_authenticated,
    require_role, set_auth_config, AuthConfig,
};
pub use jwt::{init_jwks_cache, validate_token};
pub use provisioning::provision_user;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentUser {
    pub user_id: String,
    pub keycloak_user_id: String,
    pub email: Option<String>,
    pub role: Role,
    pub partner_id: Option<String>,
}

impl CurrentUser {
    pub fn new(user_id: String, keycloak_user_id: String, email: Option<String>, role: Role) -> Self {
        Self {
            user_id,
            keycloak_user_id,
            email,
            role,
            partner_id: None,
        }
    }
}

use serde::{Deserialize, Serialize};

pub const ADMIN_ROLE: &str = "admin";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealmAccess {
    pub roles: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub preferred_username: String,
    pub realm_access: RealmAccess,
    pub iat: i64,
    pub exp: i64,
}

impl TokenClaims {
    pub fn is_admin(&self) -> bool {
        self.realm_access.roles.iter().any(|r| r == ADMIN_ROLE)
    }
}

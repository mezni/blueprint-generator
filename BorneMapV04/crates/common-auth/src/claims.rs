use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Role {
    #[serde(rename = "registered_driver")]
    RegisteredDriver,
    #[serde(rename = "partner")]
    Partner,
    #[serde(rename = "admin")]
    Admin,
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::RegisteredDriver => write!(f, "registered_driver"),
            Role::Partner => write!(f, "partner"),
            Role::Admin => write!(f, "admin"),
        }
    }
}

impl TryFrom<&str> for Role {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "registered_driver" => Ok(Role::RegisteredDriver),
            "partner" => Ok(Role::Partner),
            "admin" => Ok(Role::Admin),
            _ => Err(format!("Unknown role: {s}")),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AuthContext {
    pub sub: String,
    pub roles: Vec<Role>,
    pub tenant_id: Option<String>,
}

impl AuthContext {
    pub fn require_tenant(&self) -> Result<&str, super::error::AuthError> {
        self.tenant_id
            .as_deref()
            .filter(|t| !t.is_empty())
            .ok_or(super::error::AuthError::MissingTenantId)
    }

    pub fn has_role(&self, role: &Role) -> bool {
        self.roles.contains(role)
    }

    pub fn has_any_role(&self, roles: &[Role]) -> bool {
        roles.iter().any(|r| self.roles.contains(r))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeycloakClaims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    pub iss: String,
    pub aud: serde_json::Value,
    #[serde(rename = "realm_access")]
    pub realm_access: Option<RealmAccess>,
    pub tenant_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealmAccess {
    pub roles: Vec<String>,
}

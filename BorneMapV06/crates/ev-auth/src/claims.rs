//! JWT Claims structure for Keycloak tokens

use serde::{Deserialize, Serialize};
use std::fmt;

/// User role type — strict set of allowed roles
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    /// Public/anonymous user (no authentication required)
    #[serde(rename = "public_driver")]
    PublicDriver,

    /// Registered driver with favorites and reviews
    #[serde(rename = "registered_driver")]
    RegisteredDriver,

    /// Partner user with station management
    #[serde(rename = "partner")]
    Partner,

    /// Admin with full platform access
    #[serde(rename = "admin")]
    Admin,
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::PublicDriver => write!(f, "public_driver"),
            Role::RegisteredDriver => write!(f, "registered_driver"),
            Role::Partner => write!(f, "partner"),
            Role::Admin => write!(f, "admin"),
        }
    }
}

/// JWT Claims from Keycloak token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// Subject (user ID from Keycloak)
    pub sub: String,

    /// User email
    pub email: Option<String>,

    /// Full name from Keycloak profile
    pub name: Option<String>,

    /// User role (registered_driver, partner, admin)
    pub role: Role,

    /// Partner ID (non-null only for partner role)
    pub partner_id: Option<String>,

    /// Issued at timestamp
    pub iat: i64,

    /// Expiration timestamp
    pub exp: i64,

    /// JWT ID (jti)
    pub jti: Option<String>,
}

impl Claims {
    /// Check if token is expired based on exp claim
    pub fn is_expired(&self, now: i64) -> bool {
        now > self.exp
    }

    /// Validate partner scope (non-null for partner role)
    pub fn validate_partner_scope(&self) -> Result<String, crate::AuthError> {
        match self.role {
            Role::Partner => {
                self.partner_id
                    .clone()
                    .ok_or(crate::AuthError::MissingPartnerScope)
            }
            _ => {
                if self.partner_id.is_some() {
                    Err(crate::AuthError::InvalidRole(
                        "Non-partner user should not have partner_id".to_string(),
                    ))
                } else {
                    Ok(String::new()) // Empty scope for non-partners
                }
            }
        }
    }
}

use common_utils::error::DomainError;
use jsonwebtoken::{encode, EncodingKey, Header};

use super::claims::{RealmAccess, TokenClaims, ADMIN_ROLE};
use super::models::MockLoginResponse;

const JWT_EXPIRY_SECS: i64 = 3600;

pub struct MockAuthService;

impl MockAuthService {
    pub fn login(
        username: &str,
        role: &str,
        allowlist: &[String],
        secret: &str,
    ) -> Result<MockLoginResponse, DomainError> {
        if role == "driver" {
            return Err(DomainError::Validation(
                "Driver accounts are not supported in MVP 1".into(),
            ));
        }

        if role != ADMIN_ROLE {
            return Err(DomainError::Validation(format!(
                "Invalid role: {role}. Only 'admin' is accepted in MVP 1"
            )));
        }

        if !allowlist.iter().any(|u| u == username) {
            return Err(DomainError::Forbidden(format!(
                "Username '{username}' is not on the admin allowlist"
            )));
        }

        let now = chrono::Utc::now().timestamp();
        let claims = TokenClaims {
            sub: uuid::Uuid::new_v4().to_string(),
            preferred_username: username.to_string(),
            realm_access: RealmAccess {
                roles: vec![ADMIN_ROLE.to_string()],
            },
            iat: now,
            exp: now + JWT_EXPIRY_SECS,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .map_err(|e| DomainError::Internal(format!("JWT encoding failed: {e}")))?;

        Ok(MockLoginResponse {
            access_token: token,
            token_type: "Bearer".to_string(),
            expires_in: JWT_EXPIRY_SECS,
        })
    }
}

use crate::error::{AuthError, AuthErrorResponse};
use crate::validator::JwtValidator;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct UserContext {
    pub user_id: String,
    pub keycloak_id: String,
    pub roles: Vec<String>,
    pub email: Option<String>,
    pub display_name: Option<String>,
    pub app_user_id: Option<String>,
}

impl UserContext {
    pub fn has_role(&self, role: &str) -> bool {
        self.roles.iter().any(|r| r == role)
    }

    pub fn has_any_role(&self, roles: &[&str]) -> bool {
        roles.iter().any(|r| self.has_role(r))
    }
}

pub struct AuthMiddleware {
    validator: JwtValidator,
    public_paths: Vec<String>,
}

impl AuthMiddleware {
    pub fn new(validator: JwtValidator) -> Self {
        Self {
            validator,
            public_paths: vec![
                "/health".to_string(),
                "/ready".to_string(),
            ],
        }
    }

    pub fn is_public_path(&self, path: &str) -> bool {
        self.public_paths.iter().any(|p| path.starts_with(p))
            || path.starts_with("/auth/")
    }

    pub async fn authenticate(&self, token: &str) -> Result<UserContext, AuthError> {
        let validated = self.validator.validate_token(token).await?;
        Ok(UserContext {
            user_id: validated.subject.clone(),
            keycloak_id: validated.subject,
            roles: validated.roles,
            email: None,
            display_name: None,
            app_user_id: None,
        })
    }

    pub async fn authenticate_request(
        &self,
        path: &str,
        auth_header: Option<&str>,
    ) -> Result<Option<UserContext>, AuthErrorResponse> {
        if self.is_public_path(path) {
            return Ok(None);
        }

        let bearer = auth_header
            .and_then(|h| h.strip_prefix("Bearer "))
            .ok_or_else(|| AuthErrorResponse {
                error_code: "UNAUTHORIZED",
                message: "Token is missing or invalid".to_string(),
                trace_id: Uuid::new_v4().to_string(),
            })?;

        let ctx = match self.authenticate(bearer).await {
            Ok(ctx) => ctx,
            Err(AuthError::TokenExpired) => {
                return Err(AuthErrorResponse {
                    error_code: "TOKEN_EXPIRED",
                    message: "Token has expired".to_string(),
                    trace_id: Uuid::new_v4().to_string(),
                })
            }
            Err(AuthError::Forbidden) => {
                return Err(AuthErrorResponse {
                    error_code: "FORBIDDEN",
                    message: "Insufficient permissions".to_string(),
                    trace_id: Uuid::new_v4().to_string(),
                })
            }
            Err(e) => {
                return Err(AuthErrorResponse {
                    error_code: e.error_code(),
                    message: e.to_string(),
                    trace_id: Uuid::new_v4().to_string(),
                })
            }
        };

        Ok(Some(ctx))
    }

    pub fn check_role(
        ctx: &UserContext,
        path: &str,
    ) -> Result<(), AuthErrorResponse> {
        if path.starts_with("/api/v1/admin")
            && !ctx.has_role("admin")
        {
            return Err(AuthErrorResponse {
                error_code: "FORBIDDEN",
                message: "Admin role required".to_string(),
                trace_id: Uuid::new_v4().to_string(),
            });
        }

        if path.starts_with("/api/v1/partner")
            && !ctx.has_any_role(&["partner", "admin"])
        {
            return Err(AuthErrorResponse {
                error_code: "FORBIDDEN",
                message: "Partner or admin role required".to_string(),
                trace_id: Uuid::new_v4().to_string(),
            });
        }

        Ok(())
    }
}

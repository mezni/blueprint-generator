use common_utils::error::DomainError;

pub struct AppConfig {
    pub mock_admin_usernames: Vec<String>,
    pub mock_jwt_secret: String,
    pub bind_addr: String,
    pub database_url: String,
    pub hide_test_rows: bool,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, DomainError> {
        let mock_admin_usernames = std::env::var("MOCK_ADMIN_USERNAMES")
            .map_err(|_| DomainError::Internal("MOCK_ADMIN_USERNAMES must be set".into()))?;

        let usernames: Vec<String> = mock_admin_usernames
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        if usernames.is_empty() {
            return Err(DomainError::Internal(
                "MOCK_ADMIN_USERNAMES must contain at least one username (fail-closed)".into(),
            ));
        }

        let mock_jwt_secret = std::env::var("MOCK_JWT_SECRET")
            .map_err(|_| DomainError::Internal("MOCK_JWT_SECRET must be set".into()))?;

        let bind_addr = std::env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:8000".to_string());

        let database_url = std::env::var("DATABASE_URL")
            .map_err(|_| DomainError::Internal("DATABASE_URL must be set".into()))?;

        let hide_test_rows = std::env::var("BORNEMAP_HIDE_TEST_ROWS")
            .map(|v| v.to_lowercase() != "false")
            .unwrap_or(true);

        Ok(Self {
            mock_admin_usernames: usernames,
            mock_jwt_secret,
            bind_addr,
            database_url,
            hide_test_rows,
        })
    }
}

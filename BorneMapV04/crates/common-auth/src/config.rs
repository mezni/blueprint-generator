use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AuthConfig {
    pub jwks_url: String,
    #[serde(default = "default_refresh_interval")]
    pub jwks_refresh_interval: u64,
    pub allowed_issuers: Vec<String>,
    pub required_audience: String,
}

fn default_refresh_interval() -> u64 {
    3600
}

impl AuthConfig {
    pub fn from_env() -> Result<Self, envy::Error> {
        envy::from_env::<AuthConfig>()
    }
}

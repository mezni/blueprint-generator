use sqlx::PgPool;
use common_utils::error::DomainError;
use crate::station::models::Company;

fn db_err(e: sqlx::Error) -> DomainError {
    DomainError::Internal(e.to_string())
}

pub struct CompanyRepository;

impl CompanyRepository {
    pub async fn get_by_id(pool: &PgPool, id: &str) -> Result<Company, DomainError> {
        let row: Option<(uuid::Uuid, String)> = sqlx::query_as(
            "SELECT id, name FROM station_domain.companies WHERE id = $1 AND deleted_at IS NULL",
        )
        .bind(uuid::Uuid::parse_str(id).map_err(|e| DomainError::Validation(format!("Invalid company ID: {e}")))?)
        .fetch_optional(pool)
        .await
        .map_err(db_err)?;

        match row {
            Some((id, name)) => Ok(Company {
                id: id.to_string(),
                name,
            }),
            None => Err(DomainError::NotFound("Company not found".into())),
        }
    }
}

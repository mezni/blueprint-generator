use sqlx::postgres::PgPoolOptions;

pub async fn setup_test_db() -> Result<sqlx::PgPool, Box<dyn std::error::Error>> {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://bornemap:bornemap@localhost:5432/bornemap_dev".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    sqlx::query("SELECT 1")
        .execute(&pool)
        .await?;

    Ok(pool)
}

pub fn sample_company_id() -> uuid::Uuid {
    uuid::Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap()
}

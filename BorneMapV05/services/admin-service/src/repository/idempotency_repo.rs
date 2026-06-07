use chrono::Utc;
use sqlx::PgPool;
use sqlx::Postgres;
use sqlx::Transaction;

pub async fn check_and_insert(pool: &PgPool, key: &str, _station_id: &str) -> Result<bool, sqlx::Error> {
    let existing: Option<(String,)> =
        sqlx::query_as("SELECT id FROM inventory.idempotency_key WHERE key = $1")
            .bind(key)
            .fetch_optional(pool)
            .await?;

    if existing.is_some() {
        return Ok(true);
    }

    let id = common_types::generate_id(common_types::EntityPrefix::Evt);
    sqlx::query(
        "INSERT INTO inventory.idempotency_key (id, key, station_id, created_at) VALUES ($1, $2, $3, $4)",
    )
    .bind(&id)
    .bind(key)
    .bind("")
    .bind(Utc::now())
    .execute(pool)
    .await?;

    Ok(false)
}

pub async fn insert_in_tx(
    tx: &mut Transaction<'_, Postgres>,
    key: &str,
    station_id: &str,
) -> Result<(), sqlx::Error> {
    let id = common_types::generate_id(common_types::EntityPrefix::Evt);
    sqlx::query(
        "INSERT INTO inventory.idempotency_key (id, key, station_id, created_at) VALUES ($1, $2, $3, $4)",
    )
    .bind(&id)
    .bind(key)
    .bind(station_id)
    .bind(Utc::now())
    .execute(tx as &mut sqlx::PgConnection)
    .await?;
    Ok(())
}

use common_types::generate_id;
use common_types::EntityPrefix;
use sqlx::Postgres;
use sqlx::Transaction;

pub async fn insert_outbox_entry(
    tx: &mut Transaction<'_, Postgres>,
    entity_type: &str,
    entity_id: &str,
    operation: &str,
) -> Result<(), sqlx::Error> {
    let id = generate_id(EntityPrefix::Evt);
    sqlx::query(
        r#"INSERT INTO gis.sync_queue (id, entity_type, entity_id, operation, status, created_at)
           VALUES ($1, $2, $3, $4, 'pending', now())"#,
    )
    .bind(&id)
    .bind(entity_type)
    .bind(entity_id)
    .bind(operation)
    .execute(tx as &mut sqlx::PgConnection)
    .await?;
    Ok(())
}

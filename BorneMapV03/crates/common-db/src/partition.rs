use sqlx::postgres::PgPool;

pub async fn create_daily_partition(
    pool: &PgPool,
    date: chrono::NaiveDate,
) -> Result<(), sqlx::Error> {
    let partition_name = format!("raw_event_{}", date.format("%Y%m%d"));
    let start = date.and_hms_opt(0, 0, 0).unwrap();
    let end = date + chrono::Duration::days(1);
    let end = end.and_hms_opt(0, 0, 0).unwrap();

    sqlx::query(&format!(
        "CREATE TABLE IF NOT EXISTS analytics.{partition_name} PARTITION OF analytics.raw_event FOR VALUES FROM ('{start}') TO ('{end}')"
    ))
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn ensure_tomorrow_partition(pool: &PgPool) -> Result<(), sqlx::Error> {
    let tomorrow = chrono::Utc::now().date_naive() + chrono::Duration::days(1);
    create_daily_partition(pool, tomorrow).await
}

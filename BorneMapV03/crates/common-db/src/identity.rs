use sqlx::postgres::PgPool;

pub async fn provision_user(
    pool: &PgPool,
    keycloak_id: &str,
    email: Option<&str>,
    display_name: Option<&str>,
    user_id: &str,
) -> Result<String, sqlx::Error> {
    let row: (String,) = sqlx::query_as(
        r#"
        INSERT INTO users.user_account (id, keycloak_id, email, display_name)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (keycloak_id) DO UPDATE
        SET last_login_at = NOW(),
            email = COALESCE(EXCLUDED.email, users.user_account.email),
            display_name = COALESCE(EXCLUDED.display_name, users.user_account.display_name)
        RETURNING id
        "#,
    )
    .bind(user_id)
    .bind(keycloak_id)
    .bind(email)
    .bind(display_name)
    .fetch_one(pool)
    .await?;

    Ok(row.0)
}

pub async fn find_user_by_keycloak_id(
    pool: &PgPool,
    keycloak_id: &str,
) -> Result<Option<String>, sqlx::Error> {
    let row: Option<(String,)> = sqlx::query_as(
        "SELECT id FROM users.user_account WHERE keycloak_id = $1",
    )
    .bind(keycloak_id)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| r.0))
}

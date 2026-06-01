use sqlx::postgres::PgPool;

async fn setup_test_pool() -> PgPool {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://borne:devpassword@localhost:5432/borne_map_test".to_string());
    PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to test database")
}

async fn cleanup(pool: &PgPool) {
    sqlx::query("DELETE FROM users.favorite")
        .execute(pool)
        .await
        .ok();
    sqlx::query("DELETE FROM users.user_account")
        .execute(pool)
        .await
        .ok();
}

#[tokio::test]
#[ignore]
async fn test_provision_new_user() {
    let pool = setup_test_pool().await;
    cleanup(&pool).await;

    let keycloak_id = "test-sub-new-user-001";
    let user_id = contracts::generate_user_id();

    let result = common_db::identity::provision_user(
        &pool,
        keycloak_id,
        Some("newuser@example.com"),
        Some("New User"),
        &user_id,
    )
    .await;

    assert!(result.is_ok(), "Provisioning a new user should succeed");
    assert_eq!(result.unwrap(), user_id, "Should return the provided user_id");

    let found = common_db::identity::find_user_by_keycloak_id(&pool, keycloak_id)
        .await
        .expect("Should find user");

    assert!(found.is_some(), "User should be findable by keycloak_id");
    assert_eq!(found.unwrap(), user_id);

    cleanup(&pool).await;
}

#[tokio::test]
#[ignore]
async fn test_provision_returning_user_reuses_record() {
    let pool = setup_test_pool().await;
    cleanup(&pool).await;

    let keycloak_id = "test-sub-returning-user-002";
    let user_id = contracts::generate_user_id();

    common_db::identity::provision_user(
        &pool,
        keycloak_id,
        Some("returning@example.com"),
        Some("Returning User"),
        &user_id,
    )
    .await
    .expect("First provisioning should succeed");

    let user_id2 = contracts::generate_user_id();
    let result = common_db::identity::provision_user(
        &pool,
        keycloak_id,
        Some("returning@example.com"),
        Some("Updated Name"),
        &user_id2,
    )
    .await
    .expect("Second provisioning should succeed");

    assert_eq!(result, user_id, "Should reuse the original user_id, not the new one");

    let count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM users.user_account WHERE keycloak_id = $1"
    )
    .bind(keycloak_id)
    .fetch_one(&pool)
    .await
    .expect("Should count users");

    assert_eq!(count.0, 1, "Should have exactly one user_account for this keycloak_id");

    cleanup(&pool).await;
}

#[tokio::test]
#[ignore]
async fn test_keycloak_id_uniqueness() {
    let pool = setup_test_pool().await;
    cleanup(&pool).await;

    let keycloak_id = "test-sub-unique-003";
    let user_id1 = contracts::generate_user_id();

    common_db::identity::provision_user(
        &pool,
        keycloak_id,
        Some("unique@example.com"),
        None,
        &user_id1,
    )
    .await
    .expect("First user should succeed");

    let user_id2 = contracts::generate_user_id();
    let result = sqlx::query(
        "INSERT INTO users.user_account (id, keycloak_id, email) VALUES ($1, $2, $3)"
    )
    .bind(&user_id2)
    .bind(keycloak_id)
    .bind("duplicate@example.com")
    .execute(&pool)
    .await;

    assert!(result.is_err(), "Duplicate keycloak_id should be rejected by UNIQUE constraint");

    cleanup(&pool).await;
}

#[tokio::test]
#[ignore]
async fn test_provision_updates_last_login() {
    let pool = setup_test_pool().await;
    cleanup(&pool).await;

    let keycloak_id = "test-sub-login-004";
    let user_id = contracts::generate_user_id();

    common_db::identity::provision_user(
        &pool,
        keycloak_id,
        Some("login@example.com"),
        None,
        &user_id,
    )
    .await
    .expect("First provisioning should succeed");

    let before: Option<(Option<chrono::DateTime<chrono::Utc>>,)> = sqlx::query_as(
        "SELECT last_login_at FROM users.user_account WHERE keycloak_id = $1"
    )
    .bind(keycloak_id)
    .fetch_optional(&pool)
    .await
    .expect("Should query user")
    .expect("User should exist");

    assert!(before.0.is_none(), "last_login_at should be NULL after first insert");

    tokio::time::sleep(std::time::Duration::from_millis(10)).await;

    common_db::identity::provision_user(
        &pool,
        keycloak_id,
        Some("login@example.com"),
        None,
        &contracts::generate_user_id(),
    )
    .await
    .expect("Second provisioning should succeed");

    let after: (chrono::DateTime<chrono::Utc>,) = sqlx::query_as(
        "SELECT last_login_at FROM users.user_account WHERE keycloak_id = $1"
    )
    .bind(keycloak_id)
    .fetch_one(&pool)
    .await
    .expect("Should query user");

    assert!(after.0 > chrono::Utc::now() - chrono::Duration::seconds(5), "last_login_at should be updated");

    cleanup(&pool).await;
}

use crate::CurrentUser;
use common_types::generate_id;
use common_types::EntityPrefix;
use common_types::Role;
use sqlx::PgPool;
use tracing::info;

#[derive(Debug, Clone)]
pub struct ProvisionedUser {
    pub user_id: String,
    pub keycloak_user_id: String,
    pub email: Option<String>,
    pub role: Role,
    pub partner_id: Option<String>,
}

/// Provision a user on first login.
///
/// When a `PgPool` is provided, this performs a real DB lookup/insert into
/// `platform_db.user_account` and resolves `partner_id` from `partner_membership`.
/// When `None`, falls back to the deterministic FNV-1a stub (for tests/other services).
pub async fn provision_user(
    pool: Option<&PgPool>,
    keycloak_user_id: &str,
    email: Option<&str>,
    role: Role,
) -> ProvisionedUser {
    if let Some(pool) = pool {
        let user_id = provision_user_in_db(pool, keycloak_user_id, email, role).await;
        let partner_id = resolve_partner_id(pool, &user_id).await;
        info!(
            user_id = %user_id,
            keycloak_user_id = %keycloak_user_id,
            role = ?role,
            partner_id = ?partner_id,
            "User provisioned (DB)"
        );
        return ProvisionedUser {
            user_id,
            keycloak_user_id: keycloak_user_id.to_string(),
            email: email.map(|e| e.to_string()),
            role,
            partner_id,
        };
    }

    let user_id = derive_user_id_stub(keycloak_user_id);
    info!(
        user_id = %user_id,
        keycloak_user_id = %keycloak_user_id,
        role = ?role,
        "User provisioned (stub)"
    );
    ProvisionedUser {
        user_id,
        keycloak_user_id: keycloak_user_id.to_string(),
        email: email.map(|e| e.to_string()),
        role,
        partner_id: None,
    }
}

async fn provision_user_in_db(
    pool: &PgPool,
    keycloak_user_id: &str,
    email: Option<&str>,
    role: Role,
) -> String {
    let existing: Option<(String,)> =
        sqlx::query_as(
            "SELECT user_id FROM platform_db.user_account WHERE keycloak_user_id = $1",
        )
        .bind(keycloak_user_id)
        .fetch_optional(pool)
        .await
        .ok()
        .flatten();

    if let Some((user_id,)) = existing {
        // Update email/role if changed
        let _ = sqlx::query(
            "UPDATE platform_db.user_account SET email = COALESCE($1, email), role = $2, updated_at = now() WHERE user_id = $3",
        )
        .bind(email)
        .bind(role.as_str())
        .bind(&user_id)
        .execute(pool)
        .await;
        return user_id;
    }

    let user_id = generate_id(EntityPrefix::Usr);
    let _ = sqlx::query(
        "INSERT INTO platform_db.user_account (user_id, keycloak_user_id, email, role, created_at, updated_at) VALUES ($1, $2, $3, $4, now(), now())",
    )
    .bind(&user_id)
    .bind(keycloak_user_id)
    .bind(email)
    .bind(role.as_str())
    .execute(pool)
    .await;
    user_id
}

async fn resolve_partner_id(pool: &PgPool, user_id: &str) -> Option<String> {
    let result: Option<(String,)> = sqlx::query_as(
        "SELECT partner_id FROM platform_db.partner_membership WHERE user_id = $1 LIMIT 1",
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .ok()
    .flatten();
    result.map(|(id,)| id)
}

/// FNV-1a based stub for environments without a database.
fn derive_user_id_stub(keycloak_user_id: &str) -> String {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for b in keycloak_user_id.as_bytes() {
        hash ^= u64::from(*b);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }

    const ALPHABET: &[u8; 32] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";
    let mut suffix = [b'0'; 13];
    let mut value = hash;
    for slot in suffix.iter_mut().rev() {
        *slot = ALPHABET[(value & 0x1f) as usize];
        value >>= 5;
    }
    format!("USR-{}", std::str::from_utf8(&suffix).unwrap())
}

impl From<ProvisionedUser> for CurrentUser {
    fn from(p: ProvisionedUser) -> Self {
        CurrentUser {
            user_id: p.user_id,
            keycloak_user_id: p.keycloak_user_id,
            email: p.email,
            role: p.role,
            partner_id: p.partner_id,
        }
    }
}

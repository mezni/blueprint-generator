use chrono::Utc;
use common_types::generate_id;
use common_types::api::PaginationMeta;
use common_types::EntityPrefix;
use common_types::PartnerStatus;
use sqlx::PgPool;
use sqlx::Postgres;
use sqlx::Transaction;

use crate::error::ServiceError;
use crate::extractors::PaginationParams;
use crate::models::partner::Partner;

pub async fn find_by_id(pool: &PgPool, partner_id: &str) -> Result<Partner, ServiceError> {
    sqlx::query_as::<_, Partner>(
        "SELECT partner_id, name, email, phone, status, created_at, updated_at \
         FROM inventory.partner WHERE partner_id = $1 AND deleted_at IS NULL",
    )
    .bind(partner_id)
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => ServiceError::not_found("Partner", partner_id),
        other => ServiceError::Db(other),
    })
}

pub async fn list_admin_partners(
    pool: &PgPool,
    params: &PaginationParams,
    include_deleted: bool,
    status_filter: Option<PartnerStatus>,
) -> Result<(Vec<Partner>, PaginationMeta), ServiceError> {
    let offset = params.offset();
    let limit = params.limit();

    let (partners, total): (Vec<Partner>, i64) = if let Some(ref status) = status_filter {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM inventory.partner WHERE $1 OR deleted_at IS NULL AND status = $2",
        )
        .bind(include_deleted)
        .bind(status.as_str())
        .fetch_one(pool)
        .await
        .map_err(ServiceError::Db)?;

        let partners = sqlx::query_as::<_, Partner>(
            "SELECT partner_id, name, email, phone, status, created_at, updated_at \
             FROM inventory.partner WHERE $1 OR deleted_at IS NULL AND status = $2 \
             ORDER BY created_at DESC LIMIT $3 OFFSET $4",
        )
        .bind(include_deleted)
        .bind(status.as_str())
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
        .map_err(ServiceError::Db)?;

        (partners, count.0)
    } else {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM inventory.partner WHERE $1 OR deleted_at IS NULL",
        )
        .bind(include_deleted)
        .fetch_one(pool)
        .await
        .map_err(ServiceError::Db)?;

        let partners = sqlx::query_as::<_, Partner>(
            "SELECT partner_id, name, email, phone, status, created_at, updated_at \
             FROM inventory.partner WHERE $1 OR deleted_at IS NULL \
             ORDER BY created_at DESC LIMIT $2 OFFSET $3",
        )
        .bind(include_deleted)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
        .map_err(ServiceError::Db)?;

        (partners, count.0)
    };

    let total_i32 = total as i32;
    let size = params.size();
    let total_pages = total_i32.div_euclid(size) + if total_i32 % size != 0 { 1 } else { 0 };

    let meta = PaginationMeta {
        page: params.page(),
        size,
        total: total_i32,
        total_pages: total_pages.max(0),
        has_next: params.page() < total_pages,
        has_prev: params.page() > 1,
    };

    Ok((partners, meta))
}

pub async fn create_partner(
    tx: &mut Transaction<'_, Postgres>,
    name: &str,
    email: Option<&str>,
    phone: Option<&str>,
) -> Result<Partner, ServiceError> {
    let partner_id = generate_id(EntityPrefix::Prt);
    let now = Utc::now();

    sqlx::query(
        "INSERT INTO inventory.partner (partner_id, name, email, phone, status, created_at, updated_at) \
         VALUES ($1, $2, $3, $4, 'active', $5, $5)",
    )
    .bind(&partner_id)
    .bind(name)
    .bind(email)
    .bind(phone)
    .bind(now)
    .execute(tx as &mut sqlx::PgConnection)
    .await
    .map_err(ServiceError::Db)?;

    Ok(Partner {
        partner_id,
        name: name.to_string(),
        email: email.map(|s| s.to_string()),
        phone: phone.map(|s| s.to_string()),
        status: PartnerStatus::Active,
        created_at: now,
        updated_at: now,
    })
}

#[derive(serde::Deserialize)]
pub struct PartnerUpdateRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub status: Option<PartnerStatus>,
}

pub async fn update_partner(
    tx: &mut Transaction<'_, Postgres>,
    id: &str,
    req: &PartnerUpdateRequest,
    expected_updated_at: chrono::DateTime<Utc>,
) -> Result<Partner, ServiceError> {
    let now = Utc::now();

    let result = sqlx::query_as::<_, Partner>(
        "UPDATE inventory.partner SET \
         name = COALESCE($1, name), \
         email = COALESCE($2, email), \
         phone = COALESCE($3, phone), \
         status = CAST(COALESCE($4, CAST(status AS text)) AS inventory.partner_status), \
         updated_at = $5 \
         WHERE partner_id = $6 AND updated_at = $7 AND deleted_at IS NULL \
         RETURNING partner_id, name, email, phone, status, created_at, updated_at",
    )
    .bind(&req.name)
    .bind(&req.email)
    .bind(&req.phone)
    .bind(req.status.map(|s| s.as_str()))
    .bind(now)
    .bind(id)
    .bind(expected_updated_at)
    .fetch_optional(tx as &mut sqlx::PgConnection)
    .await
    .map_err(ServiceError::Db)?;

    result.ok_or_else(|| {
        ServiceError::Api(common_errors::ApiError {
            code: common_errors::ErrorCode::ConcurrentModification,
            message: format!("Partner '{}' was modified by another request", id),
            details: None,
        })
    })
}

pub async fn soft_delete_partner(tx: &mut Transaction<'_, Postgres>, id: &str) -> Result<(), ServiceError> {
    let result = sqlx::query(
        "UPDATE inventory.partner SET deleted_at = now() \
         WHERE partner_id = $1 AND deleted_at IS NULL",
    )
    .bind(id)
    .execute(tx as &mut sqlx::PgConnection)
    .await?;

    if result.rows_affected() == 0 {
        return Err(ServiceError::not_found("Partner", id));
    }
    Ok(())
}

pub async fn check_active_stations(pool: &PgPool, partner_id: &str) -> Result<bool, ServiceError> {
    let count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM inventory.station \
         WHERE partner_id = $1 AND deleted_at IS NULL AND status = 'active'",
    )
    .bind(partner_id)
    .fetch_one(pool)
    .await
    .map_err(ServiceError::Db)?;
    Ok(count.0 > 0)
}

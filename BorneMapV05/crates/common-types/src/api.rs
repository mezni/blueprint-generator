use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationMeta {
    pub page: i32,
    pub size: i32,
    pub total: i32,
    pub total_pages: i32,
    pub has_next: bool,
    pub has_prev: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessEnvelope<T = serde_json::Value> {
    pub success: bool,
    pub data: T,
    pub meta: PaginationMeta,
}

impl<T> SuccessEnvelope<T> {
    pub fn new(data: T, meta: PaginationMeta) -> Self {
        SuccessEnvelope {
            success: true,
            data,
            meta,
        }
    }

    pub fn new_raw(data: T, meta: PaginationMeta) -> Self {
        SuccessEnvelope { success: true, data, meta }
    }
}

impl<T: Serialize> IntoResponse for SuccessEnvelope<T> {
    fn into_response(self) -> Response {
        (axum::http::StatusCode::OK, Json(self)).into_response()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemEnvelope<T = serde_json::Value> {
    pub success: bool,
    pub data: T,
    pub meta: EmptyMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmptyMeta {}

impl<T> ItemEnvelope<T> {
    pub fn new(data: T) -> Self {
        ItemEnvelope {
            success: true,
            data,
            meta: EmptyMeta {},
        }
    }
}

impl<T: Serialize> IntoResponse for ItemEnvelope<T> {
    fn into_response(self) -> Response {
        (axum::http::StatusCode::OK, Json(self)).into_response()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorDetail {
    pub code: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorEnvelope {
    pub success: bool,
    pub error: ErrorDetail,
}

impl IntoResponse for ErrorEnvelope {
    fn into_response(self) -> Response {
        let status = axum::http::StatusCode::BAD_REQUEST;
        (status, Json(self)).into_response()
    }
}

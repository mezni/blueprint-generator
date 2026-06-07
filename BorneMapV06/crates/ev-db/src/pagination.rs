use serde::{Deserialize, Serialize};

/// Pagination query parameters
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PaginationQuery {
    pub offset: u64,
    pub limit: u64,
}

impl PaginationQuery {
    pub fn new(offset: u64, limit: u64) -> Result<Self, PaginationError> {
        if limit > 100 {
            return Err(PaginationError::LimitExceeded(limit));
        }
        Ok(PaginationQuery { offset, limit })
    }

    pub fn default_page() -> Self {
        PaginationQuery {
            offset: 0,
            limit: 20,
        }
    }
}

/// Paginated response metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub pagination: PaginationMeta,
}

/// Pagination metadata returned to clients
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PaginationMeta {
    pub offset: u64,
    pub limit: u64,
    pub total: u64,
    pub has_more: bool,
}

impl PaginationMeta {
    pub fn new(offset: u64, limit: u64, total: u64) -> Self {
        PaginationMeta {
            offset,
            limit,
            total,
            has_more: (offset + limit) < total,
        }
    }
}

/// Errors related to pagination
#[derive(Debug, Clone, PartialEq)]
pub enum PaginationError {
    LimitExceeded(u64),
}

impl std::fmt::Display for PaginationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PaginationError::LimitExceeded(limit) => {
                write!(f, "pagination limit {} exceeds maximum (100)", limit)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagination_default() {
        let page = PaginationQuery::default_page();
        assert_eq!(page.offset, 0);
        assert_eq!(page.limit, 20);
    }

    #[test]
    fn test_pagination_valid() {
        let page = PaginationQuery::new(0, 50).unwrap();
        assert_eq!(page.offset, 0);
        assert_eq!(page.limit, 50);
    }

    #[test]
    fn test_pagination_limit_exceeded() {
        assert!(PaginationQuery::new(0, 200).is_err());
    }

    #[test]
    fn test_pagination_boundary_max() {
        let page = PaginationQuery::new(0, 100).unwrap();
        assert_eq!(page.limit, 100);
    }

    #[test]
    fn test_pagination_meta_has_more() {
        let meta = PaginationMeta::new(0, 20, 50);
        assert!(meta.has_more);

        let meta = PaginationMeta::new(40, 20, 50);
        assert!(!meta.has_more);
    }

    #[test]
    fn test_pagination_meta_exact() {
        let meta = PaginationMeta::new(0, 20, 20);
        assert!(!meta.has_more);
    }
}

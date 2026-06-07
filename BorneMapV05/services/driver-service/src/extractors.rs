use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct PaginationParams {
    pub page: Option<i32>,
    pub size: Option<i32>,
}

impl PaginationParams {
    pub fn offset(&self) -> i64 {
        ((self.page().max(1) - 1) * self.size()) as i64
    }

    pub fn limit(&self) -> i64 {
        self.size() as i64
    }

    pub fn page(&self) -> i32 {
        self.page.unwrap_or(1).max(1)
    }

    pub fn size(&self) -> i32 {
        self.size.unwrap_or(20).clamp(1, 100)
    }
}

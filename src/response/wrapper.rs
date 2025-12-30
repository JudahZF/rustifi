use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    pub data: T,
    #[serde(default)]
    pub meta: Option<ResponseMeta>,
    #[serde(default)]
    pub errors: Option<Vec<ApiError>>,
}

#[derive(Debug, Deserialize)]
pub struct ResponseMeta {
    pub rc: String,
}

#[derive(Debug, Deserialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub meta: PaginationMeta,
}

#[derive(Debug, Deserialize)]
pub struct PaginationMeta {
    pub total_count: usize,
    pub first: usize,
    pub last: usize,
}

/// Response format for site-scoped endpoints in the new API.
/// Pagination fields are at the top level (not nested in meta).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteResponse<T> {
    pub data: Vec<T>,
    #[serde(default)]
    pub offset: usize,
    #[serde(default)]
    pub limit: usize,
    #[serde(default)]
    pub count: usize,
    #[serde(default)]
    pub total_count: usize,
}

impl<T> SiteResponse<T> {
    /// Check if there are more pages available.
    pub fn has_more(&self) -> bool {
        self.offset + self.count < self.total_count
    }

    /// Get the offset for the next page.
    pub fn next_offset(&self) -> usize {
        self.offset + self.limit
    }
}

pub type ListResponse<T> = ApiResponse<Vec<T>>;
pub type SingleResponse<T> = ApiResponse<T>;

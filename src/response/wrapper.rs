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
    /// Returns `None` if there are no more pages or no valid offset can be computed.
    pub fn next_offset(&self) -> Option<usize> {
        if !self.has_more() {
            return None;
        }
        if self.limit > 0 {
            Some(self.offset + self.limit)
        } else if self.count > 0 {
            Some(self.offset + self.count)
        } else {
            None
        }
    }
}

pub type ListResponse<T> = ApiResponse<Vec<T>>;
pub type SingleResponse<T> = ApiResponse<T>;

/// Response for create/update operations that return the created/updated entity.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MutationResponse<T> {
    pub data: T,
}

/// Response for delete operations.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteResponse {
    #[serde(default)]
    pub http_status_code: Option<u16>,
}

/// Empty response for operations that don't return data.
#[derive(Debug, Clone, Deserialize)]
pub struct EmptyResponse {}

/// Response for action operations that return a status.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionResponse {
    #[serde(default)]
    pub success: bool,
    #[serde(default)]
    pub message: Option<String>,
}

use crate::api::endpoint::{Endpoint, HttpMethod};
use crate::models::Client;
use crate::response::SiteResponse;

/// Fetch all clients for a specific site with optional pagination.
/// Endpoint: GET /v1/sites/{site_id}/clients
#[derive(Debug, Clone)]
pub struct GetClients {
    pub site_id: String,
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

impl GetClients {
    /// Create a new GetClients endpoint without pagination.
    pub fn new(site_id: impl Into<String>) -> Self {
        Self {
            site_id: site_id.into(),
            offset: None,
            limit: None,
        }
    }

    /// Create a new GetClients endpoint with pagination parameters.
    pub fn with_pagination(site_id: impl Into<String>, offset: usize, limit: usize) -> Self {
        Self {
            site_id: site_id.into(),
            offset: Some(offset),
            limit: Some(limit),
        }
    }

    /// Set the offset for pagination.
    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Set the limit for pagination.
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }
}

impl Endpoint for GetClients {
    const PATH: &'static str = "sites/{site_id}/clients";
    const METHOD: HttpMethod = HttpMethod::Get;
    type Response = SiteResponse<Client>;

    fn build_path(&self) -> String {
        format!("sites/{}/clients", self.site_id)
    }

    fn query_params(&self) -> Vec<(&'static str, String)> {
        let mut params = vec![];
        if let Some(offset) = self.offset {
            params.push(("offset", offset.to_string()));
        }
        if let Some(limit) = self.limit {
            params.push(("limit", limit.to_string()));
        }
        params
    }
}

/// Fetch a specific client by ID within a site.
/// Endpoint: GET /v1/sites/{site_id}/clients/{id}
#[derive(Debug, Clone)]
pub struct GetClient {
    pub site_id: String,
    pub id: String,
}

impl GetClient {
    pub fn new(site_id: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            site_id: site_id.into(),
            id: id.into(),
        }
    }
}

impl Endpoint for GetClient {
    const PATH: &'static str = "sites/{site_id}/clients/{id}";
    const METHOD: HttpMethod = HttpMethod::Get;
    type Response = SiteResponse<Client>;

    fn build_path(&self) -> String {
        format!("sites/{}/clients/{}", self.site_id, self.id)
    }
}

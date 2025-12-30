use crate::api::endpoint::{Endpoint, HttpMethod};
use crate::models::Client;
use crate::response::SiteResponse;

/// Fetch all clients for a specific site.
/// Endpoint: GET /v1/sites/{site_id}/clients
#[derive(Debug, Clone)]
pub struct GetClients {
    pub site_id: String,
}

impl GetClients {
    pub fn new(site_id: impl Into<String>) -> Self {
        Self {
            site_id: site_id.into(),
        }
    }
}

impl Endpoint for GetClients {
    const PATH: &'static str = "sites/{site_id}/clients";
    const METHOD: HttpMethod = HttpMethod::Get;
    type Response = SiteResponse<Client>;

    fn build_path(&self) -> String {
        format!("sites/{}/clients", self.site_id)
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

use crate::api::endpoint::{Endpoint, HttpMethod};
use crate::response::{DeleteResponse, MutationResponse, SiteResponse};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// DNS policy action.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DnsAction {
    Allow,
    Block,
}

/// DNS policy.
#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DnsPolicy {
    pub id: String,
    pub name: String,
    pub action: DnsAction,
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub domains: Vec<String>,
    #[serde(default)]
    pub categories: Vec<String>,
}

/// Fetch all DNS policies for a specific site.
/// Endpoint: GET /v1/sites/{siteId}/dns/policies
#[derive(Debug, Clone)]
pub struct GetDnsPolicies {
    pub site_id: String,
}

impl GetDnsPolicies {
    pub fn new(site_id: impl Into<String>) -> Self {
        Self {
            site_id: site_id.into(),
        }
    }
}

impl Endpoint for GetDnsPolicies {
    const PATH: &'static str = "sites/{site_id}/dns/policies";
    const METHOD: HttpMethod = HttpMethod::Get;
    type Response = SiteResponse<DnsPolicy>;

    fn build_path(&self) -> String {
        format!("sites/{}/dns/policies", self.site_id)
    }
}

/// Fetch a specific DNS policy by ID within a site.
/// Endpoint: GET /v1/sites/{siteId}/dns/policies/{id}
#[derive(Debug, Clone)]
pub struct GetDnsPolicy {
    pub site_id: String,
    pub id: String,
}

impl GetDnsPolicy {
    pub fn new(site_id: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            site_id: site_id.into(),
            id: id.into(),
        }
    }
}

impl Endpoint for GetDnsPolicy {
    const PATH: &'static str = "sites/{site_id}/dns/policies/{id}";
    const METHOD: HttpMethod = HttpMethod::Get;
    type Response = SiteResponse<DnsPolicy>;

    fn build_path(&self) -> String {
        format!("sites/{}/dns/policies/{}", self.site_id, self.id)
    }
}

/// Request body for creating or updating a DNS policy.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DnsPolicyRequest {
    pub name: String,
    pub action: DnsAction,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,
}

impl DnsPolicyRequest {
    pub fn new(name: impl Into<String>, action: DnsAction) -> Self {
        Self {
            name: name.into(),
            action,
            enabled: None,
            description: None,
            domains: None,
            categories: None,
        }
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = Some(enabled);
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn domains(mut self, domains: Vec<String>) -> Self {
        self.domains = Some(domains);
        self
    }

    pub fn categories(mut self, categories: Vec<String>) -> Self {
        self.categories = Some(categories);
        self
    }
}

/// Create a new DNS policy within a site.
/// Endpoint: POST /v1/sites/{siteId}/dns/policies
#[derive(Debug, Clone)]
pub struct CreateDnsPolicy {
    pub site_id: String,
    pub request: DnsPolicyRequest,
}

impl CreateDnsPolicy {
    pub fn new(site_id: impl Into<String>, request: DnsPolicyRequest) -> Self {
        Self {
            site_id: site_id.into(),
            request,
        }
    }
}

impl Endpoint for CreateDnsPolicy {
    const PATH: &'static str = "sites/{site_id}/dns/policies";
    const METHOD: HttpMethod = HttpMethod::Post;
    type Response = MutationResponse<DnsPolicy>;

    fn build_path(&self) -> String {
        format!("sites/{}/dns/policies", self.site_id)
    }

    fn request_body(&self) -> Result<Option<Value>, serde_json::Error> {
        Ok(Some(serde_json::to_value(&self.request)?))
    }
}

/// Update an existing DNS policy within a site.
/// Endpoint: PUT /v1/sites/{siteId}/dns/policies/{id}
#[derive(Debug, Clone)]
pub struct UpdateDnsPolicy {
    pub site_id: String,
    pub id: String,
    pub request: DnsPolicyRequest,
}

impl UpdateDnsPolicy {
    pub fn new(
        site_id: impl Into<String>,
        id: impl Into<String>,
        request: DnsPolicyRequest,
    ) -> Self {
        Self {
            site_id: site_id.into(),
            id: id.into(),
            request,
        }
    }
}

impl Endpoint for UpdateDnsPolicy {
    const PATH: &'static str = "sites/{site_id}/dns/policies/{id}";
    const METHOD: HttpMethod = HttpMethod::Put;
    type Response = MutationResponse<DnsPolicy>;

    fn build_path(&self) -> String {
        format!("sites/{}/dns/policies/{}", self.site_id, self.id)
    }

    fn request_body(&self) -> Result<Option<Value>, serde_json::Error> {
        Ok(Some(serde_json::to_value(&self.request)?))
    }
}

/// Delete a DNS policy within a site.
/// Endpoint: DELETE /v1/sites/{siteId}/dns/policies/{id}
#[derive(Debug, Clone)]
pub struct DeleteDnsPolicy {
    pub site_id: String,
    pub id: String,
}

impl DeleteDnsPolicy {
    pub fn new(site_id: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            site_id: site_id.into(),
            id: id.into(),
        }
    }
}

impl Endpoint for DeleteDnsPolicy {
    const PATH: &'static str = "sites/{site_id}/dns/policies/{id}";
    const METHOD: HttpMethod = HttpMethod::Delete;
    type Response = DeleteResponse;

    fn build_path(&self) -> String {
        format!("sites/{}/dns/policies/{}", self.site_id, self.id)
    }
}

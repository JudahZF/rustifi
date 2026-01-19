use crate::api::endpoint::{Endpoint, HttpMethod};
use crate::response::{DeleteResponse, MutationResponse, SiteResponse};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Network management type.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Management {
    Unmanaged,
    Gateway,
    Switch,
}

/// Network origin type.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Origin {
    UserDefined,
    SystemDefined,
    Orchestrated,
}

/// Network metadata.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct NetworkMetadata {
    pub origin: Option<Origin>,
}

/// Network resource from the UniFi API.
#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Network {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub vlan_id: Option<i64>,
    #[serde(default)]
    pub management: Option<Management>,
    #[serde(default)]
    pub metadata: Option<NetworkMetadata>,
}

/// Fetch all networks for a specific site.
/// Endpoint: GET /v1/sites/{siteId}/networks
#[derive(Debug, Clone)]
pub struct GetNetworks {
    pub site_id: String,
}

impl GetNetworks {
    pub fn new(site_id: impl Into<String>) -> Self {
        Self {
            site_id: site_id.into(),
        }
    }
}

impl Endpoint for GetNetworks {
    const PATH: &'static str = "sites/{site_id}/networks";
    const METHOD: HttpMethod = HttpMethod::Get;
    type Response = SiteResponse<Network>;

    fn build_path(&self) -> String {
        format!("sites/{}/networks", self.site_id)
    }
}

/// Fetch a specific network by ID within a site.
/// Endpoint: GET /v1/sites/{siteId}/networks/{id}
#[derive(Debug, Clone)]
pub struct GetNetwork {
    pub site_id: String,
    pub id: String,
}

impl GetNetwork {
    pub fn new(site_id: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            site_id: site_id.into(),
            id: id.into(),
        }
    }
}

impl Endpoint for GetNetwork {
    const PATH: &'static str = "sites/{site_id}/networks/{id}";
    const METHOD: HttpMethod = HttpMethod::Get;
    type Response = SiteResponse<Network>;

    fn build_path(&self) -> String {
        format!("sites/{}/networks/{}", self.site_id, self.id)
    }
}

/// Request body for creating or updating a network.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vlan_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl NetworkRequest {
    /// Create a new network request with only the required name field.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            vlan_id: None,
            enabled: None,
        }
    }

    /// Set the VLAN ID for the network.
    pub fn vlan_id(mut self, vlan_id: i64) -> Self {
        self.vlan_id = Some(vlan_id);
        self
    }

    /// Set whether the network is enabled.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = Some(enabled);
        self
    }
}

/// Create a new network within a site.
/// Endpoint: POST /v1/sites/{siteId}/networks
#[derive(Debug, Clone)]
pub struct CreateNetwork {
    pub site_id: String,
    pub request: NetworkRequest,
}

impl CreateNetwork {
    pub fn new(site_id: impl Into<String>, request: NetworkRequest) -> Self {
        Self {
            site_id: site_id.into(),
            request,
        }
    }
}

impl Endpoint for CreateNetwork {
    const PATH: &'static str = "sites/{site_id}/networks";
    const METHOD: HttpMethod = HttpMethod::Post;
    type Response = MutationResponse<Network>;

    fn build_path(&self) -> String {
        format!("sites/{}/networks", self.site_id)
    }

    fn request_body(&self) -> Result<Option<Value>, serde_json::Error> {
        Ok(Some(serde_json::to_value(&self.request)?))
    }
}

/// Update an existing network within a site.
/// Endpoint: PUT /v1/sites/{siteId}/networks/{id}
#[derive(Debug, Clone)]
pub struct UpdateNetwork {
    pub site_id: String,
    pub id: String,
    pub request: NetworkRequest,
}

impl UpdateNetwork {
    pub fn new(site_id: impl Into<String>, id: impl Into<String>, request: NetworkRequest) -> Self {
        Self {
            site_id: site_id.into(),
            id: id.into(),
            request,
        }
    }
}

impl Endpoint for UpdateNetwork {
    const PATH: &'static str = "sites/{site_id}/networks/{id}";
    const METHOD: HttpMethod = HttpMethod::Put;
    type Response = MutationResponse<Network>;

    fn build_path(&self) -> String {
        format!("sites/{}/networks/{}", self.site_id, self.id)
    }

    fn request_body(&self) -> Result<Option<Value>, serde_json::Error> {
        Ok(Some(serde_json::to_value(&self.request)?))
    }
}

/// Delete a network within a site.
/// Endpoint: DELETE /v1/sites/{siteId}/networks/{id}
#[derive(Debug, Clone)]
pub struct DeleteNetwork {
    pub site_id: String,
    pub id: String,
}

impl DeleteNetwork {
    pub fn new(site_id: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            site_id: site_id.into(),
            id: id.into(),
        }
    }
}

impl Endpoint for DeleteNetwork {
    const PATH: &'static str = "sites/{site_id}/networks/{id}";
    const METHOD: HttpMethod = HttpMethod::Delete;
    type Response = DeleteResponse;

    fn build_path(&self) -> String {
        format!("sites/{}/networks/{}", self.site_id, self.id)
    }
}

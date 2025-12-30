use crate::api::endpoint::{Endpoint, HttpMethod};
use crate::response::SiteResponse;
use serde::Deserialize;

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

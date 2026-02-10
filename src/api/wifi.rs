use crate::api::endpoint::{Endpoint, HttpMethod};
use crate::models::wifi::{WifiBroadcast, WifiSecurity};
use crate::response::{DeleteResponse, MutationResponse, SiteResponse};
use serde::Serialize;
use serde_json::Value;

/// Fetch all WiFi broadcasts (SSIDs) for a specific site.
/// Endpoint: GET /v1/sites/{siteId}/wifi/broadcasts
#[derive(Debug, Clone)]
pub struct GetWifiBroadcasts {
    pub site_id: String,
}

impl GetWifiBroadcasts {
    pub fn new(site_id: impl Into<String>) -> Self {
        Self {
            site_id: site_id.into(),
        }
    }
}

impl Endpoint for GetWifiBroadcasts {
    const PATH: &'static str = "sites/{site_id}/wifi/broadcasts";
    const METHOD: HttpMethod = HttpMethod::Get;
    type Response = SiteResponse<WifiBroadcast>;

    fn build_path(&self) -> String {
        format!("sites/{}/wifi/broadcasts", self.site_id)
    }
}

/// Fetch a specific WiFi broadcast by ID within a site.
/// Endpoint: GET /v1/sites/{siteId}/wifi/broadcasts/{id}
#[derive(Debug, Clone)]
pub struct GetWifiBroadcast {
    pub site_id: String,
    pub id: String,
}

impl GetWifiBroadcast {
    pub fn new(site_id: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            site_id: site_id.into(),
            id: id.into(),
        }
    }
}

impl Endpoint for GetWifiBroadcast {
    const PATH: &'static str = "sites/{site_id}/wifi/broadcasts/{id}";
    const METHOD: HttpMethod = HttpMethod::Get;
    type Response = SiteResponse<WifiBroadcast>;

    fn build_path(&self) -> String {
        format!("sites/{}/wifi/broadcasts/{}", self.site_id, self.id)
    }
}

/// Request body for creating or updating a WiFi broadcast.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WifiBroadcastRequest {
    pub name: String,
    pub ssid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<WifiSecurity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passphrase: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vlan_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_ssid: Option<bool>,
}

impl WifiBroadcastRequest {
    /// Create a new WiFi broadcast request with required fields.
    pub fn new(name: impl Into<String>, ssid: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ssid: ssid.into(),
            enabled: None,
            security: None,
            passphrase: None,
            vlan_id: None,
            hide_ssid: None,
        }
    }

    /// Set whether the broadcast is enabled.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = Some(enabled);
        self
    }

    /// Set the security type.
    pub fn security(mut self, security: WifiSecurity) -> Self {
        self.security = Some(security);
        self
    }

    /// Set the WPA passphrase.
    pub fn passphrase(mut self, passphrase: impl Into<String>) -> Self {
        self.passphrase = Some(passphrase.into());
        self
    }

    /// Set the VLAN ID.
    pub fn vlan_id(mut self, vlan_id: i64) -> Self {
        self.vlan_id = Some(vlan_id);
        self
    }

    /// Set whether to hide the SSID.
    pub fn hide_ssid(mut self, hide_ssid: bool) -> Self {
        self.hide_ssid = Some(hide_ssid);
        self
    }
}

/// Create a new WiFi broadcast within a site.
/// Endpoint: POST /v1/sites/{siteId}/wifi/broadcasts
#[derive(Debug, Clone)]
pub struct CreateWifiBroadcast {
    pub site_id: String,
    pub request: WifiBroadcastRequest,
}

impl CreateWifiBroadcast {
    pub fn new(site_id: impl Into<String>, request: WifiBroadcastRequest) -> Self {
        Self {
            site_id: site_id.into(),
            request,
        }
    }
}

impl Endpoint for CreateWifiBroadcast {
    const PATH: &'static str = "sites/{site_id}/wifi/broadcasts";
    const METHOD: HttpMethod = HttpMethod::Post;
    type Response = MutationResponse<WifiBroadcast>;

    fn build_path(&self) -> String {
        format!("sites/{}/wifi/broadcasts", self.site_id)
    }

    fn request_body(&self) -> Result<Option<Value>, serde_json::Error> {
        Ok(Some(serde_json::to_value(&self.request)?))
    }
}

/// Update an existing WiFi broadcast within a site.
/// Endpoint: PUT /v1/sites/{siteId}/wifi/broadcasts/{id}
#[derive(Debug, Clone)]
pub struct UpdateWifiBroadcast {
    pub site_id: String,
    pub id: String,
    pub request: WifiBroadcastRequest,
}

impl UpdateWifiBroadcast {
    pub fn new(
        site_id: impl Into<String>,
        id: impl Into<String>,
        request: WifiBroadcastRequest,
    ) -> Self {
        Self {
            site_id: site_id.into(),
            id: id.into(),
            request,
        }
    }
}

impl Endpoint for UpdateWifiBroadcast {
    const PATH: &'static str = "sites/{site_id}/wifi/broadcasts/{id}";
    const METHOD: HttpMethod = HttpMethod::Put;
    type Response = MutationResponse<WifiBroadcast>;

    fn build_path(&self) -> String {
        format!("sites/{}/wifi/broadcasts/{}", self.site_id, self.id)
    }

    fn request_body(&self) -> Result<Option<Value>, serde_json::Error> {
        Ok(Some(serde_json::to_value(&self.request)?))
    }
}

/// Delete a WiFi broadcast within a site.
/// Endpoint: DELETE /v1/sites/{siteId}/wifi/broadcasts/{id}
#[derive(Debug, Clone)]
pub struct DeleteWifiBroadcast {
    pub site_id: String,
    pub id: String,
}

impl DeleteWifiBroadcast {
    pub fn new(site_id: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            site_id: site_id.into(),
            id: id.into(),
        }
    }
}

impl Endpoint for DeleteWifiBroadcast {
    const PATH: &'static str = "sites/{site_id}/wifi/broadcasts/{id}";
    const METHOD: HttpMethod = HttpMethod::Delete;
    type Response = DeleteResponse;

    fn build_path(&self) -> String {
        format!("sites/{}/wifi/broadcasts/{}", self.site_id, self.id)
    }
}

use crate::api::endpoint::{Endpoint, HttpMethod};
use crate::models::firewall::{FirewallAction, FirewallPolicy, FirewallZone};
use crate::response::{DeleteResponse, MutationResponse, SiteResponse};
use serde::Serialize;
use serde_json::Value;

// ============================================================================
// Firewall Zones
// ============================================================================

/// Fetch all firewall zones for a specific site.
/// Endpoint: GET /v1/sites/{siteId}/firewall/zones
#[derive(Debug, Clone)]
pub struct GetFirewallZones {
    pub site_id: String,
}

impl GetFirewallZones {
    pub fn new(site_id: impl Into<String>) -> Self {
        Self {
            site_id: site_id.into(),
        }
    }
}

impl Endpoint for GetFirewallZones {
    const PATH: &'static str = "sites/{site_id}/firewall/zones";
    const METHOD: HttpMethod = HttpMethod::Get;
    type Response = SiteResponse<FirewallZone>;

    fn build_path(&self) -> String {
        format!("sites/{}/firewall/zones", self.site_id)
    }
}

/// Fetch a specific firewall zone by ID within a site.
/// Endpoint: GET /v1/sites/{siteId}/firewall/zones/{id}
#[derive(Debug, Clone)]
pub struct GetFirewallZone {
    pub site_id: String,
    pub id: String,
}

impl GetFirewallZone {
    pub fn new(site_id: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            site_id: site_id.into(),
            id: id.into(),
        }
    }
}

impl Endpoint for GetFirewallZone {
    const PATH: &'static str = "sites/{site_id}/firewall/zones/{id}";
    const METHOD: HttpMethod = HttpMethod::Get;
    type Response = SiteResponse<FirewallZone>;

    fn build_path(&self) -> String {
        format!("sites/{}/firewall/zones/{}", self.site_id, self.id)
    }
}

/// Request body for creating or updating a firewall zone.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FirewallZoneRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<Vec<String>>,
}

impl FirewallZoneRequest {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            networks: None,
        }
    }

    pub fn networks(mut self, networks: Vec<String>) -> Self {
        self.networks = Some(networks);
        self
    }
}

/// Create a new firewall zone within a site.
/// Endpoint: POST /v1/sites/{siteId}/firewall/zones
#[derive(Debug, Clone)]
pub struct CreateFirewallZone {
    pub site_id: String,
    pub request: FirewallZoneRequest,
}

impl CreateFirewallZone {
    pub fn new(site_id: impl Into<String>, request: FirewallZoneRequest) -> Self {
        Self {
            site_id: site_id.into(),
            request,
        }
    }
}

impl Endpoint for CreateFirewallZone {
    const PATH: &'static str = "sites/{site_id}/firewall/zones";
    const METHOD: HttpMethod = HttpMethod::Post;
    type Response = MutationResponse<FirewallZone>;

    fn build_path(&self) -> String {
        format!("sites/{}/firewall/zones", self.site_id)
    }

    fn request_body(&self) -> Result<Option<Value>, serde_json::Error> {
        Ok(Some(serde_json::to_value(&self.request)?))
    }
}

/// Update an existing firewall zone within a site.
/// Endpoint: PUT /v1/sites/{siteId}/firewall/zones/{id}
#[derive(Debug, Clone)]
pub struct UpdateFirewallZone {
    pub site_id: String,
    pub id: String,
    pub request: FirewallZoneRequest,
}

impl UpdateFirewallZone {
    pub fn new(
        site_id: impl Into<String>,
        id: impl Into<String>,
        request: FirewallZoneRequest,
    ) -> Self {
        Self {
            site_id: site_id.into(),
            id: id.into(),
            request,
        }
    }
}

impl Endpoint for UpdateFirewallZone {
    const PATH: &'static str = "sites/{site_id}/firewall/zones/{id}";
    const METHOD: HttpMethod = HttpMethod::Put;
    type Response = MutationResponse<FirewallZone>;

    fn build_path(&self) -> String {
        format!("sites/{}/firewall/zones/{}", self.site_id, self.id)
    }

    fn request_body(&self) -> Result<Option<Value>, serde_json::Error> {
        Ok(Some(serde_json::to_value(&self.request)?))
    }
}

/// Delete a firewall zone within a site.
/// Endpoint: DELETE /v1/sites/{siteId}/firewall/zones/{id}
#[derive(Debug, Clone)]
pub struct DeleteFirewallZone {
    pub site_id: String,
    pub id: String,
}

impl DeleteFirewallZone {
    pub fn new(site_id: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            site_id: site_id.into(),
            id: id.into(),
        }
    }
}

impl Endpoint for DeleteFirewallZone {
    const PATH: &'static str = "sites/{site_id}/firewall/zones/{id}";
    const METHOD: HttpMethod = HttpMethod::Delete;
    type Response = DeleteResponse;

    fn build_path(&self) -> String {
        format!("sites/{}/firewall/zones/{}", self.site_id, self.id)
    }
}

// ============================================================================
// Firewall Policies
// ============================================================================

/// Fetch all firewall policies for a specific site.
/// Endpoint: GET /v1/sites/{siteId}/firewall/policies
#[derive(Debug, Clone)]
pub struct GetFirewallPolicies {
    pub site_id: String,
}

impl GetFirewallPolicies {
    pub fn new(site_id: impl Into<String>) -> Self {
        Self {
            site_id: site_id.into(),
        }
    }
}

impl Endpoint for GetFirewallPolicies {
    const PATH: &'static str = "sites/{site_id}/firewall/policies";
    const METHOD: HttpMethod = HttpMethod::Get;
    type Response = SiteResponse<FirewallPolicy>;

    fn build_path(&self) -> String {
        format!("sites/{}/firewall/policies", self.site_id)
    }
}

/// Fetch a specific firewall policy by ID within a site.
/// Endpoint: GET /v1/sites/{siteId}/firewall/policies/{id}
#[derive(Debug, Clone)]
pub struct GetFirewallPolicy {
    pub site_id: String,
    pub id: String,
}

impl GetFirewallPolicy {
    pub fn new(site_id: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            site_id: site_id.into(),
            id: id.into(),
        }
    }
}

impl Endpoint for GetFirewallPolicy {
    const PATH: &'static str = "sites/{site_id}/firewall/policies/{id}";
    const METHOD: HttpMethod = HttpMethod::Get;
    type Response = SiteResponse<FirewallPolicy>;

    fn build_path(&self) -> String {
        format!("sites/{}/firewall/policies/{}", self.site_id, self.id)
    }
}

/// Request body for creating or updating a firewall policy.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FirewallPolicyRequest {
    pub name: String,
    pub action: FirewallAction,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_zone_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_zone_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_addresses: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_addresses: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ports: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_ports: Option<Vec<String>>,
}

impl FirewallPolicyRequest {
    pub fn new(name: impl Into<String>, action: FirewallAction) -> Self {
        Self {
            name: name.into(),
            action,
            source_zone_id: None,
            destination_zone_id: None,
            enabled: None,
            order: None,
            description: None,
            protocol: None,
            source_addresses: None,
            destination_addresses: None,
            source_ports: None,
            destination_ports: None,
        }
    }

    pub fn source_zone_id(mut self, zone_id: impl Into<String>) -> Self {
        self.source_zone_id = Some(zone_id.into());
        self
    }

    pub fn destination_zone_id(mut self, zone_id: impl Into<String>) -> Self {
        self.destination_zone_id = Some(zone_id.into());
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = Some(enabled);
        self
    }

    pub fn order(mut self, order: i32) -> Self {
        self.order = Some(order);
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn protocol(mut self, protocol: impl Into<String>) -> Self {
        self.protocol = Some(protocol.into());
        self
    }

    pub fn source_addresses(mut self, addresses: Vec<String>) -> Self {
        self.source_addresses = Some(addresses);
        self
    }

    pub fn destination_addresses(mut self, addresses: Vec<String>) -> Self {
        self.destination_addresses = Some(addresses);
        self
    }

    pub fn source_ports(mut self, ports: Vec<String>) -> Self {
        self.source_ports = Some(ports);
        self
    }

    pub fn destination_ports(mut self, ports: Vec<String>) -> Self {
        self.destination_ports = Some(ports);
        self
    }
}

/// Create a new firewall policy within a site.
/// Endpoint: POST /v1/sites/{siteId}/firewall/policies
#[derive(Debug, Clone)]
pub struct CreateFirewallPolicy {
    pub site_id: String,
    pub request: FirewallPolicyRequest,
}

impl CreateFirewallPolicy {
    pub fn new(site_id: impl Into<String>, request: FirewallPolicyRequest) -> Self {
        Self {
            site_id: site_id.into(),
            request,
        }
    }
}

impl Endpoint for CreateFirewallPolicy {
    const PATH: &'static str = "sites/{site_id}/firewall/policies";
    const METHOD: HttpMethod = HttpMethod::Post;
    type Response = MutationResponse<FirewallPolicy>;

    fn build_path(&self) -> String {
        format!("sites/{}/firewall/policies", self.site_id)
    }

    fn request_body(&self) -> Result<Option<Value>, serde_json::Error> {
        Ok(Some(serde_json::to_value(&self.request)?))
    }
}

/// Update an existing firewall policy within a site.
/// Endpoint: PUT /v1/sites/{siteId}/firewall/policies/{id}
#[derive(Debug, Clone)]
pub struct UpdateFirewallPolicy {
    pub site_id: String,
    pub id: String,
    pub request: FirewallPolicyRequest,
}

impl UpdateFirewallPolicy {
    pub fn new(
        site_id: impl Into<String>,
        id: impl Into<String>,
        request: FirewallPolicyRequest,
    ) -> Self {
        Self {
            site_id: site_id.into(),
            id: id.into(),
            request,
        }
    }
}

impl Endpoint for UpdateFirewallPolicy {
    const PATH: &'static str = "sites/{site_id}/firewall/policies/{id}";
    const METHOD: HttpMethod = HttpMethod::Put;
    type Response = MutationResponse<FirewallPolicy>;

    fn build_path(&self) -> String {
        format!("sites/{}/firewall/policies/{}", self.site_id, self.id)
    }

    fn request_body(&self) -> Result<Option<Value>, serde_json::Error> {
        Ok(Some(serde_json::to_value(&self.request)?))
    }
}

/// Delete a firewall policy within a site.
/// Endpoint: DELETE /v1/sites/{siteId}/firewall/policies/{id}
#[derive(Debug, Clone)]
pub struct DeleteFirewallPolicy {
    pub site_id: String,
    pub id: String,
}

impl DeleteFirewallPolicy {
    pub fn new(site_id: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            site_id: site_id.into(),
            id: id.into(),
        }
    }
}

impl Endpoint for DeleteFirewallPolicy {
    const PATH: &'static str = "sites/{site_id}/firewall/policies/{id}";
    const METHOD: HttpMethod = HttpMethod::Delete;
    type Response = DeleteResponse;

    fn build_path(&self) -> String {
        format!("sites/{}/firewall/policies/{}", self.site_id, self.id)
    }
}

/// Reorder firewall policies within a site.
/// Endpoint: PUT /v1/sites/{siteId}/firewall/policies/order
#[derive(Debug, Clone)]
pub struct ReorderFirewallPolicies {
    pub site_id: String,
    pub policy_ids: Vec<String>,
}

impl ReorderFirewallPolicies {
    pub fn new(site_id: impl Into<String>, policy_ids: Vec<String>) -> Self {
        Self {
            site_id: site_id.into(),
            policy_ids,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct ReorderRequest {
    policy_ids: Vec<String>,
}

impl Endpoint for ReorderFirewallPolicies {
    const PATH: &'static str = "sites/{site_id}/firewall/policies/order";
    const METHOD: HttpMethod = HttpMethod::Put;
    type Response = SiteResponse<FirewallPolicy>;

    fn build_path(&self) -> String {
        format!("sites/{}/firewall/policies/order", self.site_id)
    }

    fn request_body(&self) -> Result<Option<Value>, serde_json::Error> {
        Ok(Some(serde_json::to_value(ReorderRequest {
            policy_ids: self.policy_ids.clone(),
        })?))
    }
}

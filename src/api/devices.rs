use crate::api::endpoint::{Endpoint, HttpMethod};
use crate::models::{DeviceDetails, DeviceStatistics, SiteDevice};
use crate::response::{EmptyResponse, MutationResponse, SiteResponse};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Fetch all devices for a specific site with optional pagination.
/// Endpoint: GET /v1/sites/{site_id}/devices
#[derive(Debug, Clone)]
pub struct GetDevices {
    pub site_id: String,
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

impl GetDevices {
    /// Create a new GetDevices endpoint without pagination.
    pub fn new(site_id: impl Into<String>) -> Self {
        Self {
            site_id: site_id.into(),
            offset: None,
            limit: None,
        }
    }

    /// Create a new GetDevices endpoint with pagination parameters.
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

impl Endpoint for GetDevices {
    const PATH: &'static str = "sites/{site_id}/devices";
    const METHOD: HttpMethod = HttpMethod::Get;
    type Response = SiteResponse<SiteDevice>;

    fn build_path(&self) -> String {
        format!("sites/{}/devices", self.site_id)
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

/// Fetch a specific device by ID within a site.
/// Endpoint: GET /v1/sites/{site_id}/devices/{id}
#[derive(Debug, Clone)]
pub struct GetDevice {
    pub site_id: String,
    pub id: String,
}

impl GetDevice {
    pub fn new(site_id: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            site_id: site_id.into(),
            id: id.into(),
        }
    }
}

impl Endpoint for GetDevice {
    const PATH: &'static str = "sites/{site_id}/devices/{id}";
    const METHOD: HttpMethod = HttpMethod::Get;
    type Response = SiteResponse<SiteDevice>;

    fn build_path(&self) -> String {
        format!("sites/{}/devices/{}", self.site_id, self.id)
    }
}

/// Fetch detailed information about a specific device within a site.
/// This endpoint returns comprehensive details including physical interfaces,
/// features, and configuration information.
/// Endpoint: GET /v1/sites/{site_id}/devices/{id}
#[derive(Debug, Clone)]
pub struct GetDeviceDetails {
    pub site_id: String,
    pub id: String,
}

impl GetDeviceDetails {
    pub fn new(site_id: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            site_id: site_id.into(),
            id: id.into(),
        }
    }
}

impl Endpoint for GetDeviceDetails {
    const PATH: &'static str = "sites/{site_id}/devices/{id}";
    const METHOD: HttpMethod = HttpMethod::Get;
    type Response = DeviceDetails;

    fn build_path(&self) -> String {
        format!("sites/{}/devices/{}", self.site_id, self.id)
    }
}

/// Fetch the latest statistics for a specific device within a site.
/// Returns current CPU, memory, load averages, uplink rates, and radio statistics.
/// Endpoint: GET /v1/sites/{site_id}/devices/{device_id}/statistics/latest
#[derive(Debug, Clone)]
pub struct GetDeviceStatistics {
    pub site_id: String,
    pub device_id: String,
}

impl GetDeviceStatistics {
    pub fn new(site_id: impl Into<String>, device_id: impl Into<String>) -> Self {
        Self {
            site_id: site_id.into(),
            device_id: device_id.into(),
        }
    }
}

impl Endpoint for GetDeviceStatistics {
    const PATH: &'static str = "sites/{site_id}/devices/{device_id}/statistics/latest";
    const METHOD: HttpMethod = HttpMethod::Get;
    type Response = DeviceStatistics;

    fn build_path(&self) -> String {
        format!(
            "sites/{}/devices/{}/statistics/latest",
            self.site_id, self.device_id
        )
    }
}

/// Pending device awaiting adoption.
#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PendingDevice {
    pub id: String,
    pub mac: String,
    #[serde(default)]
    pub model: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub ip_address: Option<String>,
}

/// Fetch pending devices awaiting adoption for a specific site.
/// Endpoint: GET /v1/sites/{site_id}/devices/pending
#[derive(Debug, Clone)]
pub struct GetPendingDevices {
    pub site_id: String,
}

impl GetPendingDevices {
    pub fn new(site_id: impl Into<String>) -> Self {
        Self {
            site_id: site_id.into(),
        }
    }
}

impl Endpoint for GetPendingDevices {
    const PATH: &'static str = "sites/{site_id}/devices/pending";
    const METHOD: HttpMethod = HttpMethod::Get;
    type Response = SiteResponse<PendingDevice>;

    fn build_path(&self) -> String {
        format!("sites/{}/devices/pending", self.site_id)
    }
}

/// Request body for adopting a device.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdoptDeviceRequest {
    pub mac: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl AdoptDeviceRequest {
    pub fn new(mac: impl Into<String>) -> Self {
        Self {
            mac: mac.into(),
            name: None,
        }
    }

    pub fn with_name(mac: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            mac: mac.into(),
            name: Some(name.into()),
        }
    }
}

/// Adopt a pending device.
/// Endpoint: POST /v1/sites/{siteId}/devices
#[derive(Debug, Clone)]
pub struct AdoptDevice {
    pub site_id: String,
    pub request: AdoptDeviceRequest,
}

impl AdoptDevice {
    pub fn new(site_id: impl Into<String>, request: AdoptDeviceRequest) -> Self {
        Self {
            site_id: site_id.into(),
            request,
        }
    }
}

impl Endpoint for AdoptDevice {
    const PATH: &'static str = "sites/{site_id}/devices";
    const METHOD: HttpMethod = HttpMethod::Post;
    type Response = MutationResponse<SiteDevice>;

    fn build_path(&self) -> String {
        format!("sites/{}/devices", self.site_id)
    }

    fn request_body(&self) -> Result<Option<Value>, serde_json::Error> {
        Ok(Some(serde_json::to_value(&self.request)?))
    }
}

/// Device action types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DeviceAction {
    /// Restart the device.
    Restart,
    /// Turn on locate LED to help find the device.
    LocateOn,
    /// Turn off locate LED.
    LocateOff,
}

/// Request body for device actions.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct DeviceActionRequest {
    action: DeviceAction,
}

/// Execute an action on a device (restart, locate LED).
/// Endpoint: POST /v1/sites/{siteId}/devices/{id}/action
#[derive(Debug, Clone)]
pub struct ExecuteDeviceAction {
    pub site_id: String,
    pub device_id: String,
    pub action: DeviceAction,
}

impl ExecuteDeviceAction {
    pub fn new(
        site_id: impl Into<String>,
        device_id: impl Into<String>,
        action: DeviceAction,
    ) -> Self {
        Self {
            site_id: site_id.into(),
            device_id: device_id.into(),
            action,
        }
    }

    /// Create a restart action for a device.
    pub fn restart(site_id: impl Into<String>, device_id: impl Into<String>) -> Self {
        Self::new(site_id, device_id, DeviceAction::Restart)
    }

    /// Create a locate LED on action for a device.
    pub fn locate_on(site_id: impl Into<String>, device_id: impl Into<String>) -> Self {
        Self::new(site_id, device_id, DeviceAction::LocateOn)
    }

    /// Create a locate LED off action for a device.
    pub fn locate_off(site_id: impl Into<String>, device_id: impl Into<String>) -> Self {
        Self::new(site_id, device_id, DeviceAction::LocateOff)
    }
}

impl Endpoint for ExecuteDeviceAction {
    const PATH: &'static str = "sites/{site_id}/devices/{device_id}/action";
    const METHOD: HttpMethod = HttpMethod::Post;
    type Response = EmptyResponse;

    fn build_path(&self) -> String {
        format!("sites/{}/devices/{}/action", self.site_id, self.device_id)
    }

    fn request_body(&self) -> Result<Option<Value>, serde_json::Error> {
        Ok(Some(serde_json::to_value(DeviceActionRequest {
            action: self.action,
        })?))
    }
}

/// Port action types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PortAction {
    /// Power cycle the port (PoE restart).
    PowerCycle,
}

/// Request body for port actions.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct PortActionRequest {
    action: PortAction,
    port_idx: u32,
}

/// Execute an action on a device port.
/// Endpoint: POST /v1/sites/{siteId}/devices/{id}/ports/action
#[derive(Debug, Clone)]
pub struct ExecutePortAction {
    pub site_id: String,
    pub device_id: String,
    pub port_idx: u32,
    pub action: PortAction,
}

impl ExecutePortAction {
    pub fn new(
        site_id: impl Into<String>,
        device_id: impl Into<String>,
        port_idx: u32,
        action: PortAction,
    ) -> Self {
        Self {
            site_id: site_id.into(),
            device_id: device_id.into(),
            port_idx,
            action,
        }
    }

    /// Create a power cycle action for a port.
    pub fn power_cycle(
        site_id: impl Into<String>,
        device_id: impl Into<String>,
        port_idx: u32,
    ) -> Self {
        Self::new(site_id, device_id, port_idx, PortAction::PowerCycle)
    }
}

impl Endpoint for ExecutePortAction {
    const PATH: &'static str = "sites/{site_id}/devices/{device_id}/ports/action";
    const METHOD: HttpMethod = HttpMethod::Post;
    type Response = EmptyResponse;

    fn build_path(&self) -> String {
        format!(
            "sites/{}/devices/{}/ports/action",
            self.site_id, self.device_id
        )
    }

    fn request_body(&self) -> Result<Option<Value>, serde_json::Error> {
        Ok(Some(serde_json::to_value(PortActionRequest {
            action: self.action,
            port_idx: self.port_idx,
        })?))
    }
}

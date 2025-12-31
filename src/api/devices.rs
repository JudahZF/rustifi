use crate::api::endpoint::{Endpoint, HttpMethod};
use crate::models::{DeviceDetails, DeviceStatistics, SiteDevice};
use crate::response::SiteResponse;

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

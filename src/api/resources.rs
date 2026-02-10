use crate::api::endpoint::{Endpoint, HttpMethod};
use crate::response::SiteResponse;
use serde::Deserialize;

// ============================================================================
// WAN Resources
// ============================================================================

/// WAN interface configuration.
#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Wan {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub interface_type: Option<String>,
    #[serde(default)]
    pub ip_address: Option<String>,
    #[serde(default)]
    pub gateway: Option<String>,
}

/// Fetch all WAN interfaces for a specific site.
/// Endpoint: GET /v1/sites/{siteId}/wans
#[derive(Debug, Clone)]
pub struct GetWans {
    pub site_id: String,
}

impl GetWans {
    pub fn new(site_id: impl Into<String>) -> Self {
        Self {
            site_id: site_id.into(),
        }
    }
}

impl Endpoint for GetWans {
    const PATH: &'static str = "sites/{site_id}/wans";
    const METHOD: HttpMethod = HttpMethod::Get;
    type Response = SiteResponse<Wan>;

    fn build_path(&self) -> String {
        format!("sites/{}/wans", self.site_id)
    }
}

// ============================================================================
// VPN Resources
// ============================================================================

/// VPN connection status.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VpnStatus {
    Connected,
    Disconnected,
    Connecting,
    Error,
    #[serde(other)]
    Unknown,
}

/// VPN server configuration.
#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VpnServer {
    pub id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub vpn_type: Option<String>,
    #[serde(default)]
    pub status: Option<VpnStatus>,
}

/// Fetch all VPN servers for a specific site.
/// Endpoint: GET /v1/sites/{siteId}/vpn/servers
#[derive(Debug, Clone)]
pub struct GetVpnServers {
    pub site_id: String,
}

impl GetVpnServers {
    pub fn new(site_id: impl Into<String>) -> Self {
        Self {
            site_id: site_id.into(),
        }
    }
}

impl Endpoint for GetVpnServers {
    const PATH: &'static str = "sites/{site_id}/vpn/servers";
    const METHOD: HttpMethod = HttpMethod::Get;
    type Response = SiteResponse<VpnServer>;

    fn build_path(&self) -> String {
        format!("sites/{}/vpn/servers", self.site_id)
    }
}

/// Site-to-site VPN tunnel.
#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VpnTunnel {
    pub id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub status: Option<VpnStatus>,
    #[serde(default)]
    pub remote_host: Option<String>,
}

/// Fetch all site-to-site VPN tunnels for a specific site.
/// Endpoint: GET /v1/sites/{siteId}/vpn/site-to-site-tunnels
#[derive(Debug, Clone)]
pub struct GetVpnTunnels {
    pub site_id: String,
}

impl GetVpnTunnels {
    pub fn new(site_id: impl Into<String>) -> Self {
        Self {
            site_id: site_id.into(),
        }
    }
}

impl Endpoint for GetVpnTunnels {
    const PATH: &'static str = "sites/{site_id}/vpn/site-to-site-tunnels";
    const METHOD: HttpMethod = HttpMethod::Get;
    type Response = SiteResponse<VpnTunnel>;

    fn build_path(&self) -> String {
        format!("sites/{}/vpn/site-to-site-tunnels", self.site_id)
    }
}

// ============================================================================
// RADIUS Resources
// ============================================================================

/// RADIUS profile configuration.
#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RadiusProfile {
    pub id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub ip_address: Option<String>,
    #[serde(default)]
    pub port: Option<u16>,
}

/// Fetch all RADIUS profiles for a specific site.
/// Endpoint: GET /v1/sites/{siteId}/radius/profiles
#[derive(Debug, Clone)]
pub struct GetRadiusProfiles {
    pub site_id: String,
}

impl GetRadiusProfiles {
    pub fn new(site_id: impl Into<String>) -> Self {
        Self {
            site_id: site_id.into(),
        }
    }
}

impl Endpoint for GetRadiusProfiles {
    const PATH: &'static str = "sites/{site_id}/radius/profiles";
    const METHOD: HttpMethod = HttpMethod::Get;
    type Response = SiteResponse<RadiusProfile>;

    fn build_path(&self) -> String {
        format!("sites/{}/radius/profiles", self.site_id)
    }
}

// ============================================================================
// Device Tags
// ============================================================================

/// Device tag for grouping devices.
#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DeviceTag {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub device_ids: Vec<String>,
}

/// Fetch all device tags for a specific site.
/// Endpoint: GET /v1/sites/{siteId}/device-tags
#[derive(Debug, Clone)]
pub struct GetDeviceTags {
    pub site_id: String,
}

impl GetDeviceTags {
    pub fn new(site_id: impl Into<String>) -> Self {
        Self {
            site_id: site_id.into(),
        }
    }
}

impl Endpoint for GetDeviceTags {
    const PATH: &'static str = "sites/{site_id}/device-tags";
    const METHOD: HttpMethod = HttpMethod::Get;
    type Response = SiteResponse<DeviceTag>;

    fn build_path(&self) -> String {
        format!("sites/{}/device-tags", self.site_id)
    }
}

// ============================================================================
// DPI (Deep Packet Inspection)
// ============================================================================

/// DPI application category.
#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DpiCategory {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub applications: Vec<String>,
}

/// Fetch all DPI categories.
/// Endpoint: GET /v1/dpi/categories
#[derive(Debug, Clone, Default)]
pub struct GetDpiCategories;

impl Endpoint for GetDpiCategories {
    const PATH: &'static str = "dpi/categories";
    const METHOD: HttpMethod = HttpMethod::Get;
    type Response = SiteResponse<DpiCategory>;
}

/// DPI application.
#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DpiApplication {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub category_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

/// Fetch all DPI applications.
/// Endpoint: GET /v1/dpi/applications
#[derive(Debug, Clone, Default)]
pub struct GetDpiApplications;

impl Endpoint for GetDpiApplications {
    const PATH: &'static str = "dpi/applications";
    const METHOD: HttpMethod = HttpMethod::Get;
    type Response = SiteResponse<DpiApplication>;
}

// ============================================================================
// Countries
// ============================================================================

/// Country reference data.
#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Country {
    pub code: String,
    pub name: String,
}

/// Fetch all available countries.
/// Endpoint: GET /v1/countries
#[derive(Debug, Clone, Default)]
pub struct GetCountries;

impl Endpoint for GetCountries {
    const PATH: &'static str = "countries";
    const METHOD: HttpMethod = HttpMethod::Get;
    type Response = SiteResponse<Country>;
}

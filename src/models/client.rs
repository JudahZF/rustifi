use crate::models::common::IpAddress;
use serde::Deserialize;

/// Client type from the new API format.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Default)]
pub enum ClientType {
    #[serde(alias = "WIRED")]
    Wired,
    #[serde(alias = "WIRELESS")]
    Wireless,
    #[serde(other)]
    #[default]
    Unknown,
}

/// Access type for a client.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Default)]
pub enum AccessType {
    #[serde(alias = "DEFAULT")]
    #[default]
    Default,
    #[serde(alias = "BLOCKED")]
    Blocked,
    #[serde(alias = "ALLOWED")]
    Allowed,
    #[serde(other)]
    Unknown,
}

/// Access configuration for a client.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Default)]
pub struct ClientAccess {
    #[serde(default, rename = "type")]
    pub access_type: AccessType,
}

/// Client from the new site-scoped API.
/// Endpoint: GET /v1/sites/{siteId}/clients
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Client {
    pub id: String,
    #[serde(default, rename = "type")]
    pub client_type: ClientType,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub connected_at: Option<String>,
    #[serde(default)]
    pub ip_address: Option<IpAddress>,
    #[serde(default)]
    pub access: Option<ClientAccess>,
}

impl Client {
    /// Check if the client is wireless.
    pub fn is_wireless(&self) -> bool {
        self.client_type == ClientType::Wireless
    }

    /// Check if the client is wired.
    pub fn is_wired(&self) -> bool {
        self.client_type == ClientType::Wired
    }

    /// Check if the client is connected (has a connected_at timestamp).
    pub fn is_connected(&self) -> bool {
        self.connected_at.is_some()
    }

    /// Check if the client is blocked.
    pub fn is_blocked(&self) -> bool {
        self.access
            .as_ref()
            .map(|a| a.access_type == AccessType::Blocked)
            .unwrap_or(false)
    }
}

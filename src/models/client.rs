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
    #[serde(alias = "GUEST")]
    Guest,
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
#[derive(Clone, Debug, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Client {
    /// Unique client identifier.
    pub id: String,

    /// Type of connection (wired or wireless).
    #[serde(default, rename = "type")]
    pub client_type: ClientType,

    /// Display name of the client.
    #[serde(default)]
    pub name: Option<String>,

    /// Timestamp when the client connected (ISO 8601 format).
    #[serde(default)]
    pub connected_at: Option<String>,

    /// IP address assigned to the client.
    #[serde(default)]
    pub ip_address: Option<IpAddress>,

    /// Access configuration (default, blocked, allowed, or guest).
    #[serde(default)]
    pub access: Option<ClientAccess>,

    /// The UUID of the device this client is connected to.
    /// For wireless clients, this is the access point.
    /// For wired clients, this is the switch or gateway.
    #[serde(default)]
    pub uplink_device_id: Option<String>,
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

    /// Check if the client is a guest.
    pub fn is_guest(&self) -> bool {
        self.access
            .as_ref()
            .map(|a| a.access_type == AccessType::Guest)
            .unwrap_or(false)
    }

    /// Get the device ID this client is connected to, if available.
    pub fn device_id(&self) -> Option<&str> {
        self.uplink_device_id.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_client_with_uplink_device_id() {
        let json_data = json!({
            "id": "client-123",
            "type": "WIRELESS",
            "name": "iPhone",
            "connectedAt": "2024-01-01T12:00:00Z",
            "ipAddress": "192.168.1.100",
            "uplinkDeviceId": "device-456"
        });

        let client: Client = serde_json::from_value(json_data).unwrap();

        assert_eq!(client.id, "client-123");
        assert_eq!(client.client_type, ClientType::Wireless);
        assert_eq!(client.uplink_device_id, Some("device-456".to_string()));
        assert_eq!(client.device_id(), Some("device-456"));
        assert!(client.is_wireless());
        assert!(!client.is_wired());
    }

    #[test]
    fn test_client_without_uplink_device_id() {
        let json_data = json!({
            "id": "client-789",
            "type": "WIRED"
        });

        let client: Client = serde_json::from_value(json_data).unwrap();

        assert_eq!(client.id, "client-789");
        assert!(client.uplink_device_id.is_none());
        assert_eq!(client.device_id(), None);
        assert!(client.is_wired());
    }

    #[test]
    fn test_guest_client() {
        let json_data = json!({
            "id": "guest-client",
            "type": "WIRELESS",
            "access": {
                "type": "GUEST"
            }
        });

        let client: Client = serde_json::from_value(json_data).unwrap();

        assert!(client.is_guest());
        assert!(!client.is_blocked());
    }

    #[test]
    fn test_blocked_client() {
        let json_data = json!({
            "id": "blocked-client",
            "type": "WIRELESS",
            "access": {
                "type": "BLOCKED"
            }
        });

        let client: Client = serde_json::from_value(json_data).unwrap();

        assert!(client.is_blocked());
        assert!(!client.is_guest());
    }

    #[test]
    fn test_access_type_deserialization() {
        let default: AccessType = serde_json::from_str("\"DEFAULT\"").unwrap();
        let blocked: AccessType = serde_json::from_str("\"BLOCKED\"").unwrap();
        let allowed: AccessType = serde_json::from_str("\"ALLOWED\"").unwrap();
        let guest: AccessType = serde_json::from_str("\"GUEST\"").unwrap();
        let unknown: AccessType = serde_json::from_str("\"SOMETHING_ELSE\"").unwrap();

        assert_eq!(default, AccessType::Default);
        assert_eq!(blocked, AccessType::Blocked);
        assert_eq!(allowed, AccessType::Allowed);
        assert_eq!(guest, AccessType::Guest);
        assert_eq!(unknown, AccessType::Unknown);
    }

    #[test]
    fn test_is_connected() {
        let connected = Client {
            id: "client-1".to_string(),
            connected_at: Some("2024-01-01T12:00:00Z".to_string()),
            ..Default::default()
        };
        assert!(connected.is_connected());

        let disconnected = Client {
            id: "client-2".to_string(),
            connected_at: None,
            ..Default::default()
        };
        assert!(!disconnected.is_connected());
    }
}

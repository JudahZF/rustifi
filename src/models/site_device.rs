use crate::models::common::{IpAddress, MacAddress};
use serde::Deserialize;

/// Device state as returned by the new site devices API.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DeviceState {
    Online,
    Offline,
    PendingAdoption,
    Updating,
    GettingReady,
    Adopting,
    Deleting,
    ConnectionInterrupted,
    Isolated,
    #[serde(rename = "U5G_INCORRECT_TOPOLOGY")]
    U5gIncorrectTopology,
    #[default]
    #[serde(other)]
    Unknown,
}

/// Device feature capabilities.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum DeviceFeature {
    Switching,
    AccessPoint,
    Gateway,
    #[serde(other)]
    Unknown,
}

/// Device interface types.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum DeviceInterface {
    Ports,
    Radios,
    #[serde(other)]
    Unknown,
}

/// Device model for the new site-scoped API.
/// Endpoint: GET /v1/sites/{siteId}/devices
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SiteDevice {
    /// Unique device identifier (UUID format)
    pub id: String,

    /// MAC address of the device
    pub mac_address: MacAddress,

    /// IP address of the device
    #[serde(default)]
    pub ip_address: Option<IpAddress>,

    /// Display name of the device
    pub name: String,

    /// Device model identifier (e.g., "UHDIW", "U6-Pro")
    pub model: String,

    /// Current device state
    #[serde(default)]
    pub state: DeviceState,

    /// Whether the device is supported
    #[serde(default)]
    pub supported: Option<bool>,

    /// Current firmware version
    #[serde(default)]
    pub firmware_version: Option<String>,

    /// Whether firmware can be updated
    #[serde(default)]
    pub firmware_updatable: Option<bool>,

    /// List of device features (e.g., Switching, AccessPoint, Gateway)
    #[serde(default)]
    pub features: Vec<DeviceFeature>,

    /// List of device interfaces (e.g., Ports, Radios)
    #[serde(default)]
    pub interfaces: Vec<DeviceInterface>,
}

impl SiteDevice {
    /// Check if the device is currently online.
    pub fn is_online(&self) -> bool {
        self.state == DeviceState::Online
    }

    /// Check if the device is in a transitional state (updating, adopting, etc.)
    pub fn is_transitioning(&self) -> bool {
        matches!(
            self.state,
            DeviceState::PendingAdoption
                | DeviceState::Updating
                | DeviceState::GettingReady
                | DeviceState::Adopting
                | DeviceState::Deleting
        )
    }

    /// Check if the device has connectivity issues.
    pub fn has_connectivity_issues(&self) -> bool {
        matches!(
            self.state,
            DeviceState::Offline
                | DeviceState::ConnectionInterrupted
                | DeviceState::Isolated
                | DeviceState::U5gIncorrectTopology
        )
    }

    /// Check if the device supports switching.
    pub fn has_switching(&self) -> bool {
        self.features.contains(&DeviceFeature::Switching)
    }

    /// Check if the device is an access point.
    pub fn is_access_point(&self) -> bool {
        self.features.contains(&DeviceFeature::AccessPoint)
    }

    /// Check if the device is a gateway.
    pub fn is_gateway(&self) -> bool {
        self.features.contains(&DeviceFeature::Gateway)
    }

    /// Check if the device has physical ports.
    pub fn has_ports(&self) -> bool {
        self.interfaces.contains(&DeviceInterface::Ports)
    }

    /// Check if the device has radios (wireless).
    pub fn has_radios(&self) -> bool {
        self.interfaces.contains(&DeviceInterface::Radios)
    }

    /// Check if the device is a switch (has switching feature but not gateway).
    /// Note: Gateways like UDM-Pro also have switching capability, so this
    /// method excludes devices that are primarily gateways.
    pub fn is_switch(&self) -> bool {
        self.has_switching() && !self.is_gateway()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_site_device_deserialization() {
        let json_data = json!({
            "id": "497f6eca-6276-4993-bfeb-53cbbbba6f08",
            "macAddress": "94:2a:6f:26:c6:ca",
            "ipAddress": "192.168.1.55",
            "name": "IW HD",
            "model": "UHDIW",
            "state": "ONLINE",
            "supported": true,
            "firmwareVersion": "6.6.55",
            "firmwareUpdatable": true,
            "features": ["switching"],
            "interfaces": ["ports"]
        });

        let device: SiteDevice = serde_json::from_value(json_data).unwrap();

        assert_eq!(device.id, "497f6eca-6276-4993-bfeb-53cbbbba6f08");
        assert_eq!(device.mac_address.as_str(), "94:2a:6f:26:c6:ca");
        assert_eq!(
            device.ip_address.as_ref().unwrap().to_string(),
            "192.168.1.55"
        );
        assert_eq!(device.name, "IW HD");
        assert_eq!(device.model, "UHDIW");
        assert_eq!(device.state, DeviceState::Online);
        assert_eq!(device.supported, Some(true));
        assert_eq!(device.firmware_version, Some("6.6.55".to_string()));
        assert_eq!(device.firmware_updatable, Some(true));
        assert_eq!(device.features, vec![DeviceFeature::Switching]);
        assert_eq!(device.interfaces, vec![DeviceInterface::Ports]);
    }

    #[test]
    fn test_site_device_with_access_point() {
        let json_data = json!({
            "id": "test-ap",
            "macAddress": "aa:bb:cc:dd:ee:ff",
            "name": "U6 Pro",
            "model": "U6-Pro",
            "state": "ONLINE",
            "features": ["accessPoint"],
            "interfaces": ["radios"]
        });

        let device: SiteDevice = serde_json::from_value(json_data).unwrap();

        assert!(device.is_access_point());
        assert!(device.has_radios());
        assert!(!device.has_switching());
        assert!(!device.has_ports());
    }

    #[test]
    fn test_site_device_with_gateway() {
        let json_data = json!({
            "id": "test-gw",
            "macAddress": "aa:bb:cc:dd:ee:ff",
            "name": "UDM Pro",
            "model": "UDM-Pro",
            "state": "ONLINE",
            "features": ["gateway", "switching"],
            "interfaces": ["ports"]
        });

        let device: SiteDevice = serde_json::from_value(json_data).unwrap();

        assert!(device.is_gateway());
        assert!(device.has_switching());
        assert!(device.has_ports());
        assert!(!device.is_access_point());
    }

    #[test]
    fn test_site_device_is_online() {
        let device = SiteDevice {
            id: "test".to_string(),
            mac_address: MacAddress::default(),
            ip_address: None,
            name: "Test".to_string(),
            model: "U6-Pro".to_string(),
            state: DeviceState::Online,
            supported: Some(true),
            firmware_version: None,
            firmware_updatable: None,
            features: vec![],
            interfaces: vec![],
        };

        assert!(device.is_online());
    }

    #[test]
    fn test_site_device_transitioning_states() {
        for state in [
            DeviceState::PendingAdoption,
            DeviceState::Updating,
            DeviceState::GettingReady,
            DeviceState::Adopting,
            DeviceState::Deleting,
        ] {
            let device = SiteDevice {
                id: "test".to_string(),
                mac_address: MacAddress::default(),
                ip_address: None,
                name: "Test".to_string(),
                model: "U6-Pro".to_string(),
                state,
                supported: Some(true),
                firmware_version: None,
                firmware_updatable: None,
                features: vec![],
                interfaces: vec![],
            };

            assert!(device.is_transitioning());
            assert!(!device.is_online());
        }
    }

    #[test]
    fn test_site_device_connectivity_issues() {
        for state in [
            DeviceState::Offline,
            DeviceState::ConnectionInterrupted,
            DeviceState::Isolated,
            DeviceState::U5gIncorrectTopology,
        ] {
            let device = SiteDevice {
                id: "test".to_string(),
                mac_address: MacAddress::default(),
                ip_address: None,
                name: "Test".to_string(),
                model: "U6-Pro".to_string(),
                state,
                supported: Some(true),
                firmware_version: None,
                firmware_updatable: None,
                features: vec![],
                interfaces: vec![],
            };

            assert!(device.has_connectivity_issues());
            assert!(!device.is_online());
        }
    }

    #[test]
    fn test_device_state_variants() {
        let online: DeviceState = serde_json::from_str("\"ONLINE\"").unwrap();
        let offline: DeviceState = serde_json::from_str("\"OFFLINE\"").unwrap();
        let pending: DeviceState = serde_json::from_str("\"PENDING_ADOPTION\"").unwrap();
        let updating: DeviceState = serde_json::from_str("\"UPDATING\"").unwrap();
        let getting_ready: DeviceState = serde_json::from_str("\"GETTING_READY\"").unwrap();
        let adopting: DeviceState = serde_json::from_str("\"ADOPTING\"").unwrap();
        let deleting: DeviceState = serde_json::from_str("\"DELETING\"").unwrap();
        let interrupted: DeviceState = serde_json::from_str("\"CONNECTION_INTERRUPTED\"").unwrap();
        let isolated: DeviceState = serde_json::from_str("\"ISOLATED\"").unwrap();
        let u5g: DeviceState = serde_json::from_str("\"U5G_INCORRECT_TOPOLOGY\"").unwrap();
        let unknown: DeviceState = serde_json::from_str("\"SOMETHING_ELSE\"").unwrap();

        assert_eq!(online, DeviceState::Online);
        assert_eq!(offline, DeviceState::Offline);
        assert_eq!(pending, DeviceState::PendingAdoption);
        assert_eq!(updating, DeviceState::Updating);
        assert_eq!(getting_ready, DeviceState::GettingReady);
        assert_eq!(adopting, DeviceState::Adopting);
        assert_eq!(deleting, DeviceState::Deleting);
        assert_eq!(interrupted, DeviceState::ConnectionInterrupted);
        assert_eq!(isolated, DeviceState::Isolated);
        assert_eq!(u5g, DeviceState::U5gIncorrectTopology);
        assert_eq!(unknown, DeviceState::Unknown);
    }

    #[test]
    fn test_device_feature_variants() {
        let switching: DeviceFeature = serde_json::from_str("\"switching\"").unwrap();
        let access_point: DeviceFeature = serde_json::from_str("\"accessPoint\"").unwrap();
        let gateway: DeviceFeature = serde_json::from_str("\"gateway\"").unwrap();
        let unknown: DeviceFeature = serde_json::from_str("\"somethingElse\"").unwrap();

        assert_eq!(switching, DeviceFeature::Switching);
        assert_eq!(access_point, DeviceFeature::AccessPoint);
        assert_eq!(gateway, DeviceFeature::Gateway);
        assert_eq!(unknown, DeviceFeature::Unknown);
    }

    #[test]
    fn test_device_interface_variants() {
        let ports: DeviceInterface = serde_json::from_str("\"ports\"").unwrap();
        let radios: DeviceInterface = serde_json::from_str("\"radios\"").unwrap();
        let unknown: DeviceInterface = serde_json::from_str("\"somethingElse\"").unwrap();

        assert_eq!(ports, DeviceInterface::Ports);
        assert_eq!(radios, DeviceInterface::Radios);
        assert_eq!(unknown, DeviceInterface::Unknown);
    }

    #[test]
    fn test_site_device_is_switch() {
        // A pure switch device
        let switch_json = json!({
            "id": "switch-id",
            "macAddress": "aa:bb:cc:dd:ee:ff",
            "name": "USW-24",
            "model": "USW-24",
            "state": "ONLINE",
            "features": ["switching"],
            "interfaces": ["ports"]
        });
        let switch_device: SiteDevice = serde_json::from_value(switch_json).unwrap();
        assert!(switch_device.is_switch());
        assert!(switch_device.has_switching());
        assert!(!switch_device.is_gateway());

        // An access point is not a switch
        let ap_json = json!({
            "id": "ap-id",
            "macAddress": "aa:bb:cc:dd:ee:ff",
            "name": "U6-Pro",
            "model": "U6-Pro",
            "state": "ONLINE",
            "features": ["accessPoint"],
            "interfaces": ["radios"]
        });
        let ap_device: SiteDevice = serde_json::from_value(ap_json).unwrap();
        assert!(!ap_device.is_switch());
        assert!(!ap_device.has_switching());

        // A gateway with switching capability is NOT considered a switch
        let gateway_json = json!({
            "id": "gateway-id",
            "macAddress": "aa:bb:cc:dd:ee:ff",
            "name": "UDM-Pro",
            "model": "UDM-Pro",
            "state": "ONLINE",
            "features": ["gateway", "switching"],
            "interfaces": ["ports"]
        });
        let gateway_device: SiteDevice = serde_json::from_value(gateway_json).unwrap();
        assert!(!gateway_device.is_switch()); // Gateway takes precedence
        assert!(gateway_device.has_switching());
        assert!(gateway_device.is_gateway());
    }
}

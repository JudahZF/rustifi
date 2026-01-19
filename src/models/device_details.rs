use crate::models::common::{IpAddress, MacAddress};
use serde::Deserialize;

/// Port connector type.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub enum PortConnector {
    #[serde(rename = "RJ45")]
    Rj45,
    #[serde(rename = "SFP")]
    Sfp,
    #[serde(rename = "SFPPLUS")]
    SfpPlus,
    #[serde(rename = "SFP28")]
    Sfp28,
    #[serde(rename = "QSFP28")]
    Qsfp28,
    #[serde(other)]
    Unknown,
}

/// Port interface state.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InterfaceState {
    Up,
    Down,
    #[serde(other)]
    Unknown,
}

/// Power over Ethernet configuration and state.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PoE {
    /// PoE standard (e.g., "802.3bt")
    #[serde(default)]
    pub standard: Option<String>,

    /// PoE type identifier
    #[serde(default, rename = "type")]
    pub r#type: Option<i32>,

    /// Whether PoE is enabled
    #[serde(default)]
    pub enabled: Option<bool>,

    /// Current PoE state
    #[serde(default)]
    pub state: Option<String>,
}

impl PoE {
    /// Check if PoE is enabled on this port.
    pub fn is_enabled(&self) -> bool {
        self.enabled.unwrap_or(false)
    }

    /// Check if PoE is actively delivering power (enabled and state is UP).
    pub fn is_active(&self) -> bool {
        self.is_enabled() && self.state.as_deref() == Some("UP")
    }
}

/// Port information for physical interfaces.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Port {
    /// Port index identifier (1-based)
    pub idx: i32,

    /// Current port state
    pub state: InterfaceState,

    /// Port connector type
    pub connector: PortConnector,

    /// Maximum speed in Mbps
    pub max_speed_mbps: i32,

    /// Current speed in Mbps
    #[serde(default)]
    pub speed_mbps: Option<i32>,

    /// Power over Ethernet configuration
    #[serde(default)]
    pub poe: Option<PoE>,
}

impl Port {
    /// Check if this port has PoE capability.
    pub fn has_poe(&self) -> bool {
        self.poe.is_some()
    }

    /// Check if this port is actively delivering PoE power.
    pub fn is_poe_active(&self) -> bool {
        self.poe.as_ref().map(|p| p.is_active()).unwrap_or(false)
    }

    /// Check if PoE is enabled on this port.
    pub fn is_poe_enabled(&self) -> bool {
        self.poe.as_ref().map(|p| p.is_enabled()).unwrap_or(false)
    }

    /// Check if the port link is up.
    pub fn is_up(&self) -> bool {
        self.state == InterfaceState::Up
    }

    /// Check if the port link is down.
    pub fn is_down(&self) -> bool {
        self.state == InterfaceState::Down
    }

    /// Get the current speed in Gbps, if available.
    pub fn speed_gbps(&self) -> Option<f64> {
        self.speed_mbps.map(|s| s as f64 / 1000.0)
    }
}

/// Wireless radio standard.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub enum WirelessStandard {
    #[serde(rename = "802.11a")]
    Standard802_11a,
    #[serde(rename = "802.11b")]
    Standard802_11b,
    #[serde(rename = "802.11g")]
    Standard802_11g,
    #[serde(rename = "802.11n")]
    Standard802_11n,
    #[serde(rename = "802.11ac")]
    Standard802_11ac,
    #[serde(rename = "802.11ax")]
    Standard802_11ax,
    #[serde(rename = "802.11be")]
    Standard802_11be,
    #[serde(other)]
    Unknown,
}

/// Wireless radio information.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Radio {
    /// Wireless standard (802.11a, 802.11ac, etc.)
    pub wlan_standard: WirelessStandard,

    /// Frequency in GHz (2.4, 5, 6, or 60)
    #[serde(rename = "frequencyGHz")]
    pub frequency_ghz: f64,

    /// Channel width in MHz
    #[serde(rename = "channelWidthMHz")]
    pub channel_width_mhz: i32,

    /// Current channel number
    #[serde(default)]
    pub channel: Option<i32>,
}

/// Device physical interfaces.
#[derive(Clone, Debug, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PhysicalInterfaces {
    /// List of physical ports
    #[serde(default)]
    pub ports: Vec<Port>,

    /// List of wireless radios
    #[serde(default)]
    pub radios: Vec<Radio>,
}

/// Device uplink connection information.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DeviceUplink {
    /// ID of the parent device in the network topology
    pub device_id: String,
}

/// Device switching feature details.
#[derive(Clone, Debug, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SwitchingFeature {
    // Currently empty, but structured for future expansion
}

/// Device access point feature details.
#[derive(Clone, Debug, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AccessPointFeature {
    // Currently empty, but structured for future expansion
}

/// Device features and capabilities.
#[derive(Clone, Debug, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceFeatures {
    /// Switching feature details (if supported)
    #[serde(default)]
    pub switching: Option<SwitchingFeature>,

    /// Access point feature details (if supported)
    #[serde(default)]
    pub access_point: Option<AccessPointFeature>,
}

/// Detailed device information from the device details endpoint.
/// Endpoint: GET /v1/sites/{siteId}/devices/{deviceId}
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDetails {
    /// Unique device identifier (UUID)
    pub id: String,

    /// MAC address of the device
    pub mac_address: MacAddress,

    /// IP address of the device
    pub ip_address: IpAddress,

    /// Display name of the device
    pub name: String,

    /// Device model identifier (e.g., "UHDIW", "U6-Pro")
    pub model: String,

    /// Whether the device is supported
    pub supported: bool,

    /// Current device state (ONLINE, OFFLINE, etc.)
    pub state: String,

    /// Current firmware version
    #[serde(default)]
    pub firmware_version: Option<String>,

    /// Whether firmware can be updated
    pub firmware_updatable: bool,

    /// When the device was adopted (ISO 8601 timestamp)
    #[serde(default)]
    pub adopted_at: Option<String>,

    /// When the device was provisioned (ISO 8601 timestamp)
    #[serde(default)]
    pub provisioned_at: Option<String>,

    /// Configuration ID
    pub configuration_id: String,

    /// Device uplink connection info (optional)
    #[serde(default)]
    pub uplink: Option<DeviceUplink>,

    /// Device features and capabilities
    pub features: DeviceFeatures,

    /// Physical interfaces (ports and radios)
    pub interfaces: PhysicalInterfaces,
}

impl DeviceDetails {
    /// Check if the device is currently online.
    pub fn is_online(&self) -> bool {
        self.state == "ONLINE"
    }

    /// Check if the device is currently offline.
    pub fn is_offline(&self) -> bool {
        self.state == "OFFLINE"
    }

    /// Get the number of available ports.
    pub fn port_count(&self) -> usize {
        self.interfaces.ports.len()
    }

    /// Get the number of available radios.
    pub fn radio_count(&self) -> usize {
        self.interfaces.radios.len()
    }

    /// Check if the device has switching capability.
    pub fn has_switching(&self) -> bool {
        self.features.switching.is_some()
    }

    /// Check if the device has access point capability.
    pub fn has_access_point(&self) -> bool {
        self.features.access_point.is_some()
    }

    /// Get all ports that are currently UP.
    pub fn active_ports(&self) -> Vec<&Port> {
        self.interfaces
            .ports
            .iter()
            .filter(|p| p.state == InterfaceState::Up)
            .collect()
    }

    /// Get all ports that are currently DOWN.
    pub fn inactive_ports(&self) -> Vec<&Port> {
        self.interfaces
            .ports
            .iter()
            .filter(|p| p.state == InterfaceState::Down)
            .collect()
    }

    /// Check if this device is a switch (has switching capability).
    pub fn is_switch(&self) -> bool {
        self.has_switching()
    }

    /// Get all ports with PoE capability.
    pub fn poe_ports(&self) -> Vec<&Port> {
        self.interfaces
            .ports
            .iter()
            .filter(|p| p.has_poe())
            .collect()
    }

    /// Get all ports with PoE enabled.
    pub fn poe_enabled_ports(&self) -> Vec<&Port> {
        self.interfaces
            .ports
            .iter()
            .filter(|p| p.is_poe_enabled())
            .collect()
    }

    /// Get all ports actively delivering PoE power.
    pub fn poe_active_ports(&self) -> Vec<&Port> {
        self.interfaces
            .ports
            .iter()
            .filter(|p| p.is_poe_active())
            .collect()
    }

    /// Get the count of PoE-capable ports.
    pub fn poe_port_count(&self) -> usize {
        self.poe_ports().len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_radio_deserialization() {
        let radio_json = json!({
            "wlanStandard": "802.11a",
            "frequencyGHz": 2.4,
            "channelWidthMHz": 40,
            "channel": 36
        });

        match serde_json::from_value::<Radio>(radio_json) {
            Ok(radio) => assert_eq!(radio.frequency_ghz, 2.4),
            Err(e) => panic!("Failed to deserialize radio: {}", e),
        }
    }

    #[test]
    fn test_device_details_deserialization() {
        let radio_json = json!({
            "wlanStandard": "802.11a",
            "frequencyGHz": 2.4,
            "channelWidthMHz": 40,
            "channel": 36
        });

        match serde_json::from_value::<Radio>(radio_json) {
            Ok(radio) => assert_eq!(radio.frequency_ghz, 2.4),
            Err(e) => panic!("Failed to deserialize radio in device test: {}", e),
        }

        let json_data = json!({
            "id": "497f6eca-6276-4993-bfeb-53cbbbba6f08",
            "macAddress": "94:2a:6f:26:c6:ca",
            "ipAddress": "192.168.1.55",
            "name": "IW HD",
            "model": "UHDIW",
            "supported": true,
            "state": "ONLINE",
            "firmwareVersion": "6.6.55",
            "firmwareUpdatable": true,
            "adoptedAt": "2019-08-24T14:15:22Z",
            "provisionedAt": "2019-08-24T14:15:22Z",
            "configurationId": "7596498d2f367dc2",
            "uplink": {
                "deviceId": "4de4adb9-21ee-47e3-aeb4-8cf8ed6c109a"
            },
            "features": {
                "switching": null,
                "accessPoint": null
            },
            "interfaces": {
                "ports": [
                    {
                        "idx": 1,
                        "state": "UP",
                        "connector": "RJ45",
                        "maxSpeedMbps": 10000,
                        "speedMbps": 1000,
                        "poe": {
                            "standard": "802.3bt",
                            "type": 3,
                            "enabled": true,
                            "state": "UP"
                        }
                    }
                ],
                "radios": [
                    {
                        "wlanStandard": "802.11a",
                        "frequencyGHz": 2.4,
                        "channelWidthMHz": 40,
                        "channel": 36
                    }
                ]
            }
        });

        let device: DeviceDetails = serde_json::from_value(json_data).unwrap();

        assert_eq!(device.id, "497f6eca-6276-4993-bfeb-53cbbbba6f08");
        assert_eq!(device.mac_address.as_str(), "94:2a:6f:26:c6:ca");
        assert_eq!(device.ip_address.as_str(), "192.168.1.55");
        assert_eq!(device.name, "IW HD");
        assert_eq!(device.model, "UHDIW");
        assert!(device.supported);
        assert_eq!(device.state, "ONLINE");
        assert_eq!(device.firmware_version, Some("6.6.55".to_string()));
        assert!(device.firmware_updatable);
        assert_eq!(device.port_count(), 1);
        assert_eq!(device.radio_count(), 1);
        assert!(device.is_online());
    }

    #[test]
    fn test_device_details_ports() {
        let json_data = json!({
            "id": "497f6eca-6276-4993-bfeb-53cbbbba6f08",
            "macAddress": "94:2a:6f:26:c6:ca",
            "ipAddress": "192.168.1.55",
            "name": "Test Switch",
            "model": "USW-24",
            "supported": true,
            "state": "ONLINE",
            "firmwareUpdatable": false,
            "configurationId": "config123",
            "uplink": {
                "deviceId": "uplink-device"
            },
            "features": {},
            "interfaces": {
                "ports": [
                    {
                        "idx": 1,
                        "state": "UP",
                        "connector": "RJ45",
                        "maxSpeedMbps": 1000,
                        "speedMbps": 1000
                    },
                    {
                        "idx": 2,
                        "state": "DOWN",
                        "connector": "RJ45",
                        "maxSpeedMbps": 1000
                    }
                ],
                "radios": []
            }
        });

        let device: DeviceDetails = serde_json::from_value(json_data).unwrap();

        assert_eq!(device.port_count(), 2);
        assert_eq!(device.radio_count(), 0);
        assert_eq!(device.active_ports().len(), 1);
        assert_eq!(device.inactive_ports().len(), 1);
    }

    #[test]
    fn test_device_details_features() {
        let json_data = json!({
            "id": "device-with-features",
            "macAddress": "94:2a:6f:26:c6:ca",
            "ipAddress": "192.168.1.55",
            "name": "Multi-function Device",
            "model": "UDM-Pro",
            "supported": true,
            "state": "ONLINE",
            "firmwareUpdatable": false,
            "configurationId": "config123",
            "uplink": {
                "deviceId": "uplink-device"
            },
            "features": {
                "switching": {},
                "accessPoint": {}
            },
            "interfaces": {
                "ports": [],
                "radios": []
            }
        });

        let device: DeviceDetails = serde_json::from_value(json_data).unwrap();

        assert!(device.has_switching());
        assert!(device.has_access_point());
    }

    #[test]
    fn test_wireless_standard_deserialization() {
        let standards = vec![
            ("\"802.11a\"", WirelessStandard::Standard802_11a),
            ("\"802.11b\"", WirelessStandard::Standard802_11b),
            ("\"802.11g\"", WirelessStandard::Standard802_11g),
            ("\"802.11n\"", WirelessStandard::Standard802_11n),
            ("\"802.11ac\"", WirelessStandard::Standard802_11ac),
            ("\"802.11ax\"", WirelessStandard::Standard802_11ax),
            ("\"802.11be\"", WirelessStandard::Standard802_11be),
        ];

        for (json_str, expected) in standards {
            let standard: WirelessStandard = serde_json::from_str(json_str).unwrap();
            assert_eq!(standard, expected);
        }
    }

    #[test]
    fn test_device_details_real_world_response() {
        // Test with real response from user's device
        let json_data = json!({
            "id": "4dd8e91c-9997-3736-9480-3c8591c47452",
            "macAddress": "1c:0b:8b:3e:5c:16",
            "ipAddress": "92.18.58.32",
            "name": "Battin Lane UX7",
            "model": "Express 7",
            "supported": true,
            "state": "ONLINE",
            "firmwareVersion": "5.0.10",
            "firmwareUpdatable": false,
            "provisionedAt": "2025-12-30T21:27:06Z",
            "configurationId": "b4aae6c29779cbea",
            "uplink": {
                "deviceId": "parent-device-id"
            },
            "features": {
                "switching": {},
                "accessPoint": {}
            },
            "interfaces": {
                "ports": [
                    {
                        "idx": 1,
                        "state": "UP",
                        "connector": "RJ45",
                        "maxSpeedMbps": 2500,
                        "speedMbps": 1000
                    },
                    {
                        "idx": 2,
                        "state": "UP",
                        "connector": "RJ45",
                        "maxSpeedMbps": 10000,
                        "speedMbps": 1000
                    }
                ],
                "radios": [
                    {
                        "wlanStandard": "802.11be",
                        "frequencyGHz": 2.4,
                        "channelWidthMHz": 20,
                        "channel": 1
                    },
                    {
                        "wlanStandard": "802.11be",
                        "frequencyGHz": 5,
                        "channelWidthMHz": 80,
                        "channel": 48
                    },
                    {
                        "wlanStandard": "802.11be",
                        "frequencyGHz": 6,
                        "channelWidthMHz": 160,
                        "channel": 37
                    }
                ]
            }
        });

        let device: DeviceDetails =
            serde_json::from_value(json_data).expect("Failed to deserialize real-world response");

        assert_eq!(device.id, "4dd8e91c-9997-3736-9480-3c8591c47452");
        assert_eq!(device.name, "Battin Lane UX7");
        assert_eq!(device.model, "Express 7");
        assert!(device.supported);
        assert_eq!(device.state, "ONLINE");
        assert!(device.is_online());
        assert_eq!(device.port_count(), 2);
        assert_eq!(device.radio_count(), 3);
        assert!(device.has_switching());
        assert!(device.has_access_point());

        // Verify ports
        assert_eq!(device.active_ports().len(), 2);
        assert_eq!(device.inactive_ports().len(), 0);

        let port1 = &device.interfaces.ports[0];
        assert_eq!(port1.idx, 1);
        assert_eq!(port1.max_speed_mbps, 2500);
        assert_eq!(port1.speed_mbps, Some(1000));

        let port2 = &device.interfaces.ports[1];
        assert_eq!(port2.idx, 2);
        assert_eq!(port2.max_speed_mbps, 10000);

        // Verify radios
        let radio1 = &device.interfaces.radios[0];
        assert_eq!(radio1.wlan_standard, WirelessStandard::Standard802_11be);
        assert_eq!(radio1.frequency_ghz, 2.4);
        assert_eq!(radio1.channel, Some(1));

        let radio2 = &device.interfaces.radios[1];
        assert_eq!(radio2.frequency_ghz, 5.0);
        assert_eq!(radio2.channel, Some(48));

        let radio3 = &device.interfaces.radios[2];
        assert_eq!(radio3.frequency_ghz, 6.0);
        assert_eq!(radio3.channel, Some(37));
    }

    #[test]
    fn test_device_details_without_optional_fields() {
        // Test with minimal required fields (no adoptedAt, no uplink)
        let json_data = json!({
            "id": "minimal-device",
            "macAddress": "1c:0b:8b:3e:5c:16",
            "ipAddress": "192.168.1.100",
            "name": "Minimal Device",
            "model": "Test",
            "supported": true,
            "state": "ONLINE",
            "firmwareUpdatable": false,
            "configurationId": "config123",
            "features": {},
            "interfaces": {
                "ports": [],
                "radios": []
            }
        });

        let device: DeviceDetails =
            serde_json::from_value(json_data).expect("Failed to deserialize minimal response");

        assert_eq!(device.id, "minimal-device");
        assert_eq!(device.name, "Minimal Device");
        assert!(device.is_online());
        assert!(device.adopted_at.is_none());
        assert!(device.uplink.is_none());
        assert_eq!(device.port_count(), 0);
        assert_eq!(device.radio_count(), 0);
    }

    #[test]
    fn test_port_connector_deserialization() {
        let connectors = vec![
            ("\"RJ45\"", PortConnector::Rj45),
            ("\"SFP\"", PortConnector::Sfp),
            ("\"SFPPLUS\"", PortConnector::SfpPlus),
            ("\"SFP28\"", PortConnector::Sfp28),
            ("\"QSFP28\"", PortConnector::Qsfp28),
        ];

        for (json_str, expected) in connectors {
            let connector: PortConnector = serde_json::from_str(json_str).unwrap();
            assert_eq!(connector, expected);
        }
    }

    #[test]
    fn test_interface_state_deserialization() {
        let up: InterfaceState = serde_json::from_str("\"UP\"").unwrap();
        let down: InterfaceState = serde_json::from_str("\"DOWN\"").unwrap();
        let unknown: InterfaceState = serde_json::from_str("\"UNKNOWN\"").unwrap();

        assert_eq!(up, InterfaceState::Up);
        assert_eq!(down, InterfaceState::Down);
        assert_eq!(unknown, InterfaceState::Unknown);
    }

    #[test]
    fn test_device_details_with_exact_api_response() {
        // Test with exact response from API docs
        let json_data = json!({
            "id": "497f6eca-6276-4993-bfeb-53cbbbba6f08",
            "macAddress": "94:2a:6f:26:c6:ca",
            "ipAddress": "192.168.1.55",
            "name": "IW HD",
            "model": "UHDIW",
            "supported": true,
            "state": "ONLINE",
            "firmwareVersion": "6.6.55",
            "firmwareUpdatable": true,
            "adoptedAt": "2019-08-24T14:15:22Z",
            "provisionedAt": "2019-08-24T14:15:22Z",
            "configurationId": "7596498d2f367dc2",
            "uplink": {
                "deviceId": "4de4adb9-21ee-47e3-aeb4-8cf8ed6c109a"
            },
            "features": {
                "switching": null,
                "accessPoint": null
            },
            "interfaces": {
                "ports": [
                    {
                        "idx": 1,
                        "state": "UP",
                        "connector": "RJ45",
                        "maxSpeedMbps": 10000,
                        "speedMbps": 1000,
                        "poe": {
                            "standard": "802.3bt",
                            "type": 3,
                            "enabled": true,
                            "state": "UP"
                        }
                    }
                ],
                "radios": [
                    {
                        "wlanStandard": "802.11a",
                        "frequencyGHz": 2.4,
                        "channelWidthMHz": 40,
                        "channel": 36
                    }
                ]
            }
        });

        let device: DeviceDetails =
            serde_json::from_value(json_data).expect("Failed to deserialize exact API response");

        // Verify all fields deserialized correctly
        assert_eq!(device.id, "497f6eca-6276-4993-bfeb-53cbbbba6f08");
        assert_eq!(device.mac_address.as_str(), "94:2a:6f:26:c6:ca");
        assert_eq!(device.ip_address.as_str(), "192.168.1.55");
        assert_eq!(device.name, "IW HD");
        assert_eq!(device.model, "UHDIW");
        assert!(device.supported);
        assert_eq!(device.state, "ONLINE");
        assert_eq!(device.firmware_version, Some("6.6.55".to_string()));
        assert!(device.firmware_updatable);
        assert!(device.is_online());

        // Verify ports and radios
        assert_eq!(device.port_count(), 1);
        assert_eq!(device.radio_count(), 1);

        let port = &device.interfaces.ports[0];
        assert_eq!(port.idx, 1);
        assert_eq!(port.state, InterfaceState::Up);
        assert_eq!(port.connector, PortConnector::Rj45);
        assert_eq!(port.max_speed_mbps, 10000);
        assert_eq!(port.speed_mbps, Some(1000));
        assert!(port.poe.is_some());
        let poe = port.poe.as_ref().unwrap();
        assert_eq!(poe.standard, Some("802.3bt".to_string()));
        assert_eq!(poe.r#type, Some(3));
        assert_eq!(poe.enabled, Some(true));

        let radio = &device.interfaces.radios[0];
        assert_eq!(radio.wlan_standard, WirelessStandard::Standard802_11a);
        assert_eq!(radio.frequency_ghz, 2.4);
        assert_eq!(radio.channel_width_mhz, 40);
        assert_eq!(radio.channel, Some(36));
    }

    #[test]
    fn test_poe_methods() {
        // Test PoE with enabled and UP state
        let poe_active = PoE {
            standard: Some("802.3bt".to_string()),
            r#type: Some(3),
            enabled: Some(true),
            state: Some("UP".to_string()),
        };
        assert!(poe_active.is_enabled());
        assert!(poe_active.is_active());

        // Test PoE with enabled but DOWN state
        let poe_enabled_down = PoE {
            standard: Some("802.3at".to_string()),
            r#type: Some(2),
            enabled: Some(true),
            state: Some("DOWN".to_string()),
        };
        assert!(poe_enabled_down.is_enabled());
        assert!(!poe_enabled_down.is_active());

        // Test PoE disabled
        let poe_disabled = PoE {
            standard: Some("802.3af".to_string()),
            r#type: Some(1),
            enabled: Some(false),
            state: None,
        };
        assert!(!poe_disabled.is_enabled());
        assert!(!poe_disabled.is_active());

        // Test PoE with None values
        let poe_none = PoE {
            standard: None,
            r#type: None,
            enabled: None,
            state: None,
        };
        assert!(!poe_none.is_enabled());
        assert!(!poe_none.is_active());
    }

    #[test]
    fn test_port_poe_methods() {
        // Port with active PoE
        let port_with_poe = Port {
            idx: 1,
            state: InterfaceState::Up,
            connector: PortConnector::Rj45,
            max_speed_mbps: 1000,
            speed_mbps: Some(1000),
            poe: Some(PoE {
                standard: Some("802.3bt".to_string()),
                r#type: Some(3),
                enabled: Some(true),
                state: Some("UP".to_string()),
            }),
        };
        assert!(port_with_poe.has_poe());
        assert!(port_with_poe.is_poe_enabled());
        assert!(port_with_poe.is_poe_active());

        // Port with disabled PoE
        let port_poe_disabled = Port {
            idx: 2,
            state: InterfaceState::Up,
            connector: PortConnector::Rj45,
            max_speed_mbps: 1000,
            speed_mbps: Some(1000),
            poe: Some(PoE {
                standard: Some("802.3at".to_string()),
                r#type: Some(2),
                enabled: Some(false),
                state: None,
            }),
        };
        assert!(port_poe_disabled.has_poe());
        assert!(!port_poe_disabled.is_poe_enabled());
        assert!(!port_poe_disabled.is_poe_active());

        // Port without PoE
        let port_no_poe = Port {
            idx: 3,
            state: InterfaceState::Up,
            connector: PortConnector::Sfp,
            max_speed_mbps: 10000,
            speed_mbps: Some(10000),
            poe: None,
        };
        assert!(!port_no_poe.has_poe());
        assert!(!port_no_poe.is_poe_enabled());
        assert!(!port_no_poe.is_poe_active());
    }

    #[test]
    fn test_port_state_methods() {
        let port_up = Port {
            idx: 1,
            state: InterfaceState::Up,
            connector: PortConnector::Rj45,
            max_speed_mbps: 1000,
            speed_mbps: Some(1000),
            poe: None,
        };
        assert!(port_up.is_up());
        assert!(!port_up.is_down());
        assert_eq!(port_up.speed_gbps(), Some(1.0));

        let port_down = Port {
            idx: 2,
            state: InterfaceState::Down,
            connector: PortConnector::Rj45,
            max_speed_mbps: 1000,
            speed_mbps: None,
            poe: None,
        };
        assert!(!port_down.is_up());
        assert!(port_down.is_down());
        assert_eq!(port_down.speed_gbps(), None);

        // Test 10G port speed
        let port_10g = Port {
            idx: 3,
            state: InterfaceState::Up,
            connector: PortConnector::SfpPlus,
            max_speed_mbps: 10000,
            speed_mbps: Some(10000),
            poe: None,
        };
        assert_eq!(port_10g.speed_gbps(), Some(10.0));
    }

    #[test]
    fn test_device_details_switch_methods() {
        // Create a switch device with multiple ports
        let json_data = json!({
            "id": "switch-device-id",
            "macAddress": "aa:bb:cc:dd:ee:ff",
            "ipAddress": "192.168.1.100",
            "name": "USW-24-PoE",
            "model": "USW-24-PoE",
            "supported": true,
            "state": "ONLINE",
            "firmwareUpdatable": false,
            "configurationId": "config123",
            "features": {
                "switching": {},
                "accessPoint": null
            },
            "interfaces": {
                "ports": [
                    {
                        "idx": 1,
                        "state": "UP",
                        "connector": "RJ45",
                        "maxSpeedMbps": 1000,
                        "speedMbps": 1000,
                        "poe": {
                            "standard": "802.3at",
                            "type": 2,
                            "enabled": true,
                            "state": "UP"
                        }
                    },
                    {
                        "idx": 2,
                        "state": "UP",
                        "connector": "RJ45",
                        "maxSpeedMbps": 1000,
                        "speedMbps": 1000,
                        "poe": {
                            "standard": "802.3at",
                            "type": 2,
                            "enabled": true,
                            "state": "DOWN"
                        }
                    },
                    {
                        "idx": 3,
                        "state": "DOWN",
                        "connector": "RJ45",
                        "maxSpeedMbps": 1000,
                        "poe": {
                            "standard": "802.3at",
                            "type": 2,
                            "enabled": false,
                            "state": null
                        }
                    },
                    {
                        "idx": 25,
                        "state": "UP",
                        "connector": "SFP",
                        "maxSpeedMbps": 1000,
                        "speedMbps": 1000
                    }
                ],
                "radios": []
            }
        });

        let device: DeviceDetails = serde_json::from_value(json_data).unwrap();

        // Test is_switch
        assert!(device.is_switch());
        assert!(device.has_switching());
        assert!(!device.has_access_point());

        // Test port counts
        assert_eq!(device.port_count(), 4);
        assert_eq!(device.active_ports().len(), 3);
        assert_eq!(device.inactive_ports().len(), 1);

        // Test PoE port methods
        assert_eq!(device.poe_port_count(), 3); // 3 ports have PoE capability
        assert_eq!(device.poe_ports().len(), 3);
        assert_eq!(device.poe_enabled_ports().len(), 2); // 2 ports have PoE enabled
        assert_eq!(device.poe_active_ports().len(), 1); // Only 1 port is actively delivering power

        // Verify the active PoE port is port 1
        let active_poe = device.poe_active_ports();
        assert_eq!(active_poe[0].idx, 1);
    }

    #[test]
    fn test_device_details_access_point_not_switch() {
        let json_data = json!({
            "id": "ap-device-id",
            "macAddress": "aa:bb:cc:dd:ee:ff",
            "ipAddress": "192.168.1.101",
            "name": "U6-Pro",
            "model": "U6-Pro",
            "supported": true,
            "state": "ONLINE",
            "firmwareUpdatable": false,
            "configurationId": "config123",
            "features": {
                "switching": null,
                "accessPoint": {}
            },
            "interfaces": {
                "ports": [
                    {
                        "idx": 1,
                        "state": "UP",
                        "connector": "RJ45",
                        "maxSpeedMbps": 2500,
                        "speedMbps": 1000
                    }
                ],
                "radios": [
                    {
                        "wlanStandard": "802.11ax",
                        "frequencyGHz": 5,
                        "channelWidthMHz": 80,
                        "channel": 36
                    }
                ]
            }
        });

        let device: DeviceDetails = serde_json::from_value(json_data).unwrap();

        assert!(!device.is_switch());
        assert!(!device.has_switching());
        assert!(device.has_access_point());
        assert_eq!(device.poe_port_count(), 0);
        assert_eq!(device.radio_count(), 1);
    }
}

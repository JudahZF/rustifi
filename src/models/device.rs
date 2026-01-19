use crate::models::common::{IpAddress, MacAddress};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Default)]
pub enum DeviceType {
    #[serde(alias = "uap")]
    AccessPoint,
    #[serde(alias = "usw")]
    Switch,
    #[serde(alias = "udm")]
    DreamMachine,
    #[serde(alias = "ugw")]
    Gateway,
    #[serde(other)]
    #[default]
    Unknown,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Device {
    pub id: String,
    pub mac: MacAddress,
    pub model: String,
    #[serde(default, rename = "type")]
    pub type_field: DeviceType,
    pub name: String,
    #[serde(default)]
    pub ip: Option<IpAddress>,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub serial: Option<String>,
    #[serde(default)]
    pub adopted: Option<bool>,
    #[serde(default)]
    pub disabled: Option<bool>,
    #[serde(default)]
    pub disconnected: Option<bool>,
    #[serde(default)]
    pub last_seen: Option<i64>,
    #[serde(default)]
    pub uptime: Option<i64>,
    #[serde(default)]
    pub sys_stats: Option<SystemStats>,
    #[serde(default)]
    pub config_network: Option<ConfigNetwork>,
    #[serde(default)]
    pub uplink: Option<Uplink>,
    #[serde(default)]
    pub temperature: Option<f64>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Default)]
pub struct SystemStats {
    #[serde(default)]
    pub cpu_load: f64,
    #[serde(default)]
    pub mem_used: i64,
    #[serde(default)]
    pub mem_total: i64,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct ConfigNetwork {
    #[serde(default)]
    pub ip: Option<IpAddress>,
    #[serde(default)]
    pub netmask: Option<IpAddress>,
    #[serde(default)]
    pub gateway: Option<IpAddress>,
    #[serde(default)]
    pub dns1: Option<IpAddress>,
    #[serde(default)]
    pub dns2: Option<IpAddress>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Uplink {
    #[serde(default)]
    pub mac: Option<MacAddress>,
    #[serde(default)]
    pub ip: Option<IpAddress>,
    #[serde(default)]
    pub speed: Option<i64>,
    #[serde(default)]
    pub full_duplex: Option<bool>,
    #[serde(default)]
    pub type_: Option<String>,
    #[serde(default)]
    pub state: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_stats_missing_all_fields() {
        let json = "{}";
        let stats: SystemStats = serde_json::from_str(json).unwrap();
        assert_eq!(stats.cpu_load, 0.0);
        assert_eq!(stats.mem_used, 0);
        assert_eq!(stats.mem_total, 0);
    }

    #[test]
    fn test_system_stats_missing_some_fields() {
        let json = r#"{"cpu_load": 45.5}"#;
        let stats: SystemStats = serde_json::from_str(json).unwrap();
        assert_eq!(stats.cpu_load, 45.5);
        assert_eq!(stats.mem_used, 0);
        assert_eq!(stats.mem_total, 0);
    }

    #[test]
    fn test_system_stats_all_fields_present() {
        let json = r#"{"cpu_load": 25.0, "mem_used": 1024, "mem_total": 4096}"#;
        let stats: SystemStats = serde_json::from_str(json).unwrap();
        assert_eq!(stats.cpu_load, 25.0);
        assert_eq!(stats.mem_used, 1024);
        assert_eq!(stats.mem_total, 4096);
    }
}

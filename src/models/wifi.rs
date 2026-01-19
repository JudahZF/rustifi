use serde::{Deserialize, Serialize};

/// WiFi security type.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WifiSecurity {
    Open,
    Wpa2,
    Wpa3,
    WpaEnterprise,
}

/// WiFi broadcast (SSID) configuration.
#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WifiBroadcast {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub ssid: Option<String>,
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub security: Option<WifiSecurity>,
    #[serde(default)]
    pub vlan_id: Option<i64>,
    #[serde(default)]
    pub hide_ssid: Option<bool>,
}

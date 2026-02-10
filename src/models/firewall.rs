use serde::{Deserialize, Serialize};

/// Firewall policy action type.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FirewallActionType {
    Allow,
    Block,
    Drop,
    Reject,
    #[default]
    #[serde(other)]
    Unknown,
}

/// Firewall policy action wrapper (API returns action as an object).
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct FirewallAction {
    #[serde(rename = "type", default)]
    pub action_type: FirewallActionType,
}

/// Firewall zone.
#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FirewallZone {
    pub id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub networks: Vec<String>,
}

/// Firewall policy rule.
#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FirewallPolicy {
    pub id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub action: FirewallAction,
    #[serde(default)]
    pub source_zone_id: Option<String>,
    #[serde(default)]
    pub destination_zone_id: Option<String>,
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub order: Option<i32>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub protocol: Option<String>,
    #[serde(default)]
    pub source_addresses: Vec<String>,
    #[serde(default)]
    pub destination_addresses: Vec<String>,
    #[serde(default)]
    pub source_ports: Vec<String>,
    #[serde(default)]
    pub destination_ports: Vec<String>,
}

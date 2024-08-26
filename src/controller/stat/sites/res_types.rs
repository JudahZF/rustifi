use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SiteListResponse {
    pub meta: Meta,
    pub data: Vec<Daum>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Meta {
    pub rc: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Daum {
    pub anonymous_id: String,
    pub name: String,
    #[serde(rename = "_id")]
    pub id: String,
    pub attr_no_delete: bool,
    pub attr_hidden_id: String,
    pub desc: String,
    pub health: Vec<Health>,
    pub num_new_alarms: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Health {
    pub subsystem: String,
    pub num_user: Option<i64>,
    pub num_guest: Option<i64>,
    pub num_iot: Option<i64>,
    #[serde(rename = "tx_bytes-r")]
    pub tx_bytes_r: Option<i64>,
    #[serde(rename = "rx_bytes-r")]
    pub rx_bytes_r: Option<i64>,
    pub status: String,
    pub num_ap: Option<i64>,
    pub num_adopted: Option<i64>,
    pub num_disabled: Option<i64>,
    pub num_disconnected: Option<i64>,
    pub num_pending: Option<i64>,
    pub num_gw: Option<i64>,
    pub wan_ip: Option<String>,
    pub lan_ip: Option<String>,
    #[serde(default)]
    pub gateways: Vec<String>,
    pub netmask: Option<String>,
    #[serde(default)]
    pub nameservers: Vec<Value>,
    pub num_sta: Option<i64>,
    pub gw_mac: Option<String>,
    pub gw_name: Option<String>,
    #[serde(rename = "gw_system-stats")]
    pub gw_system_stats: Option<GwSystemStats>,
    pub gw_version: Option<String>,
    pub isp_name: Option<String>,
    pub isp_organization: Option<String>,
    pub uptime_stats: Option<UptimeStats>,
    pub latency: Option<i64>,
    pub uptime: Option<i64>,
    pub drops: Option<i64>,
    pub xput_up: Option<f64>,
    pub xput_down: Option<f64>,
    pub speedtest_status: Option<String>,
    pub speedtest_lastrun: Option<i64>,
    pub speedtest_ping: Option<i64>,
    pub num_sw: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GwSystemStats {
    pub cpu: String,
    pub mem: String,
    pub uptime: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UptimeStats {
    #[serde(rename = "WAN")]
    pub wan: Wan,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Wan {
    pub alerting_monitors: Vec<AlertingMonitor>,
    pub availability: f64,
    pub latency_average: i64,
    pub monitors: Vec<Monitor>,
    pub time_period: i64,
    pub uptime: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AlertingMonitor {
    pub availability: f64,
    pub latency_average: i64,
    pub target: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Monitor {
    pub availability: f64,
    pub latency_average: i64,
    pub target: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

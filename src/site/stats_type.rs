use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeviceStats {
    pub port_table: Vec<PortTable>,
    pub last_wan_ip: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub setup_provision_completed: bool,
    pub ethernet_overrides: Vec<EthernetOverride>,
    pub reboot_duration: i64,
    pub hostname: String,
    pub config_network: ConfigNetwork,
    pub model: String,
    pub bandsteering_mode: String,
    pub ip: String,
    pub support_wifi6e: bool,
    pub jumboframe_enabled: bool,
    pub last_connection_network_name: String,
    pub config_network_lan: ConfigNetworkLan,
    pub version: String,
    pub adoption_completed: bool,
    pub scan_radio_table: Vec<Value>,
    pub shortname: String,
    pub stp_version: String,
    pub antenna_table: Vec<AntennaTable>,
    pub wifi_caps: i64,
    pub site_id: String,
    pub adopted_at: i64,
    pub name: String,
    pub fw_caps: i64,
    #[serde(rename = "_id")]
    pub id: String,
    pub wlangroup_id_ng: String,
    pub atf_enabled: bool,
    pub radio_table: Vec<RadioTable>,
    pub stp_priority: String,
    pub connected_at: i64,
    pub inform_ip: String,
    pub mac: String,
    pub ruleset_interfaces: RulesetInterfaces,
    pub provisioned_at: i64,
    pub upgrade_duration: i64,
    pub ethernet_table: Vec<EthernetTable>,
    pub flowctrl_enabled: bool,
    pub unsupported: bool,
    pub ipv6: Vec<String>,
    pub disconnected_at: i64,
    pub architecture: String,
    pub has_fan: bool,
    pub has_eth1: bool,
    pub has_temperature: bool,
    pub adopted: bool,
    pub device_id: String,
    pub state: i64,
    pub start_disconnected_millis: i64,
    pub last_seen: i64,
    pub uptime: i64,
    pub next_interval: i64,
    pub min_inform_interval_seconds: i64,
    pub upgradable: bool,
    pub last_config_applied_successfully: bool,
    pub locating: bool,
    pub start_connected_millis: i64,
    pub sys_stats: SysStats,
    #[serde(rename = "system-stats")]
    pub system_stats: SystemStats,
    pub connection_network_name: String,
    pub startup_timestamp: i64,
    pub guest_kicks: i64,
    pub temperatures: Vec<Temperature>,
    pub satisfaction: i64,
    pub overheating: bool,
    pub isolated: bool,
    pub radio_table_stats: Vec<RadioTableStat>,
    pub reported_networks: Vec<ReportedNetwork>,
    pub wan1: Wan1,
    pub uplink: Uplink,
    pub uplink_depth: i64,
    pub vap_table: Vec<VapTable>,
    pub downlink_table: Vec<Value>,
    pub stat: Stat,
    pub num_sta: i64,
    #[serde(rename = "wlan-num_sta")]
    pub wlan_num_sta: i64,
    #[serde(rename = "lan-num_sta")]
    pub lan_num_sta: i64,
    #[serde(rename = "user-num_sta")]
    pub user_num_sta: i64,
    #[serde(rename = "guest-num_sta")]
    pub guest_num_sta: i64,
    pub lan_ip: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PortTable {
    pub port_idx: i64,
    pub media: String,
    pub port_poe: bool,
    pub op_mode: String,
    pub forward: String,
    pub autoneg: bool,
    pub enable: bool,
    pub full_duplex: bool,
    pub is_uplink: bool,
    pub mac: String,
    pub name: String,
    pub num_port: i64,
    pub poe_enable: bool,
    pub poe_power: String,
    pub rx: Rx,
    pub speed: i64,
    pub tx: Tx,
    #[serde(rename = "type")]
    pub type_field: String,
    pub up: bool,
    pub ifname: String,
    pub ip: String,
    pub netmask: String,
    pub network_name: String,
    pub masked: bool,
    pub aggregated_by: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rx {
    pub broadcast: i64,
    pub bytes: i64,
    pub dropped: i64,
    pub errors: i64,
    pub multicast: i64,
    pub packets: i64,
    pub rate: i64,
    #[serde(rename = "rate-max")]
    pub rate_max: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tx {
    pub broadcast: i64,
    pub bytes: i64,
    pub dropped: i64,
    pub errors: i64,
    pub multicast: i64,
    pub packets: i64,
    pub rate: i64,
    #[serde(rename = "rate-max")]
    pub rate_max: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EthernetOverride {
    pub ifname: String,
    pub networkgroup: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfigNetwork {
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfigNetworkLan {
    pub dhcp_enabled: bool,
    pub dhcp_range_stop: String,
    pub vlan: i64,
    pub dhcp_range_start: String,
    pub cidr: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AntennaTable {
    pub wifi_gain: Vec<i64>,
    pub name: String,
    pub id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RadioTable {
    pub antenna_gain: i64,
    pub builtin_antenna: bool,
    pub hard_noise_floor_enabled: bool,
    pub tx_power: i64,
    pub sens_level_enabled: bool,
    pub channel: i64,
    pub max_txpower: i64,
    pub min_rssi_enabled: bool,
    pub is_11ac: bool,
    pub builtin_ant_gain: i64,
    pub ht: i64,
    pub radio: String,
    pub nss: i64,
    pub tx_power_mode: String,
    pub is_11ax: bool,
    pub min_rssi: i64,
    pub name: String,
    pub min_txpower: i64,
    pub antenna_id: i64,
    pub current_antenna_gain: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RulesetInterfaces {
    pub br0: String,
    pub eth1: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EthernetTable {
    pub port_idx: i64,
    pub num_port: i64,
    pub name: String,
    pub mac: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SysStats {
    pub loadavg_1: String,
    pub loadavg_15: String,
    pub loadavg_5: String,
    pub mem_buffer: i64,
    pub mem_total: i64,
    pub mem_used: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SystemStats {
    pub cpu: String,
    pub mem: String,
    pub uptime: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Temperature {
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub value: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RadioTableStat {
    pub name: String,
    pub last_channel: i64,
    pub radio: String,
    pub ast_txto: Value,
    pub ast_cst: Value,
    pub ast_be_xmit: Value,
    pub cu_total: i64,
    pub cu_self_rx: i64,
    pub cu_self_tx: i64,
    pub gain: i64,
    pub satisfaction: i64,
    pub state: String,
    pub channel: i64,
    pub extchannel: i64,
    pub tx_power: i64,
    pub tx_packets: i64,
    pub tx_retries: i64,
    pub num_sta: i64,
    #[serde(rename = "guest-num_sta")]
    pub guest_num_sta: i64,
    #[serde(rename = "user-num_sta")]
    pub user_num_sta: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReportedNetwork {
    pub name: String,
    pub address: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Wan1 {
    pub latency: i64,
    pub availability: i64,
    pub ipv6: Vec<String>,
    pub uplink_ifname: String,
    pub max_speed: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub autoneg: bool,
    pub enable: bool,
    pub error_disabled: bool,
    pub full_duplex: bool,
    pub is_uplink: bool,
    pub mac: String,
    pub media: String,
    pub name: String,
    pub num_port: i64,
    pub poe_enable: bool,
    pub poe_power: String,
    pub port_idx: i64,
    pub port_poe: bool,
    pub rx: Rx2,
    pub speed: i64,
    pub speed_caps: i64,
    pub tx: Tx2,
    pub up: bool,
    pub ifname: String,
    pub ip: String,
    pub netmask: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rx2 {
    pub broadcast: i64,
    pub bytes: i64,
    pub dropped: i64,
    pub errors: i64,
    pub multicast: i64,
    pub packets: i64,
    pub rate: i64,
    #[serde(rename = "rate-max")]
    pub rate_max: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tx2 {
    pub broadcast: i64,
    pub bytes: i64,
    pub dropped: i64,
    pub errors: i64,
    pub multicast: i64,
    pub packets: i64,
    pub rate: i64,
    #[serde(rename = "rate-max")]
    pub rate_max: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Uplink {
    pub comment: String,
    pub drops: i64,
    pub gateway_present: Vec<String>,
    pub gateways: Vec<String>,
    pub ip: String,
    pub latency: i64,
    pub name: String,
    pub nameservers_dynamic: Vec<String>,
    pub netmask: String,
    pub num_port: i64,
    pub physical_ports: Vec<i64>,
    pub rx: Rx3,
    pub tx: Tx3,
    pub up: bool,
    pub uptime: i64,
    pub xput_down: f64,
    pub xput_up: f64,
    pub port_idx: i64,
    pub media: String,
    pub speed: i64,
    pub full_duplex: bool,
    pub max_speed: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub uplink_source: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rx3 {
    pub bytes: i64,
    pub dropped: i64,
    pub errors: i64,
    pub multicast: i64,
    pub packets: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tx3 {
    pub bytes: i64,
    pub dropped: i64,
    pub errors: i64,
    pub packets: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VapTable {
    pub avg_client_signal: i64,
    pub bssid: String,
    pub bw: i64,
    pub ccq: i64,
    pub channel: i64,
    pub essid: String,
    pub id: String,
    pub mac_filter_rejections: i64,
    pub name: String,
    pub num_sta: i64,
    pub radio: String,
    pub radio_name: String,
    pub rx: Rx4,
    pub satisfaction: i64,
    pub satisfaction_real: i64,
    pub state: String,
    pub tx: Tx4,
    pub up: bool,
    pub usage: String,
    pub wifi_tx_attempts: i64,
    pub wifi_tx_dropped: i64,
    pub t: String,
    pub wlanconf_id: String,
    pub is_guest: bool,
    pub is_wep: bool,
    pub ap_mac: String,
    pub map_id: Value,
    pub site_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rx4 {
    pub bytes: i64,
    pub crypts: i64,
    pub dropped: i64,
    pub errors: i64,
    pub frags: i64,
    pub nwids: i64,
    pub packets: i64,
    pub tcp_stats: TcpStats,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TcpStats {
    pub goodbytes: i64,
    pub lat_avg: i64,
    pub lat_max: i64,
    pub lat_min: i64,
    pub lat_samples: i64,
    pub lat_sum: i64,
    pub retries: i64,
    pub stalls: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tx4 {
    pub bytes: i64,
    pub combined_retries: i64,
    pub data_mpdu_bytes: i64,
    pub dropped: i64,
    pub errors: i64,
    pub packets: i64,
    pub power: i64,
    pub retries: i64,
    pub rts_retries: i64,
    pub success: i64,
    pub tcp_stats: TcpStats2,
    pub total: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TcpStats2 {
    pub goodbytes: i64,
    pub lat_avg: i64,
    pub lat_max: i64,
    pub lat_min: i64,
    pub lat_samples: i64,
    pub lat_sum: i64,
    pub retries: i64,
    pub stalls: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stat {
    pub ap: Ap,
    pub sw: Sw,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ap {
    pub site_id: String,
    pub o: String,
    pub oid: String,
    pub ap: String,
    pub time: i64,
    pub datetime: String,
    pub user: User,
    pub guest: Guest,
    pub rx: Rx7,
    pub tx: Tx7,
    pub wifi: Vec<Wifi>,
    pub client: Client,
    pub mac_filter_rejections: i64,
    pub signal_duration: SignalDuration,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub rx: Rx5,
    pub tx: Tx5,
    pub mac_filter_rejections: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rx5 {
    pub packets: f64,
    pub bytes: f64,
    pub errors: i64,
    pub dropped: i64,
    pub crypts: i64,
    pub frags: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tx5 {
    pub packets: f64,
    pub bytes: f64,
    pub errors: i64,
    pub dropped: i64,
    pub retries: f64,
    pub wifi_attempts: f64,
    pub wifi_dropped: i64,
    pub wifi_errors: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Guest {
    pub rx: Rx6,
    pub tx: Tx6,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rx6 {
    pub packets: f64,
    pub bytes: f64,
    pub errors: i64,
    pub dropped: i64,
    pub crypts: i64,
    pub frags: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tx6 {
    pub packets: f64,
    pub bytes: f64,
    pub errors: i64,
    pub dropped: i64,
    pub retries: f64,
    pub wifi_attempts: f64,
    pub wifi_dropped: i64,
    pub wifi_errors: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rx7 {
    pub packets: f64,
    pub bytes: f64,
    pub errors: i64,
    pub dropped: i64,
    pub crypts: i64,
    pub frags: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tx7 {
    pub packets: f64,
    pub bytes: f64,
    pub errors: i64,
    pub dropped: i64,
    pub retries: f64,
    pub wifi_attempts: f64,
    pub wifi_dropped: i64,
    pub wifi_errors: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Wifi {
    pub wifi: String,
    pub rx: Rx8,
    pub tx: Tx8,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rx8 {
    pub packets: f64,
    pub bytes: f64,
    pub errors: i64,
    pub dropped: i64,
    pub crypts: i64,
    pub frags: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tx8 {
    pub packets: f64,
    pub bytes: f64,
    pub errors: i64,
    pub dropped: i64,
    pub retries: f64,
    pub wifi_attempts: f64,
    pub wifi_dropped: i64,
    pub wifi_errors: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Client {
    pub tx_bytes: f64,
    pub rx_bytes: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SignalDuration {
    #[serde(rename = "60")]
    pub n60: f64,
    #[serde(rename = "70")]
    pub n70: f64,
    #[serde(rename = "80")]
    pub n80: f64,
    #[serde(rename = "90")]
    pub n90: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sw {
    pub site_id: String,
    pub o: String,
    pub oid: String,
    pub sw: String,
    pub time: i64,
    pub datetime: String,
    pub rx: Rx9,
    pub tx: Tx9,
    pub duration: f64,
    pub port: Vec<Port>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rx9 {
    pub packets: f64,
    pub bytes: f64,
    pub errors: f64,
    pub dropped: f64,
    pub crypts: f64,
    pub frags: f64,
    pub multicast: f64,
    pub broadcast: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tx9 {
    pub packets: f64,
    pub bytes: f64,
    pub errors: f64,
    pub dropped: f64,
    pub retries: f64,
    pub multicast: f64,
    pub broadcast: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Port {
    pub port: i64,
    pub rx: Rx10,
    pub tx: Tx10,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rx10 {
    pub packets: f64,
    pub bytes: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tx10 {
    pub packets: f64,
    pub bytes: f64,
}

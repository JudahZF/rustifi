use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeviceListResponse {
    pub meta: Meta,
    pub data: Vec<RawDevice>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Meta {
    pub rc: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RawDevice {
    pub required_version: String,
    pub port_table: Vec<PortTable>,
    #[serde(rename = "heightInMeters")]
    pub height_in_meters: Option<f64>,
    pub license_state: String,
    pub lcm_brightness_override: Option<bool>,
    #[serde(rename = "type")]
    pub type_field: String,
    pub board_rev: i64,
    pub setup_id: Option<String>,
    pub hw_caps: i64,
    pub reboot_duration: Option<u32>,
    pub snmp_contact: Option<String>,
    pub config_network: ConfigNetwork,
    pub syslog_key: String,
    pub model: String,
    pub outdoor_mode_override: Option<String>,
    pub slimcfg_caps: Option<i64>,
    pub lcm_tracker_enabled: Option<bool>,
    pub sysid: i64,
    pub ip: String,
    pub fw2_caps: Option<i64>,
    pub jumboframe_enabled: Option<bool>,
    pub last_connection_network_name: Option<String>,
    pub led_override_color: Option<String>,
    pub version: String,
    pub unsupported_reason: i64,
    pub adoption_completed: bool,
    pub anon_id: Option<String>,
    pub stp_version: Option<String>,
    pub last_connection_network_id: Option<String>,
    pub site_id: String,
    pub name: String,
    pub fw_caps: i64,
    #[serde(rename = "_id")]
    pub id: String,
    pub internet: Option<bool>,
    pub mgmt_network_id: Option<String>,
    pub gateway_mac: Option<String>,
    pub external_id: String,
    pub connected_at: i64,
    pub two_phase_adopt: bool,
    #[serde(default)]
    pub port_overrides: Vec<PortOverride>,
    pub inform_ip: String,
    pub cfgversion: String,
    pub mac: String,
    pub provisioned_at: i64,
    pub inform_url: String,
    pub ethernet_table: Vec<EthernetTable>,
    pub upgrade_duration: Option<i64>,
    pub flowctrl_enabled: Option<bool>,
    pub unsupported: bool,
    pub sys_error_caps: i64,
    pub ble_caps: i64,
    pub dot1x_portctrl_enabled: bool,
    pub map_id: Option<String>,
    pub last_uplink: LastUplink,
    pub led_override: Option<String>,
    pub ether_lighting: Option<EtherLighting>,
    pub disconnected_at: i64,
    pub architecture: Option<String>,
    pub x_aes_gcm: bool,
    pub has_fan: bool,
    pub lcm_idle_timeout_override: Option<bool>,
    pub model_incompatible: bool,
    pub x_authkey: String,
    pub satisfaction: Option<i64>,
    pub x_ssh_hostkey_fingerprint: Option<String>,
    pub model_in_eol: bool,
    pub anomalies: Option<i64>,
    pub has_temperature: bool,
    pub switch_caps: SwitchCaps,
    pub adopted_by_client: Option<String>,
    pub snmp_location: Option<String>,
    pub model_in_lts: bool,
    pub kernel_version: Option<String>,
    pub serial: Option<String>,
    pub power_source_ctrl_enabled: Option<bool>,
    pub led_override_color_brightness: Option<i64>,
    pub adopted: bool,
    pub hash_id: Option<String>,
    pub device_id: String,
    pub uplink: Uplink,
    pub state: i64,
    pub start_disconnected_millis: i64,
    pub upgrade_state: Option<i64>,
    pub num_sta: u64,
    #[serde(rename = "user-num_sta")]
    pub user_num_sta: u64,
    #[serde(rename = "guest-num_sta")]
    pub guest_num_sta: u64,
    pub tx_bytes: Option<i64>,
    pub rx_bytes: Option<i64>,
    pub bytes: Option<i64>,
    pub x_has_ssh_hostkey: bool,
    pub has_speaker: Option<bool>,
    pub mesh_sta_vap_enabled: Option<bool>,
    pub multi_active_antennas: Option<bool>,
    pub bandsteering_mode: Option<String>,
    pub support_wifi6e: Option<bool>,
    #[serde(default)]
    pub scan_radio_table: Vec<ScanRadioTable>,
    pub x_vwirekey: Option<String>,
    pub supports_fingerprint_ml: Option<bool>,
    pub country_code: Option<i64>,
    pub wlangroup_id_na: Option<String>,
    #[serde(default)]
    pub antenna_table: Vec<AntennaTable>,
    pub wifi_caps: Option<i64>,
    pub wlangroup_id_ng: Option<String>,
    pub atf_enabled: Option<bool>,
    #[serde(default)]
    pub radio_table: Vec<RadioTable>,
    pub wifi_caps2: Option<i64>,
    pub has_eth1: Option<bool>,
    pub fixed_ap_available: Option<bool>,
    #[serde(rename = "vwireEnabled")]
    pub vwire_enabled: Option<bool>,
    #[serde(default)]
    pub radio_table_stats: Vec<RadioTableStat>,
    #[serde(rename = "user-wlan-num_sta")]
    pub user_wlan_num_sta: Option<u64>,
    #[serde(rename = "guest-wlan-num_sta")]
    pub guest_wlan_num_sta: Option<u64>,
    pub adopted_at: Option<i64>,
    pub lcm_brightness: Option<i64>,
    pub lcm_night_mode_begins: Option<String>,
    pub lcm_night_mode_ends: Option<String>,
    pub x_fingerprint: Option<String>,
    pub locked: Option<bool>,
    pub shortname: Option<String>,
    pub service_mac: Option<String>,
    #[serde(default)]
    pub ipv6: Vec<String>,
    pub last_seen: Option<i64>,
    pub min_inform_interval_seconds: Option<i64>,
    pub upgradable: Option<bool>,
    pub adoptable_when_upgraded: Option<bool>,
    pub rollupgrade: Option<bool>,
    pub start_connected_millis: Option<i64>,
    pub next_interval: Option<u16>,
    pub known_cfgversion: Option<String>,
    pub uptime: Option<i64>,
    #[serde(rename = "_uptime")]
    pub uptime2: Option<i64>,
    pub locating: Option<bool>,
    pub sys_stats: Option<SysStats>,
    #[serde(rename = "system-stats")]
    pub system_stats: Option<SystemStats>,
    #[serde(default)]
    pub lldp_table: Vec<LldpTable>,
    pub displayable_version: Option<String>,
    pub connection_network_id: Option<String>,
    pub connection_network_name: Option<String>,
    pub startup_timestamp: Option<i64>,
    pub is_access_point: Option<bool>,
    pub safe_for_autoupgrade: Option<bool>,
    pub fan_level: Option<i64>,
    pub general_temperature: Option<i64>,
    pub overheating: Option<bool>,
    pub total_max_power: Option<i64>,
    #[serde(default)]
    pub downlink_table: Vec<DownlinkTable>,
    pub uplink_depth: Option<i64>,
    pub total_used_power: Option<f64>,
    pub detailed_states: Option<DetailedStates>,
    pub stat: Option<Stat>,
    pub lcm_orientation_override: Option<i64>,
    pub power_source_ctrl: Option<String>,
    pub credential_caps: Option<i64>,
    pub default: Option<bool>,
    pub discovered_via: Option<String>,
    pub adopt_ip: Option<String>,
    pub adopt_url: Option<String>,
    pub prev_non_busy_state: Option<i64>,
    pub connect_request_ip: Option<String>,
    pub connect_request_port: Option<String>,
    pub lcm_tracker_seed: Option<String>,
    pub lcm_night_mode_enabled: Option<bool>,
    pub radiusprofile_id: Option<String>,
    pub rps: Option<Rps>,
    pub mesh_uplink_1: Option<String>,
    pub mesh_uplink_2: Option<String>,
    pub scanning: Option<bool>,
    pub spectrum_scanning: Option<bool>,
    pub meshv3_peer_mac: Option<String>,
    pub element_peer_mac: Option<String>,
    pub guest_kicks: Option<i64>,
    pub guest_token: Option<String>,
    pub hide_ch_width: Option<String>,
    pub isolated: Option<bool>,
    pub vap_table: Option<Vec<VapTable>>,
    #[serde(rename = "bytes-d")]
    pub bytes_d: Option<i64>,
    #[serde(rename = "tx_bytes-d")]
    pub tx_bytes_d: Option<i64>,
    #[serde(rename = "rx_bytes-d")]
    pub rx_bytes_d: Option<i64>,
    #[serde(rename = "bytes-r")]
    pub bytes_r: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PortTable {
    pub port_idx: i64,
    pub media: Option<String>,
    pub port_poe: Option<bool>,
    pub poe_caps: Option<i64>,
    pub speed_caps: i64,
    pub op_mode: String,
    pub forward: String,
    pub name: String,
    pub enable: bool,
    pub masked: bool,
    pub aggregated_by: bool,
    pub poe_mode: Option<String>,
    pub setting_preference: Option<String>,
    pub port_security_enabled: Option<bool>,
    pub egress_rate_limit_kbps_enabled: Option<bool>,
    pub isolation: Option<bool>,
    pub tagged_vlan_mgmt: Option<String>,
    pub voice_networkconf_id: Option<String>,
    pub stp_port_mode: Option<bool>,
    #[serde(default)]
    pub excluded_networkconf_ids: Vec<String>,
    pub port_keepalive_enabled: Option<bool>,
    pub native_networkconf_id: Option<String>,
    pub autoneg: Option<bool>,
    pub lldpmed_enabled: Option<bool>,
    pub portconf_id: Option<String>,
    pub speed: Option<i64>,
    pub full_duplex: Option<bool>,
    pub anomalies: Option<i64>,
    pub dot1x_mode: Option<String>,
    pub dot1x_status: Option<String>,
    pub flowctrl_rx: Option<bool>,
    pub flowctrl_tx: Option<bool>,
    pub is_uplink: Option<bool>,
    pub jumbo: Option<bool>,
    pub mac_table_count: Option<i64>,
    pub poe_class: Option<String>,
    pub poe_current: Option<String>,
    pub poe_enable: Option<bool>,
    pub poe_good: Option<bool>,
    pub poe_power: Option<String>,
    pub poe_voltage: Option<String>,
    pub rx_broadcast: Option<i64>,
    pub rx_bytes: Option<i64>,
    pub rx_dropped: Option<i64>,
    pub rx_errors: Option<i64>,
    pub rx_multicast: Option<i64>,
    pub rx_packets: Option<i64>,
    pub satisfaction: Option<i64>,
    pub satisfaction_reason: Option<i64>,
    pub stp_pathcost: Option<i64>,
    pub stp_state: Option<String>,
    pub tx_broadcast: Option<i64>,
    pub tx_bytes: Option<i64>,
    pub tx_dropped: Option<i64>,
    pub tx_errors: Option<i64>,
    pub tx_multicast: Option<i64>,
    pub tx_packets: Option<i64>,
    pub up: Option<bool>,
    #[serde(rename = "tx_bytes-r")]
    pub tx_bytes_r: Option<f64>,
    #[serde(rename = "rx_bytes-r")]
    pub rx_bytes_r: Option<f64>,
    #[serde(rename = "bytes-r")]
    pub bytes_r: Option<f64>,
    pub sfp_found: Option<bool>,
    pub sfp_compliance: Option<String>,
    pub sfp_part: Option<String>,
    pub sfp_rev: Option<String>,
    pub sfp_rxfault: Option<bool>,
    pub sfp_serial: Option<String>,
    pub sfp_txfault: Option<bool>,
    pub sfp_vendor: Option<String>,
    pub sfp_current: Option<String>,
    pub sfp_rxpower: Option<String>,
    pub sfp_temperature: Option<String>,
    pub sfp_txpower: Option<String>,
    pub sfp_voltage: Option<String>,
    pub lldpmed_notify_enabled: Option<bool>,
    pub site_id: Option<String>,
    pub dot1x_ctrl: Option<String>,
    pub fec: Option<String>,
    pub attr_no_edit: Option<bool>,
    pub port_delta: Option<PortDelta>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PortDelta {
    pub time_delta: f64,
    pub time_delta_activity: f64,
    pub tx_bytes: Option<i64>,
    pub rx_bytes: Option<i64>,
    pub tx_packets: Option<i64>,
    pub rx_packets: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfigNetwork {
    #[serde(rename = "type")]
    pub type_field: String,
    pub bonding_enabled: Option<bool>,
    pub netmask: Option<String>,
    pub dns2: Option<String>,
    pub ip: Option<String>,
    pub dns1: Option<String>,
    pub dnssuffix: Option<String>,
    pub gateway: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PortOverride {
    pub setting_preference: Option<String>,
    pub op_mode: Option<String>,
    pub port_security_enabled: Option<bool>,
    pub poe_mode: Option<String>,
    pub forward: Option<String>,
    pub egress_rate_limit_kbps_enabled: Option<bool>,
    pub isolation: Option<bool>,
    pub tagged_vlan_mgmt: Option<String>,
    pub port_idx: i64,
    pub voice_networkconf_id: Option<String>,
    pub stp_port_mode: Option<bool>,
    pub port_keepalive_enabled: Option<bool>,
    #[serde(default)]
    pub excluded_networkconf_ids: Vec<String>,
    pub full_duplex: Option<bool>,
    pub native_networkconf_id: Option<String>,
    pub name: Option<String>,
    pub autoneg: Option<bool>,
    pub lldpmed_enabled: Option<bool>,
    pub speed: Option<i64>,
    pub portconf_id: Option<String>,
    pub lldpmed_notify_enabled: Option<bool>,
    pub site_id: Option<String>,
    pub dot1x_ctrl: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EthernetTable {
    pub num_port: Option<u16>,
    pub name: String,
    pub mac: String,
    pub native_bond: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LastUplink {
    pub port_idx: Option<i64>,
    pub uplink_mac: String,
    pub uplink_remote_port: Option<u16>,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EtherLighting {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SwitchCaps {
    pub max_aggregate_sessions: Option<i64>,
    pub feature_caps: Option<i64>,
    pub etherlight_caps: Option<i64>,
    pub max_custom_ip_acls: Option<i64>,
    pub max_custom_mac_acls: Option<i64>,
    pub max_global_acls: Option<i64>,
    pub max_reserved_routes: Option<i64>,
    pub vlan_caps: Option<i64>,
    pub max_vlan_count: Option<i64>,
    pub max_static_routes: Option<i64>,
    pub max_mirror_sessions: Option<i64>,
    pub max_l3_intf: Option<i64>,
    pub max_qos_profiles: Option<i64>,
    pub max_class_maps: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Uplink {
    pub uplink_mac: Option<String>,
    pub uplink_remote_port: Option<u16>,
    pub port_idx: Option<i64>,
    #[serde(rename = "type")]
    pub type_field: String,
    pub full_duplex: Option<bool>,
    pub ip: Option<String>,
    pub mac: Option<String>,
    pub name: Option<String>,
    pub netmask: Option<String>,
    pub num_port: Option<u16>,
    pub rx_bytes: Option<u64>,
    pub rx_dropped: Option<u64>,
    pub rx_errors: Option<u64>,
    pub rx_multicast: Option<u64>,
    pub rx_packets: Option<u64>,
    pub speed: Option<u32>,
    pub tx_bytes: Option<u64>,
    pub tx_dropped: Option<u64>,
    pub tx_errors: Option<u64>,
    pub tx_packets: Option<u64>,
    pub up: Option<bool>,
    pub media: Option<String>,
    pub max_speed: Option<u32>,
    #[serde(rename = "tx_bytes-r")]
    pub tx_bytes_r: Option<f64>,
    #[serde(rename = "rx_bytes-r")]
    pub rx_bytes_r: Option<f64>,
    pub uplink_source: Option<String>,
    pub uplink_device_name: Option<String>,
    pub max_vlan: Option<i64>,
    pub uplink_port_poe_caps: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScanRadioTable {
    pub name: String,
    pub radio_caps: i64,
    pub radio_caps2: i64,
    pub radio: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AntennaTable {
    pub wifi1_gain: Option<u16>,
    pub default: Option<bool>,
    pub name: String,
    pub id: i64,
    pub wifi0_gain: Option<u16>,
    pub wifi2_gain: Option<u16>,
    pub wifi3_gain: Option<u16>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RadioTable {
    pub ht: Value,
    pub channel: Value,
    pub antenna_gain: u16,
    pub builtin_antenna: bool,
    pub vwire_enabled: bool,
    pub hard_noise_floor_enabled: Option<bool>,
    pub sens_level_enabled: Option<bool>,
    pub max_txpower: u16,
    pub min_rssi_enabled: bool,
    pub builtin_ant_gain: u16,
    pub radio: String,
    pub nss: u16,
    pub tx_power_mode: String,
    pub min_rssi: Option<i16>,
    pub name: String,
    pub min_txpower: u16,
    pub radio_caps: i64,
    pub antenna_id: i16,
    pub radio_caps2: i64,
    pub current_antenna_gain: u16,
    pub has_dfs: Option<bool>,
    pub channel_optimization_enabled: Option<bool>,
    pub is_11ac: Option<bool>,
    pub max_chan_cntr_frq: Option<u32>,
    pub has_ht160: Option<bool>,
    pub has_restricted_channels: Option<bool>,
    pub min_chan_cntr_frq: Option<u32>,
    pub has_fccdfs: Option<bool>,
    pub backup_channel: Option<i64>,
    pub loadbalance_enabled: Option<bool>,
    pub sens_level: Option<i64>,
    pub is_11ax: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RadioTableStat {
    pub name: String,
    pub last_channel: i64,
    pub radio: String,
    pub ast_be_xmit: Option<i64>,
    pub cu_total: i64,
    pub cu_self_rx: i64,
    pub cu_self_tx: i64,
    pub gain: i64,
    pub satisfaction: i64,
    pub state: String,
    pub tx_packets: i64,
    pub tx_retries: i64,
    pub num_sta: u64,
    #[serde(rename = "guest-num_sta")]
    pub guest_num_sta: u64,
    #[serde(rename = "user-num_sta")]
    pub user_num_sta: u64,
    pub channel: Option<i64>,
    pub extchannel: Option<i64>,
    pub tx_power: Option<i64>,
    pub tx_retries_pct: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SysStats {
    pub loadavg_1: Option<String>,
    pub loadavg_15: Option<String>,
    pub loadavg_5: Option<String>,
    pub mem_buffer: Option<i64>,
    pub mem_total: Option<i64>,
    pub mem_used: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SystemStats {
    pub cpu: Option<String>,
    pub mem: Option<String>,
    pub uptime: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LldpTable {
    pub chassis_id: String,
    pub chassis_id_subtype: Option<String>,
    pub is_wired: bool,
    pub local_port_idx: i64,
    pub local_port_name: String,
    pub port_id: String,
    pub power_allocated: Option<i64>,
    pub power_requested: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DownlinkTable {
    pub mac: String,
    pub port_idx: i64,
    pub speed: i64,
    pub full_duplex: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DetailedStates {
    pub uplink_near_power_limit: Option<bool>,
    pub device_near_power_limit: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stat {
    pub sw: Option<Sw>,
    pub ap: Option<Ap>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sw {
    pub site_id: String,
    pub o: String,
    pub oid: String,
    pub sw: String,
    pub time: i64,
    pub datetime: String,
    pub rx_packets: f64,
    pub rx_bytes: f64,
    pub rx_errors: f64,
    pub rx_dropped: f64,
    pub rx_crypts: f64,
    pub rx_frags: f64,
    pub tx_packets: f64,
    pub tx_bytes: f64,
    pub tx_errors: f64,
    pub tx_dropped: f64,
    pub tx_retries: f64,
    pub rx_multicast: f64,
    pub rx_broadcast: f64,
    pub tx_multicast: f64,
    pub tx_broadcast: f64,
    pub bytes: f64,
    pub duration: f64,
    #[serde(rename = "port_2-rx_packets")]
    pub port_2_rx_packets: Option<f64>,
    #[serde(rename = "port_2-rx_bytes")]
    pub port_2_rx_bytes: Option<f64>,
    #[serde(rename = "port_2-tx_packets")]
    pub port_2_tx_packets: Option<f64>,
    #[serde(rename = "port_2-tx_bytes")]
    pub port_2_tx_bytes: Option<f64>,
    #[serde(rename = "port_2-rx_multicast")]
    pub port_2_rx_multicast: Option<f64>,
    #[serde(rename = "port_2-rx_broadcast")]
    pub port_2_rx_broadcast: Option<f64>,
    #[serde(rename = "port_2-tx_multicast")]
    pub port_2_tx_multicast: Option<f64>,
    #[serde(rename = "port_2-tx_broadcast")]
    pub port_2_tx_broadcast: Option<f64>,
    #[serde(rename = "port_31-rx_packets")]
    pub port_31_rx_packets: Option<f64>,
    #[serde(rename = "port_31-rx_bytes")]
    pub port_31_rx_bytes: Option<f64>,
    #[serde(rename = "port_31-rx_dropped")]
    pub port_31_rx_dropped: Option<f64>,
    #[serde(rename = "port_31-tx_packets")]
    pub port_31_tx_packets: Option<f64>,
    #[serde(rename = "port_31-tx_bytes")]
    pub port_31_tx_bytes: Option<f64>,
    #[serde(rename = "port_31-rx_multicast")]
    pub port_31_rx_multicast: Option<f64>,
    #[serde(rename = "port_31-rx_broadcast")]
    pub port_31_rx_broadcast: Option<f64>,
    #[serde(rename = "port_31-tx_multicast")]
    pub port_31_tx_multicast: Option<f64>,
    #[serde(rename = "port_31-tx_broadcast")]
    pub port_31_tx_broadcast: Option<f64>,
    #[serde(rename = "port_2-rx_dropped")]
    pub port_2_rx_dropped: Option<f64>,
    #[serde(rename = "port_1-rx_packets")]
    pub port_1_rx_packets: Option<f64>,
    #[serde(rename = "port_1-rx_bytes")]
    pub port_1_rx_bytes: Option<f64>,
    #[serde(rename = "port_1-tx_packets")]
    pub port_1_tx_packets: Option<f64>,
    #[serde(rename = "port_1-tx_bytes")]
    pub port_1_tx_bytes: Option<f64>,
    #[serde(rename = "port_1-rx_multicast")]
    pub port_1_rx_multicast: Option<f64>,
    #[serde(rename = "port_1-tx_multicast")]
    pub port_1_tx_multicast: Option<f64>,
    #[serde(rename = "port_1-tx_broadcast")]
    pub port_1_tx_broadcast: Option<f64>,
    #[serde(rename = "port_3-rx_packets")]
    pub port_3_rx_packets: Option<f64>,
    #[serde(rename = "port_3-rx_bytes")]
    pub port_3_rx_bytes: Option<f64>,
    #[serde(rename = "port_3-tx_packets")]
    pub port_3_tx_packets: Option<f64>,
    #[serde(rename = "port_3-tx_bytes")]
    pub port_3_tx_bytes: Option<f64>,
    #[serde(rename = "port_3-rx_multicast")]
    pub port_3_rx_multicast: Option<f64>,
    #[serde(rename = "port_3-tx_multicast")]
    pub port_3_tx_multicast: Option<f64>,
    #[serde(rename = "port_3-tx_broadcast")]
    pub port_3_tx_broadcast: Option<f64>,
    #[serde(rename = "port_27-tx_packets")]
    pub port_27_tx_packets: Option<f64>,
    #[serde(rename = "port_27-tx_bytes")]
    pub port_27_tx_bytes: Option<f64>,
    #[serde(rename = "port_27-tx_multicast")]
    pub port_27_tx_multicast: Option<f64>,
    #[serde(rename = "port_27-tx_broadcast")]
    pub port_27_tx_broadcast: Option<f64>,
    #[serde(rename = "port_28-tx_packets")]
    pub port_28_tx_packets: Option<f64>,
    #[serde(rename = "port_28-tx_bytes")]
    pub port_28_tx_bytes: Option<f64>,
    #[serde(rename = "port_28-tx_multicast")]
    pub port_28_tx_multicast: Option<f64>,
    #[serde(rename = "port_28-tx_broadcast")]
    pub port_28_tx_broadcast: Option<f64>,
    #[serde(rename = "port_32-rx_packets")]
    pub port_32_rx_packets: Option<f64>,
    #[serde(rename = "port_32-rx_bytes")]
    pub port_32_rx_bytes: Option<f64>,
    #[serde(rename = "port_32-rx_dropped")]
    pub port_32_rx_dropped: Option<f64>,
    #[serde(rename = "port_32-tx_packets")]
    pub port_32_tx_packets: Option<f64>,
    #[serde(rename = "port_32-tx_bytes")]
    pub port_32_tx_bytes: Option<f64>,
    #[serde(rename = "port_32-rx_multicast")]
    pub port_32_rx_multicast: Option<f64>,
    #[serde(rename = "port_32-rx_broadcast")]
    pub port_32_rx_broadcast: Option<f64>,
    #[serde(rename = "port_32-tx_multicast")]
    pub port_32_tx_multicast: Option<f64>,
    #[serde(rename = "port_32-tx_broadcast")]
    pub port_32_tx_broadcast: Option<f64>,
    #[serde(rename = "port_1-rx_broadcast")]
    pub port_1_rx_broadcast: Option<f64>,
    #[serde(rename = "port_3-rx_dropped")]
    pub port_3_rx_dropped: Option<f64>,
    #[serde(rename = "port_28-rx_packets")]
    pub port_28_rx_packets: Option<f64>,
    #[serde(rename = "port_28-rx_bytes")]
    pub port_28_rx_bytes: Option<f64>,
    #[serde(rename = "port_28-rx_dropped")]
    pub port_28_rx_dropped: Option<f64>,
    #[serde(rename = "port_28-rx_multicast")]
    pub port_28_rx_multicast: Option<f64>,
    #[serde(rename = "port_3-rx_broadcast")]
    pub port_3_rx_broadcast: Option<f64>,
    #[serde(rename = "port_9-rx_packets")]
    pub port_9_rx_packets: Option<f64>,
    #[serde(rename = "port_9-rx_bytes")]
    pub port_9_rx_bytes: Option<f64>,
    #[serde(rename = "port_9-rx_dropped")]
    pub port_9_rx_dropped: Option<f64>,
    #[serde(rename = "port_9-tx_packets")]
    pub port_9_tx_packets: Option<f64>,
    #[serde(rename = "port_9-tx_bytes")]
    pub port_9_tx_bytes: Option<f64>,
    #[serde(rename = "port_9-rx_multicast")]
    pub port_9_rx_multicast: Option<f64>,
    #[serde(rename = "port_9-rx_broadcast")]
    pub port_9_rx_broadcast: Option<f64>,
    #[serde(rename = "port_9-tx_multicast")]
    pub port_9_tx_multicast: Option<f64>,
    #[serde(rename = "port_9-tx_broadcast")]
    pub port_9_tx_broadcast: Option<f64>,
    #[serde(rename = "port_1-rx_errors")]
    pub port_1_rx_errors: Option<f64>,
    #[serde(rename = "port_1-rx_dropped")]
    pub port_1_rx_dropped: Option<f64>,
    #[serde(rename = "port_18-rx_packets")]
    pub port_18_rx_packets: Option<f64>,
    #[serde(rename = "port_18-rx_bytes")]
    pub port_18_rx_bytes: Option<f64>,
    #[serde(rename = "port_18-tx_packets")]
    pub port_18_tx_packets: Option<f64>,
    #[serde(rename = "port_18-tx_bytes")]
    pub port_18_tx_bytes: Option<f64>,
    #[serde(rename = "port_18-rx_multicast")]
    pub port_18_rx_multicast: Option<f64>,
    #[serde(rename = "port_18-tx_multicast")]
    pub port_18_tx_multicast: Option<f64>,
    #[serde(rename = "port_18-tx_broadcast")]
    pub port_18_tx_broadcast: Option<f64>,
    #[serde(rename = "port_18-rx_broadcast")]
    pub port_18_rx_broadcast: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ap {
    pub site_id: String,
    pub o: String,
    pub oid: String,
    pub ap: String,
    pub time: i64,
    pub datetime: String,
    #[serde(rename = "guest-wifi1-rx_packets")]
    pub guest_wifi1_rx_packets: f64,
    #[serde(rename = "user-wifi1-rx_packets")]
    pub user_wifi1_rx_packets: f64,
    #[serde(rename = "user-rx_packets")]
    pub user_rx_packets: f64,
    #[serde(rename = "guest-rx_packets")]
    pub guest_rx_packets: f64,
    #[serde(rename = "wifi0-rx_packets")]
    pub wifi0_rx_packets: f64,
    #[serde(rename = "wifi1-rx_packets")]
    pub wifi1_rx_packets: f64,
    pub rx_packets: f64,
    #[serde(rename = "guest-wifi1-rx_bytes")]
    pub guest_wifi1_rx_bytes: f64,
    #[serde(rename = "user-wifi1-rx_bytes")]
    pub user_wifi1_rx_bytes: f64,
    #[serde(rename = "user-rx_bytes")]
    pub user_rx_bytes: f64,
    #[serde(rename = "guest-rx_bytes")]
    pub guest_rx_bytes: f64,
    #[serde(rename = "wifi0-rx_bytes")]
    pub wifi0_rx_bytes: f64,
    #[serde(rename = "wifi1-rx_bytes")]
    pub wifi1_rx_bytes: f64,
    pub rx_bytes: f64,
    #[serde(rename = "guest-wifi1-rx_errors")]
    pub guest_wifi1_rx_errors: f64,
    #[serde(rename = "user-wifi1-rx_errors")]
    pub user_wifi1_rx_errors: f64,
    #[serde(rename = "user-rx_errors")]
    pub user_rx_errors: f64,
    #[serde(rename = "guest-rx_errors")]
    pub guest_rx_errors: f64,
    #[serde(rename = "wifi0-rx_errors")]
    pub wifi0_rx_errors: f64,
    #[serde(rename = "wifi1-rx_errors")]
    pub wifi1_rx_errors: f64,
    pub rx_errors: f64,
    #[serde(rename = "guest-wifi1-rx_dropped")]
    pub guest_wifi1_rx_dropped: f64,
    #[serde(rename = "user-wifi1-rx_dropped")]
    pub user_wifi1_rx_dropped: f64,
    #[serde(rename = "user-rx_dropped")]
    pub user_rx_dropped: f64,
    #[serde(rename = "guest-rx_dropped")]
    pub guest_rx_dropped: f64,
    #[serde(rename = "wifi0-rx_dropped")]
    pub wifi0_rx_dropped: f64,
    #[serde(rename = "wifi1-rx_dropped")]
    pub wifi1_rx_dropped: f64,
    pub rx_dropped: f64,
    #[serde(rename = "guest-wifi1-rx_crypts")]
    pub guest_wifi1_rx_crypts: f64,
    #[serde(rename = "user-wifi1-rx_crypts")]
    pub user_wifi1_rx_crypts: f64,
    #[serde(rename = "user-rx_crypts")]
    pub user_rx_crypts: f64,
    #[serde(rename = "guest-rx_crypts")]
    pub guest_rx_crypts: f64,
    #[serde(rename = "wifi0-rx_crypts")]
    pub wifi0_rx_crypts: f64,
    #[serde(rename = "wifi1-rx_crypts")]
    pub wifi1_rx_crypts: f64,
    pub rx_crypts: f64,
    #[serde(rename = "guest-wifi1-rx_frags")]
    pub guest_wifi1_rx_frags: f64,
    #[serde(rename = "user-wifi1-rx_frags")]
    pub user_wifi1_rx_frags: f64,
    #[serde(rename = "user-rx_frags")]
    pub user_rx_frags: f64,
    #[serde(rename = "guest-rx_frags")]
    pub guest_rx_frags: f64,
    #[serde(rename = "wifi0-rx_frags")]
    pub wifi0_rx_frags: f64,
    #[serde(rename = "wifi1-rx_frags")]
    pub wifi1_rx_frags: f64,
    pub rx_frags: f64,
    #[serde(rename = "guest-wifi1-tx_packets")]
    pub guest_wifi1_tx_packets: f64,
    #[serde(rename = "user-wifi1-tx_packets")]
    pub user_wifi1_tx_packets: f64,
    #[serde(rename = "user-tx_packets")]
    pub user_tx_packets: f64,
    #[serde(rename = "guest-tx_packets")]
    pub guest_tx_packets: f64,
    #[serde(rename = "wifi0-tx_packets")]
    pub wifi0_tx_packets: f64,
    #[serde(rename = "wifi1-tx_packets")]
    pub wifi1_tx_packets: f64,
    pub tx_packets: f64,
    #[serde(rename = "guest-wifi1-tx_bytes")]
    pub guest_wifi1_tx_bytes: f64,
    #[serde(rename = "user-wifi1-tx_bytes")]
    pub user_wifi1_tx_bytes: f64,
    #[serde(rename = "user-tx_bytes")]
    pub user_tx_bytes: f64,
    #[serde(rename = "guest-tx_bytes")]
    pub guest_tx_bytes: f64,
    #[serde(rename = "wifi0-tx_bytes")]
    pub wifi0_tx_bytes: f64,
    #[serde(rename = "wifi1-tx_bytes")]
    pub wifi1_tx_bytes: f64,
    pub tx_bytes: f64,
    #[serde(rename = "guest-wifi1-tx_errors")]
    pub guest_wifi1_tx_errors: f64,
    #[serde(rename = "user-wifi1-tx_errors")]
    pub user_wifi1_tx_errors: f64,
    #[serde(rename = "user-tx_errors")]
    pub user_tx_errors: f64,
    #[serde(rename = "guest-tx_errors")]
    pub guest_tx_errors: f64,
    #[serde(rename = "wifi0-tx_errors")]
    pub wifi0_tx_errors: f64,
    #[serde(rename = "wifi1-tx_errors")]
    pub wifi1_tx_errors: f64,
    pub tx_errors: f64,
    #[serde(rename = "guest-wifi1-tx_dropped")]
    pub guest_wifi1_tx_dropped: f64,
    #[serde(rename = "user-wifi1-tx_dropped")]
    pub user_wifi1_tx_dropped: f64,
    #[serde(rename = "user-tx_dropped")]
    pub user_tx_dropped: f64,
    #[serde(rename = "guest-tx_dropped")]
    pub guest_tx_dropped: f64,
    #[serde(rename = "wifi0-tx_dropped")]
    pub wifi0_tx_dropped: f64,
    #[serde(rename = "wifi1-tx_dropped")]
    pub wifi1_tx_dropped: f64,
    pub tx_dropped: f64,
    #[serde(rename = "guest-wifi1-tx_retries")]
    pub guest_wifi1_tx_retries: f64,
    #[serde(rename = "user-wifi1-tx_retries")]
    pub user_wifi1_tx_retries: f64,
    #[serde(rename = "user-tx_retries")]
    pub user_tx_retries: f64,
    #[serde(rename = "guest-tx_retries")]
    pub guest_tx_retries: f64,
    #[serde(rename = "wifi0-tx_retries")]
    pub wifi0_tx_retries: f64,
    #[serde(rename = "wifi1-tx_retries")]
    pub wifi1_tx_retries: f64,
    pub tx_retries: f64,
    #[serde(rename = "guest-wifi1-mac_filter_rejections")]
    pub guest_wifi1_mac_filter_rejections: f64,
    #[serde(rename = "user-wifi1-mac_filter_rejections")]
    pub user_wifi1_mac_filter_rejections: f64,
    #[serde(rename = "user-mac_filter_rejections")]
    pub user_mac_filter_rejections: f64,
    #[serde(rename = "guest-mac_filter_rejections")]
    pub guest_mac_filter_rejections: f64,
    #[serde(rename = "wifi0-mac_filter_rejections")]
    pub wifi0_mac_filter_rejections: f64,
    #[serde(rename = "wifi1-mac_filter_rejections")]
    pub wifi1_mac_filter_rejections: f64,
    pub mac_filter_rejections: f64,
    #[serde(rename = "guest-wifi1-wifi_tx_attempts")]
    pub guest_wifi1_wifi_tx_attempts: f64,
    #[serde(rename = "user-wifi1-wifi_tx_attempts")]
    pub user_wifi1_wifi_tx_attempts: f64,
    #[serde(rename = "user-wifi_tx_attempts")]
    pub user_wifi_tx_attempts: f64,
    #[serde(rename = "guest-wifi_tx_attempts")]
    pub guest_wifi_tx_attempts: f64,
    #[serde(rename = "wifi0-wifi_tx_attempts")]
    pub wifi0_wifi_tx_attempts: f64,
    #[serde(rename = "wifi1-wifi_tx_attempts")]
    pub wifi1_wifi_tx_attempts: f64,
    pub wifi_tx_attempts: f64,
    #[serde(rename = "guest-wifi1-wifi_tx_dropped")]
    pub guest_wifi1_wifi_tx_dropped: f64,
    #[serde(rename = "user-wifi1-wifi_tx_dropped")]
    pub user_wifi1_wifi_tx_dropped: f64,
    #[serde(rename = "user-wifi_tx_dropped")]
    pub user_wifi_tx_dropped: f64,
    #[serde(rename = "guest-wifi_tx_dropped")]
    pub guest_wifi_tx_dropped: f64,
    #[serde(rename = "wifi0-wifi_tx_dropped")]
    pub wifi0_wifi_tx_dropped: f64,
    #[serde(rename = "wifi1-wifi_tx_dropped")]
    pub wifi1_wifi_tx_dropped: f64,
    pub wifi_tx_dropped: f64,
    #[serde(rename = "guest-wifi1-duration")]
    pub guest_wifi1_duration: f64,
    #[serde(rename = "user-wifi1-duration")]
    pub user_wifi1_duration: f64,
    #[serde(rename = "user-duration")]
    pub user_duration: f64,
    #[serde(rename = "guest-duration")]
    pub guest_duration: f64,
    #[serde(rename = "wifi0-duration")]
    pub wifi0_duration: f64,
    #[serde(rename = "wifi1-duration")]
    pub wifi1_duration: f64,
    pub duration: f64,
    #[serde(rename = "na-wifi_tx_attempts")]
    pub na_wifi_tx_attempts: f64,
    #[serde(rename = "na-wifi_tx_dropped")]
    pub na_wifi_tx_dropped: f64,
    #[serde(rename = "na-tx_retries")]
    pub na_tx_retries: f64,
    #[serde(rename = "na-tx_packets")]
    pub na_tx_packets: f64,
    #[serde(rename = "na-tx_bytes")]
    pub na_tx_bytes: f64,
    #[serde(rename = "na-rx_packets")]
    pub na_rx_packets: f64,
    #[serde(rename = "na-rx_bytes")]
    pub na_rx_bytes: f64,
    #[serde(rename = "na-bytes")]
    pub na_bytes: f64,
    #[serde(rename = "na-total_mcast_bcast")]
    pub na_total_mcast_bcast: f64,
    pub bytes: f64,
    #[serde(rename = "port_1-rx_packets")]
    pub port_1_rx_packets: f64,
    #[serde(rename = "port_1-rx_bytes")]
    pub port_1_rx_bytes: f64,
    #[serde(rename = "port_1-tx_packets")]
    pub port_1_tx_packets: f64,
    #[serde(rename = "port_1-tx_bytes")]
    pub port_1_tx_bytes: f64,
    #[serde(rename = "port_1-rx_multicast")]
    pub port_1_rx_multicast: f64,
    #[serde(rename = "port_1-rx_broadcast")]
    pub port_1_rx_broadcast: f64,
    #[serde(rename = "port_1-tx_multicast")]
    pub port_1_tx_multicast: f64,
    #[serde(rename = "port_1-tx_broadcast")]
    pub port_1_tx_broadcast: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rps {
    pub power_management_mode: String,
    pub rps_port_table: Vec<RpsPortTable>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RpsPortTable {
    pub port_idx: i64,
    pub name: String,
    pub port_mode: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VapTable {
    pub anomalies_bar_chart: AnomaliesBarChart,
    pub anomalies_bar_chart_now: AnomaliesBarChartNow,
    pub avg_client_signal: i64,
    pub bssid: String,
    pub bw: i64,
    pub ccq: i64,
    pub channel: i64,
    pub dns_avg_latency: i64,
    pub essid: String,
    pub icmp_avg_rtt: i64,
    pub id: String,
    pub mac_filter_rejections: i64,
    pub name: String,
    pub num_satisfaction_sta: i64,
    pub num_sta: i64,
    pub radio: String,
    pub radio_name: String,
    pub reasons_bar_chart: ReasonsBarChart,
    pub reasons_bar_chart_now: ReasonsBarChartNow,
    pub rx_bcast: i64,
    pub rx_bytes: i64,
    pub rx_crypts: i64,
    pub rx_dropped: i64,
    pub rx_errors: i64,
    pub rx_frags: i64,
    pub rx_mcast: i64,
    pub rx_nwids: i64,
    pub rx_packets: i64,
    pub rx_tcp_stats: RxTcpStats,
    pub satisfaction: i64,
    pub state: String,
    pub tx_bcast: i64,
    pub tx_bytes: i64,
    pub tx_combined_retries: i64,
    pub tx_data_mpdu_bytes: i64,
    pub tx_dropped: i64,
    pub tx_errors: i64,
    pub tx_mcast: i64,
    pub tx_packets: i64,
    pub tx_power: i64,
    pub tx_retries: i64,
    pub tx_rts_retries: i64,
    pub tx_success: i64,
    pub tx_tcp_stats: TxTcpStats,
    pub tx_total: i64,
    pub up: bool,
    pub usage: String,
    pub wifi_tx_attempts: i64,
    pub wifi_tx_dropped: i64,
    pub t: String,
    pub wlanconf_id: String,
    pub is_guest: bool,
    pub is_wep: bool,
    pub ap_mac: String,
    pub site_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnomaliesBarChart {
    pub high_disconnect_count: i64,
    pub high_dns_latency: i64,
    pub high_icmp_rtt: i64,
    pub high_tcp_latency: i64,
    pub high_tcp_packet_loss: i64,
    pub high_wifi_drops: i64,
    pub high_wifi_latency: i64,
    pub high_wifi_retries: i64,
    pub low_phy_rate: i64,
    pub no_dhcp_response: i64,
    pub poor_stream_eff: i64,
    pub sleepy_client: i64,
    pub sta_arp_timeout: i64,
    pub sta_dns_timeout: i64,
    pub sta_ip_timeout: i64,
    pub weak_signal: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnomaliesBarChartNow {
    pub high_disconnect_count: i64,
    pub high_dns_latency: i64,
    pub high_icmp_rtt: i64,
    pub high_tcp_latency: i64,
    pub high_tcp_packet_loss: i64,
    pub high_wifi_drops: i64,
    pub high_wifi_latency: i64,
    pub high_wifi_retries: i64,
    pub low_phy_rate: i64,
    pub no_dhcp_response: i64,
    pub poor_stream_eff: i64,
    pub sleepy_client: i64,
    pub sta_arp_timeout: i64,
    pub sta_dns_timeout: i64,
    pub sta_ip_timeout: i64,
    pub weak_signal: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReasonsBarChart {
    pub no_dhcp_response: i64,
    pub phy_rate: i64,
    pub signal: i64,
    pub sleepy_client: i64,
    pub sta_arp_timeout: i64,
    pub sta_disconnects: i64,
    pub sta_dns_latency: i64,
    pub sta_dns_timeout: i64,
    pub sta_icmp_rtt: i64,
    pub sta_ip_timeout: i64,
    pub stream_eff: i64,
    pub tcp_latency: i64,
    pub tcp_packet_loss: i64,
    pub wifi_drops: i64,
    pub wifi_latency: i64,
    pub wifi_retries: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReasonsBarChartNow {
    pub no_dhcp_response: i64,
    pub phy_rate: i64,
    pub signal: i64,
    pub sleepy_client: i64,
    pub sta_arp_timeout: i64,
    pub sta_disconnects: i64,
    pub sta_dns_latency: i64,
    pub sta_dns_timeout: i64,
    pub sta_icmp_rtt: i64,
    pub sta_ip_timeout: i64,
    pub stream_eff: i64,
    pub tcp_latency: i64,
    pub tcp_packet_loss: i64,
    pub wifi_drops: i64,
    pub wifi_latency: i64,
    pub wifi_retries: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RxTcpStats {
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
pub struct TxTcpStats {
    pub goodbytes: i64,
    pub lat_avg: i64,
    pub lat_max: i64,
    pub lat_min: i64,
    pub lat_samples: i64,
    pub lat_sum: i64,
    pub retries: i64,
    pub stalls: i64,
}

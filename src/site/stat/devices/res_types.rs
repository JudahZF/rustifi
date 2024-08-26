use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeviceStatsList {
    pub meta: Meta,
    pub data: Vec<Daum>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Meta {
    pub rc: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Daum {
    pub required_version: String,
    pub port_table: Vec<PortTable>,
    pub has_speaker: bool,
    pub license_state: String,
    pub mesh_sta_vap_enabled: bool,
    pub last_wan_ip: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub board_rev: i64,
    pub setup_id: String,
    pub setup_provision_completed: bool,
    pub ethernet_overrides: Vec<EthernetOverride>,
    pub hw_caps: i64,
    pub reboot_duration: i64,
    pub hostname: String,
    pub config_network: ConfigNetwork,
    pub syslog_key: String,
    pub model: String,
    pub udapi_caps: i64,
    pub manufacturer_id: i64,
    pub bandsteering_mode: String,
    pub sysid: i64,
    pub ip: String,
    pub fw2_caps: i64,
    pub support_wifi6e: bool,
    pub jumboframe_enabled: bool,
    pub last_connection_network_name: String,
    pub config_network_lan: ConfigNetworkLan,
    pub version: String,
    pub unsupported_reason: i64,
    pub adoption_completed: bool,
    pub scan_radio_table: Vec<Value>,
    pub shortname: String,
    pub x_vwirekey: String,
    pub anon_id: String,
    pub stp_version: String,
    pub supports_fingerprint_ml: bool,
    pub last_connection_network_id: String,
    pub country_code: i64,
    pub wlangroup_id_na: String,
    pub countrycode_table: Vec<Value>,
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
    pub two_phase_adopt: bool,
    pub inform_ip: String,
    pub setup_provision_tracking: bool,
    pub dns_shield_server_list_hash: String,
    pub cfgversion: String,
    pub teleport_version: String,
    pub device_domain: String,
    pub mac: String,
    pub ruleset_interfaces: RulesetInterfaces,
    pub provisioned_at: i64,
    pub inform_url: String,
    pub upgrade_duration: i64,
    pub ethernet_table: Vec<EthernetTable>,
    pub flowctrl_enabled: bool,
    pub unsupported: bool,
    pub dot1x_portctrl_enabled: bool,
    pub usg_caps: i64,
    pub ipv6: Vec<String>,
    pub mesh_uplink_1: String,
    pub mesh_uplink_2: String,
    pub disconnected_at: i64,
    pub usg2_caps: i64,
    pub architecture: String,
    pub wifi_caps2: i64,
    pub x_aes_gcm: bool,
    pub has_fan: bool,
    pub udapi_version: UdapiVersion,
    pub has_eth1: bool,
    pub model_incompatible: bool,
    pub x_authkey: String,
    pub ids_ips_last_known_signature: String,
    pub model_in_eol: bool,
    pub vwire_table: Vec<Value>,
    pub has_temperature: bool,
    pub switch_caps: SwitchCaps,
    pub model_in_lts: bool,
    pub kernel_version: String,
    pub serial: String,
    pub power_source_ctrl_enabled: bool,
    pub fixed_ap_available: bool,
    pub adopted: bool,
    pub hash_id: String,
    pub device_id: String,
    pub state: i64,
    pub start_disconnected_millis: i64,
    pub last_seen: i64,
    pub uptime: i64,
    #[serde(rename = "_uptime")]
    pub uptime2: i64,
    pub next_interval: i64,
    pub min_inform_interval_seconds: i64,
    pub upgradable: bool,
    pub adoptable_when_upgraded: bool,
    pub rollupgrade: bool,
    pub last_config_applied_successfully: bool,
    pub known_cfgversion: String,
    pub locating: bool,
    pub start_connected_millis: i64,
    pub sys_stats: SysStats,
    #[serde(rename = "system-stats")]
    pub system_stats: SystemStats,
    pub ssh_session_table: Vec<Value>,
    pub lldp_table: Vec<Value>,
    pub displayable_version: String,
    pub connection_network_id: String,
    pub connection_network_name: String,
    pub startup_timestamp: i64,
    pub is_access_point: bool,
    pub safe_for_autoupgrade: bool,
    pub guest_kicks: i64,
    pub guest_token: String,
    pub temperatures: Vec<Temperature>,
    pub storage: Vec<Storage>,
    pub scanning: bool,
    pub spectrum_scanning: bool,
    pub meshv3_peer_mac: String,
    pub element_peer_mac: String,
    pub satisfaction: i64,
    pub uptime_stats: UptimeStats,
    pub overheating: bool,
    pub geo_info: GeoInfo,
    pub active_geo_info: ActiveGeoInfo,
    pub led_state: LedState,
    pub outlet_table: Vec<Value>,
    pub isolated: bool,
    pub radio_table_stats: Vec<RadioTableStat>,
    #[serde(rename = "speedtest-status")]
    pub speedtest_status: SpeedtestStatus,
    #[serde(rename = "speedtest-status-saved")]
    pub speedtest_status_saved: bool,
    pub reported_networks: Vec<ReportedNetwork>,
    pub wan1: Wan1,
    pub uplink: Uplink,
    pub uplink_depth: i64,
    pub downlink_lldp_macs: Vec<Value>,
    pub vap_table: Vec<VapTable>,
    pub ap_downlink_table: Vec<Value>,
    pub vwire_vap_table: Vec<VwireVapTable>,
    #[serde(rename = "bytes-d")]
    pub bytes_d: i64,
    #[serde(rename = "tx_bytes-d")]
    pub tx_bytes_d: i64,
    #[serde(rename = "rx_bytes-d")]
    pub rx_bytes_d: i64,
    #[serde(rename = "bytes-r")]
    pub bytes_r: f64,
    pub downlink_table: Vec<Value>,
    pub network_table: Vec<NetworkTable>,
    pub connect_request_ip: String,
    pub connect_request_port: String,
    pub ipv4_active_leases: Ipv4ActiveLeases,
    pub dhcp_excluded_ip_list: DhcpExcludedIpList,
    pub has_lcm_override: bool,
    pub stat: Stat,
    pub tx_bytes: i64,
    pub rx_bytes: i64,
    pub bytes: i64,
    #[serde(rename = "vwireEnabled")]
    pub vwire_enabled: bool,
    pub uplink_table: Vec<Value>,
    pub num_sta: i64,
    #[serde(rename = "wlan-num_sta")]
    pub wlan_num_sta: i64,
    #[serde(rename = "lan-num_sta")]
    pub lan_num_sta: i64,
    #[serde(rename = "user-wlan-num_sta")]
    pub user_wlan_num_sta: i64,
    #[serde(rename = "user-lan-num_sta")]
    pub user_lan_num_sta: i64,
    #[serde(rename = "user-num_sta")]
    pub user_num_sta: i64,
    #[serde(rename = "guest-wlan-num_sta")]
    pub guest_wlan_num_sta: i64,
    #[serde(rename = "guest-lan-num_sta")]
    pub guest_lan_num_sta: i64,
    #[serde(rename = "guest-num_sta")]
    pub guest_num_sta: i64,
    pub num_desktop: i64,
    pub num_mobile: i64,
    pub num_handheld: i64,
    pub lan_ip: String,
    pub x_has_ssh_hostkey: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PortTable {
    pub port_idx: i64,
    pub media: String,
    pub port_poe: bool,
    pub speed_caps: i64,
    pub op_mode: String,
    pub forward: String,
    pub autoneg: bool,
    pub enable: bool,
    pub error_disabled: bool,
    pub flowctrl_rx: bool,
    pub flowctrl_tx: bool,
    pub full_duplex: bool,
    pub is_uplink: bool,
    pub mac: String,
    pub name: String,
    pub num_port: i64,
    pub poe_enable: bool,
    pub poe_power: String,
    pub rx_broadcast: i64,
    pub rx_bytes: i64,
    pub rx_dropped: i64,
    pub rx_errors: i64,
    pub rx_multicast: i64,
    pub rx_packets: i64,
    pub rx_rate: i64,
    #[serde(rename = "rx_rate-max")]
    pub rx_rate_max: i64,
    pub speed: i64,
    pub tx_broadcast: i64,
    pub tx_bytes: i64,
    pub tx_dropped: i64,
    pub tx_errors: i64,
    pub tx_multicast: i64,
    pub tx_packets: i64,
    pub tx_rate: i64,
    #[serde(rename = "tx_rate-max")]
    pub tx_rate_max: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub up: bool,
    pub ifname: String,
    #[serde(rename = "tx_bytes-r")]
    pub tx_bytes_r: f64,
    #[serde(rename = "rx_bytes-r")]
    pub rx_bytes_r: f64,
    #[serde(rename = "bytes-r")]
    pub bytes_r: f64,
    pub ip: String,
    pub netmask: String,
    pub network_name: String,
    pub masked: bool,
    pub aggregated_by: bool,
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
    pub wifi1_gain: i64,
    pub name: String,
    pub id: i64,
    pub wifi0_gain: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RadioTable {
    pub antenna_gain: i64,
    pub builtin_antenna: bool,
    pub vwire_enabled: bool,
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
    pub radio_caps: i64,
    pub antenna_id: i64,
    pub radio_caps2: i64,
    pub current_antenna_gain: i64,
    pub has_dfs: Option<bool>,
    pub has_ht160: Option<bool>,
    pub has_fccdfs: Option<bool>,
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
pub struct UdapiVersion {
    pub path: String,
    #[serde(rename = "versionDetail")]
    pub version_detail: VersionDetail,
    #[serde(rename = "versionFormat")]
    pub version_format: String,
    pub version: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VersionDetail {
    #[serde(rename = "services/webServer")]
    pub services_web_server: i64,
    #[serde(rename = "services/redirector")]
    pub services_redirector: i64,
    #[serde(rename = "services/upnp")]
    pub services_upnp: i64,
    #[serde(rename = "services/mdns")]
    pub services_mdns: i64,
    #[serde(rename = "services/snmpAgent")]
    pub services_snmp_agent: i64,
    #[serde(rename = "services/utm")]
    pub services_utm: i64,
    #[serde(rename = "vpn/wireguard/site-to-sites")]
    pub vpn_wireguard_site_to_sites: i64,
    #[serde(rename = "vpn/wireguard/servers")]
    pub vpn_wireguard_servers: i64,
    #[serde(rename = "services/dhcpRogueDetector")]
    pub services_dhcp_rogue_detector: i64,
    #[serde(rename = "services/sshServer")]
    pub services_ssh_server: i64,
    #[serde(rename = "firewall/nat")]
    pub firewall_nat: i64,
    pub qos: i64,
    #[serde(rename = "services/bleHTTPTransport")]
    pub services_ble_httptransport: i64,
    #[serde(rename = "vpn/openvpn/raws")]
    pub vpn_openvpn_raws: i64,
    #[serde(rename = "vpn/teleport")]
    pub vpn_teleport: i64,
    #[serde(rename = "bridge-firewall")]
    pub bridge_firewall: i64,
    #[serde(rename = "services/discoveryResponder")]
    pub services_discovery_responder: i64,
    #[serde(rename = "services/igmpSnooping")]
    pub services_igmp_snooping: i64,
    #[serde(rename = "services/dohProxy")]
    pub services_doh_proxy: i64,
    #[serde(rename = "services/vrrp")]
    pub services_vrrp: i64,
    pub peripherals: i64,
    #[serde(rename = "routes/ospf")]
    pub routes_ospf: i64,
    #[serde(rename = "vpn/ipsec/site-to-site")]
    pub vpn_ipsec_site_to_site: i64,
    #[serde(rename = "services/telnetServer")]
    pub services_telnet_server: i64,
    #[serde(rename = "firewall/settings")]
    pub firewall_settings: i64,
    #[serde(rename = "routes/ospf/interfaces")]
    pub routes_ospf_interfaces: i64,
    #[serde(rename = "services/arpInspection")]
    pub services_arp_inspection: i64,
    #[serde(rename = "vpn/wireguard/clients")]
    pub vpn_wireguard_clients: i64,
    #[serde(rename = "services/suspend")]
    pub services_suspend: i64,
    pub system: i64,
    #[serde(rename = "routes/static")]
    pub routes_static: i64,
    #[serde(rename = "system/users")]
    pub system_users: i64,
    #[serde(rename = "firewall/pbr")]
    pub firewall_pbr: i64,
    #[serde(rename = "services/lldp")]
    pub services_lldp: i64,
    #[serde(rename = "services/unms")]
    pub services_unms: i64,
    #[serde(rename = "firewall/sets")]
    pub firewall_sets: i64,
    #[serde(rename = "services/dnsForwarder")]
    pub services_dns_forwarder: i64,
    #[serde(rename = "services/igmpProxy")]
    pub services_igmp_proxy: i64,
    #[serde(rename = "services/radius-profiles")]
    pub services_radius_profiles: i64,
    pub vlans: i64,
    #[serde(rename = "services/radiusServer")]
    pub services_radius_server: i64,
    #[serde(rename = "qos/ip")]
    pub qos_ip: i64,
    #[serde(rename = "firewall/filter")]
    pub firewall_filter: i64,
    #[serde(rename = "services/wanFailover")]
    pub services_wan_failover: i64,
    #[serde(rename = "services/systemLog")]
    pub services_system_log: i64,
    #[serde(rename = "services/stunnel")]
    pub services_stunnel: i64,
    #[serde(rename = "services/clientIsolation")]
    pub services_client_isolation: i64,
    #[serde(rename = "routes/ospf/areas")]
    pub routes_ospf_areas: i64,
    #[serde(rename = "services/l2tpServer")]
    pub services_l2tp_server: i64,
    #[serde(rename = "services/geoipFiltering")]
    pub services_geoip_filtering: i64,
    #[serde(rename = "routes/access-lists")]
    pub routes_access_lists: i64,
    #[serde(rename = "services/loopProtection")]
    pub services_loop_protection: i64,
    #[serde(rename = "firewall/mangle")]
    pub firewall_mangle: i64,
    #[serde(rename = "services/dhcpServers")]
    pub services_dhcp_servers: i64,
    #[serde(rename = "services/dpi")]
    pub services_dpi: i64,
    #[serde(rename = "services/ntpClient")]
    pub services_ntp_client: i64,
    #[serde(rename = "services/unifiNetwork")]
    pub services_unifi_network: i64,
    #[serde(rename = "services/latencyMonitor")]
    pub services_latency_monitor: i64,
    pub interfaces: i64,
    #[serde(rename = "vpn/openvpn/peers")]
    pub vpn_openvpn_peers: i64,
    #[serde(rename = "services/idsIps")]
    pub services_ids_ips: i64,
    #[serde(rename = "services/wifiman")]
    pub services_wifiman: i64,
    #[serde(rename = "services/sslInspection")]
    pub services_ssl_inspection: i64,
    #[serde(rename = "services/uid")]
    pub services_uid: i64,
    #[serde(rename = "services/ipAccounting")]
    pub services_ip_accounting: i64,
    #[serde(rename = "services/ddns")]
    pub services_ddns: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SwitchCaps {
    pub max_aggregate_sessions: i64,
    pub feature_caps: i64,
    pub max_mirror_sessions: i64,
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
pub struct Storage {
    pub mount_point: String,
    pub name: String,
    pub size: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub used: i64,
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeoInfo {
    #[serde(rename = "WAN")]
    pub wan: Wan2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Wan2 {
    pub accuracy: f64,
    pub address: String,
    pub asn: i64,
    pub city: String,
    pub continent_code: String,
    pub country_code: String,
    pub country_name: String,
    pub isp_name: String,
    pub isp_organization: String,
    pub latitude: f64,
    pub longitude: f64,
    pub timezone: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActiveGeoInfo {
    #[serde(rename = "WAN")]
    pub wan: Wan3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Wan3 {
    pub accuracy: f64,
    pub address: String,
    pub asn: i64,
    pub city: String,
    pub continent_code: String,
    pub country_code: String,
    pub country_name: String,
    pub isp_name: String,
    pub isp_organization: String,
    pub latitude: f64,
    pub longitude: f64,
    pub timezone: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LedState {
    pub pattern: String,
    pub tempo: i64,
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
pub struct SpeedtestStatus {
    pub latency: i64,
    pub rundate: i64,
    pub runtime: i64,
    pub server: Server,
    pub source_interface: String,
    pub status_download: i64,
    pub status_ping: i64,
    pub status_summary: i64,
    pub status_upload: i64,
    pub xput_download: f64,
    pub xput_upload: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Server {
    pub cc: String,
    pub city: String,
    pub country: String,
    pub lat: f64,
    pub lon: f64,
    pub provider: String,
    pub provider_url: String,
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
    pub flowctrl_rx: bool,
    pub flowctrl_tx: bool,
    pub full_duplex: bool,
    pub is_uplink: bool,
    pub mac: String,
    pub mac_table: Vec<MacTable>,
    pub media: String,
    pub name: String,
    pub num_port: i64,
    pub poe_enable: bool,
    pub poe_power: String,
    pub port_idx: i64,
    pub port_poe: bool,
    pub rx_broadcast: i64,
    pub rx_bytes: i64,
    pub rx_dropped: i64,
    pub rx_errors: i64,
    pub rx_multicast: i64,
    pub rx_packets: i64,
    pub rx_rate: i64,
    #[serde(rename = "rx_rate-max")]
    pub rx_rate_max: i64,
    pub speed: i64,
    pub speed_caps: i64,
    pub tx_broadcast: i64,
    pub tx_bytes: i64,
    pub tx_dropped: i64,
    pub tx_errors: i64,
    pub tx_multicast: i64,
    pub tx_packets: i64,
    pub tx_rate: i64,
    #[serde(rename = "tx_rate-max")]
    pub tx_rate_max: i64,
    pub up: bool,
    pub ifname: String,
    #[serde(rename = "tx_bytes-r")]
    pub tx_bytes_r: i64,
    #[serde(rename = "rx_bytes-r")]
    pub rx_bytes_r: i64,
    #[serde(rename = "bytes-r")]
    pub bytes_r: i64,
    pub ip: String,
    pub netmask: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MacTable {
    pub age: i64,
    pub authorized: bool,
    pub hostname: String,
    pub ip: String,
    #[serde(rename = "lastReachable")]
    pub last_reachable: i64,
    pub mac: String,
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
    pub rx_bytes: i64,
    pub rx_dropped: i64,
    pub rx_errors: i64,
    pub rx_multicast: i64,
    pub rx_packets: i64,
    pub speedtest_lastrun: i64,
    pub speedtest_ping: i64,
    pub speedtest_status: String,
    pub tx_bytes: i64,
    pub tx_dropped: i64,
    pub tx_errors: i64,
    pub tx_packets: i64,
    pub up: bool,
    pub uptime: i64,
    pub xput_down: f64,
    pub xput_up: f64,
    pub port_idx: i64,
    pub media: String,
    pub speed: i64,
    pub full_duplex: bool,
    pub rx_rate: i64,
    pub tx_rate: i64,
    pub max_speed: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "tx_bytes-r")]
    pub tx_bytes_r: i64,
    #[serde(rename = "rx_bytes-r")]
    pub rx_bytes_r: i64,
    pub uplink_source: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VapTable {
    pub avg_client_signal: i64,
    pub bssid: String,
    pub bw: i64,
    pub ccq: i64,
    pub channel: i64,
    pub essid: String,
    pub extchannel: Option<i64>,
    pub id: String,
    pub mac_filter_rejections: i64,
    pub name: String,
    pub num_sta: i64,
    pub radio: String,
    pub radio_name: String,
    pub rx_bytes: i64,
    pub rx_crypts: i64,
    pub rx_dropped: i64,
    pub rx_errors: i64,
    pub rx_frags: i64,
    pub rx_nwids: i64,
    pub rx_packets: i64,
    pub rx_tcp_stats: RxTcpStats,
    pub satisfaction: i64,
    pub satisfaction_real: i64,
    pub state: String,
    pub tx_bytes: i64,
    pub tx_combined_retries: i64,
    pub tx_data_mpdu_bytes: i64,
    pub tx_dropped: i64,
    pub tx_errors: i64,
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
    pub wifi_tx_latency_mov: Option<WifiTxLatencyMov>,
    pub t: String,
    pub wlanconf_id: String,
    pub is_guest: bool,
    pub is_wep: bool,
    pub ap_mac: String,
    pub map_id: Value,
    pub site_id: String,
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WifiTxLatencyMov {
    pub avg: i64,
    pub max: i64,
    pub min: i64,
    pub total: i64,
    pub total_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VwireVapTable {
    pub state: String,
    pub radio: String,
    pub radio_name: String,
    pub bssid: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkTable {
    pub setting_preference: String,
    pub dhcpdv6_dns_auto: bool,
    pub purpose: String,
    pub igmp_snooping: bool,
    pub ipv6_pd_stop: String,
    pub dhcpguard_enabled: bool,
    pub ipv6_client_address_assignment: String,
    pub dhcpd_start: String,
    pub ipv6_ra_preferred_lifetime: i64,
    pub ipv6_ra_enabled: bool,
    pub dhcpd_stop: String,
    pub domain_name: String,
    pub dhcpd_enabled: bool,
    pub ip_subnet: String,
    pub ipv6_interface_type: String,
    pub networkgroup: String,
    pub dhcpdv6_start: String,
    pub vlan_enabled: bool,
    pub ipv6_setting_preference: String,
    pub dhcpdv6_stop: String,
    pub is_nat: Option<bool>,
    pub dhcpdv6_enabled: Option<bool>,
    pub ipv6_ra_priority: String,
    pub dhcpd_conflict_checking: bool,
    pub ipv6_pd_start: String,
    pub upnp_lan_enabled: bool,
    pub site_id: String,
    pub name: String,
    pub dhcpdv6_leasetime: i64,
    pub mdns_enabled: bool,
    pub ipv6_enabled: bool,
    #[serde(rename = "_id")]
    pub id: String,
    pub attr_no_delete: Option<bool>,
    pub attr_hidden_id: Option<String>,
    pub auto_scale_enabled: bool,
    pub is_guest: bool,
    pub ip: String,
    pub mac: String,
    pub up: bool,
    pub active_dhcp_lease_count: i64,
    pub gateway_interface_name: String,
    pub ipv6_link_local_address: String,
    pub dpistats_table: DpistatsTable,
    pub num_sta: i64,
    pub rx_bytes: i64,
    pub rx_packets: i64,
    pub tx_bytes: i64,
    pub tx_packets: i64,
    pub dhcpd_gateway_enabled: Option<bool>,
    pub network_isolation_enabled: Option<bool>,
    pub dhcpd_unifi_controller: Option<String>,
    pub dhcpd_dns_enabled: Option<bool>,
    pub internet_access_enabled: Option<bool>,
    #[serde(default)]
    pub nat_outbound_ip_addresses: Vec<Value>,
    pub dhcp_relay_enabled: Option<bool>,
    pub ipv6_pd_auto_prefixid_enabled: Option<bool>,
    pub lte_lan_enabled: Option<bool>,
    pub dhcpd_leasetime: Option<i64>,
    pub dhcpd_time_offset_enabled: Option<bool>,
    pub dhcpdv6_allow_slaac: Option<bool>,
    pub enabled: Option<bool>,
    pub vlan: Option<i64>,
    pub dhcpd_wpad_url: Option<String>,
    pub gateway_type: Option<String>,
    pub dhcpd_boot_enabled: Option<bool>,
    pub dhcpd_ntp_enabled: Option<bool>,
    pub dhcpd_tftp_server: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DpistatsTable {
    pub last_updated: i64,
    pub by_cat: Vec<ByCat>,
    pub by_app: Vec<ByApp>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ByCat {
    pub cat: i64,
    pub apps: Vec<i64>,
    pub rx_bytes: i64,
    pub tx_bytes: i64,
    pub rx_packets: i64,
    pub tx_packets: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ByApp {
    pub app: i64,
    pub cat: i64,
    pub clients: Vec<Client>,
    pub known_clients: i64,
    pub rx_bytes: i64,
    pub tx_bytes: i64,
    pub rx_packets: i64,
    pub tx_packets: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Client {
    pub mac: String,
    pub rx_bytes: i64,
    pub tx_bytes: i64,
    pub rx_packets: i64,
    pub tx_packets: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ipv4ActiveLeases {
    pub time: i64,
    pub active_leases: Vec<ActiveLease>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActiveLease {
    #[serde(rename = "leaseExpiry")]
    pub lease_expiry: i64,
    pub mac: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DhcpExcludedIpList {
    pub time: i64,
    pub excluded_ip_list: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stat {
    pub ap: Ap,
    pub sw: Sw,
    pub gw: Gw,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ap {
    pub site_id: String,
    pub o: String,
    pub oid: String,
    pub ap: String,
    pub time: i64,
    pub datetime: String,
    #[serde(rename = "user-wifi1-rx_packets")]
    pub user_wifi1_rx_packets: f64,
    #[serde(rename = "user-wifi0-rx_packets")]
    pub user_wifi0_rx_packets: f64,
    #[serde(rename = "user-rx_packets")]
    pub user_rx_packets: f64,
    #[serde(rename = "guest-rx_packets")]
    pub guest_rx_packets: f64,
    #[serde(rename = "wifi0-rx_packets")]
    pub wifi0_rx_packets: f64,
    #[serde(rename = "wifi1-rx_packets")]
    pub wifi1_rx_packets: f64,
    pub rx_packets: f64,
    #[serde(rename = "user-wifi1-rx_bytes")]
    pub user_wifi1_rx_bytes: f64,
    #[serde(rename = "user-wifi0-rx_bytes")]
    pub user_wifi0_rx_bytes: f64,
    #[serde(rename = "user-rx_bytes")]
    pub user_rx_bytes: f64,
    #[serde(rename = "guest-rx_bytes")]
    pub guest_rx_bytes: f64,
    #[serde(rename = "wifi0-rx_bytes")]
    pub wifi0_rx_bytes: f64,
    #[serde(rename = "wifi1-rx_bytes")]
    pub wifi1_rx_bytes: f64,
    pub rx_bytes: f64,
    #[serde(rename = "user-wifi1-rx_errors")]
    pub user_wifi1_rx_errors: f64,
    #[serde(rename = "user-wifi0-rx_errors")]
    pub user_wifi0_rx_errors: f64,
    #[serde(rename = "user-rx_errors")]
    pub user_rx_errors: f64,
    #[serde(rename = "guest-rx_errors")]
    pub guest_rx_errors: f64,
    #[serde(rename = "wifi0-rx_errors")]
    pub wifi0_rx_errors: f64,
    #[serde(rename = "wifi1-rx_errors")]
    pub wifi1_rx_errors: f64,
    pub rx_errors: f64,
    #[serde(rename = "user-wifi1-rx_dropped")]
    pub user_wifi1_rx_dropped: f64,
    #[serde(rename = "user-wifi0-rx_dropped")]
    pub user_wifi0_rx_dropped: f64,
    #[serde(rename = "user-rx_dropped")]
    pub user_rx_dropped: f64,
    #[serde(rename = "guest-rx_dropped")]
    pub guest_rx_dropped: f64,
    #[serde(rename = "wifi0-rx_dropped")]
    pub wifi0_rx_dropped: f64,
    #[serde(rename = "wifi1-rx_dropped")]
    pub wifi1_rx_dropped: f64,
    pub rx_dropped: f64,
    #[serde(rename = "user-wifi1-rx_crypts")]
    pub user_wifi1_rx_crypts: f64,
    #[serde(rename = "user-wifi0-rx_crypts")]
    pub user_wifi0_rx_crypts: f64,
    #[serde(rename = "user-rx_crypts")]
    pub user_rx_crypts: f64,
    #[serde(rename = "guest-rx_crypts")]
    pub guest_rx_crypts: f64,
    #[serde(rename = "wifi0-rx_crypts")]
    pub wifi0_rx_crypts: f64,
    #[serde(rename = "wifi1-rx_crypts")]
    pub wifi1_rx_crypts: f64,
    pub rx_crypts: f64,
    #[serde(rename = "user-wifi1-rx_frags")]
    pub user_wifi1_rx_frags: f64,
    #[serde(rename = "user-wifi0-rx_frags")]
    pub user_wifi0_rx_frags: f64,
    #[serde(rename = "user-rx_frags")]
    pub user_rx_frags: f64,
    #[serde(rename = "guest-rx_frags")]
    pub guest_rx_frags: f64,
    #[serde(rename = "wifi0-rx_frags")]
    pub wifi0_rx_frags: f64,
    #[serde(rename = "wifi1-rx_frags")]
    pub wifi1_rx_frags: f64,
    pub rx_frags: f64,
    #[serde(rename = "user-wifi1-tx_packets")]
    pub user_wifi1_tx_packets: f64,
    #[serde(rename = "user-wifi0-tx_packets")]
    pub user_wifi0_tx_packets: f64,
    #[serde(rename = "user-tx_packets")]
    pub user_tx_packets: f64,
    #[serde(rename = "guest-tx_packets")]
    pub guest_tx_packets: f64,
    #[serde(rename = "wifi0-tx_packets")]
    pub wifi0_tx_packets: f64,
    #[serde(rename = "wifi1-tx_packets")]
    pub wifi1_tx_packets: f64,
    pub tx_packets: f64,
    #[serde(rename = "user-wifi1-tx_bytes")]
    pub user_wifi1_tx_bytes: f64,
    #[serde(rename = "user-wifi0-tx_bytes")]
    pub user_wifi0_tx_bytes: f64,
    #[serde(rename = "user-tx_bytes")]
    pub user_tx_bytes: f64,
    #[serde(rename = "guest-tx_bytes")]
    pub guest_tx_bytes: f64,
    #[serde(rename = "wifi0-tx_bytes")]
    pub wifi0_tx_bytes: f64,
    #[serde(rename = "wifi1-tx_bytes")]
    pub wifi1_tx_bytes: f64,
    pub tx_bytes: f64,
    #[serde(rename = "user-wifi1-tx_errors")]
    pub user_wifi1_tx_errors: f64,
    #[serde(rename = "user-wifi0-tx_errors")]
    pub user_wifi0_tx_errors: f64,
    #[serde(rename = "user-tx_errors")]
    pub user_tx_errors: f64,
    #[serde(rename = "guest-tx_errors")]
    pub guest_tx_errors: f64,
    #[serde(rename = "wifi0-tx_errors")]
    pub wifi0_tx_errors: f64,
    #[serde(rename = "wifi1-tx_errors")]
    pub wifi1_tx_errors: f64,
    pub tx_errors: f64,
    #[serde(rename = "user-wifi1-tx_dropped")]
    pub user_wifi1_tx_dropped: f64,
    #[serde(rename = "user-wifi0-tx_dropped")]
    pub user_wifi0_tx_dropped: f64,
    #[serde(rename = "user-tx_dropped")]
    pub user_tx_dropped: f64,
    #[serde(rename = "guest-tx_dropped")]
    pub guest_tx_dropped: f64,
    #[serde(rename = "wifi0-tx_dropped")]
    pub wifi0_tx_dropped: f64,
    #[serde(rename = "wifi1-tx_dropped")]
    pub wifi1_tx_dropped: f64,
    pub tx_dropped: f64,
    #[serde(rename = "user-wifi1-tx_retries")]
    pub user_wifi1_tx_retries: f64,
    #[serde(rename = "user-wifi0-tx_retries")]
    pub user_wifi0_tx_retries: f64,
    #[serde(rename = "user-tx_retries")]
    pub user_tx_retries: f64,
    #[serde(rename = "guest-tx_retries")]
    pub guest_tx_retries: f64,
    #[serde(rename = "wifi0-tx_retries")]
    pub wifi0_tx_retries: f64,
    #[serde(rename = "wifi1-tx_retries")]
    pub wifi1_tx_retries: f64,
    pub tx_retries: f64,
    #[serde(rename = "user-wifi1-mac_filter_rejections")]
    pub user_wifi1_mac_filter_rejections: f64,
    #[serde(rename = "user-wifi0-mac_filter_rejections")]
    pub user_wifi0_mac_filter_rejections: f64,
    #[serde(rename = "user-mac_filter_rejections")]
    pub user_mac_filter_rejections: f64,
    #[serde(rename = "guest-mac_filter_rejections")]
    pub guest_mac_filter_rejections: f64,
    #[serde(rename = "wifi0-mac_filter_rejections")]
    pub wifi0_mac_filter_rejections: f64,
    #[serde(rename = "wifi1-mac_filter_rejections")]
    pub wifi1_mac_filter_rejections: f64,
    pub mac_filter_rejections: f64,
    #[serde(rename = "user-wifi1-wifi_tx_attempts")]
    pub user_wifi1_wifi_tx_attempts: f64,
    #[serde(rename = "user-wifi0-wifi_tx_attempts")]
    pub user_wifi0_wifi_tx_attempts: f64,
    #[serde(rename = "user-wifi_tx_attempts")]
    pub user_wifi_tx_attempts: f64,
    #[serde(rename = "guest-wifi_tx_attempts")]
    pub guest_wifi_tx_attempts: f64,
    #[serde(rename = "wifi0-wifi_tx_attempts")]
    pub wifi0_wifi_tx_attempts: f64,
    #[serde(rename = "wifi1-wifi_tx_attempts")]
    pub wifi1_wifi_tx_attempts: f64,
    pub wifi_tx_attempts: f64,
    #[serde(rename = "user-wifi1-wifi_tx_dropped")]
    pub user_wifi1_wifi_tx_dropped: f64,
    #[serde(rename = "user-wifi0-wifi_tx_dropped")]
    pub user_wifi0_wifi_tx_dropped: f64,
    #[serde(rename = "user-wifi_tx_dropped")]
    pub user_wifi_tx_dropped: f64,
    #[serde(rename = "guest-wifi_tx_dropped")]
    pub guest_wifi_tx_dropped: f64,
    #[serde(rename = "wifi0-wifi_tx_dropped")]
    pub wifi0_wifi_tx_dropped: f64,
    #[serde(rename = "wifi1-wifi_tx_dropped")]
    pub wifi1_wifi_tx_dropped: f64,
    pub wifi_tx_dropped: f64,
    #[serde(rename = "na-wifi_tx_attempts")]
    pub na_wifi_tx_attempts: f64,
    #[serde(rename = "ng-wifi_tx_attempts")]
    pub ng_wifi_tx_attempts: f64,
    #[serde(rename = "na-wifi_tx_dropped")]
    pub na_wifi_tx_dropped: f64,
    #[serde(rename = "ng-wifi_tx_dropped")]
    pub ng_wifi_tx_dropped: f64,
    #[serde(rename = "na-tx_retries")]
    pub na_tx_retries: f64,
    #[serde(rename = "ng-tx_retries")]
    pub ng_tx_retries: f64,
    #[serde(rename = "na-tx_packets")]
    pub na_tx_packets: f64,
    #[serde(rename = "ng-tx_packets")]
    pub ng_tx_packets: f64,
    #[serde(rename = "na-tx_bytes")]
    pub na_tx_bytes: f64,
    #[serde(rename = "ng-tx_bytes")]
    pub ng_tx_bytes: f64,
    #[serde(rename = "na-rx_packets")]
    pub na_rx_packets: f64,
    #[serde(rename = "ng-rx_packets")]
    pub ng_rx_packets: f64,
    #[serde(rename = "na-rx_bytes")]
    pub na_rx_bytes: f64,
    #[serde(rename = "ng-rx_bytes")]
    pub ng_rx_bytes: f64,
    pub bytes: f64,
    pub duration: f64,
    #[serde(rename = "user-wifi1-wifi1ap4-66b3d270fa6328019344daa5-rx_packets")]
    pub user_wifi1_wifi1ap4_66b3d270fa6328019344daa5_rx_packets: f64,
    #[serde(rename = "user-wifi1-wifi1ap4-66b3d270fa6328019344daa5-rx_bytes")]
    pub user_wifi1_wifi1ap4_66b3d270fa6328019344daa5_rx_bytes: f64,
    #[serde(rename = "user-wifi1-wifi1ap4-66b3d270fa6328019344daa5-tx_packets")]
    pub user_wifi1_wifi1ap4_66b3d270fa6328019344daa5_tx_packets: f64,
    #[serde(rename = "user-wifi1-wifi1ap4-66b3d270fa6328019344daa5-tx_bytes")]
    pub user_wifi1_wifi1ap4_66b3d270fa6328019344daa5_tx_bytes: f64,
    #[serde(rename = "user-wifi1-wifi1ap4-66b3d270fa6328019344daa5-tx_retries")]
    pub user_wifi1_wifi1ap4_66b3d270fa6328019344daa5_tx_retries: f64,
    #[serde(rename = "user-wifi1-wifi1ap4-66b3d270fa6328019344daa5-wifi_tx_attempts")]
    pub user_wifi1_wifi1ap4_66b3d270fa6328019344daa5_wifi_tx_attempts: f64,
    #[serde(rename = "user-wifi0-wifi0ap2-66b53a83d8b9ed360700bc7f-rx_packets")]
    pub user_wifi0_wifi0ap2_66b53a83d8b9ed360700bc7f_rx_packets: f64,
    #[serde(rename = "user-wifi0-wifi0ap2-66b53a83d8b9ed360700bc7f-rx_bytes")]
    pub user_wifi0_wifi0ap2_66b53a83d8b9ed360700bc7f_rx_bytes: f64,
    #[serde(rename = "user-wifi0-wifi0ap2-66b53a83d8b9ed360700bc7f-tx_packets")]
    pub user_wifi0_wifi0ap2_66b53a83d8b9ed360700bc7f_tx_packets: f64,
    #[serde(rename = "user-wifi0-wifi0ap2-66b53a83d8b9ed360700bc7f-tx_bytes")]
    pub user_wifi0_wifi0ap2_66b53a83d8b9ed360700bc7f_tx_bytes: f64,
    #[serde(rename = "user-wifi0-wifi0ap2-66b53a83d8b9ed360700bc7f-mac_filter_rejections")]
    pub user_wifi0_wifi0ap2_66b53a83d8b9ed360700bc7f_mac_filter_rejections: f64,
    #[serde(rename = "user-wifi0-wifi0ap2-66b53a83d8b9ed360700bc7f-wifi_tx_attempts")]
    pub user_wifi0_wifi0ap2_66b53a83d8b9ed360700bc7f_wifi_tx_attempts: f64,
    #[serde(rename = "user-wifi0-wifi0ap1-66b3d2c2fa6328019344daa8-tx_packets")]
    pub user_wifi0_wifi0ap1_66b3d2c2fa6328019344daa8_tx_packets: f64,
    #[serde(rename = "user-wifi0-wifi0ap1-66b3d2c2fa6328019344daa8-tx_bytes")]
    pub user_wifi0_wifi0ap1_66b3d2c2fa6328019344daa8_tx_bytes: f64,
    #[serde(rename = "user-wifi0-wifi0ap1-66b3d2c2fa6328019344daa8-mac_filter_rejections")]
    pub user_wifi0_wifi0ap1_66b3d2c2fa6328019344daa8_mac_filter_rejections: f64,
    #[serde(rename = "user-wifi0-wifi0ap0-66b3d270fa6328019344daa5-tx_packets")]
    pub user_wifi0_wifi0ap0_66b3d270fa6328019344daa5_tx_packets: f64,
    #[serde(rename = "user-wifi0-wifi0ap0-66b3d270fa6328019344daa5-tx_bytes")]
    pub user_wifi0_wifi0ap0_66b3d270fa6328019344daa5_tx_bytes: f64,
    #[serde(rename = "ng-WIFI_4-rx_bytes")]
    pub ng_wifi_4_rx_bytes: f64,
    #[serde(rename = "ng-WIFI_4-tx_bytes")]
    pub ng_wifi_4_tx_bytes: f64,
    #[serde(rename = "na-WIFI_6-rx_bytes")]
    pub na_wifi_6_rx_bytes: f64,
    #[serde(rename = "na-WIFI_6-tx_bytes")]
    pub na_wifi_6_tx_bytes: f64,
    #[serde(rename = "client-tx_bytes")]
    pub client_tx_bytes: f64,
    #[serde(rename = "client-rx_bytes")]
    pub client_rx_bytes: f64,
    #[serde(rename = "signal_60-duration")]
    pub signal_60_duration: f64,
    #[serde(rename = "signal_70-duration")]
    pub signal_70_duration: f64,
    #[serde(rename = "signal_80-duration")]
    pub signal_80_duration: f64,
    #[serde(rename = "signal_90-duration")]
    pub signal_90_duration: f64,
    #[serde(rename = "na-signal_70-tx_retries")]
    pub na_signal_70_tx_retries: f64,
    #[serde(rename = "na-signal_70-wifi_tx_attempts")]
    pub na_signal_70_wifi_tx_attempts: f64,
    #[serde(rename = "na-signal_80-tx_retries")]
    pub na_signal_80_tx_retries: f64,
    #[serde(rename = "na-signal_80-wifi_tx_attempts")]
    pub na_signal_80_wifi_tx_attempts: f64,
    #[serde(rename = "ng-signal_60-wifi_tx_attempts")]
    pub ng_signal_60_wifi_tx_attempts: f64,
    #[serde(rename = "ng-signal_70-wifi_tx_attempts")]
    pub ng_signal_70_wifi_tx_attempts: f64,
    #[serde(rename = "user-wifi0-wifi0ap0-66b3d270fa6328019344daa5-rx_packets")]
    pub user_wifi0_wifi0ap0_66b3d270fa6328019344daa5_rx_packets: f64,
    #[serde(rename = "user-wifi0-wifi0ap0-66b3d270fa6328019344daa5-rx_bytes")]
    pub user_wifi0_wifi0ap0_66b3d270fa6328019344daa5_rx_bytes: f64,
    #[serde(rename = "user-wifi0-wifi0ap0-66b3d270fa6328019344daa5-wifi_tx_attempts")]
    pub user_wifi0_wifi0ap0_66b3d270fa6328019344daa5_wifi_tx_attempts: f64,
    #[serde(rename = "ng-WIFI_6-rx_bytes")]
    pub ng_wifi_6_rx_bytes: f64,
    #[serde(rename = "ng-WIFI_6-tx_bytes")]
    pub ng_wifi_6_tx_bytes: f64,
    #[serde(rename = "na-signal_90-tx_retries")]
    pub na_signal_90_tx_retries: f64,
    #[serde(rename = "na-signal_90-wifi_tx_attempts")]
    pub na_signal_90_wifi_tx_attempts: f64,
    #[serde(rename = "user-wifi0-wifi0ap2-66b53a83d8b9ed360700bc7f-tx_retries")]
    pub user_wifi0_wifi0ap2_66b53a83d8b9ed360700bc7f_tx_retries: f64,
    #[serde(rename = "user-wifi0-wifi0ap0-66b3d270fa6328019344daa5-tx_retries")]
    pub user_wifi0_wifi0ap0_66b3d270fa6328019344daa5_tx_retries: f64,
    #[serde(rename = "ng-signal_60-tx_retries")]
    pub ng_signal_60_tx_retries: f64,
    #[serde(rename = "ng-signal_70-tx_retries")]
    pub ng_signal_70_tx_retries: f64,
    #[serde(rename = "user-wifi0-wifi0ap1-66b3d2c2fa6328019344daa8-tx_errors")]
    pub user_wifi0_wifi0ap1_66b3d2c2fa6328019344daa8_tx_errors: f64,
    #[serde(rename = "user-wifi0-wifi0ap2-66b53a83d8b9ed360700bc7f-tx_errors")]
    pub user_wifi0_wifi0ap2_66b53a83d8b9ed360700bc7f_tx_errors: f64,
    #[serde(rename = "user-wifi0-wifi0ap2-66b53a83d8b9ed360700bc7f-wifi_tx_dropped")]
    pub user_wifi0_wifi0ap2_66b53a83d8b9ed360700bc7f_wifi_tx_dropped: f64,
    #[serde(rename = "user-wifi0-wifi0ap0-66b3d270fa6328019344daa5-tx_errors")]
    pub user_wifi0_wifi0ap0_66b3d270fa6328019344daa5_tx_errors: f64,
    #[serde(rename = "signal_50-duration")]
    pub signal_50_duration: f64,
    #[serde(rename = "user-wifi0-wifi0ap0-66b3d270fa6328019344daa5-rx_errors")]
    pub user_wifi0_wifi0ap0_66b3d270fa6328019344daa5_rx_errors: f64,
    #[serde(rename = "user-wifi0-wifi0ap0-66b3d270fa6328019344daa5-rx_dropped")]
    pub user_wifi0_wifi0ap0_66b3d270fa6328019344daa5_rx_dropped: f64,
    #[serde(rename = "user-wifi0-wifi0ap0-66b3d270fa6328019344daa5-rx_crypts")]
    pub user_wifi0_wifi0ap0_66b3d270fa6328019344daa5_rx_crypts: f64,
    #[serde(rename = "user-wifi1-wifi1ap4-66b3d270fa6328019344daa5-rx_errors")]
    pub user_wifi1_wifi1ap4_66b3d270fa6328019344daa5_rx_errors: f64,
    #[serde(rename = "user-wifi1-wifi1ap4-66b3d270fa6328019344daa5-rx_dropped")]
    pub user_wifi1_wifi1ap4_66b3d270fa6328019344daa5_rx_dropped: f64,
    #[serde(rename = "user-wifi1-wifi1ap4-66b3d270fa6328019344daa5-rx_crypts")]
    pub user_wifi1_wifi1ap4_66b3d270fa6328019344daa5_rx_crypts: f64,
    #[serde(rename = "ng-signal_80-tx_retries")]
    pub ng_signal_80_tx_retries: f64,
    #[serde(rename = "ng-signal_80-wifi_tx_attempts")]
    pub ng_signal_80_wifi_tx_attempts: f64,
    #[serde(rename = "user-wifi1-wifi1ap4-66b3d270fa6328019344daa5-tx_errors")]
    pub user_wifi1_wifi1ap4_66b3d270fa6328019344daa5_tx_errors: f64,
    #[serde(rename = "ng-signal_50-wifi_tx_attempts")]
    pub ng_signal_50_wifi_tx_attempts: f64,
    #[serde(rename = "user-wifi1-wifi1ap4-66b3d270fa6328019344daa5-wifi_tx_dropped")]
    pub user_wifi1_wifi1ap4_66b3d270fa6328019344daa5_wifi_tx_dropped: f64,
    #[serde(rename = "user-wifi0-wifi0ap0-66b3d270fa6328019344daa5-wifi_tx_dropped")]
    pub user_wifi0_wifi0ap0_66b3d270fa6328019344daa5_wifi_tx_dropped: f64,
    #[serde(rename = "user-wifi0-wifi0ap1-66b3d2c2fa6328019344daa8-rx_packets")]
    pub user_wifi0_wifi0ap1_66b3d2c2fa6328019344daa8_rx_packets: f64,
    #[serde(rename = "user-wifi0-wifi0ap1-66b3d2c2fa6328019344daa8-rx_bytes")]
    pub user_wifi0_wifi0ap1_66b3d2c2fa6328019344daa8_rx_bytes: f64,
    #[serde(rename = "user-wifi0-wifi0ap1-66b3d2c2fa6328019344daa8-tx_retries")]
    pub user_wifi0_wifi0ap1_66b3d2c2fa6328019344daa8_tx_retries: f64,
    #[serde(rename = "user-wifi0-wifi0ap1-66b3d2c2fa6328019344daa8-wifi_tx_attempts")]
    pub user_wifi0_wifi0ap1_66b3d2c2fa6328019344daa8_wifi_tx_attempts: f64,
    #[serde(rename = "na-signal_60-wifi_tx_attempts")]
    pub na_signal_60_wifi_tx_attempts: f64,
    #[serde(rename = "na-signal_60-tx_retries")]
    pub na_signal_60_tx_retries: f64,
    #[serde(rename = "na-WIFI_5-rx_bytes")]
    pub na_wifi_5_rx_bytes: f64,
    #[serde(rename = "na-WIFI_5-tx_bytes")]
    pub na_wifi_5_tx_bytes: f64,
    #[serde(rename = "ng-signal_90-tx_retries")]
    pub ng_signal_90_tx_retries: f64,
    #[serde(rename = "ng-signal_90-wifi_tx_attempts")]
    pub ng_signal_90_wifi_tx_attempts: f64,
    #[serde(rename = "user-wifi0-wifi0ap2-66b53a83d8b9ed360700bc7f-rx_errors")]
    pub user_wifi0_wifi0ap2_66b53a83d8b9ed360700bc7f_rx_errors: f64,
    #[serde(rename = "user-wifi0-wifi0ap2-66b53a83d8b9ed360700bc7f-rx_dropped")]
    pub user_wifi0_wifi0ap2_66b53a83d8b9ed360700bc7f_rx_dropped: f64,
    #[serde(rename = "user-wifi0-wifi0ap2-66b53a83d8b9ed360700bc7f-rx_crypts")]
    pub user_wifi0_wifi0ap2_66b53a83d8b9ed360700bc7f_rx_crypts: f64,
    #[serde(rename = "ng-signal_50-tx_retries")]
    pub ng_signal_50_tx_retries: f64,
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
    #[serde(rename = "port_1-tx_packets")]
    pub port_1_tx_packets: f64,
    #[serde(rename = "port_1-tx_bytes")]
    pub port_1_tx_bytes: f64,
    #[serde(rename = "port_1-rx_packets")]
    pub port_1_rx_packets: f64,
    #[serde(rename = "port_1-rx_bytes")]
    pub port_1_rx_bytes: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Gw {
    pub site_id: String,
    pub o: String,
    pub oid: String,
    pub gw: String,
    pub time: i64,
    pub datetime: String,
    pub duration: f64,
    #[serde(rename = "wan-rx_packets")]
    pub wan_rx_packets: f64,
    #[serde(rename = "wan-rx_bytes")]
    pub wan_rx_bytes: f64,
    #[serde(rename = "wan-tx_packets")]
    pub wan_tx_packets: f64,
    #[serde(rename = "wan-tx_bytes")]
    pub wan_tx_bytes: f64,
    #[serde(rename = "lan-tx_packets")]
    pub lan_tx_packets: f64,
    #[serde(rename = "lan-tx_bytes")]
    pub lan_tx_bytes: f64,
    #[serde(rename = "lan-rx_packets")]
    pub lan_rx_packets: f64,
    #[serde(rename = "lan-rx_bytes")]
    pub lan_rx_bytes: f64,
}

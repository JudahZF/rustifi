use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Root {
    pub meta: Meta,
    pub data: Vec<Daum>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Meta {
    pub rc: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Daum {
    pub hostname: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "_id")]
    pub id: String,
    pub key: String,
    pub data_retention_time_in_hours_for_5minutes_scale: Option<i64>,
    pub data_retention_time_in_hours_for_monthly_scale: Option<i64>,
    pub enable_analytics: Option<bool>,
    pub data_retention_time_in_hours_for_hourly_scale: Option<i64>,
    pub data_retention_time_in_hours_for_daily_scale: Option<i64>,
    pub autobackup_enabled: Option<bool>,
    pub multiple_sites_enabled: Option<bool>,
    pub minimum_usable_hd_space: Option<i64>,
    pub autobackup_max_files: Option<i64>,
    pub data_retention_setting_preference: Option<String>,
    pub time_series_per_client_stats_enabled: Option<bool>,
    pub autobackup_timezone: Option<String>,
    pub data_retention_time_in_hours_for_others: Option<i64>,
    pub discoverable: Option<bool>,
    pub data_retention_time_enabled: Option<bool>,
    pub live_updates: Option<String>,
    pub autobackup_days: Option<i64>,
    pub backup_to_cloud_enabled: Option<bool>,
    pub override_inform_host: Option<bool>,
    pub autobackup_cron_expr: Option<String>,
    pub fingerbank_key: Option<String>,
    pub provider: Option<String>,
    pub uplink_type: Option<String>,
    pub x_mesh_psk: Option<String>,
    pub site_id: Option<String>,
    pub x_mesh_essid: Option<String>,
    pub enabled: Option<bool>,
    pub x_element_essid: Option<String>,
    pub x_element_psk: Option<String>,
    pub portal_customized_text_color: Option<String>,
    pub portal_customized_title: Option<String>,
    pub auth: Option<String>,
    pub redirect_https: Option<bool>,
    pub portal_enabled: Option<bool>,
    pub portal_customized_button_color: Option<String>,
    pub portal_customized: Option<bool>,
    #[serde(default)]
    pub portal_customized_languages: Vec<String>,
    pub portal_customized_button_text: Option<String>,
    pub portal_customized_button_text_color: Option<String>,
    pub portal_customized_welcome_text: Option<String>,
    pub portal_customized_bg_image_enabled: Option<bool>,
    pub portal_customized_success_text: Option<String>,
    pub portal_use_hostname: Option<bool>,
    pub expire_number: Option<i64>,
    pub portal_customized_link_color: Option<String>,
    pub portal_customized_box_radius: Option<i64>,
    pub redirect_enabled: Option<bool>,
    pub portal_customized_logo_size: Option<i64>,
    pub portal_customized_bg_color: Option<String>,
    pub portal_customized_logo_position: Option<String>,
    pub expire_unit: Option<i64>,
    pub restricted_subnet_3: Option<String>,
    pub restricted_subnet_2: Option<String>,
    pub password_enabled: Option<bool>,
    pub restricted_subnet_1: Option<String>,
    pub voucher_enabled: Option<bool>,
    pub portal_customized_box_opacity: Option<i64>,
    pub ec_enabled: Option<bool>,
    pub portal_customized_bg_type: Option<String>,
    pub expire: Option<i64>,
    pub portal_customized_box_text_color: Option<String>,
    pub template_engine: Option<String>,
    pub redirect_to_https: Option<bool>,
    pub portal_customized_box_color: Option<String>,
    pub redirect_url: Option<String>,
    pub portal_customized_authentication_text: Option<String>,
    pub setting_preference: Option<String>,
    pub ntp_server_1: Option<String>,
    pub ntp_server_2: Option<String>,
    pub ntp_server_3: Option<String>,
    pub ntp_server_4: Option<String>,
    pub x_ssh_bind_wildcard: Option<bool>,
    pub wifiman_enabled: Option<bool>,
    pub advanced_feature_enabled: Option<bool>,
    pub x_api_token: Option<String>,
    pub x_ssh_auth_password_enabled: Option<bool>,
    pub x_mgmt_key: Option<String>,
    pub unifi_idp_enabled: Option<bool>,
    pub x_ssh_username: Option<String>,
    #[serde(default)]
    pub x_ssh_keys: Vec<Value>,
    pub debug_tools_enabled: Option<bool>,
    pub auto_upgrade: Option<bool>,
    pub x_ssh_password: Option<String>,
    pub x_ssh_enabled: Option<bool>,
    pub auto_upgrade_hour: Option<i64>,
    #[serde(rename = "fingerprintingEnabled")]
    pub fingerprinting_enabled: Option<bool>,
    pub brightness: Option<i64>,
    pub touch_event: Option<bool>,
    pub idle_timeout: Option<i64>,
    pub sync: Option<bool>,
    pub h323_module: Option<bool>,
    pub send_redirects: Option<bool>,
    pub upnp_enabled: Option<bool>,
    pub broadcast_ping: Option<bool>,
    pub geo_ip_filtering_traffic_direction: Option<String>,
    pub mss_clamp_mss: Option<i64>,
    pub tcp_close_timeout: Option<i64>,
    pub upnp_secure_mode: Option<bool>,
    pub tcp_fin_wait_timeout: Option<i64>,
    pub geo_ip_filtering_block: Option<String>,
    pub geo_ip_filtering_countries: Option<String>,
    pub offload_sch: Option<bool>,
    pub other_timeout: Option<i64>,
    pub arp_cache_timeout: Option<String>,
    pub firewall_wan_default_log: Option<bool>,
    pub icmp_timeout: Option<i64>,
    pub echo_server: Option<String>,
    pub geo_ip_filtering_enabled: Option<bool>,
    pub udp_other_timeout: Option<i64>,
    pub firewall_lan_default_log: Option<bool>,
    pub gre_module: Option<bool>,
    pub arp_cache_base_reachable: Option<i64>,
    pub timeout_setting_preference: Option<String>,
    pub tcp_time_wait_timeout: Option<i64>,
    pub ftp_module: Option<bool>,
    pub firewall_guest_default_log: Option<bool>,
    pub offload_accounting: Option<bool>,
    pub mss_clamp: Option<String>,
    pub udp_stream_timeout: Option<i64>,
    pub receive_redirects: Option<bool>,
    pub syn_cookies: Option<bool>,
    pub tcp_last_ack_timeout: Option<i64>,
    pub upnp_nat_pmp_enabled: Option<bool>,
    pub tftp_module: Option<bool>,
    pub offload_l2_blocking: Option<bool>,
    pub tcp_syn_recv_timeout: Option<i64>,
    pub tcp_close_wait_timeout: Option<i64>,
    pub tcp_syn_sent_timeout: Option<i64>,
    pub tcp_established_timeout: Option<i64>,
    pub pptp_module: Option<bool>,
    pub sip_module: Option<bool>,
    pub this_controller: Option<bool>,
    pub this_controller_encrypted_only: Option<bool>,
    pub stp_version: Option<String>,
    pub flowctrl_enabled: Option<bool>,
    pub dot1x_fallback_networkconf_id: Option<String>,
    pub dot1x_portctrl_enabled: Option<bool>,
    pub dhcp_snoop: Option<bool>,
    pub jumboframe_enabled: Option<bool>,
    pub radiusprofile_id: Option<String>,
    #[serde(default)]
    pub switch_exclusions: Vec<Value>,
    pub subnet_cidr: Option<String>,
    pub public_key: Option<String>,
    pub x_private_key: Option<String>,
    pub auto_adjust_channels_to_country: Option<bool>,
    #[serde(default)]
    pub channels_na: Vec<String>,
    pub ht_modes_na: Option<Vec<String>>,
    #[serde(default)]
    pub ht_modes_ng: Vec<String>,
    #[serde(default)]
    pub channels_ng: Vec<String>,
    pub default: Option<bool>,
    #[serde(default)]
    pub optimize: Vec<String>,
    pub cron_expr: Option<String>,
    pub radios: Option<Vec<String>>,
    pub auto_enabled: Option<bool>,
    #[serde(default)]
    pub exclude_devices: Vec<String>,
    #[serde(default)]
    pub channels_6e: Vec<String>,
    pub utm_token: Option<String>,
    pub ad_blocking_enabled: Option<bool>,
    pub advanced_filtering_preference: Option<String>,
    #[serde(default)]
    pub enabled_categories: Vec<Value>,
    pub dns_filters: Option<Vec<DnsFilter>>,
    pub dns_filtering: Option<bool>,
    pub last_alert_id: Option<String>,
    pub enabled_networks: Option<Vec<String>>,
    pub restrict_ip_addresses: Option<bool>,
    pub ips_mode: Option<String>,
    #[serde(default)]
    pub ad_blocking_configurations: Vec<AdBlockingConfiguration>,
    pub suppression: Option<Suppression>,
    pub restrict_tor: Option<bool>,
    pub endpoint_scanning: Option<bool>,
    pub honeypot_enabled: Option<bool>,
    pub timezone: Option<String>,
    pub code: Option<String>,
    #[serde(default)]
    pub server_names: Vec<String>,
    pub state: Option<String>,
    pub network_defaults: Option<Vec<NetworkDefault>>,
    #[serde(default)]
    pub network_overrides: Vec<Value>,
    pub speed_defaults: Option<Vec<SpeedDefault>>,
    #[serde(default)]
    pub speed_overrides: Vec<Value>,
    pub x_pregenerated_dh_key: Option<String>,
    pub psk: Option<String>,
    pub ssid: Option<String>,
    pub layout_preference: Option<String>,
    pub mode: Option<String>,
    #[serde(default)]
    pub excluded_network_ids: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DnsFilter {
    pub filter: String,
    pub network_id: String,
    pub blocked_tld: Vec<Value>,
    pub blocked_sites: Vec<Value>,
    pub allowed_sites: Vec<Value>,
    pub name: String,
    pub description: String,
    pub version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdBlockingConfiguration {
    pub network_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Suppression {
    pub alerts: Vec<Value>,
    pub whitelist: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkDefault {
    pub raw_color_hex: String,
    pub key: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpeedDefault {
    pub raw_color_hex: String,
    pub key: String,
}

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
    pub setting_preference: String,
    pub dtim_6e: i64,
    pub wpa3_support: bool,
    pub wpa_mode: String,
    pub minrate_na_advertising_rates: bool,
    pub minrate_setting_preference: String,
    pub minrate_ng_advertising_rates: bool,
    pub optimize_iot_wifi_connectivity: bool,
    pub b_supported: bool,
    pub radius_das_enabled: bool,
    pub group_rekey: Option<i64>,
    pub radius_macacl_format: String,
    pub pmf_mode: String,
    pub wpa3_transition: bool,
    pub passphrase_autogenerated: bool,
    pub private_preshared_keys: Vec<Value>,
    pub bc_filter_enabled: bool,
    pub mcastenhance_enabled: bool,
    pub usergroup_id: String,
    pub proxy_arp: bool,
    pub sae_sync: Option<i64>,
    pub uapsd_enabled: bool,
    pub iapp_enabled: bool,
    pub name: String,
    pub site_id: String,
    pub hide_ssid: bool,
    pub wlan_band: String,
    #[serde(rename = "_id")]
    pub id: String,
    pub private_preshared_keys_enabled: bool,
    pub no2ghz_oui: bool,
    pub x_iapp_key: String,
    pub networkconf_id: String,
    pub dtim_na: i64,
    pub is_guest: bool,
    pub minrate_na_enabled: bool,
    pub sae_groups: Vec<Value>,
    pub enabled: bool,
    pub sae_psk: Vec<Value>,
    pub wlan_bands: Vec<String>,
    pub mac_filter_policy: String,
    pub security: String,
    pub minrate_ng_enabled: bool,
    pub ap_group_ids: Vec<String>,
    pub l2_isolation: bool,
    pub schedule_with_duration: Vec<Value>,
    pub bss_transition: bool,
    pub minrate_ng_data_rate_kbps: i64,
    pub radius_mac_auth_enabled: bool,
    pub wpa3_fast_roaming: bool,
    pub ap_group_mode: String,
    pub wpa3_enhanced_192: bool,
    pub fast_roaming_enabled: bool,
    pub wpa_enc: String,
    pub x_passphrase: Option<String>,
    pub mac_filter_list: Vec<String>,
    pub dtim_mode: String,
    pub schedule: Vec<Value>,
    pub bc_filter_list: Vec<Value>,
    pub minrate_na_data_rate_kbps: i64,
    pub mac_filter_enabled: bool,
    pub sae_anti_clogging: Option<i64>,
    pub dtim_ng: i64,
    pub radiusprofile_id: Option<String>,
}

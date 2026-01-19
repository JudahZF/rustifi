use serde::Deserialize;

/// Hotspot voucher.
#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Voucher {
    pub id: String,
    pub code: String,
    #[serde(default)]
    pub used: bool,
    #[serde(default)]
    pub duration_minutes: Option<u32>,
    #[serde(default)]
    pub data_limit_mb: Option<u64>,
    #[serde(default)]
    pub bandwidth_limit_down: Option<u64>,
    #[serde(default)]
    pub bandwidth_limit_up: Option<u64>,
    #[serde(default)]
    pub note: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub used_at: Option<String>,
}

use serde::Deserialize;

/// Hotspot voucher for guest network access.
#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Voucher {
    /// Unique voucher identifier.
    pub id: String,

    /// Human-readable voucher code for guest entry.
    pub code: String,

    /// Whether the voucher has been redeemed.
    #[serde(default)]
    pub used: bool,

    /// Access duration in minutes once activated.
    #[serde(default)]
    pub duration_minutes: Option<u32>,

    /// Data usage limit in megabytes.
    #[serde(default)]
    pub data_limit_mb: Option<u64>,

    /// Download bandwidth limit in kbps.
    #[serde(default)]
    pub bandwidth_limit_down: Option<u64>,

    /// Upload bandwidth limit in kbps.
    #[serde(default)]
    pub bandwidth_limit_up: Option<u64>,

    /// Optional admin note for the voucher.
    #[serde(default)]
    pub note: Option<String>,

    /// ISO 8601 timestamp when the voucher was created.
    #[serde(default)]
    pub created_at: Option<String>,

    /// ISO 8601 timestamp when the voucher was redeemed.
    #[serde(default)]
    pub used_at: Option<String>,
}

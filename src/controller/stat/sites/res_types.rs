use serde::{Deserialize, Serialize};

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
    pub external_id: String,
    #[serde(rename = "_id")]
    pub id: String,
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
    pub num_sw: Option<i64>,
}

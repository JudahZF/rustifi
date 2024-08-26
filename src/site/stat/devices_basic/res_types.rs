use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeviceListResponse {
    pub meta: Meta,
    pub data: Vec<Devices>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Meta {
    pub rc: String,
    pub msg: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Devices {
    pub mac: String,
    pub state: i64,
    pub adopted: bool,
    pub disabled: bool,
    #[serde(rename = "type")]
    pub type_field: String,
    pub model: String,
    pub in_gateway_mode: bool,
    pub name: String,
}

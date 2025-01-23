use crate::responses::stat::devices::ConfigNetwork;

#[derive(Debug, Clone)]
pub struct NetConfig {
    pub type_str: String,
    pub bonding_enabled: Option<bool>,
}

impl NetConfig {
    pub fn from_raw(raw: ConfigNetwork) -> NetConfig {
        NetConfig {
            type_str: raw.type_field,
            bonding_enabled: raw.bonding_enabled,
        }
    }
}

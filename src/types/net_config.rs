use crate::responses::stat::devices::ConfigNetwork;
use std::fmt::Display;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct NetConfig {
    pub type_str: String,
    pub bonding_enabled: Option<bool>,
}

impl From<ConfigNetwork> for NetConfig {
    fn from(raw: ConfigNetwork) -> Self {
        NetConfig {
            type_str: raw.type_field,
            bonding_enabled: raw.bonding_enabled,
        }
    }
}

impl Display for NetConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Type: {}", self.type_str)
    }
}

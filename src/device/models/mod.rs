pub mod access_points;

use std::fmt::Display;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub enum DeviceType {
    Switch,
    #[default]
    AccessPoint,
    DreamMachine,
}

impl std::fmt::Display for DeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeviceType::Switch => write!(f, "Switch"),
            DeviceType::AccessPoint => write!(f, "Access Point"),
            DeviceType::DreamMachine => write!(f, "Dream Machine"),
        }
    }
}

impl From<&str> for DeviceType {
    fn from(s: &str) -> Self {
        match s {
            "usw" => DeviceType::Switch,
            "uap" => DeviceType::AccessPoint,
            "udm" => DeviceType::DreamMachine,
            _ => panic!("Unknown device DeviceType: {}", s),
        }
    }
}

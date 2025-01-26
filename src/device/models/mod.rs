use std::fmt::Display;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub enum Type {
    Switch,
    #[default]
    AccessPoint,
    DreamMachine,
}

impl From<&str> for Type {
    fn from(s: &str) -> Self {
        match s {
            "usw" => Type::Switch,
            "uap" => Type::AccessPoint,
            "udm" => Type::DreamMachine,
            _ => panic!("Unknown device type: {}", s),
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Switch => write!(f, "Switch"),
            Type::AccessPoint => write!(f, "Access Point"),
            Type::DreamMachine => write!(f, "Dream Machine"),
        }
    }
}

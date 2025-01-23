#[derive(Debug, Clone)]
pub enum Type {
    Switch,
    AccessPoint,
    DreamMachine,
}

impl Type {
    pub fn from_str(s: &str) -> Type {
        match s {
            "usw" => Type::Switch,
            "uap" => Type::AccessPoint,
            "udm" => Type::DreamMachine,
            _ => panic!("Unknown device type: {}", s),
        }
    }
}

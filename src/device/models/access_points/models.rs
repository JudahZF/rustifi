use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, PartialOrd, Default)]
pub enum APModel {
    BasestationXG,
    #[default]
    Unknown,
}

impl From<String> for APModel {
    fn from(s: String) -> Self {
        match s.as_str() {
            "UXSDM" => APModel::BasestationXG,
            _ => APModel::Unknown,
        }
    }
}

impl Display for APModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            APModel::BasestationXG => write!(f, "Basestation XG"),
            APModel::Unknown => write!(f, "Unknown"),
        }
    }
}

use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Default)]
pub enum APModel {
    ACMesh,
    ACMeshPro,
    ACPro,
    BasestationXG,
    E7Campus,
    NanoHD,
    U6Mesh,
    U7ProXGS,
    U7ProOutdoor,
    #[default]
    Unknown,
    XG,
}

impl From<&str> for APModel {
    fn from(s: &str) -> Self {
        match s {
            "UXSDM" => APModel::BasestationXG,
            "U7MSH" => APModel::ACMesh,
            "U6M" => APModel::U6Mesh,
            "U7MP" => APModel::ACMeshPro,
            "UCXG" => APModel::XG,
            "U7PG2" => APModel::ACPro,
            "UAPA6A4" => APModel::U7ProXGS,
            "UAPA6B0" => APModel::U7ProOutdoor,
            "UAPA6B1" => APModel::E7Campus,
            "U7NHD" => APModel::NanoHD,
            _ => {
                eprintln!("Unknown model: {}", s);
                APModel::Unknown
            }
        }
    }
}

impl From<String> for APModel {
    fn from(s: String) -> Self {
        APModel::from(s.as_str())
    }
}

impl Display for APModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            APModel::BasestationXG => write!(f, "Basestation XG"),
            APModel::U6Mesh => write!(f, "U6 Mesh"),
            APModel::ACMesh => write!(f, "ACMesh"),
            APModel::ACMeshPro => write!(f, "ACMesh Pro"),
            APModel::ACPro => write!(f, "AC Pro"),
            APModel::XG => write!(f, "XG"),
            APModel::E7Campus => write!(f, "E7 Campus"),
            APModel::U7ProXGS => write!(f, "U7 Pro XGS"),
            APModel::U7ProOutdoor => write!(f, "U7 Pro Outdoor"),
            APModel::NanoHD => write!(f, "Nano HD"),
            APModel::Unknown => write!(f, "Unknown"),
        }
    }
}

use std::fmt::Display;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub enum Media {
    GigabitEthernet,
    #[default]
    Unknown,
}

impl From<String> for Media {
    fn from(s: String) -> Self {
        match s.as_str() {
            "GbE" => Media::GigabitEthernet,
            _ => Media::Unknown,
        }
    }
}

impl Display for Media {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Media::GigabitEthernet => write!(f, "Gigabit Ethernet"),
            Media::Unknown => write!(f, "Unknown"),
        }
    }
}

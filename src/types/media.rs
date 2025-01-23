#[derive(Debug, Clone)]
pub enum Media {
    GigabitEthernet,
    Unknown,
}

impl Media {
    pub fn from_string(s: String) -> Media {
        match s.as_str() {
            "GbE" => Media::GigabitEthernet,
            _ => Media::Unknown,
        }
    }
}

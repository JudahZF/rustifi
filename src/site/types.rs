use reqwest::{header::HeaderValue, Client};

#[derive(Debug, Clone)]
pub struct Site {
    pub addr: String,
    pub name: String,
    pub id: String,
    pub client: Option<Client>,
    pub cookies: Option<HeaderValue>,
}

#[derive(Debug, Clone)]
pub struct Device {
    id: String,
    mac: String,
    model: String,
    pub name: String
}

impl Device {
    pub fn new(
        id: String,
        mac: String,
        model: String,
        name: String,
    ) -> Self {
        Device {
            id,
            mac,
            model,
            name,
        }
    }
}

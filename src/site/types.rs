use crate::site::stats_type::DeviceStats;
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
    mac: String,
    state: i64,
    type_field: String,
    model: String,
    in_gateway_mode: bool,
    pub name: String,
    stats: Option<DeviceStats>,
}

impl Device {
    pub fn new(
        mac: String,
        state: i64,
        type_field: String,
        model: String,
        in_gateway_mode: bool,
        name: String,
    ) -> Self {
        Device {
            mac,
            state,
            type_field,
            model,
            in_gateway_mode,
            name,
            stats: None,
        }
    }
}

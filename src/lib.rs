pub mod api;
pub mod client;
pub mod error;
pub mod models;
pub mod pagination;
pub mod response;
pub mod stats;
pub mod wrappers;

pub use client::UnifiClient;
pub use error::{Error, Result};
pub use pagination::DEFAULT_PAGE_SIZE;
pub use stats::{DeviceClientStats, aggregate_clients_by_device, get_device_client_stats};
pub use wrappers::DeviceWithInfo;

pub mod prelude {
    pub use crate::api::Endpoint;
    pub use crate::api::networks::Network;
    pub use crate::client::UnifiClient;
    pub use crate::error::{Error, Result};
    pub use crate::models::{Client, Device, DeviceType, Site};
    pub use crate::stats::DeviceClientStats;
    pub use crate::wrappers::DeviceWithInfo;
}

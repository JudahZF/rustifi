pub mod api;
pub mod client;
pub mod error;
pub mod models;
pub mod pagination;
pub mod response;
pub mod stats;
pub mod wrappers;

pub use client::{UnifiClient, REMOTE_API_URL};
pub use error::{Error, Result};
pub use pagination::DEFAULT_PAGE_SIZE;
pub use stats::{aggregate_clients_by_device, get_device_client_stats, DeviceClientStats};
pub use wrappers::DeviceWithInfo;

pub mod prelude {
    pub use crate::api::networks::{Network, NetworkRequest};
    pub use crate::api::Endpoint;
    pub use crate::client::UnifiClient;
    pub use crate::error::{Error, Result};
    pub use crate::models::{
        APModel, Client, Device, DeviceType, FirewallAction, FirewallPolicy, FirewallZone, Site,
        Voucher, WifiBroadcast, WifiSecurity,
    };
    pub use crate::stats::DeviceClientStats;
    pub use crate::wrappers::DeviceWithInfo;
}

pub mod api;
pub mod client;
pub mod error;
pub mod models;
pub mod response;

pub use client::UnifiClient;
pub use error::{Error, Result};

pub mod prelude {
    pub use crate::api::Endpoint;
    pub use crate::api::networks::Network;
    pub use crate::client::UnifiClient;
    pub use crate::error::{Error, Result};
    pub use crate::models::{Client, Device, DeviceType, Site};
}

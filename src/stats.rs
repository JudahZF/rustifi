//! Statistics and aggregation utilities for UniFi data.
//!
//! This module provides functions to aggregate client data by device,
//! making it easy to get per-device client counts.
//!
//! # Example
//!
//! ```no_run
//! use rustifi::UnifiClient;
//!
//! # async fn example() -> rustifi::Result<()> {
//! let client = UnifiClient::with_api_key("https://unifi.example.com", "api-key")?;
//!
//! // Fetch all clients and aggregate by device using the convenience method
//! let stats = client.fetch_client_stats_by_device("site-id").await?;
//!
//! for (device_id, device_stats) in &stats {
//!     println!("Device {}: {} total clients ({} guests)",
//!         device_id,
//!         device_stats.total_clients,
//!         device_stats.guest_clients
//!     );
//! }
//! # Ok(())
//! # }
//! ```

use crate::models::{Client, ClientType};
use std::collections::HashMap;

/// Statistics about clients connected to a specific device.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DeviceClientStats {
    /// The device ID these statistics belong to.
    pub device_id: String,

    /// Total number of connected clients.
    pub total_clients: usize,

    /// Number of wired clients.
    pub wired_clients: usize,

    /// Number of wireless clients.
    pub wireless_clients: usize,

    /// Number of guest clients.
    pub guest_clients: usize,
}

impl DeviceClientStats {
    /// Create new empty statistics for a device.
    pub fn new(device_id: impl Into<String>) -> Self {
        Self {
            device_id: device_id.into(),
            ..Default::default()
        }
    }

    /// Add a client to the statistics.
    ///
    /// This increments the appropriate counters based on the client's type
    /// and access settings.
    pub fn add_client(&mut self, client: &Client) {
        self.total_clients += 1;

        match client.client_type {
            ClientType::Wired => self.wired_clients += 1,
            ClientType::Wireless => self.wireless_clients += 1,
            ClientType::Unknown => {}
        }

        if client.is_guest() {
            self.guest_clients += 1;
        }
    }

    /// Check if there are any clients connected.
    pub fn has_clients(&self) -> bool {
        self.total_clients > 0
    }

    /// Get the number of non-guest clients.
    pub fn non_guest_clients(&self) -> usize {
        self.total_clients.saturating_sub(self.guest_clients)
    }
}

/// Aggregate client counts by device from a list of clients.
///
/// Returns a HashMap where keys are device IDs and values are client statistics.
/// Clients without an `uplink_device_id` are ignored.
///
/// # Example
///
/// ```
/// use rustifi::models::{Client, ClientType, AccessType, ClientAccess};
/// use rustifi::stats::aggregate_clients_by_device;
///
/// // Create some test clients
/// let clients = vec![
///     // Two clients on device-1
///     Client {
///         id: "c1".to_string(),
///         client_type: ClientType::Wireless,
///         uplink_device_id: Some("device-1".to_string()),
///         ..Default::default()
///     },
///     Client {
///         id: "c2".to_string(),
///         client_type: ClientType::Wired,
///         uplink_device_id: Some("device-1".to_string()),
///         ..Default::default()
///     },
///     // One client on device-2
///     Client {
///         id: "c3".to_string(),
///         client_type: ClientType::Wireless,
///         uplink_device_id: Some("device-2".to_string()),
///         ..Default::default()
///     },
/// ];
///
/// let stats = aggregate_clients_by_device(&clients);
///
/// assert_eq!(stats.get("device-1").unwrap().total_clients, 2);
/// assert_eq!(stats.get("device-2").unwrap().total_clients, 1);
/// ```
pub fn aggregate_clients_by_device(clients: &[Client]) -> HashMap<String, DeviceClientStats> {
    let mut stats: HashMap<String, DeviceClientStats> = HashMap::new();

    for client in clients {
        if let Some(device_id) = &client.uplink_device_id {
            stats
                .entry(device_id.clone())
                .or_insert_with(|| DeviceClientStats::new(device_id))
                .add_client(client);
        }
    }

    stats
}

/// Get client statistics for a specific device from a list of clients.
///
/// This is useful when you only need stats for one device and don't want
/// to build a full HashMap.
///
/// # Example
///
/// ```
/// use rustifi::models::{Client, ClientType};
/// use rustifi::stats::get_device_client_stats;
///
/// let clients = vec![
///     Client {
///         id: "c1".to_string(),
///         client_type: ClientType::Wireless,
///         uplink_device_id: Some("device-1".to_string()),
///         ..Default::default()
///     },
///     Client {
///         id: "c2".to_string(),
///         client_type: ClientType::Wireless,
///         uplink_device_id: Some("device-2".to_string()),
///         ..Default::default()
///     },
/// ];
///
/// let stats = get_device_client_stats(&clients, "device-1");
/// assert_eq!(stats.total_clients, 1);
/// ```
pub fn get_device_client_stats(clients: &[Client], device_id: &str) -> DeviceClientStats {
    let mut stats = DeviceClientStats::new(device_id);

    for client in clients {
        if client.uplink_device_id.as_deref() == Some(device_id) {
            stats.add_client(client);
        }
    }

    stats
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{AccessType, ClientAccess};

    fn make_client(
        id: &str,
        client_type: ClientType,
        device_id: Option<&str>,
        is_guest: bool,
    ) -> Client {
        Client {
            id: id.to_string(),
            client_type,
            name: None,
            connected_at: None,
            ip_address: None,
            access: if is_guest {
                Some(ClientAccess {
                    access_type: AccessType::Guest,
                })
            } else {
                None
            },
            uplink_device_id: device_id.map(String::from),
        }
    }

    #[test]
    fn test_device_client_stats_new() {
        let stats = DeviceClientStats::new("device-123");
        assert_eq!(stats.device_id, "device-123");
        assert_eq!(stats.total_clients, 0);
        assert_eq!(stats.wired_clients, 0);
        assert_eq!(stats.wireless_clients, 0);
        assert_eq!(stats.guest_clients, 0);
    }

    #[test]
    fn test_device_client_stats_add_client() {
        let mut stats = DeviceClientStats::new("device-1");

        // Add a wireless client
        stats.add_client(&make_client(
            "c1",
            ClientType::Wireless,
            Some("device-1"),
            false,
        ));
        assert_eq!(stats.total_clients, 1);
        assert_eq!(stats.wireless_clients, 1);
        assert_eq!(stats.wired_clients, 0);
        assert_eq!(stats.guest_clients, 0);

        // Add a wired client
        stats.add_client(&make_client(
            "c2",
            ClientType::Wired,
            Some("device-1"),
            false,
        ));
        assert_eq!(stats.total_clients, 2);
        assert_eq!(stats.wireless_clients, 1);
        assert_eq!(stats.wired_clients, 1);

        // Add a guest wireless client
        stats.add_client(&make_client(
            "c3",
            ClientType::Wireless,
            Some("device-1"),
            true,
        ));
        assert_eq!(stats.total_clients, 3);
        assert_eq!(stats.wireless_clients, 2);
        assert_eq!(stats.guest_clients, 1);
    }

    #[test]
    fn test_device_client_stats_non_guest_clients() {
        let mut stats = DeviceClientStats::new("device-1");
        stats.total_clients = 10;
        stats.guest_clients = 3;

        assert_eq!(stats.non_guest_clients(), 7);
    }

    #[test]
    fn test_aggregate_clients_by_device() {
        let clients = vec![
            make_client("c1", ClientType::Wireless, Some("device-1"), false),
            make_client("c2", ClientType::Wired, Some("device-1"), false),
            make_client("c3", ClientType::Wireless, Some("device-1"), true),
            make_client("c4", ClientType::Wireless, Some("device-2"), false),
            make_client("c5", ClientType::Wireless, Some("device-2"), true),
            // Client without device ID should be ignored
            make_client("c6", ClientType::Wireless, None, false),
        ];

        let stats = aggregate_clients_by_device(&clients);

        assert_eq!(stats.len(), 2);

        let device1_stats = stats.get("device-1").unwrap();
        assert_eq!(device1_stats.total_clients, 3);
        assert_eq!(device1_stats.wireless_clients, 2);
        assert_eq!(device1_stats.wired_clients, 1);
        assert_eq!(device1_stats.guest_clients, 1);

        let device2_stats = stats.get("device-2").unwrap();
        assert_eq!(device2_stats.total_clients, 2);
        assert_eq!(device2_stats.wireless_clients, 2);
        assert_eq!(device2_stats.wired_clients, 0);
        assert_eq!(device2_stats.guest_clients, 1);
    }

    #[test]
    fn test_aggregate_clients_empty_list() {
        let clients: Vec<Client> = vec![];
        let stats = aggregate_clients_by_device(&clients);
        assert!(stats.is_empty());
    }

    #[test]
    fn test_aggregate_clients_no_device_ids() {
        let clients = vec![
            make_client("c1", ClientType::Wireless, None, false),
            make_client("c2", ClientType::Wired, None, false),
        ];

        let stats = aggregate_clients_by_device(&clients);
        assert!(stats.is_empty());
    }

    #[test]
    fn test_get_device_client_stats() {
        let clients = vec![
            make_client("c1", ClientType::Wireless, Some("device-1"), false),
            make_client("c2", ClientType::Wired, Some("device-1"), false),
            make_client("c3", ClientType::Wireless, Some("device-2"), false),
        ];

        let stats = get_device_client_stats(&clients, "device-1");
        assert_eq!(stats.device_id, "device-1");
        assert_eq!(stats.total_clients, 2);
        assert_eq!(stats.wireless_clients, 1);
        assert_eq!(stats.wired_clients, 1);
    }

    #[test]
    fn test_get_device_client_stats_no_matches() {
        let clients = vec![make_client(
            "c1",
            ClientType::Wireless,
            Some("device-1"),
            false,
        )];

        let stats = get_device_client_stats(&clients, "device-999");
        assert_eq!(stats.device_id, "device-999");
        assert_eq!(stats.total_clients, 0);
        assert!(!stats.has_clients());
    }

    #[test]
    fn test_has_clients() {
        let mut stats = DeviceClientStats::new("device-1");
        assert!(!stats.has_clients());

        stats.total_clients = 1;
        assert!(stats.has_clients());
    }
}

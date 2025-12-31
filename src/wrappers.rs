//! Higher-level wrapper types that combine related API data.
//!
//! This module provides convenience types that fetch and combine
//! multiple pieces of related data in a single call.
//!
//! # Example
//!
//! ```no_run
//! use rustifi::UnifiClient;
//!
//! # async fn example() -> rustifi::Result<()> {
//! let client = UnifiClient::with_api_key("https://unifi.example.com", "api-key")?;
//!
//! // Fetch a device with all its details and statistics in parallel
//! let device_info = client.fetch_device_with_info("site-id", "device-id").await?;
//!
//! println!("Device: {} ({})", device_info.name(), device_info.id());
//! println!("Online: {}", device_info.is_online());
//! println!("CPU: {:?}%", device_info.statistics.cpu_utilization_pct);
//! println!("Ports: {}", device_info.details.port_count());
//! # Ok(())
//! # }
//! ```

use crate::UnifiClient;
use crate::api::devices::{GetDevice, GetDeviceDetails, GetDeviceStatistics};
use crate::error::{Error, Result};
use crate::models::{DeviceDetails, DeviceStatistics, SiteDevice};
use crate::stats::{DeviceClientStats, aggregate_clients_by_device};
use std::collections::HashMap;

/// A device with its detailed information and statistics fetched together.
///
/// This wrapper combines three pieces of information that are often needed together:
/// - Basic device information (`SiteDevice`)
/// - Detailed device information (`DeviceDetails`) - ports, radios, features
/// - Latest statistics (`DeviceStatistics`) - CPU, memory, uplink rates
#[derive(Debug, Clone)]
pub struct DeviceWithInfo {
    /// Basic device information from the devices list endpoint.
    pub device: SiteDevice,

    /// Detailed device information including physical interfaces.
    pub details: DeviceDetails,

    /// Latest device statistics including CPU, memory, and uplink rates.
    pub statistics: DeviceStatistics,
}

impl DeviceWithInfo {
    /// Create a new DeviceWithInfo from its components.
    pub fn new(device: SiteDevice, details: DeviceDetails, statistics: DeviceStatistics) -> Self {
        Self {
            device,
            details,
            statistics,
        }
    }

    /// Get the device ID.
    pub fn id(&self) -> &str {
        &self.device.id
    }

    /// Get the device name.
    pub fn name(&self) -> &str {
        &self.device.name
    }

    /// Get the device model.
    pub fn model(&self) -> &str {
        &self.device.model
    }

    /// Check if the device is online.
    pub fn is_online(&self) -> bool {
        self.device.is_online()
    }

    /// Check if the device is an access point.
    pub fn is_access_point(&self) -> bool {
        self.device.is_access_point()
    }

    /// Check if the device is a gateway.
    pub fn is_gateway(&self) -> bool {
        self.device.is_gateway()
    }

    /// Check if the device supports switching.
    pub fn has_switching(&self) -> bool {
        self.device.has_switching()
    }

    /// Get the number of physical ports.
    pub fn port_count(&self) -> usize {
        self.details.port_count()
    }

    /// Get the number of radios.
    pub fn radio_count(&self) -> usize {
        self.details.radio_count()
    }

    /// Get the device uptime in seconds.
    pub fn uptime_sec(&self) -> u64 {
        self.statistics.uptime_sec
    }

    /// Get the formatted uptime string.
    pub fn uptime_formatted(&self) -> String {
        self.statistics.uptime_formatted()
    }

    /// Get CPU utilization percentage if available.
    pub fn cpu_utilization(&self) -> Option<f64> {
        self.statistics.cpu_utilization_pct
    }

    /// Get memory utilization percentage if available.
    pub fn memory_utilization(&self) -> Option<f64> {
        self.statistics.memory_utilization_pct
    }
}

/// Extension methods for UnifiClient to fetch combined device information.
impl UnifiClient {
    /// Fetch a device with its details and statistics in parallel.
    ///
    /// This method makes three API calls in parallel:
    /// - Get the device basic info
    /// - Get the device details (ports, radios, features)
    /// - Get the device statistics (CPU, memory, uplink rates)
    ///
    /// # Arguments
    ///
    /// * `site_id` - The site ID
    /// * `device_id` - The device ID
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use rustifi::UnifiClient;
    /// # async fn example() -> rustifi::Result<()> {
    /// let client = UnifiClient::with_api_key("https://unifi.example.com", "api-key")?;
    ///
    /// let device = client.fetch_device_with_info("site-id", "device-id").await?;
    ///
    /// println!("Device: {}", device.name());
    /// println!("Uptime: {}", device.uptime_formatted());
    /// println!("CPU: {:?}%", device.cpu_utilization());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn fetch_device_with_info(
        &self,
        site_id: &str,
        device_id: &str,
    ) -> Result<DeviceWithInfo> {
        // Create endpoints before the join to ensure they live long enough
        let device_endpoint = GetDevice::new(site_id, device_id);
        let details_endpoint = GetDeviceDetails::new(site_id, device_id);
        let stats_endpoint = GetDeviceStatistics::new(site_id, device_id);

        // Fetch all three in parallel
        let (device_result, details_result, stats_result) = tokio::join!(
            self.execute(&device_endpoint),
            self.execute(&details_endpoint),
            self.execute(&stats_endpoint),
        );

        let device_response = device_result?;
        let device = device_response
            .data
            .into_iter()
            .next()
            .ok_or_else(|| Error::NotFound(format!("Device {} not found", device_id)))?;
        let details = details_result?;
        let statistics = stats_result?;

        Ok(DeviceWithInfo::new(device, details, statistics))
    }

    /// Fetch all devices with their details and statistics.
    ///
    /// This method first fetches all devices, then fetches details and statistics
    /// for each device in parallel. This is more efficient than making sequential
    /// calls for each device.
    ///
    /// # Arguments
    ///
    /// * `site_id` - The site ID
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use rustifi::UnifiClient;
    /// # async fn example() -> rustifi::Result<()> {
    /// let client = UnifiClient::with_api_key("https://unifi.example.com", "api-key")?;
    ///
    /// let devices = client.fetch_all_devices_with_info("site-id").await?;
    ///
    /// for device in devices {
    ///     println!("{}: {} ({})",
    ///         device.name(),
    ///         if device.is_online() { "online" } else { "offline" },
    ///         device.uptime_formatted()
    ///     );
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn fetch_all_devices_with_info(&self, site_id: &str) -> Result<Vec<DeviceWithInfo>> {
        // First fetch all devices
        let devices = self.fetch_all_devices(site_id).await?;

        // Then fetch details and stats for each device in parallel
        let futures: Vec<_> = devices
            .into_iter()
            .map(|device| {
                let site_id = site_id.to_string();
                let device_id = device.id.clone();
                async move {
                    // Create endpoints before the join to ensure they live long enough
                    let details_endpoint = GetDeviceDetails::new(&site_id, &device_id);
                    let stats_endpoint = GetDeviceStatistics::new(&site_id, &device_id);

                    let (details_result, stats_result) = tokio::join!(
                        self.execute(&details_endpoint),
                        self.execute(&stats_endpoint),
                    );

                    let details = details_result?;
                    let statistics = stats_result?;

                    Ok(DeviceWithInfo::new(device, details, statistics))
                }
            })
            .collect();

        futures::future::try_join_all(futures).await
    }

    /// Fetch all clients and aggregate statistics by device.
    ///
    /// This fetches all clients (handling pagination) and returns
    /// a HashMap of device_id -> DeviceClientStats.
    ///
    /// # Arguments
    ///
    /// * `site_id` - The site ID
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use rustifi::UnifiClient;
    /// # async fn example() -> rustifi::Result<()> {
    /// let client = UnifiClient::with_api_key("https://unifi.example.com", "api-key")?;
    ///
    /// let stats = client.fetch_client_stats_by_device("site-id").await?;
    ///
    /// for (device_id, device_stats) in &stats {
    ///     println!("Device {}: {} clients ({} guests)",
    ///         device_id,
    ///         device_stats.total_clients,
    ///         device_stats.guest_clients
    ///     );
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn fetch_client_stats_by_device(
        &self,
        site_id: &str,
    ) -> Result<HashMap<String, DeviceClientStats>> {
        let clients = self.fetch_all_clients(site_id).await?;
        Ok(aggregate_clients_by_device(&clients))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::common::MacAddress;
    use crate::models::device_details::{DeviceFeatures, PhysicalInterfaces};
    use crate::models::{DeviceFeature, DeviceInterface, DeviceState};

    fn make_site_device() -> SiteDevice {
        SiteDevice {
            id: "device-123".to_string(),
            mac_address: MacAddress::default(),
            ip_address: None,
            name: "Test Device".to_string(),
            model: "U6-Pro".to_string(),
            state: DeviceState::Online,
            supported: Some(true),
            firmware_version: Some("6.0.0".to_string()),
            firmware_updatable: Some(false),
            features: vec![DeviceFeature::AccessPoint],
            interfaces: vec![DeviceInterface::Radios],
        }
    }

    fn make_device_details() -> DeviceDetails {
        DeviceDetails {
            id: "device-123".to_string(),
            mac_address: MacAddress::default(),
            ip_address: crate::models::common::IpAddress::default(),
            name: "Test Device".to_string(),
            model: "U6-Pro".to_string(),
            supported: true,
            state: "ONLINE".to_string(),
            firmware_version: Some("6.0.0".to_string()),
            firmware_updatable: false,
            adopted_at: None,
            provisioned_at: None,
            configuration_id: "config-123".to_string(),
            uplink: None,
            features: DeviceFeatures::default(),
            interfaces: PhysicalInterfaces::default(),
        }
    }

    fn make_device_statistics() -> DeviceStatistics {
        DeviceStatistics {
            uptime_sec: 86400,
            last_heartbeat_at: None,
            next_heartbeat_at: None,
            load_average_1_min: Some(0.5),
            load_average_5_min: Some(0.4),
            load_average_15_min: Some(0.3),
            cpu_utilization_pct: Some(25.0),
            memory_utilization_pct: Some(50.0),
            uplink: None,
            interfaces: None,
        }
    }

    #[test]
    fn test_device_with_info_accessors() {
        let device = make_site_device();
        let details = make_device_details();
        let statistics = make_device_statistics();

        let info = DeviceWithInfo::new(device, details, statistics);

        assert_eq!(info.id(), "device-123");
        assert_eq!(info.name(), "Test Device");
        assert_eq!(info.model(), "U6-Pro");
        assert!(info.is_online());
        assert!(info.is_access_point());
        assert!(!info.is_gateway());
        assert!(!info.has_switching());
        assert_eq!(info.uptime_sec(), 86400);
        assert_eq!(info.cpu_utilization(), Some(25.0));
        assert_eq!(info.memory_utilization(), Some(50.0));
    }

    #[test]
    fn test_device_with_info_uptime_formatted() {
        let device = make_site_device();
        let details = make_device_details();
        let mut statistics = make_device_statistics();
        statistics.uptime_sec = 90061; // 1 day, 1 hour, 1 minute, 1 second

        let info = DeviceWithInfo::new(device, details, statistics);

        assert_eq!(info.uptime_formatted(), "1d 1h 1m 1s");
    }
}

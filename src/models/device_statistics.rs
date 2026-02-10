use serde::Deserialize;

/// Statistics for a device, retrieved from the latest statistics endpoint.
/// Endpoint: GET /v1/sites/{site_id}/devices/{device_id}/statistics/latest
#[derive(Debug, Clone, PartialEq, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceStatistics {
    /// Device uptime in seconds.
    #[serde(default)]
    pub uptime_sec: u64,

    /// Timestamp of the last heartbeat from the device.
    #[serde(default)]
    pub last_heartbeat_at: Option<String>,

    /// Timestamp of the next expected heartbeat from the device.
    #[serde(default)]
    pub next_heartbeat_at: Option<String>,

    /// 1-minute load average.
    #[serde(default)]
    pub load_average_1_min: Option<f64>,

    /// 5-minute load average.
    #[serde(default)]
    pub load_average_5_min: Option<f64>,

    /// 15-minute load average.
    #[serde(default)]
    pub load_average_15_min: Option<f64>,

    /// CPU utilization percentage (0.0 to 100.0).
    #[serde(default)]
    pub cpu_utilization_pct: Option<f64>,

    /// Memory utilization percentage (0.0 to 100.0).
    #[serde(default)]
    pub memory_utilization_pct: Option<f64>,

    /// Uplink statistics.
    #[serde(default)]
    pub uplink: Option<StatisticsUplink>,

    /// Interface statistics.
    #[serde(default)]
    pub interfaces: Option<StatisticsInterfaces>,
}

/// Uplink statistics showing transmit and receive rates.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatisticsUplink {
    /// Transmit rate in bits per second.
    #[serde(default)]
    pub tx_rate_bps: u64,

    /// Receive rate in bits per second.
    #[serde(default)]
    pub rx_rate_bps: u64,
}

/// Interface statistics container.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatisticsInterfaces {
    /// Radio interface statistics.
    #[serde(default)]
    pub radios: Vec<RadioStatistics>,
}

/// Statistics for a radio interface.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RadioStatistics {
    /// Radio frequency in GHz (e.g., 2.4 or 5.0).
    #[serde(default)]
    pub frequency_ghz: Option<f64>,

    /// Percentage of transmission retries.
    #[serde(default)]
    pub tx_retries_pct: Option<f64>,
}

impl DeviceStatistics {
    /// Returns the uptime as a human-readable string.
    pub fn uptime_formatted(&self) -> String {
        let secs = self.uptime_sec;
        let days = secs / 86400;
        let hours = (secs % 86400) / 3600;
        let mins = (secs % 3600) / 60;
        let secs = secs % 60;

        if days > 0 {
            format!("{}d {}h {}m {}s", days, hours, mins, secs)
        } else if hours > 0 {
            format!("{}h {}m {}s", hours, mins, secs)
        } else if mins > 0 {
            format!("{}m {}s", mins, secs)
        } else {
            format!("{}s", secs)
        }
    }

    /// Returns the total uplink throughput (tx + rx) in bits per second.
    pub fn total_uplink_bps(&self) -> Option<u64> {
        self.uplink.as_ref().map(|u| u.tx_rate_bps + u.rx_rate_bps)
    }

    /// Returns true if the device has radio interfaces.
    pub fn has_radios(&self) -> bool {
        self.interfaces
            .as_ref()
            .map(|i| !i.radios.is_empty())
            .unwrap_or(false)
    }
}

impl StatisticsUplink {
    /// Returns the transmit rate in megabits per second.
    pub fn tx_rate_mbps(&self) -> f64 {
        self.tx_rate_bps as f64 / 1_000_000.0
    }

    /// Returns the receive rate in megabits per second.
    pub fn rx_rate_mbps(&self) -> f64 {
        self.rx_rate_bps as f64 / 1_000_000.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uptime_formatted_days() {
        let stats = DeviceStatistics {
            uptime_sec: 90061, // 1 day, 1 hour, 1 minute, 1 second
            ..Default::default()
        };
        assert_eq!(stats.uptime_formatted(), "1d 1h 1m 1s");
    }

    #[test]
    fn test_uptime_formatted_hours() {
        let stats = DeviceStatistics {
            uptime_sec: 3661, // 1 hour, 1 minute, 1 second
            ..Default::default()
        };
        assert_eq!(stats.uptime_formatted(), "1h 1m 1s");
    }

    #[test]
    fn test_uptime_formatted_minutes() {
        let stats = DeviceStatistics {
            uptime_sec: 61, // 1 minute, 1 second
            ..Default::default()
        };
        assert_eq!(stats.uptime_formatted(), "1m 1s");
    }

    #[test]
    fn test_uptime_formatted_seconds_only() {
        let stats = DeviceStatistics {
            uptime_sec: 45,
            ..Default::default()
        };
        assert_eq!(stats.uptime_formatted(), "45s");
    }

    #[test]
    fn test_uptime_formatted_zero() {
        let stats = DeviceStatistics {
            uptime_sec: 0,
            ..Default::default()
        };
        assert_eq!(stats.uptime_formatted(), "0s");
    }

    #[test]
    fn test_total_uplink_bps_with_uplink() {
        let stats = DeviceStatistics {
            uplink: Some(StatisticsUplink {
                tx_rate_bps: 1_000_000,
                rx_rate_bps: 2_000_000,
            }),
            ..Default::default()
        };
        assert_eq!(stats.total_uplink_bps(), Some(3_000_000));
    }

    #[test]
    fn test_total_uplink_bps_without_uplink() {
        let stats = DeviceStatistics::default();
        assert_eq!(stats.total_uplink_bps(), None);
    }

    #[test]
    fn test_has_radios_true() {
        let stats = DeviceStatistics {
            interfaces: Some(StatisticsInterfaces {
                radios: vec![RadioStatistics {
                    frequency_ghz: Some(5.0),
                    tx_retries_pct: Some(1.5),
                }],
            }),
            ..Default::default()
        };
        assert!(stats.has_radios());
    }

    #[test]
    fn test_has_radios_empty() {
        let stats = DeviceStatistics {
            interfaces: Some(StatisticsInterfaces { radios: vec![] }),
            ..Default::default()
        };
        assert!(!stats.has_radios());
    }

    #[test]
    fn test_has_radios_no_interfaces() {
        let stats = DeviceStatistics::default();
        assert!(!stats.has_radios());
    }

    #[test]
    fn test_tx_rate_mbps() {
        let uplink = StatisticsUplink {
            tx_rate_bps: 100_000_000, // 100 Mbps
            rx_rate_bps: 0,
        };
        assert!((uplink.tx_rate_mbps() - 100.0).abs() < 0.001);
    }

    #[test]
    fn test_rx_rate_mbps() {
        let uplink = StatisticsUplink {
            tx_rate_bps: 0,
            rx_rate_bps: 50_000_000, // 50 Mbps
        };
        assert!((uplink.rx_rate_mbps() - 50.0).abs() < 0.001);
    }
}

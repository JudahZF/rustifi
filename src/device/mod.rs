pub mod models;

use crate::responses::stat::devices::RawDevice;
use crate::types::{
    config_net::ConfigNet,
    ip::IP,
    system_stats::SystemStats,
    temperature::Temperature,
    uplink::Uplink,
    user_stats::{InterfaceUserStats, UserStats},
    version::Version,
};
use chrono::prelude::*;
use models::DeviceType;

#[derive(Clone, Debug, PartialEq, PartialOrd, Default)]
pub struct Device {
    id: String,
    pub mac: String,
    pub model: String,
    pub dev_type: DeviceType,
    pub name: String,
    config_network: ConfigNet,
    ip: IP,
    connected_at: DateTime<Utc>,
    provisioned_at: DateTime<Utc>,
    disconnected_at: DateTime<Utc>,
    startup_time: DateTime<Utc>,
    serial: String,
    last_seen: DateTime<Utc>,
    next_interval: u16,
    system_stats: Option<SystemStats>,
    connected_network: String,
    temperatures: Option<Vec<Temperature>>,
    overheating: bool,
    isolated: bool,
    uplink: Uplink,
    user_stats: UserStats,
    version: Version,
}

impl Device {
    pub fn update(&mut self, new: Device) -> Result<(), Box<dyn std::error::Error>> {
        if self.id != new.id && self.mac != new.mac && self.model != new.model {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Cannot update device with different id, mac or model",
            )));
        }
        self.name = new.name;
        self.config_network = new.config_network;
        self.ip = new.ip;
        self.connected_at = new.connected_at;
        self.provisioned_at = new.provisioned_at;
        self.disconnected_at = new.disconnected_at;
        self.startup_time = new.startup_time;
        self.serial = new.serial;
        self.last_seen = new.last_seen;
        self.next_interval = new.next_interval;
        self.system_stats = new.system_stats;
        self.connected_network = new.connected_network;
        self.temperatures = new.temperatures;
        self.overheating = new.overheating;
        self.isolated = new.isolated;
        self.uplink = new.uplink;
        self.user_stats = new.user_stats;
        self.version = new.version;
        Ok(())
    }
}

impl From<RawDevice> for Device {
    fn from(raw: RawDevice) -> Self {
        let device = Device {
            id: raw.id,
            mac: raw.mac,
            model: raw.model,
            dev_type: DeviceType::from(raw.type_field.as_str()),
            name: raw.name,
            config_network: ConfigNet::from(raw.config_network),
            ip: IP::from(raw.ip),
            connected_at: DateTime::from_timestamp(raw.connected_at, 0).expect("Invalid timestamp"),
            provisioned_at: DateTime::from_timestamp(raw.provisioned_at, 0)
                .expect("Invalid timestamp"),
            disconnected_at: DateTime::from_timestamp(raw.disconnected_at, 0)
                .expect("Invalid timestamp"),
            startup_time: DateTime::from_timestamp(
                match raw.startup_timestamp {
                    Some(ts) => ts,
                    None => 0,
                },
                0,
            )
            .expect("Invalid timestamp"),
            last_seen: DateTime::from_timestamp(
                match raw.last_seen {
                    Some(ts) => ts,
                    None => 0,
                },
                0,
            )
            .expect("Invalid timestamp"),
            serial: match raw.serial {
                Some(s) => s,
                None => "".to_string(),
            },
            next_interval: match raw.next_interval {
                Some(i) => i,
                None => 30,
            },
            system_stats: (match raw.system_stats {
                Some(s) => Some(SystemStats::from(s)),
                None => None,
            }),
            connected_network: match raw.connection_network_name {
                Some(n) => n,
                None => "".to_string(),
            },
            temperatures: None,
            overheating: match raw.overheating {
                Some(o) => o,
                None => false,
            },
            isolated: match raw.isolated {
                Some(i) => i,
                None => false,
            },
            uplink: Uplink::from(raw.uplink),
            user_stats: UserStats {
                users: InterfaceUserStats {
                    total: raw.user_num_sta,
                    wlan: match raw.user_wlan_num_sta {
                        Some(i) => i,
                        None => 0,
                    },
                },
                guests: InterfaceUserStats {
                    total: raw.guest_num_sta,
                    wlan: match raw.guest_wlan_num_sta {
                        Some(i) => i,
                        None => 0,
                    },
                },
                total: raw.num_sta,
            },
            version: Version::from(raw.version),
        };
        device
    }
}

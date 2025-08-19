mod models;

use crate::responses::stat::devices::RawDevice;
use crate::types::{
    ConfigNet, IP, InterfaceUserStats, SystemStats, Temperature, Uplink, UserStats, Version,
};
use chrono::prelude::*;
use models::DeviceType;

pub use models::{APModel, AccessPoint};

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
            connected_at: DateTime::from_timestamp(
                match raw.connected_at {
                    Some(ts) => ts,
                    None => 0,
                },
                0,
            )
            .expect("Invalid timestamp"),
            provisioned_at: DateTime::from_timestamp(raw.provisioned_at, 0)
                .expect("Invalid timestamp"),
            disconnected_at: DateTime::from_timestamp(
                match raw.disconnected_at {
                    Some(ts) => ts,
                    None => 0,
                },
                0,
            )
            .expect("Invalid timestamp"),
            startup_time: DateTime::from_timestamp(raw.startup_timestamp.unwrap_or_default(), 0)
                .expect("Invalid timestamp"),
            last_seen: DateTime::from_timestamp(raw.last_seen.unwrap_or_default(), 0)
                .expect("Invalid timestamp"),
            serial: match raw.serial {
                Some(s) => s,
                None => "".to_string(),
            },
            next_interval: raw.next_interval.unwrap_or(30),
            system_stats: raw.system_stats.map(SystemStats::from),
            connected_network: match raw.connection_network_name {
                Some(n) => n,
                None => "".to_string(),
            },
            temperatures: None,
            overheating: raw.overheating.unwrap_or_default(),
            isolated: raw.isolated.unwrap_or_default(),
            uplink: Uplink::from(raw.uplink),
            user_stats: UserStats {
                users: InterfaceUserStats {
                    total: raw.user_num_sta,
                    wlan: raw.user_wlan_num_sta.unwrap_or_default(),
                },
                guests: InterfaceUserStats {
                    total: raw.guest_num_sta,
                    wlan: raw.guest_wlan_num_sta.unwrap_or_default(),
                },
                total: raw.num_sta,
            },
            version: Version::from(raw.version),
        };
        device
    }
}

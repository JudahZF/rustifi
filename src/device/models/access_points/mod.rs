mod models;

use crate::responses::stat::devices::RawDevice;
use crate::types::{
    Antenna, ConfigNet, InterfaceUserStats, Port, Radio, SystemStats, Temperature, Uplink,
    UserStats, Version, IP,
};
use chrono::prelude::*;

pub use models::APModel;

#[derive(Clone, Debug, PartialEq, PartialOrd, Default)]
pub struct AccessPoint {
    pub id: String,
    pub mac: String,
    pub model: APModel,
    pub name: String,
    pub config_network: ConfigNet,
    pub ip: IP,
    pub connected_at: DateTime<Utc>,
    pub provisioned_at: DateTime<Utc>,
    pub disconnected_at: DateTime<Utc>,
    pub startup_time: DateTime<Utc>,
    pub serial: String,
    pub last_seen: DateTime<Utc>,
    pub next_interval: u16,
    pub system_stats: Option<SystemStats>,
    pub connected_network: String,
    pub temperatures: Option<Vec<Temperature>>,
    pub overheating: bool,
    pub isolated: bool,
    pub uplink: Uplink,
    pub user_stats: UserStats,
    pub reboot_duration: Option<u32>,
    pub version: Version,
    pub antennas: Vec<Antenna>,
    pub radios: Vec<Radio>,
    pub ports: Vec<Port>,
}

impl AccessPoint {
    pub fn update(&mut self, new: AccessPoint) -> Result<(), Box<dyn std::error::Error>> {
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
        self.reboot_duration = new.reboot_duration;
        self.version = new.version;
        self.antennas = new.antennas;
        self.radios = new.radios;
        self.ports = new.ports;
        Ok(())
    }
}

impl From<RawDevice> for AccessPoint {
    fn from(raw: RawDevice) -> Self {
        let device = AccessPoint {
            id: raw.id,
            mac: raw.mac,
            model: APModel::from(raw.model),
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
            reboot_duration: match raw.reboot_duration {
                Some(d) => Some(d),
                None => None,
            },
            version: Version::from(raw.version),
            antennas: raw
                .antenna_table
                .iter()
                .map(|a| Antenna::from(a.clone()))
                .collect(),
            radios: raw
                .radio_table
                .iter()
                .map(|r| Radio::from(r.clone()))
                .collect(),
            ports: raw
                .ethernet_table
                .iter()
                .map(|p| Port::from(p.clone()))
                .collect(),
        };
        device
    }
}

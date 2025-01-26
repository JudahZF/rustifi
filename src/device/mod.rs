mod models;

use crate::responses::stat::devices::{RawDevice, SystemStats as RawSystemStats};
use crate::types::{
    ip::IP,
    net_config::NetConfig,
    system_stats::SystemStats,
    temperature::Temperature,
    uplink::Uplink,
    user_stats::{InterfaceUserStats, UserStats},
};
use chrono::prelude::*;
use models::Type;

#[derive(Clone, Debug, PartialEq, PartialOrd, Default)]
pub struct Device {
    id: String,
    pub mac: String,
    pub model: String,
    pub dev_type: Type,
    pub name: String,
    network_config: NetConfig,
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
}

impl From<RawDevice> for Device {
    fn from(raw: RawDevice) -> Self {
        let device = Device {
            id: raw.id,
            mac: raw.mac,
            model: raw.model,
            dev_type: Type::from(raw.type_field.as_str()),
            name: raw.name,
            network_config: NetConfig::from(raw.config_network),
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
        };
        device
    }
}

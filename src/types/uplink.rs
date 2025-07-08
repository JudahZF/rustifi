use crate::{
    responses::stat::devices::Uplink as RawUplink,
    types::{ip::IP, media::Media, port::PortStats},
};
use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, PartialOrd, Default)]
pub struct Uplink {
    ip: IP,
    name: String,
    netmask: IP,
    port_num: u16,
    remote_port: Option<u16>,
    up: bool,
    media: Media,
    stats: PortStats,
}

impl Display for Uplink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IP: {}\nName: {}\nNetmask: {}\nPort Number: {}\nUp: {}\nMedia: {}\n{}",
            self.ip, self.name, self.netmask, self.port_num, self.up, self.media, self.stats
        )
    }
}

impl From<RawUplink> for Uplink {
    fn from(raw: RawUplink) -> Self {
        Uplink {
            ip: IP::from(match raw.ip {
                Some(i) => i,
                None => "0.0.0.0".to_string(),
            }),
            name: match raw.name {
                Some(n) => n,
                None => "".to_string(),
            },
            netmask: IP::from(match raw.netmask {
                Some(n) => n,
                None => "0.0.0.0".to_string(),
            }),
            port_num: raw.num_port.unwrap_or_default(),
            up: raw.up.unwrap_or(true),
            media: match raw.media {
                Some(s) => Media::from(s),
                None => Media::GigabitEthernet,
            },
            stats: PortStats {
                rx_bytes: raw.rx_bytes.unwrap_or_default(),
                tx_bytes: raw.tx_bytes.unwrap_or_default(),
                rx_packets: raw.rx_packets.unwrap_or_default(),
                tx_packets: raw.tx_packets.unwrap_or_default(),
                rx_errors: raw.rx_errors.unwrap_or_default(),
                tx_errors: raw.tx_errors.unwrap_or_default(),
                rx_dropped: raw.rx_dropped.unwrap_or_default(),
                tx_dropped: raw.tx_dropped.unwrap_or_default(),
                rx_multicast: raw.rx_multicast.unwrap_or_default(),
                rx_rate: raw.rx_bytes_r.unwrap_or(0.0),
                tx_rate: raw.tx_bytes_r.unwrap_or(0.0),
                speed: raw.speed.unwrap_or_default(),
                max_speed: raw.max_speed.unwrap_or_default(),
                full_duplex: raw.full_duplex.unwrap_or(true),
            },
            remote_port: raw.uplink_remote_port,
        }
    }
}

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
            port_num: match raw.num_port {
                Some(i) => i,
                None => 0,
            },
            up: match raw.up {
                Some(b) => b,
                None => true,
            },
            media: match raw.media {
                Some(s) => Media::from(s),
                None => Media::GigabitEthernet,
            },
            stats: PortStats {
                rx_bytes: match raw.rx_bytes {
                    Some(u) => u,
                    None => 0,
                },
                tx_bytes: match raw.tx_bytes {
                    Some(u) => u,
                    None => 0,
                },
                rx_packets: match raw.rx_packets {
                    Some(u) => u,
                    None => 0,
                },
                tx_packets: match raw.tx_packets {
                    Some(u) => u,
                    None => 0,
                },
                rx_errors: match raw.rx_errors {
                    Some(u) => u,
                    None => 0,
                },
                tx_errors: match raw.tx_errors {
                    Some(u) => u,
                    None => 0,
                },
                rx_dropped: match raw.rx_dropped {
                    Some(u) => u,
                    None => 0,
                },
                tx_dropped: match raw.tx_dropped {
                    Some(u) => u,
                    None => 0,
                },
                rx_multicast: match raw.rx_multicast {
                    Some(u) => u,
                    None => 0,
                },
                rx_rate: match raw.rx_bytes_r {
                    Some(f) => f,
                    None => 0.0,
                },
                tx_rate: match raw.tx_bytes_r {
                    Some(f) => f,
                    None => 0.0,
                },
                speed: match raw.speed {
                    Some(u) => u,
                    None => 0,
                },
                max_speed: match raw.max_speed {
                    Some(u) => u,
                    None => 0,
                },
                full_duplex: match raw.full_duplex {
                    Some(b) => b,
                    None => true,
                },
            },
        }
    }
}

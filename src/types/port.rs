use crate::responses::stat::devices::EthernetTable as RawEthernet;
use std::fmt::Display;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct Port {
    pub name: String,
    pub num_port: u16,
    pub mac: String,
}

impl From<RawEthernet> for Port {
    fn from(raw: RawEthernet) -> Self {
        Port {
            name: raw.name,
            num_port: match raw.num_port {
                Some(n) => n,
                None => 0,
            },
            mac: raw.mac,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default)]
pub struct PortStats {
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub rx_packets: u64,
    pub tx_packets: u64,
    pub rx_errors: u64,
    pub tx_errors: u64,
    pub rx_dropped: u64,
    pub tx_dropped: u64,
    pub rx_multicast: u64,
    pub rx_rate: f64,
    pub tx_rate: f64,
    pub speed: u32,
    pub max_speed: u32,
    pub full_duplex: bool,
}

impl Display for PortStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RX Bytes: {}\nTX Bytes: {}\nRX Packets: {}\nTX Packets: {}\nRX Errors: {}\nTX Errors: {}\nRX Dropped: {}\nTX Dropped: {}\nRX Multicast: {}\nRX Rate: {}\nTX Rate: {}\nSpeed: {}\nMax Speed: {}\nFull Duplex: {}",
            self.rx_bytes,
            self.tx_bytes,
            self.rx_packets,
            self.tx_packets,
            self.rx_errors,
            self.tx_errors,
            self.rx_dropped,
            self.tx_dropped,
            self.rx_multicast,
            self.rx_rate,
            self.tx_rate,
            self.speed,
            self.max_speed,
            self.full_duplex
        )
    }
}

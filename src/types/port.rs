#[derive(Debug, Clone)]
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

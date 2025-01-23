#[derive(Debug, Clone)]
pub struct UserStats {
    pub total: u64,
    pub users: InterfaceUserStats,
    pub guests: InterfaceUserStats,
}

#[derive(Debug, Clone)]
pub struct InterfaceUserStats {
    pub total: u64,
    pub wlan: u64,
}

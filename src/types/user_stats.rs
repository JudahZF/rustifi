use std::fmt::Display;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct UserStats {
    pub total: u64,
    pub users: InterfaceUserStats,
    pub guests: InterfaceUserStats,
}

impl Display for UserStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Total Users: {}\nWLAN Users: {}\nTotal Guests: {}\nWLAN Guests: {}",
            self.total, self.users.wlan, self.guests.total, self.guests.wlan
        )
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct InterfaceUserStats {
    pub total: u64,
    pub wlan: u64,
}

impl Display for InterfaceUserStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Total Users: {}\nWLAN Users: {}", self.total, self.wlan)
    }
}

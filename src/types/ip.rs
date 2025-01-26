use std::fmt::Display;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct IP {
    octets: [u8; 4],
    netmask: Option<u8>,
}

impl IP {
    pub fn new(octets: [u8; 4], netmask: Option<u8>) -> Self {
        IP { octets, netmask }
    }
}

impl From<String> for IP {
    fn from(s: String) -> Self {
        let netmask = s.split("/").collect::<Vec<&str>>();
        let octets = netmask[0]
            .split(".")
            .map(|s| s.parse::<u8>().unwrap())
            .collect::<Vec<u8>>();
        if netmask.len() == 2 {
            IP::new(
                octets.try_into().expect("IP must be 4 octets"),
                Some(netmask[1].parse::<u8>().unwrap()),
            )
        } else {
            IP::new(octets.try_into().expect("IP must be 4 octets"), None)
        }
    }
}

impl Display for IP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(netmask) = self.netmask {
            write!(
                f,
                "{}.{}.{}.{}/{}",
                self.octets[0], self.octets[1], self.octets[2], self.octets[3], netmask
            )
        } else {
            write!(
                f,
                "{}.{}.{}.{}",
                self.octets[0], self.octets[1], self.octets[2], self.octets[3]
            )
        }
    }
}

#[derive(Debug, Clone)]
pub struct IP {
    octets: [u8; 4],
    netmask: Option<u8>,
}

impl IP {
    pub fn new(octets: [u8; 4]) -> Self {
        IP { octets, netmask: None }
    }
}

impl From<String> for IP {
    fn from(s: String) -> Self {
        let octets = s.split(".").map(|s| s.parse::<u8>().unwrap()).collect::<Vec<u8>>();
        IP::new(octets.try_into().expect("IP must be 4 octets"))
    }
}

impl Into<String> for IP {
    fn into(self) -> String {
        format!(
            "{}.{}.{}.{}",
            self.octets[0], self.octets[1], self.octets[2], self.octets[3]
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ip() {
        let ip = IP::new([192, 168, 1, 1]);
        assert_eq!(ip.into(), "192.168.1.1".to_string());
    }
}

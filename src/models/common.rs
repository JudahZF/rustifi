use serde::Deserialize;
use std::fmt;
use std::net::Ipv4Addr;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct IpAddress(pub Ipv4Addr);

impl IpAddress {
    pub fn new(addr: Ipv4Addr) -> Self {
        IpAddress(addr)
    }
}

impl fmt::Display for IpAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for IpAddress {
    fn default() -> Self {
        IpAddress(Ipv4Addr::new(0, 0, 0, 0))
    }
}

impl<'de> Deserialize<'de> for IpAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let addr = Ipv4Addr::from_str(&s).map_err(serde::de::Error::custom)?;
        Ok(IpAddress(addr))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct MacAddress(pub String);

impl MacAddress {
    pub fn new(mac: impl Into<String>) -> Self {
        MacAddress(mac.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'de> Deserialize<'de> for MacAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(MacAddress(s))
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Timestamp(pub i64);

impl Timestamp {
    pub fn as_secs(&self) -> i64 {
        self.0
    }
}

impl<'de> Deserialize<'de> for Timestamp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let n = i64::deserialize(deserializer)?;
        Ok(Timestamp(n))
    }
}

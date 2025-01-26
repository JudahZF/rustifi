use crate::types::ip::IP;
use std::fmt::Display;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct Network {
    name: String,
    address: IP,
}

impl Display for Network {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Name: {}\nAddress: {}", self.name, self.address)
    }
}

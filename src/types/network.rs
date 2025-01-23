use crate::types::ip::IP;

#[derive(Debug, Clone)]
pub struct Network {
    name: String,
    address: IP
}

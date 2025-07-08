use rustifi::types::IP;
use rustifi::types::{RadioPowerMode, RadioType};

#[test]
fn test_ip_from_string() {
    let ip1 = IP::from("192.168.1.1".to_string());
    let ip2 = IP::from("10.0.0.1/24".to_string());

    assert_eq!(format!("{}", ip1), "192.168.1.1");
    assert_eq!(format!("{}", ip2), "10.0.0.1/24");
}

#[test]
fn test_ip_equality() {
    let ip1 = IP::new([192, 168, 1, 1], None);
    let ip2 = IP::new([192, 168, 1, 1], None);
    let ip3 = IP::new([192, 168, 1, 2], None);
    let ip4 = IP::new([192, 168, 1, 1], Some(24));

    assert_eq!(ip1, ip2);
    assert_ne!(ip1, ip3);
    assert_ne!(ip1, ip4);
}

#[test]
fn test_radio_type_defaults() {
    let default_radio_type = RadioType::default();
    assert_eq!(default_radio_type, RadioType::N24);

    let default_power_mode = RadioPowerMode::default();
    assert_eq!(default_power_mode, RadioPowerMode::Unknown);
}

#[test]
fn test_radio_type_equality() {
    assert_eq!(RadioType::N24, RadioType::N24);
    assert_ne!(RadioType::N24, RadioType::AC5);
    assert_ne!(RadioType::AC5, RadioType::AX6);

    assert_eq!(RadioPowerMode::Low, RadioPowerMode::Low);
    assert_ne!(RadioPowerMode::Low, RadioPowerMode::Medium);
    assert_ne!(RadioPowerMode::Medium, RadioPowerMode::High);
    assert_ne!(RadioPowerMode::High, RadioPowerMode::Unknown);
}

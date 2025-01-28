use rustifi::types::ip::IP;

#[test]
fn test_ip() {
    let ip = IP::new([192, 168, 1, 1], None);
    let with_mask = IP::new([192, 168, 1, 1], Some(24));
    assert_eq!(format!("{}", ip), "192.168.1.1".to_string());
    assert_eq!(format!("{}", with_mask), "192.168.1.1/24".to_string());
}

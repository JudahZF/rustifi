use crate::types::ip::IP;

#[test]
fn test_ip() {
    let ip = IP::new([192, 168, 1, 1]);
    assert_eq!(ip.into(), "192.168.1.1".to_string());
}

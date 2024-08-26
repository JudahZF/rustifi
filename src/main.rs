use unifi_rs::UnifiController;

#[tokio::main]
async fn main() {
    let addr = "https://192.168.1.1".to_string();
    let username = "testing".to_string();
    let password = "amputate-impiety4CHUFF3woodshed".to_string();
    let is_udm = true;
    let mut controller = match UnifiController::new(addr, username, password, is_udm).await {
        Ok(controller) => controller,
        Err(e) => panic!("Unable to connect to the controller: {}", e),
    };

    match controller.set_site("default".to_string()) {
        Ok(_) => {}
        Err(e) => panic!("Unable to set the site: {}", e),
    };

    println!("Site: {}", controller.current_site.clone().unwrap().name);

    let current_site = controller.current_site.unwrap();
    let devices = current_site.clone().get_devices().await.unwrap();
    println!("Devices: ");
    for device in devices {
        println!("  - {}", device.name);
    }
    current_site.get_device_stats().await.unwrap();
}

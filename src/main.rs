use dotenv;
use std::env;
use tokio;
use unifi_rs::UnifiController;

static ADDR: &str = "API_ADDR";
static USER: &str = "API_USER";
static PASS: &str = "API_PASS";
static IS_UDM: &str = "API_UDM";

#[tokio::main]
async fn main() -> Result<(), ()> {
    dotenv::dotenv().ok();
    let addr = env::var(ADDR).unwrap().to_string();
    let username = env::var(USER).unwrap().to_string();
    let password = env::var(PASS).unwrap().to_string();
    let is_udm: bool = env::var(IS_UDM).unwrap().parse().unwrap();
    println!("Connecting to controller at {}", addr);
    println!("Username: {}", username);
    println!("Is UDM: {}", is_udm);
    let mut controller = match UnifiController::new(addr, username, password, is_udm).await {
        Ok(controller) => controller,
        Err(e) => panic!("Unable to connect to the controller: {}", e),
    };

    match controller.set_site("BCF 2025".to_string()) {
        Ok(_) => {}
        Err(e) => panic!("Unable to set the site: {}", e),
    };

    println!("Site: {}", controller.current_site.clone().unwrap().name);

    let current_site = controller.current_site.unwrap();
    let devices = current_site.clone().get_devices().await.unwrap();
    println!("Devices: ");
    for device in devices {
        println!(
            "  - {}, type: {:?}, mac: {}, model: {}",
            device.name, device.dev_type, device.mac, device.model
        );
    }
    current_site.get_device_stats().await.unwrap();
    Ok(())
}

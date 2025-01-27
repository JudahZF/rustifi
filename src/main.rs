use dotenv;
use rustifi::{device::models::access_points::models::APModel, UnifiController};
use std::env;
use tokio;

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
    for ap in devices.AccessPoints {
        if ap.model == APModel::BasestationXG {
            println!("----------");
            println!("AP: {}", ap.name);
            println!("MAC: {}", ap.mac);
            println!("IP: {}", ap.ip);
            println!("Serial: {}", ap.serial);
            println!("Last seen: {}", ap.last_seen);
            println!("System stats: {:?}", ap.system_stats);
            println!("Connected network: {}", ap.connected_network);
            println!("User stats: {:?}", ap.user_stats);
            println!("Radio 1 Channel: {:?}", ap.radios[0].current_channel);
        }
    }
    Ok(())
}

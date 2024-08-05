use unifi_rs::UnifiController;

#[tokio::main]
async fn main() {
    let addr = "https://192.168.4.43:8443".to_string();
    let username = "admin".to_string();
    let password = "admin".to_string();
    let is_udm = false;
    let mut controller = match UnifiController::new(addr, username, password, is_udm).await {
        Ok(controller) => controller,
        Err(e) => panic!("Unable to connect to the controller: {}", e),
    };

    match controller.set_site("default".to_string()) {
        Ok(_) => {}
        Err(e) => panic!("Unable to set the site: {}", e),
    };

    println!("Site: {}", controller.current_site.unwrap().name);
}

use unifi_rs::UnifiController;

#[tokio::main]
async fn main() {
    let addr = "https://192.168.4.43:8443".to_string();
    let username = "admin".to_string();
    let password = "admin".to_string();
    let is_udm = false;
    let controller = match UnifiController::new(addr, username, password, is_udm).await {
        Ok(controller) => controller,
        Err(_) => panic!("Unable to connect to the controller"),
    };

    let sites = controller.get_sites();

    let site = &sites[0];
    site.get_active_clients().await.unwrap();
}

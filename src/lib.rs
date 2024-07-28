mod auth;

use auth::sign_in;
use reqwest::{header, Client};

async fn connect(
    addr: &String,
    username: String,
    password: String,
    is_udm: Option<bool>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        "Content-Type",
        header::HeaderValue::from_static("application/json"),
    );
    let client = Client::builder()
        .default_headers(headers)
        .user_agent("unifi-rs")
        .cookie_store(true)
        .build()?;

    let mut udm = false;

    if let Some(is_udm) = is_udm {
        udm = is_udm;
    }

    match sign_in(&client, addr, username, password, udm).await {
        Ok(_) => {}
        Err(_) => panic!("Unable to sign into the controller API"),
    };
    Ok(())
}

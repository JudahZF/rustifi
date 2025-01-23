use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct SignInRequest {
    username: String,
    password: String,
}

pub async fn sign_in(
    client: &Client,
    addr: &String,
    username: String,
    password: String,
    is_udm: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut url = Url::parse((addr.clone() + "/api/login").as_str()).unwrap();
    if is_udm {
        url = Url::parse((addr.clone() + "/api/auth/login").as_str()).unwrap();
    }
    println!("Signing in to {}", url);
    let data: SignInRequest = SignInRequest { username, password };

    let response = match client.post(url.clone()).json(&data).send().await {
        Ok(r) => r,
        Err(e) => return Err(Box::new(e)),
    };

    if !response.status().is_success() {
        panic!("Failed to sign in: {}", response.text().await?);
    }

    println!("{:?}", response.text().await?);
    Ok(())
}

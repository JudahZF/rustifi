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
    let data: SignInRequest = SignInRequest { username, password };

    match client.post(url.clone()).json(&data).send().await {
        Ok(_) => return Ok(()),
        Err(e) => return Err(Box::new(e)),
    };
}

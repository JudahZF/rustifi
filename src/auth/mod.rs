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
) -> Result<(), Box<dyn std::error::Error>> {
    let url = Url::parse((addr.clone() + "/api/login").as_str()).unwrap();
    let data: SignInRequest = SignInRequest { username, password };
    match client.post(url).json(&data).send().await {
        Ok(_response) => return Ok(()),
        Err(e) => return Err(Box::new(e)),
    };
}

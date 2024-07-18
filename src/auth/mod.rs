use std::str::FromStr;

use reqwest::{Client, Request, Url};

struct sign_in_request {
    username: String,
    password: String,
}

async fn sign_in(
    client: &Client,
    addr: &String,
    username: String,
    password: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = Url::parse((addr.clone() + "/api/login").as_str()).unwrap();
    let data: sign_in_request = sign_in_request {
        username,
        password,
    };
    let response = client.post(url).header("", "").body(sign_in_request).
    Ok(())
}

use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use std::{error::Error, fmt};

#[derive(Debug, Serialize, Deserialize)]
struct SignInRequest {
    username: String,
    password: String,
}

struct AuthError {
    message: String,
}

impl AuthError {
    fn new(message: String) -> Self {
        Self { message }
    }
}

impl Error for AuthError {}

// Implement std::fmt::Display for AppError
impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to sign in. Got response: {}", self.message) // user-facing output
    }
}

// Implement std::fmt::Debug for AppError
impl fmt::Debug for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}

pub async fn sign_in(
    client: &Client,
    addr: &str,
    username: String,
    password: String,
    is_udm: bool,
) -> Result<(), Box<dyn Error>> {
    let mut url = Url::parse(format!("{}/api/login", addr).as_str()).unwrap();
    if is_udm {
        url = Url::parse(format!("{}/api/auth/login", addr).as_str()).unwrap();
    }
    println!("Signing in to {}", url);
    let data: SignInRequest = SignInRequest { username, password };

    let response = match client.post(url.clone()).json(&data).send().await {
        Ok(r) => r,
        Err(e) => return Err(Box::new(e)),
    };

    if !response.status().is_success() {
        return Err(Box::new(AuthError::new(response.text().await.unwrap())));
    }

    Ok(())
}

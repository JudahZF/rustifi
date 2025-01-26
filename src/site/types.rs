use reqwest::{header::HeaderValue, Client};

#[derive(Clone, Debug, Default)]
pub struct Site {
    pub addr: String,
    pub name: String,
    pub id: String,
    pub client: Option<Client>,
    pub cookies: Option<HeaderValue>,
}

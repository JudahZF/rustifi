use crate::api::endpoint::{Endpoint, HttpMethod};
use crate::error::Result;
use reqwest::{Client, cookie::Jar};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct UnifiClient {
    http: Client,
    base_url: String,
    base_path: String,
    api_key: Option<String>,
}

impl UnifiClient {
    pub fn new(base_url: impl Into<String>) -> Result<Self> {
        Self::with_base_path(base_url, "/api/v1")
    }

    pub fn with_base_path(
        base_url: impl Into<String>,
        base_path: impl Into<String>,
    ) -> Result<Self> {
        let jar = Jar::default();
        let cookie_store = Arc::new(jar);

        let http = Client::builder()
            .danger_accept_invalid_certs(true)
            .user_agent("rustifi/1.0")
            .cookie_store(true)
            .cookie_provider(cookie_store)
            .build()?;

        Ok(Self {
            http,
            base_url: base_url.into().trim_end_matches('/').to_string(),
            base_path: base_path.into().trim_start_matches('/').to_string(),
            api_key: None,
        })
    }

    pub fn with_api_key(base_url: impl Into<String>, api_key: impl Into<String>) -> Result<Self> {
        let client = Self::new(base_url)?;
        Ok(client.set_api_key(api_key))
    }

    pub fn with_base_path_and_key(
        base_url: impl Into<String>,
        base_path: impl Into<String>,
        api_key: impl Into<String>,
    ) -> Result<Self> {
        let client = Self::with_base_path(base_url, base_path)?;
        Ok(client.set_api_key(api_key))
    }

    fn set_api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    pub fn api_key(&self) -> Option<&str> {
        self.api_key.as_deref()
    }

    /// Execute a request for an endpoint instance.
    /// Use this when the endpoint has dynamic path parameters.
    pub async fn execute<E>(&self, endpoint: &E) -> Result<E::Response>
    where
        E: Endpoint,
        E::Response: for<'a> serde::Deserialize<'a>,
    {
        let path = endpoint.build_path();
        let url = format!("{}/{}/{}", self.base_url, self.base_path, path);
        let mut headers = reqwest::header::HeaderMap::new();

        if let Some(api_key) = &self.api_key {
            headers.insert("X-API-Key", api_key.parse().unwrap());
        }

        let request = self.http.request(E::METHOD.into(), &url).headers(headers);

        let response = request.send().await?;

        if !response.status().is_success() {
            return Err(crate::error::Error::Request(
                response.error_for_status().unwrap_err(),
            ));
        }

        let body = response.text().await?;
        let response_data = serde_json::from_str::<E::Response>(&body)
            .map_err(|e| crate::error::Error::Parse(format!("{}\nResponse body: {}", e, body)))?;
        Ok(response_data)
    }

    /// Execute a request for endpoints without dynamic path parameters.
    /// For endpoints with path parameters, use `execute()` instead.
    pub async fn request<E>(&self) -> Result<E::Response>
    where
        E: Endpoint + Default,
        E::Response: for<'a> serde::Deserialize<'a>,
    {
        self.execute(&E::default()).await
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub fn base_path(&self) -> &str {
        &self.base_path
    }
}

impl From<HttpMethod> for reqwest::Method {
    fn from(method: HttpMethod) -> Self {
        match method {
            HttpMethod::Get => reqwest::Method::GET,
            HttpMethod::Post => reqwest::Method::POST,
            HttpMethod::Put => reqwest::Method::PUT,
            HttpMethod::Patch => reqwest::Method::PATCH,
            HttpMethod::Delete => reqwest::Method::DELETE,
        }
    }
}

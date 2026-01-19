use crate::api::endpoint::{Endpoint, HttpMethod};
use crate::error::Result;
use reqwest::{cookie::Jar, Client};
use std::sync::Arc;

/// The base URL for the UniFi remote cloud API.
pub const REMOTE_API_URL: &str = "https://api.ui.com";

#[derive(Clone, Debug)]
pub struct UnifiClient {
    http: Client,
    base_url: String,
    base_path: String,
    api_key: Option<String>,
    /// Host ID for remote API access via api.ui.com
    host_id: Option<String>,
}

impl UnifiClient {
    /// Create a new client for UniFi controller access with strict TLS validation.
    ///
    /// This constructor requires valid TLS certificates. If your controller uses
    /// a self-signed certificate, use [`new_insecure`](Self::new_insecure) instead.
    ///
    /// # Example
    /// ```no_run
    /// use rustifi::UnifiClient;
    ///
    /// let client = UnifiClient::new("https://unifi.example.com")?;
    /// # Ok::<(), rustifi::Error>(())
    /// ```
    pub fn new(base_url: impl Into<String>) -> Result<Self> {
        Self::with_base_path(base_url, "/api/v1")
    }

    /// Create a new client that accepts invalid/self-signed TLS certificates.
    ///
    /// # Security Warning
    /// **This disables TLS certificate validation.** Only use this for local
    /// controllers with self-signed certificates on trusted networks. Using this
    /// over untrusted networks exposes you to man-in-the-middle attacks.
    ///
    /// For production environments or controllers with valid certificates,
    /// use [`new`](Self::new) instead.
    ///
    /// # Example
    /// ```no_run
    /// use rustifi::UnifiClient;
    ///
    /// // Only for local controllers with self-signed certs
    /// let client = UnifiClient::new_insecure("https://192.168.1.1")?;
    /// # Ok::<(), rustifi::Error>(())
    /// ```
    pub fn new_insecure(base_url: impl Into<String>) -> Result<Self> {
        Self::build_client(base_url, "/api/v1", true)
    }

    /// Create a new client with a custom base path and strict TLS validation.
    ///
    /// # Arguments
    /// * `base_url` - The base URL of the UniFi controller
    /// * `base_path` - The API base path (e.g., "/api/v1")
    ///
    /// # Example
    /// ```no_run
    /// use rustifi::UnifiClient;
    ///
    /// let client = UnifiClient::with_base_path("https://unifi.example.com", "/api/v2")?;
    /// # Ok::<(), rustifi::Error>(())
    /// ```
    pub fn with_base_path(
        base_url: impl Into<String>,
        base_path: impl Into<String>,
    ) -> Result<Self> {
        Self::build_client(base_url, base_path, false)
    }

    /// Create a new client with a custom base path that accepts invalid TLS certificates.
    ///
    /// # Security Warning
    /// **This disables TLS certificate validation.** Only use this for local
    /// controllers with self-signed certificates on trusted networks. Using this
    /// over untrusted networks exposes you to man-in-the-middle attacks.
    ///
    /// # Arguments
    /// * `base_url` - The base URL of the UniFi controller
    /// * `base_path` - The API base path (e.g., "/api/v1")
    ///
    /// # Example
    /// ```no_run
    /// use rustifi::UnifiClient;
    ///
    /// // Only for local controllers with self-signed certs
    /// let client = UnifiClient::with_base_path_insecure("https://192.168.1.1", "/api/v1")?;
    /// # Ok::<(), rustifi::Error>(())
    /// ```
    pub fn with_base_path_insecure(
        base_url: impl Into<String>,
        base_path: impl Into<String>,
    ) -> Result<Self> {
        Self::build_client(base_url, base_path, true)
    }

    /// Internal: Build a client with configurable TLS validation.
    fn build_client(
        base_url: impl Into<String>,
        base_path: impl Into<String>,
        accept_invalid_certs: bool,
    ) -> Result<Self> {
        let jar = Jar::default();
        let cookie_store = Arc::new(jar);

        let mut builder = Client::builder()
            .user_agent("rustifi/1.0")
            .cookie_store(true)
            .cookie_provider(cookie_store)
            .connect_timeout(std::time::Duration::from_secs(30))
            .timeout(std::time::Duration::from_secs(60));

        if accept_invalid_certs {
            builder = builder
                .danger_accept_invalid_certs(true)
                .danger_accept_invalid_hostnames(true);
        }

        let http = builder.build()?;

        Ok(Self {
            http,
            base_url: base_url.into().trim_end_matches('/').to_string(),
            base_path: base_path.into().trim_start_matches('/').to_string(),
            api_key: None,
            host_id: None,
        })
    }

    /// Create a client with an API key and strict TLS validation.
    ///
    /// # Errors
    /// Returns an error if the API key contains invalid HTTP header characters.
    ///
    /// # Example
    /// ```no_run
    /// use rustifi::UnifiClient;
    ///
    /// let client = UnifiClient::with_api_key("https://unifi.example.com", "your-api-key")?;
    /// # Ok::<(), rustifi::Error>(())
    /// ```
    pub fn with_api_key(base_url: impl Into<String>, api_key: impl Into<String>) -> Result<Self> {
        let client = Self::new(base_url)?;
        client.set_api_key(api_key)
    }

    /// Create a client with an API key that accepts invalid TLS certificates.
    ///
    /// # Security Warning
    /// **This disables TLS certificate validation.** Only use this for local
    /// controllers with self-signed certificates on trusted networks.
    ///
    /// # Errors
    /// Returns an error if the API key contains invalid HTTP header characters.
    ///
    /// # Example
    /// ```no_run
    /// use rustifi::UnifiClient;
    ///
    /// // Only for local controllers with self-signed certs
    /// let client = UnifiClient::with_api_key_insecure("https://192.168.1.1", "your-api-key")?;
    /// # Ok::<(), rustifi::Error>(())
    /// ```
    pub fn with_api_key_insecure(
        base_url: impl Into<String>,
        api_key: impl Into<String>,
    ) -> Result<Self> {
        let client = Self::new_insecure(base_url)?;
        client.set_api_key(api_key)
    }

    /// Create a client with a custom base path and API key (strict TLS).
    ///
    /// # Errors
    /// Returns an error if the API key contains invalid HTTP header characters.
    ///
    /// # Example
    /// ```no_run
    /// use rustifi::UnifiClient;
    ///
    /// let client = UnifiClient::with_base_path_and_key(
    ///     "https://unifi.example.com",
    ///     "/api/v1",
    ///     "your-api-key"
    /// )?;
    /// # Ok::<(), rustifi::Error>(())
    /// ```
    pub fn with_base_path_and_key(
        base_url: impl Into<String>,
        base_path: impl Into<String>,
        api_key: impl Into<String>,
    ) -> Result<Self> {
        let client = Self::with_base_path(base_url, base_path)?;
        client.set_api_key(api_key)
    }

    /// Create a client with a custom base path and API key that accepts invalid TLS certificates.
    ///
    /// # Security Warning
    /// **This disables TLS certificate validation.** Only use this for local
    /// controllers with self-signed certificates on trusted networks.
    ///
    /// # Errors
    /// Returns an error if the API key contains invalid HTTP header characters.
    ///
    /// # Example
    /// ```no_run
    /// use rustifi::UnifiClient;
    ///
    /// // Only for local controllers with self-signed certs
    /// let client = UnifiClient::with_base_path_and_key_insecure(
    ///     "https://192.168.1.1",
    ///     "/api/v1",
    ///     "your-api-key"
    /// )?;
    /// # Ok::<(), rustifi::Error>(())
    /// ```
    pub fn with_base_path_and_key_insecure(
        base_url: impl Into<String>,
        base_path: impl Into<String>,
        api_key: impl Into<String>,
    ) -> Result<Self> {
        let client = Self::with_base_path_insecure(base_url, base_path)?;
        client.set_api_key(api_key)
    }

    /// Create a client for remote API access via api.ui.com.
    ///
    /// This allows accessing UniFi consoles remotely through Ubiquiti's cloud.
    /// Requires firmware version >= 5.0.3 on the target console.
    ///
    /// # Arguments
    /// * `api_key` - Your UI.com API key (site-manager-api-key)
    /// * `host_id` - The Host ID of the console to connect to
    ///   (format: `900A6F00301100000000074A6BA90000000007A3387E0000000063EC9853:123456789`)
    ///
    /// # Errors
    /// Returns an error if the API key contains invalid HTTP header characters.
    ///
    /// # Example
    /// ```no_run
    /// use rustifi::UnifiClient;
    ///
    /// let client = UnifiClient::remote("your-api-key", "your-host-id")?;
    /// # Ok::<(), rustifi::Error>(())
    /// ```
    pub fn remote(api_key: impl Into<String>, host_id: impl Into<String>) -> Result<Self> {
        let jar = Jar::default();
        let cookie_store = Arc::new(jar);

        // Validate the API key can be parsed as a header value
        let key = api_key.into();
        let _: reqwest::header::HeaderValue = key.parse()?;

        // For remote API, we don't need to accept invalid certs
        let http = Client::builder()
            .user_agent("rustifi/1.0")
            .cookie_store(true)
            .cookie_provider(cookie_store)
            .connect_timeout(std::time::Duration::from_secs(30))
            .timeout(std::time::Duration::from_secs(60))
            .build()?;

        Ok(Self {
            http,
            base_url: REMOTE_API_URL.to_string(),
            base_path: "v1".to_string(),
            api_key: Some(key),
            host_id: Some(host_id.into()),
        })
    }

    /// Set the API key, validating it can be used as an HTTP header value.
    fn set_api_key(mut self, api_key: impl Into<String>) -> Result<Self> {
        let key = api_key.into();
        // Validate the API key can be parsed as a header value
        // This catches invalid characters early rather than at request time
        let _: reqwest::header::HeaderValue = key.parse()?;
        self.api_key = Some(key);
        Ok(self)
    }

    pub fn api_key(&self) -> Option<&str> {
        self.api_key.as_deref()
    }

    /// Returns the host ID if this is a remote API client.
    pub fn host_id(&self) -> Option<&str> {
        self.host_id.as_deref()
    }

    /// Returns true if this client is configured for remote API access.
    pub fn is_remote(&self) -> bool {
        self.host_id.is_some()
    }

    /// Execute a request for an endpoint instance.
    /// Use this when the endpoint has dynamic path parameters.
    pub async fn execute<E>(&self, endpoint: &E) -> Result<E::Response>
    where
        E: Endpoint,
        E::Response: for<'a> serde::Deserialize<'a>,
    {
        let path = endpoint.build_path();

        // Build URL based on whether this is a remote or local client
        let url = if let Some(host_id) = &self.host_id {
            // Remote API: https://api.ui.com/v1/connector/consoles/{host_id}/{path}
            format!(
                "{}/{}/connector/consoles/{}/{}",
                self.base_url, self.base_path, host_id, path
            )
        } else {
            // Local API: {base_url}/{base_path}/{path}
            format!("{}/{}/{}", self.base_url, self.base_path, path)
        };

        let mut headers = reqwest::header::HeaderMap::new();

        if let Some(api_key) = &self.api_key {
            headers.insert("X-API-Key", api_key.parse()?);
        }

        let mut request = self.http.request(E::METHOD.into(), &url).headers(headers);

        // Append query parameters using reqwest's query() for proper URL encoding
        let params = endpoint.query_params();
        if !params.is_empty() {
            request = request.query(&params);
        }

        // Add JSON body for POST/PUT/PATCH methods
        if let Some(body) = endpoint.request_body()? {
            request = request.json(&body);
        }

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

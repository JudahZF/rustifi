use crate::api::endpoint::{Endpoint, HttpMethod};
use crate::models::Client;
use crate::response::{EmptyResponse, SiteResponse};
use serde::Serialize;
use serde_json::Value;

/// Fetch all clients for a specific site with optional pagination.
/// Endpoint: GET /v1/sites/{site_id}/clients
#[derive(Debug, Clone)]
pub struct GetClients {
    pub site_id: String,
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

impl GetClients {
    /// Create a new GetClients endpoint without pagination.
    pub fn new(site_id: impl Into<String>) -> Self {
        Self {
            site_id: site_id.into(),
            offset: None,
            limit: None,
        }
    }

    /// Create a new GetClients endpoint with pagination parameters.
    pub fn with_pagination(site_id: impl Into<String>, offset: usize, limit: usize) -> Self {
        Self {
            site_id: site_id.into(),
            offset: Some(offset),
            limit: Some(limit),
        }
    }

    /// Set the offset for pagination.
    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Set the limit for pagination.
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }
}

impl Endpoint for GetClients {
    const PATH: &'static str = "sites/{site_id}/clients";
    const METHOD: HttpMethod = HttpMethod::Get;
    type Response = SiteResponse<Client>;

    fn build_path(&self) -> String {
        format!("sites/{}/clients", self.site_id)
    }

    fn query_params(&self) -> Vec<(&'static str, String)> {
        let mut params = vec![];
        if let Some(offset) = self.offset {
            params.push(("offset", offset.to_string()));
        }
        if let Some(limit) = self.limit {
            params.push(("limit", limit.to_string()));
        }
        params
    }
}

/// Fetch a specific client by ID within a site.
/// Endpoint: GET /v1/sites/{site_id}/clients/{id}
#[derive(Debug, Clone)]
pub struct GetClient {
    pub site_id: String,
    pub id: String,
}

impl GetClient {
    pub fn new(site_id: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            site_id: site_id.into(),
            id: id.into(),
        }
    }
}

impl Endpoint for GetClient {
    const PATH: &'static str = "sites/{site_id}/clients/{id}";
    const METHOD: HttpMethod = HttpMethod::Get;
    type Response = SiteResponse<Client>;

    fn build_path(&self) -> String {
        format!("sites/{}/clients/{}", self.site_id, self.id)
    }
}

/// Client action types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ClientAction {
    /// Block the client from the network.
    Block,
    /// Unblock a previously blocked client.
    Unblock,
    /// Force the client to reconnect.
    Reconnect,
}

/// Request body for client actions.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct ClientActionRequest {
    action: ClientAction,
}

/// Execute an action on a client (block, unblock, reconnect).
/// Endpoint: POST /v1/sites/{siteId}/clients/{id}/action
#[derive(Debug, Clone)]
pub struct ExecuteClientAction {
    pub site_id: String,
    pub client_id: String,
    pub action: ClientAction,
}

impl ExecuteClientAction {
    pub fn new(
        site_id: impl Into<String>,
        client_id: impl Into<String>,
        action: ClientAction,
    ) -> Self {
        Self {
            site_id: site_id.into(),
            client_id: client_id.into(),
            action,
        }
    }

    /// Create a block action for a client.
    pub fn block(site_id: impl Into<String>, client_id: impl Into<String>) -> Self {
        Self::new(site_id, client_id, ClientAction::Block)
    }

    /// Create an unblock action for a client.
    pub fn unblock(site_id: impl Into<String>, client_id: impl Into<String>) -> Self {
        Self::new(site_id, client_id, ClientAction::Unblock)
    }

    /// Create a reconnect action for a client.
    pub fn reconnect(site_id: impl Into<String>, client_id: impl Into<String>) -> Self {
        Self::new(site_id, client_id, ClientAction::Reconnect)
    }
}

impl Endpoint for ExecuteClientAction {
    const PATH: &'static str = "sites/{site_id}/clients/{client_id}/action";
    const METHOD: HttpMethod = HttpMethod::Post;
    type Response = EmptyResponse;

    fn build_path(&self) -> String {
        format!("sites/{}/clients/{}/action", self.site_id, self.client_id)
    }

    fn request_body(&self) -> Result<Option<Value>, serde_json::Error> {
        Ok(Some(serde_json::to_value(ClientActionRequest {
            action: self.action,
        })?))
    }
}

use crate::api::endpoint::{Endpoint, HttpMethod};
use crate::response::{DeleteResponse, MutationResponse, SiteResponse};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Traffic matching list type.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TrafficListType {
    IpAddress,
    Domain,
    Port,
    Application,
}

/// Traffic matching list.
#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TrafficList {
    pub id: String,
    pub name: String,
    pub list_type: TrafficListType,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub entries: Vec<String>,
}

/// Fetch all traffic matching lists for a specific site.
/// Endpoint: GET /v1/sites/{siteId}/traffic-matching-lists
#[derive(Debug, Clone)]
pub struct GetTrafficLists {
    pub site_id: String,
}

impl GetTrafficLists {
    pub fn new(site_id: impl Into<String>) -> Self {
        Self {
            site_id: site_id.into(),
        }
    }
}

impl Endpoint for GetTrafficLists {
    const PATH: &'static str = "sites/{site_id}/traffic-matching-lists";
    const METHOD: HttpMethod = HttpMethod::Get;
    type Response = SiteResponse<TrafficList>;

    fn build_path(&self) -> String {
        format!("sites/{}/traffic-matching-lists", self.site_id)
    }
}

/// Fetch a specific traffic matching list by ID within a site.
/// Endpoint: GET /v1/sites/{siteId}/traffic-matching-lists/{id}
#[derive(Debug, Clone)]
pub struct GetTrafficList {
    pub site_id: String,
    pub id: String,
}

impl GetTrafficList {
    pub fn new(site_id: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            site_id: site_id.into(),
            id: id.into(),
        }
    }
}

impl Endpoint for GetTrafficList {
    const PATH: &'static str = "sites/{site_id}/traffic-matching-lists/{id}";
    const METHOD: HttpMethod = HttpMethod::Get;
    type Response = SiteResponse<TrafficList>;

    fn build_path(&self) -> String {
        format!("sites/{}/traffic-matching-lists/{}", self.site_id, self.id)
    }
}

/// Request body for creating or updating a traffic matching list.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TrafficListRequest {
    pub name: String,
    pub list_type: TrafficListType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<String>>,
}

impl TrafficListRequest {
    pub fn new(name: impl Into<String>, list_type: TrafficListType) -> Self {
        Self {
            name: name.into(),
            list_type,
            description: None,
            entries: None,
        }
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn entries(mut self, entries: Vec<String>) -> Self {
        self.entries = Some(entries);
        self
    }
}

/// Create a new traffic matching list within a site.
/// Endpoint: POST /v1/sites/{siteId}/traffic-matching-lists
#[derive(Debug, Clone)]
pub struct CreateTrafficList {
    pub site_id: String,
    pub request: TrafficListRequest,
}

impl CreateTrafficList {
    pub fn new(site_id: impl Into<String>, request: TrafficListRequest) -> Self {
        Self {
            site_id: site_id.into(),
            request,
        }
    }
}

impl Endpoint for CreateTrafficList {
    const PATH: &'static str = "sites/{site_id}/traffic-matching-lists";
    const METHOD: HttpMethod = HttpMethod::Post;
    type Response = MutationResponse<TrafficList>;

    fn build_path(&self) -> String {
        format!("sites/{}/traffic-matching-lists", self.site_id)
    }

    fn request_body(&self) -> Result<Option<Value>, serde_json::Error> {
        Ok(Some(serde_json::to_value(&self.request)?))
    }
}

/// Update an existing traffic matching list within a site.
/// Endpoint: PUT /v1/sites/{siteId}/traffic-matching-lists/{id}
#[derive(Debug, Clone)]
pub struct UpdateTrafficList {
    pub site_id: String,
    pub id: String,
    pub request: TrafficListRequest,
}

impl UpdateTrafficList {
    pub fn new(
        site_id: impl Into<String>,
        id: impl Into<String>,
        request: TrafficListRequest,
    ) -> Self {
        Self {
            site_id: site_id.into(),
            id: id.into(),
            request,
        }
    }
}

impl Endpoint for UpdateTrafficList {
    const PATH: &'static str = "sites/{site_id}/traffic-matching-lists/{id}";
    const METHOD: HttpMethod = HttpMethod::Put;
    type Response = MutationResponse<TrafficList>;

    fn build_path(&self) -> String {
        format!("sites/{}/traffic-matching-lists/{}", self.site_id, self.id)
    }

    fn request_body(&self) -> Result<Option<Value>, serde_json::Error> {
        Ok(Some(serde_json::to_value(&self.request)?))
    }
}

/// Delete a traffic matching list within a site.
/// Endpoint: DELETE /v1/sites/{siteId}/traffic-matching-lists/{id}
#[derive(Debug, Clone)]
pub struct DeleteTrafficList {
    pub site_id: String,
    pub id: String,
}

impl DeleteTrafficList {
    pub fn new(site_id: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            site_id: site_id.into(),
            id: id.into(),
        }
    }
}

impl Endpoint for DeleteTrafficList {
    const PATH: &'static str = "sites/{site_id}/traffic-matching-lists/{id}";
    const METHOD: HttpMethod = HttpMethod::Delete;
    type Response = DeleteResponse;

    fn build_path(&self) -> String {
        format!("sites/{}/traffic-matching-lists/{}", self.site_id, self.id)
    }
}

use crate::api::endpoint::{Endpoint, HttpMethod};
use crate::response::{DeleteResponse, MutationResponse, SiteResponse};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// ACL rule action.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AclAction {
    Allow,
    Block,
}

/// ACL rule.
#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AclRule {
    pub id: String,
    pub name: String,
    pub action: AclAction,
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub order: Option<i32>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub source_type: Option<String>,
    #[serde(default)]
    pub source_value: Option<String>,
    #[serde(default)]
    pub destination_type: Option<String>,
    #[serde(default)]
    pub destination_value: Option<String>,
}

/// Fetch all ACL rules for a specific site.
/// Endpoint: GET /v1/sites/{siteId}/acl-rules
#[derive(Debug, Clone)]
pub struct GetAclRules {
    pub site_id: String,
}

impl GetAclRules {
    pub fn new(site_id: impl Into<String>) -> Self {
        Self {
            site_id: site_id.into(),
        }
    }
}

impl Endpoint for GetAclRules {
    const PATH: &'static str = "sites/{site_id}/acl-rules";
    const METHOD: HttpMethod = HttpMethod::Get;
    type Response = SiteResponse<AclRule>;

    fn build_path(&self) -> String {
        format!("sites/{}/acl-rules", self.site_id)
    }
}

/// Fetch a specific ACL rule by ID within a site.
/// Endpoint: GET /v1/sites/{siteId}/acl-rules/{id}
#[derive(Debug, Clone)]
pub struct GetAclRule {
    pub site_id: String,
    pub id: String,
}

impl GetAclRule {
    pub fn new(site_id: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            site_id: site_id.into(),
            id: id.into(),
        }
    }
}

impl Endpoint for GetAclRule {
    const PATH: &'static str = "sites/{site_id}/acl-rules/{id}";
    const METHOD: HttpMethod = HttpMethod::Get;
    type Response = SiteResponse<AclRule>;

    fn build_path(&self) -> String {
        format!("sites/{}/acl-rules/{}", self.site_id, self.id)
    }
}

/// Request body for creating or updating an ACL rule.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AclRuleRequest {
    pub name: String,
    pub action: AclAction,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_value: Option<String>,
}

impl AclRuleRequest {
    pub fn new(name: impl Into<String>, action: AclAction) -> Self {
        Self {
            name: name.into(),
            action,
            enabled: None,
            order: None,
            description: None,
            source_type: None,
            source_value: None,
            destination_type: None,
            destination_value: None,
        }
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = Some(enabled);
        self
    }

    pub fn order(mut self, order: i32) -> Self {
        self.order = Some(order);
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn source(
        mut self,
        source_type: impl Into<String>,
        source_value: impl Into<String>,
    ) -> Self {
        self.source_type = Some(source_type.into());
        self.source_value = Some(source_value.into());
        self
    }

    pub fn destination(
        mut self,
        dest_type: impl Into<String>,
        dest_value: impl Into<String>,
    ) -> Self {
        self.destination_type = Some(dest_type.into());
        self.destination_value = Some(dest_value.into());
        self
    }
}

/// Create a new ACL rule within a site.
/// Endpoint: POST /v1/sites/{siteId}/acl-rules
#[derive(Debug, Clone)]
pub struct CreateAclRule {
    pub site_id: String,
    pub request: AclRuleRequest,
}

impl CreateAclRule {
    pub fn new(site_id: impl Into<String>, request: AclRuleRequest) -> Self {
        Self {
            site_id: site_id.into(),
            request,
        }
    }
}

impl Endpoint for CreateAclRule {
    const PATH: &'static str = "sites/{site_id}/acl-rules";
    const METHOD: HttpMethod = HttpMethod::Post;
    type Response = MutationResponse<AclRule>;

    fn build_path(&self) -> String {
        format!("sites/{}/acl-rules", self.site_id)
    }

    fn request_body(&self) -> Result<Option<Value>, serde_json::Error> {
        Ok(Some(serde_json::to_value(&self.request)?))
    }
}

/// Update an existing ACL rule within a site.
/// Endpoint: PUT /v1/sites/{siteId}/acl-rules/{id}
#[derive(Debug, Clone)]
pub struct UpdateAclRule {
    pub site_id: String,
    pub id: String,
    pub request: AclRuleRequest,
}

impl UpdateAclRule {
    pub fn new(site_id: impl Into<String>, id: impl Into<String>, request: AclRuleRequest) -> Self {
        Self {
            site_id: site_id.into(),
            id: id.into(),
            request,
        }
    }
}

impl Endpoint for UpdateAclRule {
    const PATH: &'static str = "sites/{site_id}/acl-rules/{id}";
    const METHOD: HttpMethod = HttpMethod::Put;
    type Response = MutationResponse<AclRule>;

    fn build_path(&self) -> String {
        format!("sites/{}/acl-rules/{}", self.site_id, self.id)
    }

    fn request_body(&self) -> Result<Option<Value>, serde_json::Error> {
        Ok(Some(serde_json::to_value(&self.request)?))
    }
}

/// Delete an ACL rule within a site.
/// Endpoint: DELETE /v1/sites/{siteId}/acl-rules/{id}
#[derive(Debug, Clone)]
pub struct DeleteAclRule {
    pub site_id: String,
    pub id: String,
}

impl DeleteAclRule {
    pub fn new(site_id: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            site_id: site_id.into(),
            id: id.into(),
        }
    }
}

impl Endpoint for DeleteAclRule {
    const PATH: &'static str = "sites/{site_id}/acl-rules/{id}";
    const METHOD: HttpMethod = HttpMethod::Delete;
    type Response = DeleteResponse;

    fn build_path(&self) -> String {
        format!("sites/{}/acl-rules/{}", self.site_id, self.id)
    }
}

/// Reorder ACL rules within a site.
/// Endpoint: PUT /v1/sites/{siteId}/acl-rules/order
#[derive(Debug, Clone)]
pub struct ReorderAclRules {
    pub site_id: String,
    pub rule_ids: Vec<String>,
}

impl ReorderAclRules {
    pub fn new(site_id: impl Into<String>, rule_ids: Vec<String>) -> Self {
        Self {
            site_id: site_id.into(),
            rule_ids,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ReorderRequest {
    rule_ids: Vec<String>,
}

impl Endpoint for ReorderAclRules {
    const PATH: &'static str = "sites/{site_id}/acl-rules/order";
    const METHOD: HttpMethod = HttpMethod::Put;
    type Response = SiteResponse<AclRule>;

    fn build_path(&self) -> String {
        format!("sites/{}/acl-rules/order", self.site_id)
    }

    fn request_body(&self) -> Result<Option<Value>, serde_json::Error> {
        Ok(Some(serde_json::to_value(ReorderRequest {
            rule_ids: self.rule_ids.clone(),
        })?))
    }
}

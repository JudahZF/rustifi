use crate::api::endpoint::{Endpoint, HttpMethod};
use crate::models::voucher::Voucher;
use crate::response::{DeleteResponse, MutationResponse, SiteResponse};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Fetch all vouchers for a specific site.
/// Endpoint: GET /v1/sites/{siteId}/hotspot/vouchers
#[derive(Debug, Clone)]
pub struct GetVouchers {
    pub site_id: String,
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

impl GetVouchers {
    pub fn new(site_id: impl Into<String>) -> Self {
        Self {
            site_id: site_id.into(),
            offset: None,
            limit: None,
        }
    }

    pub fn with_pagination(site_id: impl Into<String>, offset: usize, limit: usize) -> Self {
        Self {
            site_id: site_id.into(),
            offset: Some(offset),
            limit: Some(limit),
        }
    }

    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }
}

impl Endpoint for GetVouchers {
    const PATH: &'static str = "sites/{site_id}/hotspot/vouchers";
    const METHOD: HttpMethod = HttpMethod::Get;
    type Response = SiteResponse<Voucher>;

    fn build_path(&self) -> String {
        format!("sites/{}/hotspot/vouchers", self.site_id)
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

/// Fetch a specific voucher by ID within a site.
/// Endpoint: GET /v1/sites/{siteId}/hotspot/vouchers/{id}
#[derive(Debug, Clone)]
pub struct GetVoucher {
    pub site_id: String,
    pub id: String,
}

impl GetVoucher {
    pub fn new(site_id: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            site_id: site_id.into(),
            id: id.into(),
        }
    }
}

impl Endpoint for GetVoucher {
    const PATH: &'static str = "sites/{site_id}/hotspot/vouchers/{id}";
    const METHOD: HttpMethod = HttpMethod::Get;
    type Response = SiteResponse<Voucher>;

    fn build_path(&self) -> String {
        format!("sites/{}/hotspot/vouchers/{}", self.site_id, self.id)
    }
}

/// Request body for generating vouchers.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateVouchersRequest {
    pub count: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_minutes: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_limit_mb: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth_limit_down: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth_limit_up: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

impl GenerateVouchersRequest {
    /// Create a new request to generate a specified number of vouchers.
    pub fn new(count: u32) -> Self {
        Self {
            count,
            duration_minutes: None,
            data_limit_mb: None,
            bandwidth_limit_down: None,
            bandwidth_limit_up: None,
            note: None,
        }
    }

    /// Set the duration in minutes for the vouchers.
    pub fn duration_minutes(mut self, minutes: u32) -> Self {
        self.duration_minutes = Some(minutes);
        self
    }

    /// Set the data limit in MB for the vouchers.
    pub fn data_limit_mb(mut self, mb: u64) -> Self {
        self.data_limit_mb = Some(mb);
        self
    }

    /// Set the download bandwidth limit for the vouchers.
    pub fn bandwidth_limit_down(mut self, kbps: u64) -> Self {
        self.bandwidth_limit_down = Some(kbps);
        self
    }

    /// Set the upload bandwidth limit for the vouchers.
    pub fn bandwidth_limit_up(mut self, kbps: u64) -> Self {
        self.bandwidth_limit_up = Some(kbps);
        self
    }

    /// Set a note for the vouchers.
    pub fn note(mut self, note: impl Into<String>) -> Self {
        self.note = Some(note.into());
        self
    }
}

/// Response containing generated vouchers.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneratedVouchers {
    pub vouchers: Vec<Voucher>,
}

/// Generate new vouchers for a site.
/// Endpoint: POST /v1/sites/{siteId}/hotspot/vouchers
#[derive(Debug, Clone)]
pub struct GenerateVouchers {
    pub site_id: String,
    pub request: GenerateVouchersRequest,
}

impl GenerateVouchers {
    pub fn new(site_id: impl Into<String>, request: GenerateVouchersRequest) -> Self {
        Self {
            site_id: site_id.into(),
            request,
        }
    }
}

impl Endpoint for GenerateVouchers {
    const PATH: &'static str = "sites/{site_id}/hotspot/vouchers";
    const METHOD: HttpMethod = HttpMethod::Post;
    type Response = MutationResponse<GeneratedVouchers>;

    fn build_path(&self) -> String {
        format!("sites/{}/hotspot/vouchers", self.site_id)
    }

    fn request_body(&self) -> Result<Option<Value>, serde_json::Error> {
        Ok(Some(serde_json::to_value(&self.request)?))
    }
}

/// Delete a voucher within a site.
/// Endpoint: DELETE /v1/sites/{siteId}/hotspot/vouchers/{id}
#[derive(Debug, Clone)]
pub struct DeleteVoucher {
    pub site_id: String,
    pub id: String,
}

impl DeleteVoucher {
    pub fn new(site_id: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            site_id: site_id.into(),
            id: id.into(),
        }
    }
}

impl Endpoint for DeleteVoucher {
    const PATH: &'static str = "sites/{site_id}/hotspot/vouchers/{id}";
    const METHOD: HttpMethod = HttpMethod::Delete;
    type Response = DeleteResponse;

    fn build_path(&self) -> String {
        format!("sites/{}/hotspot/vouchers/{}", self.site_id, self.id)
    }
}

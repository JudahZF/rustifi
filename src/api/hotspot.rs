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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_vouchers_build_path() {
        let endpoint = GetVouchers::new("site-123");
        assert_eq!(endpoint.build_path(), "sites/site-123/hotspot/vouchers");
    }

    #[test]
    fn test_get_vouchers_path_and_method() {
        assert_eq!(GetVouchers::PATH, "sites/{site_id}/hotspot/vouchers");
        assert!(matches!(GetVouchers::METHOD, HttpMethod::Get));
    }

    #[test]
    fn test_get_vouchers_no_pagination() {
        let endpoint = GetVouchers::new("site-123");
        let params = endpoint.query_params();
        assert!(params.is_empty());
    }

    #[test]
    fn test_get_vouchers_with_pagination() {
        let endpoint = GetVouchers::with_pagination("site-123", 10, 25);
        let params = endpoint.query_params();
        assert_eq!(params.len(), 2);
        assert!(params.contains(&("offset", "10".to_string())));
        assert!(params.contains(&("limit", "25".to_string())));
    }

    #[test]
    fn test_get_vouchers_offset_only() {
        let endpoint = GetVouchers::new("site-123").offset(5);
        let params = endpoint.query_params();
        assert_eq!(params.len(), 1);
        assert!(params.contains(&("offset", "5".to_string())));
    }

    #[test]
    fn test_get_vouchers_limit_only() {
        let endpoint = GetVouchers::new("site-123").limit(50);
        let params = endpoint.query_params();
        assert_eq!(params.len(), 1);
        assert!(params.contains(&("limit", "50".to_string())));
    }

    #[test]
    fn test_get_voucher_build_path() {
        let endpoint = GetVoucher::new("site-123", "voucher-456");
        assert_eq!(
            endpoint.build_path(),
            "sites/site-123/hotspot/vouchers/voucher-456"
        );
    }

    #[test]
    fn test_get_voucher_path_and_method() {
        assert_eq!(GetVoucher::PATH, "sites/{site_id}/hotspot/vouchers/{id}");
        assert!(matches!(GetVoucher::METHOD, HttpMethod::Get));
    }

    #[test]
    fn test_generate_vouchers_build_path() {
        let request = GenerateVouchersRequest::new(5);
        let endpoint = GenerateVouchers::new("site-123", request);
        assert_eq!(endpoint.build_path(), "sites/site-123/hotspot/vouchers");
    }

    #[test]
    fn test_generate_vouchers_path_and_method() {
        assert_eq!(GenerateVouchers::PATH, "sites/{site_id}/hotspot/vouchers");
        assert!(matches!(GenerateVouchers::METHOD, HttpMethod::Post));
    }

    #[test]
    fn test_generate_vouchers_request_body_minimal() {
        let request = GenerateVouchersRequest::new(3);
        let endpoint = GenerateVouchers::new("site-123", request);
        let body = endpoint.request_body().unwrap().unwrap();

        assert_eq!(body["count"], 3);
        assert!(body.get("durationMinutes").is_none());
        assert!(body.get("dataLimitMb").is_none());
        assert!(body.get("bandwidthLimitDown").is_none());
        assert!(body.get("bandwidthLimitUp").is_none());
        assert!(body.get("note").is_none());
    }

    #[test]
    fn test_generate_vouchers_request_body_full() {
        let request = GenerateVouchersRequest::new(10)
            .duration_minutes(60)
            .data_limit_mb(1024)
            .bandwidth_limit_down(10000)
            .bandwidth_limit_up(5000)
            .note("Test vouchers");
        let endpoint = GenerateVouchers::new("site-123", request);
        let body = endpoint.request_body().unwrap().unwrap();

        assert_eq!(body["count"], 10);
        assert_eq!(body["durationMinutes"], 60);
        assert_eq!(body["dataLimitMb"], 1024);
        assert_eq!(body["bandwidthLimitDown"], 10000);
        assert_eq!(body["bandwidthLimitUp"], 5000);
        assert_eq!(body["note"], "Test vouchers");
    }

    #[test]
    fn test_delete_voucher_build_path() {
        let endpoint = DeleteVoucher::new("site-123", "voucher-789");
        assert_eq!(
            endpoint.build_path(),
            "sites/site-123/hotspot/vouchers/voucher-789"
        );
    }

    #[test]
    fn test_delete_voucher_path_and_method() {
        assert_eq!(DeleteVoucher::PATH, "sites/{site_id}/hotspot/vouchers/{id}");
        assert!(matches!(DeleteVoucher::METHOD, HttpMethod::Delete));
    }
}

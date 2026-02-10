use crate::api::endpoint::{Endpoint, HttpMethod};
use crate::models::Site;
use crate::response::ApiResponse;

/// Fetches all sites from the controller.
#[derive(Debug, Default)]
pub struct GetSites;

impl Endpoint for GetSites {
    const PATH: &'static str = "sites";
    const METHOD: HttpMethod = HttpMethod::Get;
    type Response = ApiResponse<Vec<Site>>;
}

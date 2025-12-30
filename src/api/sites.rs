use crate::api::endpoint::{Endpoint, HttpMethod};
use crate::models::Site;
use crate::response::ApiResponse;

#[derive(Debug, Default)]
pub struct GetSites;

impl Endpoint for GetSites {
    const PATH: &'static str = "sites";
    const METHOD: HttpMethod = HttpMethod::Get;
    type Response = ApiResponse<Vec<Site>>;
}

use serde::Deserialize;
use serde_json::Value;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

pub trait Endpoint {
    const PATH: &'static str;
    const METHOD: HttpMethod;
    type Response: for<'a> Deserialize<'a>;

    /// Build the actual path for this endpoint.
    /// Override this method when the endpoint has dynamic path parameters.
    fn build_path(&self) -> String {
        Self::PATH.to_string()
    }

    /// Return query parameters for this endpoint.
    /// Override this method to add query parameters like pagination.
    fn query_params(&self) -> Vec<(&'static str, String)> {
        vec![]
    }

    /// Return JSON body for POST/PUT/PATCH requests.
    /// Override this method when the endpoint needs a request body.
    /// Returns `Err` if serialization fails, allowing the error to propagate.
    fn request_body(&self) -> Result<Option<Value>, serde_json::Error> {
        Ok(None)
    }
}

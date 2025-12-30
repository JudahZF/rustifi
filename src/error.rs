use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Authentication failed: {0}")]
    Auth(String),

    #[error("API request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("Response parsing failed: {0}")]
    Parse(String),

    #[error("Endpoint not found: {0}")]
    NotFound(String),

    #[error("Invalid state: {0}")]
    InvalidState(String),

    #[error("CSRF token missing")]
    MissingCsrfToken,

    #[error("URL parsing failed: {0}")]
    UrlParse(#[from] url::ParseError),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

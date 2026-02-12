use thiserror::Error;

#[derive(Error, Debug)]
pub enum KsefError {
    #[error("HTTP request error: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("API error: HTTP {0} - {1}")]
    ApiError(u16, String),

    #[error("JSON processing error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("OpenSSL error: {0}")]
    OpenSslError(#[from] openssl::error::ErrorStack),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Request timed out")]
    TimeoutError,

    #[error("Runtime initialization error: {0}")]
    RuntimeError(String),

    #[error("Invalid response: {0}")]
    InvalidResponse(String),

    #[error("Application error: Code {0} - {1}")]
    ApplicationError(i32, String),

    #[error("Unexpected error: {0}")]
    Unexpected(String),
}

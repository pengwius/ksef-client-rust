use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum KsefError {
    #[error("Transport error: {0}")]
    TransportError(String),

    #[error("API error: HTTP {0} - {1}")]
    ApiError(u16, String),

    #[error("Failed to deserialize response: {0}")]
    DeserializeError(String),

    #[error("Request timed out")]
    TimeoutError,

    #[error("Unexpected error: {0}")]
    Unexpected(String),
}

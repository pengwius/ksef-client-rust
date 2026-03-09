use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Clone, Deserialize)]
pub struct KsefApiException {
    pub exception: KsefException,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KsefException {
    #[serde(rename = "serviceCtx")]
    pub service_ctx: String,
    #[serde(rename = "serviceCode")]
    pub service_code: String,
    #[serde(rename = "serviceName")]
    pub service_name: String,
    pub timestamp: String,
    #[serde(rename = "referenceNumber")]
    pub reference_number: Option<String>,
    #[serde(rename = "exceptionDetailList")]
    pub exception_detail_list: Vec<KsefExceptionDetail>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KsefExceptionDetail {
    #[serde(rename = "exceptionCode")]
    pub exception_code: i32,
    #[serde(rename = "exceptionDescription")]
    pub exception_description: String,
}

#[derive(Error, Debug)]
pub enum KsefError {
    #[error("HTTP request error: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("API error: HTTP {0} - {1:?}")]
    ApiError(u16, KsefApiException),

    #[error("API error (raw): HTTP {0} - {1}")]
    ApiErrorRaw(u16, String),

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

impl KsefError {
    pub fn from_api_response(code: u16, body: String) -> Self {
        match serde_json::from_str::<KsefApiException>(&body) {
            Ok(api_exception) => KsefError::ApiError(code, api_exception),
            Err(_) => KsefError::ApiErrorRaw(code, body),
        }
    }
}

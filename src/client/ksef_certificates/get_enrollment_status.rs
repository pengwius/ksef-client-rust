use crate::client::KsefClient;
use crate::client::error::KsefError;
use crate::client::routes;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnrollmentStatusResponse {
    pub request_date: DateTime<Utc>,
    pub status: EnrollmentStatus,
    pub certificate_serial_number: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnrollmentStatus {
    pub code: i32,
    pub description: String,
    pub details: Option<Vec<String>>,
}

pub fn get_enrollment_status(
    client: &KsefClient,
    reference_number: &str,
) -> Result<EnrollmentStatusResponse, KsefError> {
    let fut = async {
        let url = format!(
            "{}{}",
            client.url_for(routes::CERTIFICATES_ENROLLMENT_STATUS_PATH),
            reference_number
        );

        let access_token = &client.access_token.access_token;

        let resp = client
            .client
            .get(&url)
            .header("Accept", "application/json")
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(KsefError::RequestError)?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(KsefError::ApiError(status.as_u16(), body));
        }

        let parsed: EnrollmentStatusResponse =
            resp.json().await.map_err(KsefError::RequestError)?;
        Ok(parsed)
    };

    match tokio::runtime::Handle::try_current() {
        Ok(handle) => handle.block_on(fut),
        Err(_) => {
            let rt = tokio::runtime::Runtime::new()
                .map_err(|e| KsefError::RuntimeError(e.to_string()))?;
            rt.block_on(fut)
        }
    }
}

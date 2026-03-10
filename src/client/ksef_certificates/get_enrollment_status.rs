use crate::client::KsefClient;
use crate::client::error::KsefError;
use crate::client::routes;
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::time::Duration;
use tokio::time::sleep;

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

pub async fn get_enrollment_status(
    client: &KsefClient,
    reference_number: &str,
) -> Result<EnrollmentStatusResponse, KsefError> {
    let url = format!(
        "{}{}",
        client.url_for(routes::CERTIFICATES_ENROLLMENT_STATUS_PATH),
        reference_number
    );

    let access_token = client.access_token.access_token.expose_secret();
    let timeout = Duration::from_secs(60);
    let interval = Duration::from_secs(2);
    let start = std::time::Instant::now();

    loop {
        if start.elapsed() > timeout {
            return Err(KsefError::RuntimeError(
                "Enrollment status polling timed out".to_string(),
            ));
        }

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
            return Err(KsefError::from_api_response(status.as_u16(), body));
        }

        let parsed: EnrollmentStatusResponse =
            resp.json().await.map_err(KsefError::RequestError)?;

        if parsed.status.code == 100 {
            sleep(interval).await;
            continue;
        }

        return Ok(parsed);
    }
}

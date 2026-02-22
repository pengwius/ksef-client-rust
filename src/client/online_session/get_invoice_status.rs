use crate::client::KsefClient;
use crate::client::error::KsefError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInvoiceStatusResponse {
    pub ordinal_number: i32,
    pub invoice_number: Option<String>,
    #[serde(rename = "ksefNumber")]
    pub ksef_number: Option<String>,
    #[serde(rename = "referenceNumber")]
    pub reference_number: String,
    pub invoice_hash: String,
    pub invoice_file_name: Option<String>,
    pub acquisition_date: Option<DateTime<Utc>>,
    pub invoicing_date: DateTime<Utc>,
    pub permanent_storage_date: Option<DateTime<Utc>>,
    pub upo_download_url: Option<String>,
    pub upo_download_url_expiration_date: Option<DateTime<Utc>>,
    pub invoicing_mode: Option<String>,
    #[serde(rename = "status")]
    pub invoice_status: InvoiceStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceStatus {
    pub code: i32,
    pub description: String,
    pub details: Option<Vec<String>>,
    pub extensions: Option<HashMap<String, String>>,
}

pub fn get_invoice_status(
    client: &KsefClient,
    session_reference_number: &str,
    invoice_reference_number: &str,
) -> Result<GetInvoiceStatusResponse, KsefError> {
    let fut = async {
        let url = client.url_for(&format!(
            "/v2/sessions/{}/invoices/{}",
            session_reference_number, invoice_reference_number
        ));

        let access_token = &client.access_token.access_token;
        if access_token.is_empty() {
            return Err(KsefError::ApplicationError(
                0,
                "No access token available".to_string(),
            ));
        }

        let timeout = Duration::from_secs(120);
        let interval = Duration::from_secs(2);
        let start = std::time::Instant::now();

        loop {
            if start.elapsed() > timeout {
                return Err(KsefError::RuntimeError(
                    "Invoice status polling timed out".to_string(),
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
                return Err(KsefError::ApiError(status.as_u16(), body));
            }

            let parsed: GetInvoiceStatusResponse =
                resp.json().await.map_err(KsefError::RequestError)?;

            if parsed.invoice_status.code == 100 || parsed.invoice_status.code == 150 {
                sleep(interval).await;
                continue;
            }

            return Ok(parsed);
        }
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

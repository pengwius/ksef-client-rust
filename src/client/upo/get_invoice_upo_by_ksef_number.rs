use crate::client::KsefClient;
use crate::client::error::KsefError;
use crate::client::routes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetInvoiceUpoByKsefNumberResponse {
    pub content: String,

    pub hash: String,
}

pub async fn get_invoice_upo_by_ksef_number(
    client: &KsefClient,
    reference_number: &str,
    ksef_number: &str,
) -> Result<GetInvoiceUpoByKsefNumberResponse, KsefError> {
    let sessions_segment = routes::SESSIONS_PATH
        .trim_start_matches('/')
        .trim_end_matches('/');
    let path = format!(
        "{}/{}/invoices/ksef/{}/upo",
        sessions_segment, reference_number, ksef_number
    );
    let url = client.url_for(&path);

    let access_token = &client.access_token.access_token;
    if access_token.is_empty() {
        return Err(KsefError::ApplicationError(
            0,
            "No access token available. Please authenticate and redeem token first.".to_string(),
        ));
    }

    let resp = client
        .client
        .get(&url)
        .header("Accept", "application/xml")
        .bearer_auth(access_token)
        .send()
        .await
        .map_err(KsefError::RequestError)?;

    let status = resp.status();
    if !status.is_success() {
        let body = resp.text().await.unwrap_or_default();
        return Err(KsefError::ApiError(status.as_u16(), body));
    }

    let hash = resp
        .headers()
        .get("x-ms-meta-hash")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_default();

    let content = resp.text().await.map_err(KsefError::RequestError)?;

    Ok(GetInvoiceUpoByKsefNumberResponse { content, hash })
}

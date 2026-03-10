use crate::client::KsefClient;
use crate::client::error::KsefError;
use crate::client::routes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FetchInvoiceResponse {
    pub content: Vec<u8>,
    pub hash: String,
}

pub async fn fetch_invoice(
    client: &KsefClient,
    ksef_number: &str,
) -> Result<FetchInvoiceResponse, KsefError> {
    let url = client.url_for(&format!("{}/{}", routes::INVOICES_PATH, ksef_number));
    let http = &client.client;

    let token = KsefClient::secret_str(&client.access_token.access_token);
    if token.is_empty() {
        return Err(KsefError::ApplicationError(
            0,
            "No access token available. Please authenticate and redeem token first.".to_string(),
        ));
    }

    let resp = http
        .get(&url)
        .header("Accept", "application/octet-stream")
        .bearer_auth(token)
        .send()
        .await?;

    let status = resp.status();

    if !status.is_success() {
        let code = status.as_u16();
        let body = resp.text().await.unwrap_or_default();
        return Err(KsefError::from_api_response(code, body));
    }

    let hash = resp
        .headers()
        .get("x-ms-meta-hash")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_default();

    let content = resp.bytes().await?.to_vec();

    Ok(FetchInvoiceResponse { content, hash })
}

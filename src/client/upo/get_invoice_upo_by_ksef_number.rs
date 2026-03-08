use crate::client::KsefClient;
use crate::client::error::KsefError;
use crate::client::routes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetInvoiceUpoResponse {
    pub content: String,

    pub hash: String,
}

#[derive(Debug, Clone)]
pub enum InvoiceIdentifier {
    KsefNumber(String),
    InvoiceReference(String),
}

impl From<&str> for InvoiceIdentifier {
    fn from(s: &str) -> Self {
        InvoiceIdentifier::InvoiceReference(s.to_string())
    }
}

pub async fn get_invoice_upo(
    client: &KsefClient,
    reference_number: &str,
    identifier: InvoiceIdentifier,
) -> Result<GetInvoiceUpoResponse, KsefError> {
    let sessions_segment = routes::SESSIONS_PATH
        .trim_start_matches('/')
        .trim_end_matches('/');

    let path = match identifier {
        InvoiceIdentifier::KsefNumber(k) => {
            format!(
                "{}/{}/invoices/ksef/{}/upo",
                sessions_segment, reference_number, k
            )
        }
        InvoiceIdentifier::InvoiceReference(r) => {
            format!(
                "{}/{}/invoices/{}/upo",
                sessions_segment, reference_number, r
            )
        }
    };

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

    Ok(GetInvoiceUpoResponse { content, hash })
}

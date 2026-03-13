use crate::client::KsefClient;
use crate::client::error::KsefError;
use crate::client::online_session::encryption::{EncryptionData, encrypt_invoice, hash_invoice};
use crate::client::routes;
use crate::client::types::ReferenceNumber;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct SendInvoiceRequest {
    #[serde(rename = "invoiceHash")]
    pub invoice_hash: String,
    #[serde(rename = "invoiceSize")]
    pub invoice_size: u64,
    #[serde(rename = "encryptedInvoiceHash")]
    pub encrypted_invoice_hash: String,
    #[serde(rename = "encryptedInvoiceSize")]
    pub encrypted_invoice_size: u64,
    #[serde(rename = "encryptedInvoiceContent")]
    pub encrypted_invoice_content: String,
    #[serde(rename = "offlineMode")]
    pub offline_mode: bool,
    #[serde(rename = "hashOfCorrectedInvoice")]
    pub hash_of_corrected_invoice: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SendInvoiceResponse {
    #[serde(rename = "referenceNumber")]
    pub reference_number: String,
}

pub async fn send_invoice(
    client: &KsefClient,
    session_reference_number: &ReferenceNumber,
    invoice_xml: &[u8],
    encryption_data: &EncryptionData,
) -> Result<SendInvoiceResponse, KsefError> {
    let invoice_hash = hash_invoice(invoice_xml);
    let invoice_size = invoice_xml.len() as u64;

    let encrypted_content = encrypt_invoice(
        invoice_xml,
        &encryption_data.symmetric_key,
        &encryption_data.initialization_vector,
    )?;

    let encrypted_invoice_hash = hash_invoice(&encrypted_content);
    let encrypted_invoice_size = encrypted_content.len() as u64;

    let request = SendInvoiceRequest {
        invoice_hash: BASE64.encode(invoice_hash),
        invoice_size,
        encrypted_invoice_hash: BASE64.encode(encrypted_invoice_hash),
        encrypted_invoice_size,
        encrypted_invoice_content: BASE64.encode(encrypted_content),
        offline_mode: false,
        hash_of_corrected_invoice: None,
    };
    let url = client.url_for(&format!(
        "{}/{}/invoices",
        routes::SESSIONS_ONLINE_PATH,
        session_reference_number
    ));
    let http = &client.client;

    let token = KsefClient::secret_str(&client.access_token.access_token);
    if token.is_empty() {
        return Err(KsefError::ApplicationError(
            0,
            "No access token available".to_string(),
        ));
    }

    let resp = http
        .post(&url)
        .header("Accept", "application/json")
        .bearer_auth(token)
        .json(&request)
        .send()
        .await?;

    let status = resp.status();

    if !status.is_success() {
        let code = status.as_u16();
        let body = resp.text().await.unwrap_or_default();
        return Err(KsefError::from_api_response(code, body));
    }

    let parsed: SendInvoiceResponse = resp.json().await?;

    Ok(parsed)
}

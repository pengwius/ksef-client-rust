use crate::client::KsefClient;
use crate::client::error::KsefError;
use crate::client::fetching_invoices::fetch_invoice_metadata::{InvoiceMetadata, QueryCriteria};
use crate::client::routes;
use crate::client::traits::*;
use crate::client::types::ReferenceNumber;
use openssl::symm::{Cipher, decrypt};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug, Serialize)]
pub struct ExportInvoicesRequest {
    #[serde(rename = "encryption")]
    pub encryption: EncryptionInfo,
    #[serde(rename = "filters")]
    pub filters: QueryCriteria,
}

#[derive(Debug, Serialize)]
pub struct EncryptionInfo {
    #[serde(rename = "encryptedSymmetricKey")]
    pub encrypted_symmetric_key: String,
    #[serde(rename = "initializationVector")]
    pub initialization_vector: String,
}

#[derive(Debug, Deserialize)]
pub struct ExportInvoicesResponse {
    #[serde(rename = "referenceNumber")]
    pub reference_number: ReferenceNumber,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ExportInvoicesStatusResponse {
    #[serde(rename = "status")]
    pub status: ExportStatus,
    #[serde(rename = "package")]
    pub package: Option<ExportPackage>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ExportStatus {
    #[serde(rename = "code")]
    pub code: i32,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "details")]
    pub details: Option<Vec<String>>,
    #[serde(rename = "completedDate")]
    pub completed_date: Option<String>,
    #[serde(rename = "packageExpirationDate")]
    pub package_expiration_date: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ExportPackage {
    #[serde(rename = "invoiceCount")]
    pub invoice_count: i64,
    #[serde(rename = "size")]
    pub size: i64,
    #[serde(rename = "parts")]
    pub parts: Vec<ExportPackagePart>,
    #[serde(rename = "isTruncated")]
    pub is_truncated: bool,
    #[serde(rename = "lastIssueDate")]
    pub last_issue_date: Option<String>,
    #[serde(rename = "lastInvoicingDate")]
    pub last_invoicing_date: Option<String>,
    #[serde(rename = "lastPermanentStorageDate")]
    pub last_permanent_storage_date: Option<String>,
    #[serde(rename = "permanentStorageHwmDate")]
    pub permanent_storage_hwm_date: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ExportPackagePart {
    #[serde(rename = "ordinalNumber")]
    pub ordinal_number: i32,
    #[serde(rename = "partName")]
    pub part_name: String,
    #[serde(rename = "method")]
    pub method: String,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "partSize")]
    pub part_size: i64,
    #[serde(rename = "partHash")]
    pub part_hash: String,
    #[serde(rename = "encryptedPartSize")]
    pub encrypted_part_size: i64,
    #[serde(rename = "encryptedPartHash")]
    pub encrypted_part_hash: String,
    #[serde(rename = "expirationDate")]
    pub expiration_date: String,
}

#[derive(Debug)]
pub struct ExportedPart {
    pub metadata: ExportPackagePart,
    pub content: Vec<u8>,
}

#[derive(Debug)]
pub struct ExportResult {
    pub status: ExportInvoicesStatusResponse,
    pub parts: Vec<ExportedPart>,
}

#[derive(Debug, Deserialize)]
pub struct InvoicePackageMetadata {
    #[serde(rename = "invoices")]
    pub invoices: Vec<InvoiceMetadata>,
}

pub async fn start_export_invoices(
    client: &KsefClient,
    request: ExportInvoicesRequest,
) -> Result<ExportInvoicesResponse, KsefError> {
    let url = client.url_for(routes::INVOICES_EXPORTS_PATH);
    let http = &client.client;

    let token = KsefClient::secret_str(&client.access_token.access_token);
    if token.is_empty() {
        return Err(KsefError::ApplicationError(
            0,
            "No access token available. Please authenticate and redeem token first.".to_string(),
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

    let parsed: ExportInvoicesResponse = resp.json().await?;

    Ok(parsed)
}

pub async fn get_export_status(
    client: &KsefClient,
    reference_number: &ReferenceNumber,
) -> Result<ExportInvoicesStatusResponse, KsefError> {
    let url = client.url_for(&format!(
        "{}/{}",
        routes::INVOICES_EXPORTS_PATH,
        reference_number
    ));
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
        .header("Accept", "application/json")
        .bearer_auth(token)
        .send()
        .await?;

    let status = resp.status();

    if !status.is_success() {
        let code = status.as_u16();
        let body = resp.text().await.unwrap_or_default();
        return Err(KsefError::from_api_response(code, body));
    }

    let parsed: ExportInvoicesStatusResponse = resp.json().await?;

    Ok(parsed)
}

pub async fn export_invoices(
    client: &KsefClient,
    query: QueryCriteria,
) -> Result<ExportResult, KsefError> {
    use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

    let encryption_data = client.generate_encryption_data().await?;

    let request = ExportInvoicesRequest {
        encryption: EncryptionInfo {
            encrypted_symmetric_key: BASE64.encode(&encryption_data.encrypted_symmetric_key),
            initialization_vector: BASE64.encode(&encryption_data.initialization_vector),
        },
        filters: query,
    };

    let export_response = start_export_invoices(client, request).await?;
    let reference_number = export_response.reference_number;

    let interval = Duration::from_secs(5);
    let timeout_duration = Duration::from_secs(300);
    let start_time = std::time::Instant::now();

    loop {
        if start_time.elapsed() > timeout_duration {
            return Err(KsefError::RuntimeError(
                "Export invoices polling timed out".to_string(),
            ));
        }

        let status_response = get_export_status(client, &reference_number).await?;

        match status_response.status.code {
            200 => {
                let status_clone = status_response.clone();
                if let Some(package) = status_response.package {
                    let mut decrypted_parts = Vec::new();
                    for part in package.parts {
                        let part_bytes = client
                            .client
                            .get(&part.url)
                            .send()
                            .await?
                            .bytes()
                            .await?
                            .to_vec();

                        let decrypted_part = decrypt(
                            Cipher::aes_256_cbc(),
                            &encryption_data.symmetric_key,
                            Some(&encryption_data.initialization_vector),
                            &part_bytes,
                        )
                        .map_err(|e| {
                            KsefError::ApplicationError(
                                0,
                                format!("Failed to decrypt export part: {}", e),
                            )
                        })?;

                        decrypted_parts.push(ExportedPart {
                            metadata: part,
                            content: decrypted_part,
                        });
                    }
                    return Ok(ExportResult {
                        status: status_clone,
                        parts: decrypted_parts,
                    });
                } else {
                    return Err(KsefError::ApplicationError(
                        0,
                        "Export status 200 but no package details found".to_string(),
                    ));
                }
            }
            100 => {
                sleep(interval).await;
                continue;
            }
            code => {
                return Err(KsefError::ApiErrorRaw(
                    code as u16,
                    format!(
                        "Export failed with status {}: {}",
                        code, status_response.status.description
                    ),
                ));
            }
        }
    }
}

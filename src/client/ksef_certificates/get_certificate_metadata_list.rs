use crate::client::KsefClient;
use crate::client::error::KsefError;
use crate::client::ksef_certificates::enroll_certificate::CertificateType;
use crate::client::routes;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetCertificateMetadataListRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_serial_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub certificate_type: Option<CertificateType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CertificateStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CertificateStatus {
    Active,
    Blocked,
    Revoked,
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCertificateMetadataListResponse {
    pub certificates: Vec<CertificateListItem>,
    pub has_more: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CertificateListItem {
    pub certificate_serial_number: String,
    pub name: String,
    #[serde(rename = "type")]
    pub certificate_type: CertificateType,
    pub common_name: String,
    pub status: CertificateStatus,
    pub subject_identifier: CertificateSubjectIdentifier,
    pub valid_from: DateTime<Utc>,
    pub valid_to: DateTime<Utc>,
    pub last_use_date: Option<DateTime<Utc>>,
    pub request_date: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CertificateSubjectIdentifier {
    #[serde(rename = "type")]
    pub id_type: CertificateSubjectIdentifierType,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CertificateSubjectIdentifierType {
    Nip,
    Pesel,
    Fingerprint,
}

pub fn get_certificate_metadata_list(
    client: &KsefClient,
    query: GetCertificateMetadataListRequest,
    page_size: Option<i32>,
    page_offset: Option<i32>,
) -> Result<GetCertificateMetadataListResponse, KsefError> {
    let fut = async {
        let url = client.url_for(routes::CERTIFICATES_QUERY_PATH);

        let mut query_params = Vec::new();
        if let Some(size) = page_size {
            query_params.push(("pageSize", size.to_string()));
        }
        if let Some(offset) = page_offset {
            query_params.push(("pageOffset", offset.to_string()));
        }

        let access_token = &client.access_token.access_token;

        let resp = client
            .client
            .post(&url)
            .query(&query_params)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .bearer_auth(access_token)
            .json(&query)
            .send()
            .await
            .map_err(KsefError::RequestError)?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(KsefError::ApiError(status.as_u16(), body));
        }

        let parsed: GetCertificateMetadataListResponse =
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

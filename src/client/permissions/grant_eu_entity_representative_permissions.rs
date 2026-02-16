use crate::client::KsefClient;
use crate::client::error::KsefError;
use crate::client::routes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrantEuEntityRepresentativePermissionsRequest {
    pub subject_identifier: EuEntityRepresentativeSubjectIdentifier,
    pub permissions: Vec<EuEntityRepresentativePermissionType>,
    pub description: String,
    pub subject_details: EuEntityRepresentativeSubjectDetails,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EuEntityRepresentativeSubjectIdentifier {
    #[serde(rename = "type")]
    pub identifier_type: EuEntityRepresentativeSubjectIdentifierType,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EuEntityRepresentativeSubjectIdentifierType {
    Fingerprint,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EuEntityRepresentativePermissionType {
    InvoiceWrite,
    InvoiceRead,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EuEntityRepresentativeSubjectDetails {
    pub subject_details_type: EuEntityRepresentativeSubjectDetailsType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person_by_fp_with_id: Option<EuEntityRepresentativePersonByFpWithId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person_by_fp_no_id: Option<EuEntityRepresentativePersonByFpNoId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_by_fp: Option<EuEntityRepresentativeEntityByFp>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EuEntityRepresentativeSubjectDetailsType {
    PersonByFingerprintWithIdentifier,
    PersonByFingerprintWithoutIdentifier,
    EntityByFingerprint,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EuEntityRepresentativePersonByFpWithId {
    pub first_name: String,
    pub last_name: String,
    pub identifier: EuEntityRepresentativePersonIdentifier,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EuEntityRepresentativePersonIdentifier {
    #[serde(rename = "type")]
    pub identifier_type: EuEntityRepresentativePersonIdentifierType,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EuEntityRepresentativePersonIdentifierType {
    Nip,
    Pesel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EuEntityRepresentativePersonByFpNoId {
    pub first_name: String,
    pub last_name: String,
    pub birth_date: String,
    pub id_document: EuEntityRepresentativeIdDocument,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EuEntityRepresentativeIdDocument {
    #[serde(rename = "type")]
    pub document_type: String,
    pub number: String,
    pub country: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EuEntityRepresentativeEntityByFp {
    pub full_name: String,
    pub address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrantEuEntityRepresentativePermissionsResponse {
    pub reference_number: String,
}

pub fn grant_eu_entity_representative_permissions(
    client: &KsefClient,
    request: GrantEuEntityRepresentativePermissionsRequest,
) -> Result<GrantEuEntityRepresentativePermissionsResponse, KsefError> {
    let fut = async {
        let url = client.url_for(routes::PERMISSIONS_EU_ENTITIES_REPRESENTATIVE_GRANTS_PATH);
        let access_token = &client.access_token.access_token;

        let resp = client
            .client
            .post(&url)
            .header("Accept", "application/json")
            .bearer_auth(access_token)
            .json(&request)
            .send()
            .await
            .map_err(KsefError::RequestError)?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(KsefError::ApiError(status.as_u16(), body));
        }

        let parsed: GrantEuEntityRepresentativePermissionsResponse =
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

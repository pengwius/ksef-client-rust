use crate::client::KsefClient;
use crate::client::error::KsefError;
use crate::client::routes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrantEuEntityPermissionsRequest {
    pub subject_identifier: EuEntitySubjectIdentifier,
    pub context_identifier: EuEntityContextIdentifier,
    pub description: String,
    pub eu_entity_name: String,
    pub subject_details: EuEntitySubjectDetails,
    pub eu_entity_details: EuEntityDetails,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EuEntitySubjectIdentifier {
    #[serde(rename = "type")]
    pub identifier_type: EuEntitySubjectIdentifierType,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EuEntitySubjectIdentifierType {
    Fingerprint,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EuEntityContextIdentifier {
    #[serde(rename = "type")]
    pub identifier_type: EuEntityContextIdentifierType,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EuEntityContextIdentifierType {
    NipVatUe,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EuEntitySubjectDetails {
    pub subject_details_type: EuEntitySubjectDetailsType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person_by_fp_with_id: Option<EuEntityPersonByFpWithId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person_by_fp_no_id: Option<EuEntityPersonByFpNoId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_by_fp: Option<EuEntityByFp>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EuEntitySubjectDetailsType {
    PersonByFingerprintWithIdentifier,
    PersonByFingerprintWithoutIdentifier,
    EntityByFingerprint,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EuEntityPersonByFpWithId {
    pub first_name: String,
    pub last_name: String,
    pub identifier: EuEntityPersonIdentifier,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EuEntityPersonIdentifier {
    #[serde(rename = "type")]
    pub identifier_type: EuEntityPersonIdentifierType,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EuEntityPersonIdentifierType {
    Nip,
    Pesel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EuEntityPersonByFpNoId {
    pub first_name: String,
    pub last_name: String,
    pub birth_date: String,
    pub id_document: EuEntityIdDocument,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EuEntityIdDocument {
    #[serde(rename = "type")]
    pub document_type: String,
    pub number: String,
    pub country: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EuEntityByFp {
    pub full_name: String,
    pub address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EuEntityDetails {
    pub full_name: String,
    pub address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrantEuEntityPermissionsResponse {
    pub reference_number: String,
}

pub fn grant_eu_entity_permissions(
    client: &KsefClient,
    request: GrantEuEntityPermissionsRequest,
) -> Result<GrantEuEntityPermissionsResponse, KsefError> {
    let fut = async {
        let url = client.url_for(routes::PERMISSIONS_EU_ENTITIES_GRANTS_PATH);
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

        let parsed: GrantEuEntityPermissionsResponse =
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

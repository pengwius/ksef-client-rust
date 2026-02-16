use crate::client::KsefClient;
use crate::client::error::KsefError;
use crate::client::routes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrantSubunitPermissionsRequest {
    pub subject_identifier: SubunitSubjectIdentifier,
    pub context_identifier: SubunitContextIdentifier,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subunit_name: Option<String>,
    pub subject_details: SubunitSubjectDetails,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubunitSubjectIdentifier {
    #[serde(rename = "type")]
    pub identifier_type: SubunitSubjectIdentifierType,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubunitSubjectIdentifierType {
    Nip,
    Pesel,
    Fingerprint,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubunitContextIdentifier {
    #[serde(rename = "type")]
    pub identifier_type: SubunitContextIdentifierType,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubunitContextIdentifierType {
    InternalId,
    Nip,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubunitSubjectDetails {
    pub subject_details_type: SubunitSubjectDetailsType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person_by_id: Option<SubunitPersonById>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person_by_fp_with_id: Option<SubunitPersonByFpWithId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person_by_fp_no_id: Option<SubunitPersonByFpNoId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubunitSubjectDetailsType {
    PersonByIdentifier,
    PersonByFingerprintWithIdentifier,
    PersonByFingerprintWithoutIdentifier,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubunitPersonById {
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubunitPersonByFpWithId {
    pub first_name: String,
    pub last_name: String,
    pub identifier: SubunitPersonIdentifier,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubunitPersonIdentifier {
    #[serde(rename = "type")]
    pub identifier_type: SubunitPersonIdentifierType,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubunitPersonIdentifierType {
    Nip,
    Pesel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubunitPersonByFpNoId {
    pub first_name: String,
    pub last_name: String,
    pub birth_date: String,
    pub id_document: SubunitIdDocument,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubunitIdDocument {
    #[serde(rename = "type")]
    pub document_type: String,
    pub number: String,
    pub country: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrantSubunitPermissionsResponse {
    pub reference_number: String,
}

pub fn grant_subunit_permissions(
    client: &KsefClient,
    request: GrantSubunitPermissionsRequest,
) -> Result<GrantSubunitPermissionsResponse, KsefError> {
    let fut = async {
        let url = client.url_for(routes::PERMISSIONS_SUBUNITS_GRANTS_PATH);
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

        let parsed: GrantSubunitPermissionsResponse =
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

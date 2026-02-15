use crate::client::KsefClient;
use crate::client::error::KsefError;
use crate::client::routes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrantPersonPermissionsRequest {
    pub subject_identifier: SubjectIdentifier,
    pub permissions: Vec<PersonPermissionType>,
    pub description: String,
    pub subject_details: SubjectDetails,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubjectIdentifier {
    #[serde(rename = "type")]
    pub identifier_type: SubjectIdentifierType,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubjectIdentifierType {
    Nip,
    Pesel,
    Fingerprint,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PersonPermissionType {
    InvoiceWrite,
    InvoiceRead,
    CredentialsManage,
    CredentialsRead,
    Introspection,
    SubunitManage,
    EnforcementOperations,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubjectDetails {
    pub subject_details_type: SubjectDetailsType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person_by_id: Option<PersonById>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person_by_fp_with_id: Option<PersonByFpWithId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person_by_fp_no_id: Option<PersonByFpNoId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubjectDetailsType {
    PersonByIdentifier,
    PersonByFingerprintWithIdentifier,
    PersonByFingerprintWithoutIdentifier,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonById {
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonByFpWithId {
    pub first_name: String,
    pub last_name: String,
    pub identifier: PersonIdentifier,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonIdentifier {
    #[serde(rename = "type")]
    pub identifier_type: PersonIdentifierType,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PersonIdentifierType {
    Nip,
    Pesel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonByFpNoId {
    pub first_name: String,
    pub last_name: String,
    pub birth_date: String,
    pub id_document: IdDocument,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdDocument {
    #[serde(rename = "type")]
    pub document_type: String,
    pub number: String,
    pub country: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrantPersonPermissionsResponse {
    pub reference_number: String,
}

pub fn grant_person_permissions(
    client: &KsefClient,
    request: GrantPersonPermissionsRequest,
) -> Result<GrantPersonPermissionsResponse, KsefError> {
    let fut = async {
        let url = client.url_for(routes::PERMISSIONS_PERSONS_GRANTS_PATH);
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

        let parsed: GrantPersonPermissionsResponse =
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

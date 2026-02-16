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

impl GrantPersonPermissionsRequest {
    pub fn builder() -> GrantPersonPermissionsRequestBuilder {
        GrantPersonPermissionsRequestBuilder::new()
    }
}

pub struct GrantPersonPermissionsRequestBuilder {
    subject_identifier: Option<SubjectIdentifier>,
    permissions: Vec<PersonPermissionType>,
    description: Option<String>,
    subject_details: Option<SubjectDetails>,
}

impl GrantPersonPermissionsRequestBuilder {
    pub fn new() -> Self {
        Self {
            subject_identifier: None,
            permissions: Vec::new(),
            description: None,
            subject_details: None,
        }
    }

    pub fn with_subject_identifier(mut self, identifier: SubjectIdentifier) -> Self {
        self.subject_identifier = Some(identifier);
        self
    }

    pub fn with_permission(mut self, permission: PersonPermissionType) -> Self {
        self.permissions.push(permission);
        self
    }

    pub fn with_permissions(mut self, permissions: Vec<PersonPermissionType>) -> Self {
        self.permissions = permissions;
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_subject_details(mut self, details: SubjectDetails) -> Self {
        self.subject_details = Some(details);
        self
    }

    pub fn build(self) -> Result<GrantPersonPermissionsRequest, String> {
        Ok(GrantPersonPermissionsRequest {
            subject_identifier: self
                .subject_identifier
                .ok_or("subject_identifier is required")?,
            permissions: self.permissions,
            description: self.description.ok_or("description is required")?,
            subject_details: self.subject_details.ok_or("subject_details is required")?,
        })
    }
}

impl Default for GrantPersonPermissionsRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
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

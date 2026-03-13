use crate::client::KsefClient;
use crate::client::error::KsefError;
use crate::client::permissions::get_operation_status::process_status_response;
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

impl SubjectIdentifier {
    pub fn builder() -> SubjectIdentifierBuilder {
        SubjectIdentifierBuilder::new()
    }
}

pub struct SubjectIdentifierBuilder {
    identifier_type: Option<SubjectIdentifierType>,
    value: Option<String>,
}

impl SubjectIdentifierBuilder {
    pub fn new() -> Self {
        Self {
            identifier_type: None,
            value: None,
        }
    }

    pub fn with_type(mut self, t: SubjectIdentifierType) -> Self {
        self.identifier_type = Some(t);
        self
    }

    pub fn with_value(mut self, v: impl Into<String>) -> Self {
        self.value = Some(v.into());
        self
    }

    pub fn build(self) -> Result<SubjectIdentifier, String> {
        Ok(SubjectIdentifier {
            identifier_type: self.identifier_type.ok_or("identifier_type is required")?,
            value: self.value.ok_or("value is required")?,
        })
    }
}

impl Default for SubjectIdentifierBuilder {
    fn default() -> Self {
        Self::new()
    }
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

impl SubjectDetails {
    pub fn builder() -> SubjectDetailsBuilder {
        SubjectDetailsBuilder::new()
    }
}

pub struct SubjectDetailsBuilder {
    subject_details_type: Option<SubjectDetailsType>,
    person_by_id: Option<PersonById>,
    person_by_fp_with_id: Option<PersonByFpWithId>,
    person_by_fp_no_id: Option<PersonByFpNoId>,
}

impl SubjectDetailsBuilder {
    pub fn new() -> Self {
        Self {
            subject_details_type: None,
            person_by_id: None,
            person_by_fp_with_id: None,
            person_by_fp_no_id: None,
        }
    }

    pub fn with_subject_details_type(mut self, t: SubjectDetailsType) -> Self {
        self.subject_details_type = Some(t);
        self
    }

    pub fn with_person_by_id(mut self, p: PersonById) -> Self {
        self.person_by_id = Some(p);
        self
    }

    pub fn with_person_by_fp_with_id(mut self, p: PersonByFpWithId) -> Self {
        self.person_by_fp_with_id = Some(p);
        self
    }

    pub fn with_person_by_fp_no_id(mut self, p: PersonByFpNoId) -> Self {
        self.person_by_fp_no_id = Some(p);
        self
    }

    pub fn build(self) -> Result<SubjectDetails, String> {
        Ok(SubjectDetails {
            subject_details_type: self
                .subject_details_type
                .ok_or("subject_details_type is required")?,
            person_by_id: self.person_by_id,
            person_by_fp_with_id: self.person_by_fp_with_id,
            person_by_fp_no_id: self.person_by_fp_no_id,
        })
    }
}

impl Default for SubjectDetailsBuilder {
    fn default() -> Self {
        Self::new()
    }
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

impl PersonById {
    pub fn builder() -> PersonByIdBuilder {
        PersonByIdBuilder::new()
    }
}

pub struct PersonByIdBuilder {
    first_name: Option<String>,
    last_name: Option<String>,
}

impl PersonByIdBuilder {
    pub fn new() -> Self {
        Self {
            first_name: None,
            last_name: None,
        }
    }

    pub fn with_first_name(mut self, v: impl Into<String>) -> Self {
        self.first_name = Some(v.into());
        self
    }

    pub fn with_last_name(mut self, v: impl Into<String>) -> Self {
        self.last_name = Some(v.into());
        self
    }

    pub fn build(self) -> Result<PersonById, String> {
        Ok(PersonById {
            first_name: self.first_name.ok_or("first_name is required")?,
            last_name: self.last_name.ok_or("last_name is required")?,
        })
    }
}

impl Default for PersonByIdBuilder {
    fn default() -> Self {
        Self::new()
    }
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

pub type GrantPersonPermissionsResponse =
    crate::client::permissions::get_operation_status::OperationStatusResponse;

pub async fn grant_person_permissions(
    client: &KsefClient,
    request: GrantPersonPermissionsRequest,
) -> Result<GrantPersonPermissionsResponse, KsefError> {
    let url = client.url_for(routes::PERMISSIONS_PERSONS_GRANTS_PATH);
    let access_token = &client.access_token.access_token;

    let resp = client
        .client
        .post(&url)
        .header("Accept", "application/json")
        .bearer_auth(KsefClient::secret_str(access_token))
        .json(&request)
        .send()
        .await
        .map_err(KsefError::RequestError)?;

    let status = resp.status();
    if !status.is_success() {
        let body = resp.text().await.unwrap_or_default();
        return Err(KsefError::from_api_response(status.as_u16(), body));
    }

    let parsed_value: serde_json::Value = resp.json().await.map_err(KsefError::RequestError)?;

    process_status_response(client, parsed_value, 10, 500).await
}

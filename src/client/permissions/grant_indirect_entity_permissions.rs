use crate::client::KsefClient;
use crate::client::error::KsefError;
use crate::client::routes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrantIndirectEntityPermissionsRequest {
    pub subject_identifier: IndirectSubjectIdentifier,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_identifier: Option<IndirectTargetIdentifier>,
    pub permissions: Vec<IndirectPermissionType>,
    pub description: String,
    pub subject_details: IndirectSubjectDetails,
}

impl GrantIndirectEntityPermissionsRequest {
    pub fn builder() -> GrantIndirectEntityPermissionsRequestBuilder {
        GrantIndirectEntityPermissionsRequestBuilder::new()
    }
}

pub struct GrantIndirectEntityPermissionsRequestBuilder {
    subject_identifier: Option<IndirectSubjectIdentifier>,
    target_identifier: Option<IndirectTargetIdentifier>,
    permissions: Vec<IndirectPermissionType>,
    description: Option<String>,
    subject_details: Option<IndirectSubjectDetails>,
}

impl GrantIndirectEntityPermissionsRequestBuilder {
    pub fn new() -> Self {
        Self {
            subject_identifier: None,
            target_identifier: None,
            permissions: Vec::new(),
            description: None,
            subject_details: None,
        }
    }

    pub fn with_subject_identifier(mut self, identifier: IndirectSubjectIdentifier) -> Self {
        self.subject_identifier = Some(identifier);
        self
    }

    pub fn with_target_identifier(mut self, identifier: IndirectTargetIdentifier) -> Self {
        self.target_identifier = Some(identifier);
        self
    }

    pub fn with_permission(mut self, permission: IndirectPermissionType) -> Self {
        self.permissions.push(permission);
        self
    }

    pub fn with_permissions(mut self, permissions: Vec<IndirectPermissionType>) -> Self {
        self.permissions = permissions;
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_subject_details(mut self, details: IndirectSubjectDetails) -> Self {
        self.subject_details = Some(details);
        self
    }

    pub fn build(self) -> Result<GrantIndirectEntityPermissionsRequest, String> {
        Ok(GrantIndirectEntityPermissionsRequest {
            subject_identifier: self
                .subject_identifier
                .ok_or("subject_identifier is required")?,
            target_identifier: self.target_identifier,
            permissions: self.permissions,
            description: self.description.ok_or("description is required")?,
            subject_details: self.subject_details.ok_or("subject_details is required")?,
        })
    }
}

impl Default for GrantIndirectEntityPermissionsRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndirectSubjectIdentifier {
    #[serde(rename = "type")]
    pub identifier_type: IndirectSubjectIdentifierType,
    pub value: String,
}

impl IndirectSubjectIdentifier {
    pub fn builder() -> IndirectSubjectIdentifierBuilder {
        IndirectSubjectIdentifierBuilder::new()
    }
}

pub struct IndirectSubjectIdentifierBuilder {
    identifier_type: Option<IndirectSubjectIdentifierType>,
    value: Option<String>,
}

impl IndirectSubjectIdentifierBuilder {
    pub fn new() -> Self {
        Self {
            identifier_type: None,
            value: None,
        }
    }

    pub fn with_type(mut self, t: IndirectSubjectIdentifierType) -> Self {
        self.identifier_type = Some(t);
        self
    }

    pub fn with_value(mut self, v: impl Into<String>) -> Self {
        self.value = Some(v.into());
        self
    }

    pub fn build(self) -> Result<IndirectSubjectIdentifier, String> {
        Ok(IndirectSubjectIdentifier {
            identifier_type: self.identifier_type.ok_or("identifier_type is required")?,
            value: self.value.ok_or("value is required")?,
        })
    }
}

impl Default for IndirectSubjectIdentifierBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IndirectSubjectIdentifierType {
    Nip,
    Pesel,
    Fingerprint,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndirectTargetIdentifier {
    #[serde(rename = "type")]
    pub identifier_type: IndirectTargetIdentifierType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IndirectTargetIdentifierType {
    Nip,
    AllPartners,
    InternalId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IndirectPermissionType {
    InvoiceRead,
    InvoiceWrite,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndirectSubjectDetails {
    pub subject_details_type: IndirectSubjectDetailsType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person_by_id: Option<IndirectPersonById>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person_by_fp_with_id: Option<IndirectPersonByFpWithId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person_by_fp_no_id: Option<IndirectPersonByFpNoId>,
}

impl IndirectSubjectDetails {
    pub fn builder() -> IndirectSubjectDetailsBuilder {
        IndirectSubjectDetailsBuilder::new()
    }
}

pub struct IndirectSubjectDetailsBuilder {
    subject_details_type: Option<IndirectSubjectDetailsType>,
    person_by_id: Option<IndirectPersonById>,
    person_by_fp_with_id: Option<IndirectPersonByFpWithId>,
    person_by_fp_no_id: Option<IndirectPersonByFpNoId>,
}

impl IndirectSubjectDetailsBuilder {
    pub fn new() -> Self {
        Self {
            subject_details_type: None,
            person_by_id: None,
            person_by_fp_with_id: None,
            person_by_fp_no_id: None,
        }
    }

    pub fn with_subject_details_type(mut self, t: IndirectSubjectDetailsType) -> Self {
        self.subject_details_type = Some(t);
        self
    }

    pub fn with_person_by_id(mut self, p: IndirectPersonById) -> Self {
        self.person_by_id = Some(p);
        self
    }

    pub fn with_person_by_fp_with_id(mut self, p: IndirectPersonByFpWithId) -> Self {
        self.person_by_fp_with_id = Some(p);
        self
    }

    pub fn with_person_by_fp_no_id(mut self, p: IndirectPersonByFpNoId) -> Self {
        self.person_by_fp_no_id = Some(p);
        self
    }

    pub fn build(self) -> Result<IndirectSubjectDetails, String> {
        Ok(IndirectSubjectDetails {
            subject_details_type: self
                .subject_details_type
                .ok_or("subject_details_type is required")?,
            person_by_id: self.person_by_id,
            person_by_fp_with_id: self.person_by_fp_with_id,
            person_by_fp_no_id: self.person_by_fp_no_id,
        })
    }
}

impl Default for IndirectSubjectDetailsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IndirectSubjectDetailsType {
    PersonByIdentifier,
    PersonByFingerprintWithIdentifier,
    PersonByFingerprintWithoutIdentifier,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndirectPersonById {
    pub first_name: String,
    pub last_name: String,
}

impl IndirectPersonById {
    pub fn builder() -> IndirectPersonByIdBuilder {
        IndirectPersonByIdBuilder::new()
    }
}

pub struct IndirectPersonByIdBuilder {
    first_name: Option<String>,
    last_name: Option<String>,
}

impl IndirectPersonByIdBuilder {
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

    pub fn build(self) -> Result<IndirectPersonById, String> {
        Ok(IndirectPersonById {
            first_name: self.first_name.ok_or("first_name is required")?,
            last_name: self.last_name.ok_or("last_name is required")?,
        })
    }
}

impl Default for IndirectPersonByIdBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndirectPersonByFpWithId {
    pub first_name: String,
    pub last_name: String,
    pub identifier: IndirectPersonIdentifier,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndirectPersonIdentifier {
    #[serde(rename = "type")]
    pub identifier_type: IndirectPersonIdentifierType,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IndirectPersonIdentifierType {
    Nip,
    Pesel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndirectPersonByFpNoId {
    pub first_name: String,
    pub last_name: String,
    pub birth_date: String,
    pub id_document: IndirectIdDocument,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndirectIdDocument {
    #[serde(rename = "type")]
    pub document_type: String,
    pub number: String,
    pub country: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrantIndirectEntityPermissionsResponse {
    pub reference_number: String,
}

pub async fn grant_indirect_entity_permissions(
    client: &KsefClient,
    request: GrantIndirectEntityPermissionsRequest,
) -> Result<GrantIndirectEntityPermissionsResponse, KsefError> {
    let url = client.url_for(routes::PERMISSIONS_INDIRECT_GRANTS_PATH);
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

    let parsed: GrantIndirectEntityPermissionsResponse =
        resp.json().await.map_err(KsefError::RequestError)?;
    Ok(parsed)
}

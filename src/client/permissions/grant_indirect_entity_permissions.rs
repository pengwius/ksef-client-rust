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

pub fn grant_indirect_entity_permissions(
    client: &KsefClient,
    request: GrantIndirectEntityPermissionsRequest,
) -> Result<GrantIndirectEntityPermissionsResponse, KsefError> {
    let fut = async {
        let url = client.url_for(routes::PERMISSIONS_INDIRECT_GRANTS_PATH);
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

        let parsed: GrantIndirectEntityPermissionsResponse =
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

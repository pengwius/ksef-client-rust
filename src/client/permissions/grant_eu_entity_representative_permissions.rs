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

impl GrantEuEntityRepresentativePermissionsRequest {
    pub fn builder() -> GrantEuEntityRepresentativePermissionsRequestBuilder {
        GrantEuEntityRepresentativePermissionsRequestBuilder::new()
    }
}

pub struct GrantEuEntityRepresentativePermissionsRequestBuilder {
    subject_identifier: Option<EuEntityRepresentativeSubjectIdentifier>,
    permissions: Vec<EuEntityRepresentativePermissionType>,
    description: Option<String>,
    subject_details: Option<EuEntityRepresentativeSubjectDetails>,
}

impl GrantEuEntityRepresentativePermissionsRequestBuilder {
    pub fn new() -> Self {
        Self {
            subject_identifier: None,
            permissions: Vec::new(),
            description: None,
            subject_details: None,
        }
    }

    pub fn with_subject_identifier(
        mut self,
        identifier: EuEntityRepresentativeSubjectIdentifier,
    ) -> Self {
        self.subject_identifier = Some(identifier);
        self
    }

    pub fn with_permission(mut self, permission: EuEntityRepresentativePermissionType) -> Self {
        self.permissions.push(permission);
        self
    }

    pub fn with_permissions(
        mut self,
        permissions: Vec<EuEntityRepresentativePermissionType>,
    ) -> Self {
        self.permissions = permissions;
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_subject_details(mut self, details: EuEntityRepresentativeSubjectDetails) -> Self {
        self.subject_details = Some(details);
        self
    }

    pub fn build(self) -> Result<GrantEuEntityRepresentativePermissionsRequest, String> {
        Ok(GrantEuEntityRepresentativePermissionsRequest {
            subject_identifier: self
                .subject_identifier
                .ok_or("subject_identifier is required")?,
            permissions: self.permissions,
            description: self.description.ok_or("description is required")?,
            subject_details: self.subject_details.ok_or("subject_details is required")?,
        })
    }
}

impl Default for GrantEuEntityRepresentativePermissionsRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
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

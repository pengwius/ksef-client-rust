use crate::client::KsefClient;
use crate::client::error::KsefError;
use crate::client::routes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrantAuthorizationPermissionsRequest {
    pub subject_identifier: AuthorizationSubjectIdentifier,
    pub permission: AuthorizationPermissionType,
    pub description: String,
    pub subject_details: AuthorizationSubjectDetails,
}

impl GrantAuthorizationPermissionsRequest {
    pub fn builder() -> GrantAuthorizationPermissionsRequestBuilder {
        GrantAuthorizationPermissionsRequestBuilder::new()
    }
}

pub struct GrantAuthorizationPermissionsRequestBuilder {
    subject_identifier: Option<AuthorizationSubjectIdentifier>,
    permission: Option<AuthorizationPermissionType>,
    description: Option<String>,
    subject_details: Option<AuthorizationSubjectDetails>,
}

impl GrantAuthorizationPermissionsRequestBuilder {
    pub fn new() -> Self {
        Self {
            subject_identifier: None,
            permission: None,
            description: None,
            subject_details: None,
        }
    }

    pub fn with_subject_identifier(mut self, identifier: AuthorizationSubjectIdentifier) -> Self {
        self.subject_identifier = Some(identifier);
        self
    }

    pub fn with_permission(mut self, permission: AuthorizationPermissionType) -> Self {
        self.permission = Some(permission);
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_subject_details(mut self, details: AuthorizationSubjectDetails) -> Self {
        self.subject_details = Some(details);
        self
    }

    pub fn build(self) -> Result<GrantAuthorizationPermissionsRequest, String> {
        Ok(GrantAuthorizationPermissionsRequest {
            subject_identifier: self
                .subject_identifier
                .ok_or("subject_identifier is required")?,
            permission: self.permission.ok_or("permission is required")?,
            description: self.description.ok_or("description is required")?,
            subject_details: self.subject_details.ok_or("subject_details is required")?,
        })
    }
}

impl Default for GrantAuthorizationPermissionsRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizationSubjectIdentifier {
    #[serde(rename = "type")]
    pub identifier_type: AuthorizationSubjectIdentifierType,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthorizationSubjectIdentifierType {
    Nip,
    PeppolId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthorizationPermissionType {
    SelfInvoicing,
    RRInvoicing,
    TaxRepresentative,
    PefInvoicing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizationSubjectDetails {
    pub full_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrantAuthorizationPermissionsResponse {
    pub reference_number: String,
}

pub fn grant_authorization_permissions(
    client: &KsefClient,
    request: GrantAuthorizationPermissionsRequest,
) -> Result<GrantAuthorizationPermissionsResponse, KsefError> {
    let fut = async {
        let url = client.url_for(routes::PERMISSIONS_AUTHORIZATIONS_GRANTS_PATH);
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

        let parsed: GrantAuthorizationPermissionsResponse =
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

use crate::client::KsefClient;
use crate::client::error::KsefError;
use crate::client::permissions::get_operation_status::{
    OperationStatusResponse, get_operation_status,
};
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

pub async fn grant_authorization_permissions(
    client: &KsefClient,
    request: GrantAuthorizationPermissionsRequest,
) -> Result<OperationStatusResponse, KsefError> {
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

    let max_attempts: usize = 10;
    let mut attempt: usize = 0;
    loop {
        match get_operation_status(client, &parsed.reference_number).await {
            Ok(op_status) => {
                if let Some(code) = op_status.status_code() {
                    if code != 100 {
                        if code == 200 {
                            return Ok(op_status);
                        } else {
                            let message = op_status
                                .status_message()
                                .unwrap_or_else(|| op_status.raw.to_string());
                            return Err(KsefError::ApplicationError(code as i32, message));
                        }
                    }
                } else {
                    return Err(KsefError::InvalidResponse(format!(
                        "Unexpected operation status payload: {}",
                        op_status.raw
                    )));
                }
            }
            Err(err) => {
                return Err(err);
            }
        }

        attempt += 1;
        if attempt >= max_attempts {
            let final_status = get_operation_status(client, &parsed.reference_number).await?;
            if let Some(code) = final_status.status_code() {
                if code == 200 {
                    return Ok(final_status);
                } else {
                    let message = final_status
                        .status_message()
                        .unwrap_or_else(|| final_status.raw.to_string());
                    return Err(KsefError::ApplicationError(code as i32, message));
                }
            } else {
                return Err(KsefError::InvalidResponse(format!(
                    "Unexpected operation status payload on final attempt: {}",
                    final_status.raw
                )));
            }
        }

        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    }
}

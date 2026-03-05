use crate::client::KsefClient;
use crate::client::error::KsefError;
use crate::client::permissions::get_operation_status::{
    OperationStatusResponse, get_operation_status,
};
use crate::client::routes;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrantEntityPermissionsRequest {
    pub subject_identifier: EntityIdentifier,
    pub permissions: Vec<EntityPermission>,
    pub description: String,
    pub subject_details: EntitySubjectDetails,
}

impl GrantEntityPermissionsRequest {
    pub fn builder() -> GrantEntityPermissionsRequestBuilder {
        GrantEntityPermissionsRequestBuilder::new()
    }
}

pub struct GrantEntityPermissionsRequestBuilder {
    subject_identifier: Option<EntityIdentifier>,
    permissions: Vec<EntityPermission>,
    description: Option<String>,
    subject_details: Option<EntitySubjectDetails>,
}

impl GrantEntityPermissionsRequestBuilder {
    pub fn new() -> Self {
        Self {
            subject_identifier: None,
            permissions: Vec::new(),
            description: None,
            subject_details: None,
        }
    }

    pub fn with_subject_identifier(mut self, identifier: EntityIdentifier) -> Self {
        self.subject_identifier = Some(identifier);
        self
    }

    pub fn with_permission(mut self, permission: EntityPermission) -> Self {
        self.permissions.push(permission);
        self
    }

    pub fn with_permissions(mut self, permissions: Vec<EntityPermission>) -> Self {
        self.permissions = permissions;
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_subject_details(mut self, details: EntitySubjectDetails) -> Self {
        self.subject_details = Some(details);
        self
    }

    pub fn build(self) -> Result<GrantEntityPermissionsRequest, String> {
        Ok(GrantEntityPermissionsRequest {
            subject_identifier: self
                .subject_identifier
                .ok_or("subject_identifier is required")?,
            permissions: self.permissions,
            description: self.description.ok_or("description is required")?,
            subject_details: self.subject_details.ok_or("subject_details is required")?,
        })
    }
}

impl Default for GrantEntityPermissionsRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntityIdentifier {
    #[serde(rename = "type")]
    pub identifier_type: EntityIdentifierType,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntityIdentifierType {
    Nip,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntityPermission {
    #[serde(rename = "type")]
    pub permission_type: EntityPermissionType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delegate: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntityPermissionType {
    InvoiceWrite,
    InvoiceRead,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntitySubjectDetails {
    pub full_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrantEntityPermissionsResponse {
    pub reference_number: String,
}

pub async fn grant_entity_permissions(
    client: &KsefClient,
    request: GrantEntityPermissionsRequest,
) -> Result<OperationStatusResponse, KsefError> {
    let url = client.url_for(routes::PERMISSIONS_ENTITIES_GRANTS_PATH);
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

    let parsed_value: Value = resp.json().await.map_err(KsefError::RequestError)?;

    let reference_opt = parsed_value
        .get("referenceNumber")
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .or_else(|| {
            parsed_value
                .get("reference_number")
                .and_then(|v| v.as_str().map(|s| s.to_string()))
        });

    if let Some(reference_number) = reference_opt {
        let max_attempts: usize = 10;
        let mut attempt: usize = 0;

        loop {
            match get_operation_status(client, &reference_number).await {
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
                let final_status = get_operation_status(client, &reference_number).await?;
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
    } else {
        Ok(OperationStatusResponse::from_value(parsed_value))
    }
}

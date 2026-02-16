use crate::client::KsefClient;
use crate::client::error::KsefError;
use crate::client::routes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrantEntityPermissionsRequest {
    pub subject_identifier: EntityIdentifier,
    pub permissions: Vec<EntityPermission>,
    pub description: String,
    pub subject_details: EntitySubjectDetails,
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

pub fn grant_entity_permissions(
    client: &KsefClient,
    request: GrantEntityPermissionsRequest,
) -> Result<GrantEntityPermissionsResponse, KsefError> {
    let fut = async {
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

        let parsed: GrantEntityPermissionsResponse =
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

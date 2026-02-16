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

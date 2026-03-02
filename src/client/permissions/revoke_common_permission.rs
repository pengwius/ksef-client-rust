use crate::client::KsefClient;
use crate::client::error::KsefError;
use crate::client::routes;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RevokeCommonPermissionResponse {
    pub reference_number: String,
}

pub async fn revoke_common_permission(
    client: &KsefClient,
    permission_id: &str,
) -> Result<RevokeCommonPermissionResponse, KsefError> {
    let url = client.url_for(&format!(
        "{}/{}",
        routes::PERMISSIONS_COMMON_GRANTS_PATH,
        permission_id
    ));

    let access_token = &client.access_token.access_token;

    let resp = client
        .client
        .delete(&url)
        .header("Accept", "application/json")
        .bearer_auth(access_token)
        .send()
        .await
        .map_err(KsefError::RequestError)?;

    let status = resp.status();
    if !status.is_success() {
        let body = resp.text().await.unwrap_or_default();
        return Err(KsefError::ApiError(status.as_u16(), body));
    }

    let parsed: RevokeCommonPermissionResponse =
        resp.json().await.map_err(KsefError::RequestError)?;
    Ok(parsed)
}

pub async fn get_common_permissions(client: &KsefClient) -> Result<Value, KsefError> {
    let url = client.url_for(routes::PERMISSIONS_COMMON_GRANTS_PATH);

    let access_token = &client.access_token.access_token;

    let resp = client
        .client
        .get(&url)
        .header("Accept", "application/json")
        .bearer_auth(access_token)
        .send()
        .await
        .map_err(KsefError::RequestError)?;

    let status = resp.status();
    if !status.is_success() {
        let body = resp.text().await.unwrap_or_default();
        return Err(KsefError::ApiError(status.as_u16(), body));
    }

    let parsed: Value = resp.json().await.map_err(KsefError::RequestError)?;
    Ok(parsed)
}

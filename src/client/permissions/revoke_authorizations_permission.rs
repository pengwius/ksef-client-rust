use crate::client::KsefClient;
use crate::client::error::KsefError;
use crate::client::permissions::get_operation_status::{
    OperationStatusResponse, process_status_response,
};
use crate::client::routes;
use serde_json::Value;

pub async fn revoke_authorizations_permission(
    client: &KsefClient,
    permission_id: &str,
) -> Result<OperationStatusResponse, KsefError> {
    let url = client.url_for(&format!(
        "{}/{}",
        routes::PERMISSIONS_AUTHORIZATIONS_GRANTS_PATH,
        permission_id
    ));

    let access_token = &client.access_token.access_token;

    let resp = client
        .client
        .delete(&url)
        .header("Accept", "application/json")
        .bearer_auth(KsefClient::secret_str(access_token))
        .send()
        .await
        .map_err(KsefError::RequestError)?;

    let status = resp.status();
    if !status.is_success() {
        let body = resp.text().await.unwrap_or_default();
        return Err(KsefError::from_api_response(status.as_u16(), body));
    }

    let parsed_value: Value = resp.json().await.map_err(KsefError::RequestError)?;
    process_status_response(client, parsed_value, 10, 500).await
}

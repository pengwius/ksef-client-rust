use crate::client::KsefClient;
use crate::client::error::KsefError;
use crate::client::routes;

pub async fn revoke_session(client: &KsefClient, reference_number: &str) -> Result<(), KsefError> {
    let url = client.url_for(&format!(
        "{}/{}",
        routes::AUTH_SESSIONS_PATH,
        reference_number
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

    Ok(())
}

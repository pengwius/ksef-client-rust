use crate::client::KsefClient;
use crate::client::error::KsefError;
use crate::client::routes;

pub async fn close_online_session(
    client: &KsefClient,
    session_reference_number: &str,
) -> Result<(), KsefError> {
    let url = client.url_for(&format!(
        "{}/{}/close",
        routes::SESSIONS_ONLINE_PATH,
        session_reference_number
    ));

    let access_token = &client.access_token.access_token;
    if access_token.is_empty() {
        return Err(KsefError::ApplicationError(
            0,
            "No access token available".to_string(),
        ));
    }

    let resp = client
        .client
        .post(&url)
        .header("Accept", "application/json")
        .bearer_auth(access_token)
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

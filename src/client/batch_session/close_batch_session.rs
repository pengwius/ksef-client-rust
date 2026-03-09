use crate::client::KsefClient;
use crate::client::error::KsefError;
use crate::client::routes;

pub async fn close_batch_session(
    client: &KsefClient,
    reference_number: &str,
) -> Result<(), KsefError> {
    let path = format!("{}/{}/close", routes::SESSIONS_BATCH_PATH, reference_number);
    let url = client.url_for(&path);
    let http = &client.client;

    let token = &client.access_token.access_token;
    if token.is_empty() {
        return Err(KsefError::ApplicationError(
            0,
            "No access token available".to_string(),
        ));
    }

    let resp = http
        .post(&url)
        .header("Accept", "application/json")
        .bearer_auth(token)
        .send()
        .await?;

    let status = resp.status();

    if !status.is_success() {
        let code = status.as_u16();
        let body = resp.text().await.unwrap_or_default();
        return Err(KsefError::from_api_response(code, body));
    }

    Ok(())
}

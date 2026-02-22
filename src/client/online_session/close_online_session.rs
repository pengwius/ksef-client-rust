use crate::client::KsefClient;
use crate::client::error::KsefError;

pub fn close_online_session(
    client: &KsefClient,
    session_reference_number: &str,
) -> Result<(), KsefError> {
    let fut = async {
        let url = client.url_for(&format!(
            "/v2/sessions/online/{}/close",
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
            return Err(KsefError::ApiError(status.as_u16(), body));
        }

        Ok(())
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

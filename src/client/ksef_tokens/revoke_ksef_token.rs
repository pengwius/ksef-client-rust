use crate::client::KsefClient;
use crate::client::error::KsefError;
use crate::client::routes;

pub fn revoke_ksef_token(
    client: &KsefClient,
    token_reference_number: &str,
) -> Result<(), KsefError> {
    let fut = async {
        let url = client.url_for(&format!(
            "{}/{}",
            routes::TOKENS_PATH,
            token_reference_number
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

use crate::client::KsefClient;
use crate::client::error::KsefError;
use crate::client::ksef_tokens::models::{DetailedKsefToken, QueryTokensResponse};
use crate::client::routes;

pub async fn get_ksef_tokens(client: &KsefClient) -> Result<Vec<DetailedKsefToken>, KsefError> {
    let url = client.url_for(routes::TOKENS_PATH);

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

    let parsed: QueryTokensResponse = resp.json().await.map_err(KsefError::RequestError)?;
    Ok(parsed.tokens)
}

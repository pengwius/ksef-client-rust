use crate::client::KsefClient;
use crate::client::error::KsefError;
use crate::client::routes;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Clone, Default)]
pub struct AccessTokens {
    pub access_token: String,
    pub access_token_valid_until: DateTime<Utc>,
    pub refresh_token: String,
    pub refresh_token_valid_until: DateTime<Utc>,
}

#[derive(Deserialize)]
struct TokenResponse {
    #[serde(rename = "accessToken")]
    access_token_obj: TokenObject,
    #[serde(rename = "refreshToken")]
    refresh_token_obj: TokenObject,
}

#[derive(Deserialize)]
struct RefreshTokenResponse {
    #[serde(rename = "accessToken")]
    access_token_obj: TokenObject,
}

#[derive(Deserialize)]
struct TokenObject {
    token: String,
    #[serde(rename = "validUntil")]
    valid_until: DateTime<Utc>,
}

pub fn get_access_token(client: &KsefClient) -> Result<AccessTokens, KsefError> {
    let url = client.url_for(routes::AUTH_TOKEN_REDEEM_PATH);

    let http_client = reqwest::blocking::Client::new();

    let resp = http_client
        .post(&url)
        .header("Accept", "application/json")
        .bearer_auth(&client.auth_token.authentication_token)
        .send()
        .map_err(KsefError::RequestError)?;

    let status = resp.status();
    if !status.is_success() {
        let body = resp.text().unwrap_or_default();
        return Err(KsefError::ApiError(status.as_u16(), body));
    }

    let parsed: TokenResponse = resp.json().map_err(KsefError::RequestError)?;

    Ok(AccessTokens {
        access_token: parsed.access_token_obj.token,
        access_token_valid_until: parsed.access_token_obj.valid_until,
        refresh_token: parsed.refresh_token_obj.token,
        refresh_token_valid_until: parsed.refresh_token_obj.valid_until,
    })
}

pub fn refresh_access_token(client: &KsefClient) -> Result<AccessTokens, KsefError> {
    let fut = async {
        let url = client.url_for(routes::AUTH_TOKEN_REFRESH_PATH);
        let resp = client
            .client
            .post(&url)
            .header("Accept", "application/json")
            .bearer_auth(&client.access_token.refresh_token)
            .send()
            .await
            .map_err(KsefError::RequestError)?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(KsefError::ApiError(status.as_u16(), body));
        }

        let parsed: RefreshTokenResponse = resp.json().await.map_err(KsefError::RequestError)?;
        Ok(parsed)
    };

    let token_response = match tokio::runtime::Handle::try_current() {
        Ok(handle) => handle.block_on(fut)?,
        Err(_) => {
            let rt = tokio::runtime::Runtime::new()
                .map_err(|e| KsefError::RuntimeError(e.to_string()))?;
            rt.block_on(fut)?
        }
    };

    Ok(AccessTokens {
        access_token: token_response.access_token_obj.token,
        access_token_valid_until: token_response.access_token_obj.valid_until,
        refresh_token: client.access_token.refresh_token.clone(),
        refresh_token_valid_until: client.access_token.refresh_token_valid_until,
    })
}

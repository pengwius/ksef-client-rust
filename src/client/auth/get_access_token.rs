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

pub async fn get_access_token(client: &KsefClient) -> Result<AccessTokens, KsefError> {
    let url = client.url_for(routes::AUTH_TOKEN_REDEEM_PATH);

    let resp = client
        .client
        .post(&url)
        .header("Accept", "application/json")
        .bearer_auth(&client.auth_token.authentication_token)
        .send()
        .await
        .map_err(KsefError::RequestError)?;

    let status = resp.status();
    if !status.is_success() {
        let body = resp.text().await.unwrap_or_default();
        return Err(KsefError::from_api_response(status.as_u16(), body));
    }

    let parsed: TokenResponse = resp.json().await.map_err(KsefError::RequestError)?;

    Ok(AccessTokens {
        access_token: parsed.access_token_obj.token,
        access_token_valid_until: parsed.access_token_obj.valid_until,
        refresh_token: parsed.refresh_token_obj.token,
        refresh_token_valid_until: parsed.refresh_token_obj.valid_until,
    })
}

pub async fn refresh_access_token(client: &KsefClient) -> Result<AccessTokens, KsefError> {
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
        return Err(KsefError::from_api_response(status.as_u16(), body));
    }

    let parsed: RefreshTokenResponse = resp.json().await.map_err(KsefError::RequestError)?;

    Ok(AccessTokens {
        access_token: parsed.access_token_obj.token,
        access_token_valid_until: parsed.access_token_obj.valid_until,
        refresh_token: client.access_token.refresh_token.clone(),
        refresh_token_valid_until: client.access_token.refresh_token_valid_until,
    })
}

pub async fn get_access_token_and_load(client: &mut KsefClient) -> Result<(), KsefError> {
    let tokens = get_access_token(&*client).await?;
    client.access_token = tokens;
    Ok(())
}

pub async fn refresh_access_token_and_load(client: &mut KsefClient) -> Result<(), KsefError> {
    let tokens = refresh_access_token(&*client).await?;
    client.access_token = tokens;
    Ok(())
}

use crate::client::KsefClient;
use crate::client::error::KsefError;
use crate::client::routes;
use chrono::{DateTime, Utc};
use secrecy::Secret;
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct AccessTokens {
    pub access_token: Secret<String>,
    pub access_token_valid_until: DateTime<Utc>,
    pub refresh_token: Secret<String>,
    pub refresh_token_valid_until: DateTime<Utc>,
    /// Reference number of the authentication operation these tokens were redeemed for.
    pub authentication_reference_number: String,
}

impl Default for AccessTokens {
    fn default() -> Self {
        Self {
            access_token: Secret::new(String::new()),
            access_token_valid_until: DateTime::<Utc>::default(),
            refresh_token: Secret::new(String::new()),
            refresh_token_valid_until: DateTime::<Utc>::default(),
            authentication_reference_number: String::new(),
        }
    }
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
        .bearer_auth(KsefClient::secret_str(
            &client.auth_token.authentication_token,
        ))
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
        access_token: Secret::new(parsed.access_token_obj.token),
        access_token_valid_until: parsed.access_token_obj.valid_until,
        refresh_token: Secret::new(parsed.refresh_token_obj.token),
        refresh_token_valid_until: parsed.refresh_token_obj.valid_until,
        authentication_reference_number: client.auth_token.reference_number.clone(),
    })
}

pub async fn refresh_access_token(client: &KsefClient) -> Result<AccessTokens, KsefError> {
    let url = client.url_for(routes::AUTH_TOKEN_REFRESH_PATH);
    let resp = client
        .client
        .post(&url)
        .header("Accept", "application/json")
        .bearer_auth(KsefClient::secret_str(&client.access_token.refresh_token))
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
        access_token: Secret::new(parsed.access_token_obj.token),
        access_token_valid_until: parsed.access_token_obj.valid_until,
        refresh_token: client.access_token.refresh_token.clone(),
        refresh_token_valid_until: client.access_token.refresh_token_valid_until,
        authentication_reference_number: client
            .access_token
            .authentication_reference_number
            .clone(),
    })
}

/// Redeems the access and refresh tokens of the current authentication operation and stores
/// them in the client.
///
/// KSeF allows redeeming the tokens only once per authentication operation (a second attempt
/// fails with exception 21301). `get_auth_status` already redeems them when authentication
/// succeeds, so when the tokens of the current operation are present this returns `Ok(())`
/// without calling the API. Tokens of a previous operation stay usable until the new ones are
/// redeemed.
pub async fn get_access_token_and_load(client: &mut KsefClient) -> Result<(), KsefError> {
    let redeemed = !KsefClient::secret_str(&client.access_token.access_token).is_empty();
    if redeemed
        && client.access_token.authentication_reference_number == client.auth_token.reference_number
    {
        return Ok(());
    }
    let tokens = get_access_token(&*client).await?;
    client.access_token = tokens;
    Ok(())
}

pub async fn refresh_access_token_and_load(client: &mut KsefClient) -> Result<(), KsefError> {
    let tokens = refresh_access_token(&*client).await?;
    client.access_token = tokens;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::models::{ContextIdentifier, ContextIdentifierType};

    fn client() -> KsefClient {
        // Unroutable address: any request would fail.
        KsefClient::new_with_base(
            "http://127.0.0.1:9",
            ContextIdentifier {
                id_type: ContextIdentifierType::Nip,
                value: "1111111111".to_string(),
            },
        )
    }

    #[tokio::test]
    async fn already_redeemed_tokens_are_not_redeemed_again() {
        let mut client = client();
        client.access_token.access_token = Secret::new("redeemed".to_string());
        get_access_token_and_load(&mut client)
            .await
            .expect("no second redeem");
        assert_eq!(
            KsefClient::secret_str(&client.access_token.access_token),
            "redeemed"
        );
    }

    #[tokio::test]
    async fn tokens_of_a_previous_authentication_are_replaced() {
        let mut client = client();
        client.access_token.access_token = Secret::new("old".to_string());
        client.access_token.authentication_reference_number = "20261116-AU-OLD".to_string();
        client.auth_token.reference_number = "20261116-AU-NEW".to_string();
        assert!(
            get_access_token_and_load(&mut client).await.is_err(),
            "the API is called for the new operation"
        );
        assert_eq!(
            KsefClient::secret_str(&client.access_token.access_token),
            "old",
            "the old token stays usable meanwhile"
        );
    }

    #[tokio::test]
    async fn without_tokens_the_api_is_called() {
        let mut client = client();
        assert!(get_access_token_and_load(&mut client).await.is_err());
    }
}

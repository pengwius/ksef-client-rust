use crate::client::KsefClient;
use crate::client::error::KsefError;
use crate::client::routes;
use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct AccessTokens {
    #[serde(rename = "accessToken")]
    pub access_token: String,
    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
}

#[derive(Deserialize)]
struct TokenResponse {
    #[serde(rename = "accessToken")]
    access_token_obj: TokenObject,
    #[serde(rename = "refreshToken")]
    refresh_token_obj: TokenObject,
}

#[derive(Deserialize)]
struct TokenObject {
    token: String,
}

pub fn get_access_token(client: &KsefClient) -> Result<AccessTokens, KsefError> {
    let fut = async {
        let url = client.url_for(routes::AUTH_TOKEN_REDEEM_PATH);
        let resp = client
            .client
            .post(&url)
            .header("Accept", "application/json")
            .bearer_auth(&client.auth_token.authentication_token)
            .send()
            .await?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(KsefError::ApiError(status.as_u16(), body));
        }

        let parsed: TokenResponse = resp.json().await?;
        Ok(parsed)
    };

    let token_response = match tokio::runtime::Handle::try_current() {
        Ok(handle) => handle.block_on(fut)?,
        Err(_) => {
            let rt = tokio::runtime::Runtime::new()?;
            rt.block_on(fut)?
        }
    };

    Ok(AccessTokens {
        access_token: token_response.access_token_obj.token,
        refresh_token: token_response.refresh_token_obj.token,
    })
}

use crate::client::KsefClient;
use crate::client::error::KsefError;
use crate::client::routes;
use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct AuthTokens {
    pub authentication_token: String,
    pub reference_number: String,
}

#[derive(Deserialize)]
struct AuthResponse {
    #[serde(rename = "referenceNumber")]
    reference_number: String,
    #[serde(rename = "authenticationToken")]
    authentication_token: TokenObject,
}

#[derive(Deserialize)]
struct TokenObject {
    token: String,
}

pub fn submit_xades_auth_request(
    client: &KsefClient,
    signed_xml: String,
) -> Result<AuthTokens, KsefError> {
    let fut = async {
        let url = client.url_for(routes::AUTH_XADES_SIGANTURE_PATH);
        let resp = client
            .client
            .post(&url)
            .header("Content-Type", "application/xml")
            .header("Accept", "application/json")
            .query(&[("verifyCertificateChain", "false")])
            .body(signed_xml)
            .send()
            .await?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(KsefError::ApiError(status.as_u16(), body));
        }

        let v = resp.json::<serde_json::Value>().await?;

        Ok(v)
    };

    let auth_response = match tokio::runtime::Handle::try_current() {
        Ok(handle) => handle.block_on(fut)?,
        Err(_) => {
            let rt = tokio::runtime::Runtime::new()
                .map_err(|e| KsefError::RuntimeError(e.to_string()))?;
            rt.block_on(fut)?
        }
    };

    let auth_tokens: AuthResponse =
        serde_json::from_value(auth_response).map_err(|e| KsefError::JsonError(e))?;

    Ok(AuthTokens {
        authentication_token: auth_tokens.authentication_token.token,
        reference_number: auth_tokens.reference_number,
    })
}

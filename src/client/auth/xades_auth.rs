use crate::client::KsefClient;
use crate::client::error::KsefError;
use crate::client::routes;
use serde::Deserialize;

#[derive(Debug, Deserialize, Default, Clone)]
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

pub async fn submit_xades_auth_request(
    client: &KsefClient,
    signed_xml: String,
) -> Result<AuthTokens, KsefError> {
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

    let auth_response = resp.json::<serde_json::Value>().await?;

    let auth_tokens: AuthResponse =
        serde_json::from_value(auth_response).map_err(KsefError::JsonError)?;

    Ok(AuthTokens {
        authentication_token: auth_tokens.authentication_token.token,
        reference_number: auth_tokens.reference_number,
    })
}

pub async fn submit_xades_auth_request_and_load(
    client: &mut KsefClient,
    signed_xml: String,
) -> Result<(), KsefError> {
    let tokens = submit_xades_auth_request(client, signed_xml).await?;
    client.auth_token = tokens;
    Ok(())
}

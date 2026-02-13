use crate::client::KsefClient;
use crate::client::auth_challenge::AuthChallenge;
use crate::client::auth_token_request::ContextIdentifierType;
use crate::client::error::KsefError;
use crate::client::routes;
use crate::client::submit_xades_auth_request::AuthTokens;
use base64::{Engine as _, engine::general_purpose};
use openssl::hash::MessageDigest;
use openssl::md::MdRef;
use openssl::pkey::PKey;
use openssl::rsa::Padding;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct KsefTokenAuthRequest {
    challenge: String,
    context_identifier: KsefTokenContextIdentifier,
    encrypted_token: String,
}

#[derive(Serialize)]
struct KsefTokenContextIdentifier {
    #[serde(rename = "type")]
    id_type: String,
    value: String,
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

pub fn submit_ksef_token_auth_request(
    client: &mut KsefClient,
    challenge: AuthChallenge,
    token: &str,
    context_type: ContextIdentifierType,
    context_value: &str,
    public_key_pem: &str,
) -> Result<AuthTokens, KsefError> {
    let encrypted_token = encrypt_token(token, challenge.timestamp_ms, public_key_pem)
        .map_err(|e| KsefError::OpenSslError(e))?;

    let context_type_str = match context_type {
        ContextIdentifierType::Nip => "Nip",
        ContextIdentifierType::InternalId => "InternalId",
        ContextIdentifierType::NipVatUe => "NipVatUe",
    };

    let request_body = KsefTokenAuthRequest {
        challenge: challenge.challenge,
        context_identifier: KsefTokenContextIdentifier {
            id_type: context_type_str.to_string(),
            value: context_value.to_string(),
        },
        encrypted_token,
    };

    let fut = async {
        let url = client.url_for(routes::AUTH_KSEF_TOKEN_PATH);
        let resp = client
            .client
            .post(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await
            .map_err(KsefError::RequestError)?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(KsefError::ApiError(status.as_u16(), body));
        }

        let parsed: AuthResponse = resp.json().await.map_err(KsefError::RequestError)?;
        Ok(AuthTokens {
            authentication_token: parsed.authentication_token.token,
            reference_number: parsed.reference_number,
        })
    };

    let tokens = match tokio::runtime::Handle::try_current() {
        Ok(handle) => handle.block_on(fut)?,
        Err(_) => {
            let rt = tokio::runtime::Runtime::new()
                .map_err(|e| KsefError::RuntimeError(e.to_string()))?;
            rt.block_on(fut)?
        }
    };

    client.auth_token = AuthTokens {
        authentication_token: tokens.authentication_token.clone(),
        reference_number: tokens.reference_number.clone(),
    };

    Ok(tokens)
}

fn encrypt_token(
    token: &str,
    timestamp_ms: i64,
    public_key_pem: &str,
) -> Result<String, openssl::error::ErrorStack> {
    let token_with_timestamp = format!("{}|{}", token, timestamp_ms);
    let token_bytes = token_with_timestamp.as_bytes();

    let rsa = PKey::public_key_from_pem(public_key_pem.as_bytes())?;

    let mut ctx = openssl::pkey_ctx::PkeyCtx::new(&rsa)?;
    ctx.encrypt_init()?;
    ctx.set_rsa_padding(Padding::PKCS1_OAEP)?;

    let md = MessageDigest::sha256();
    unsafe {
        let ptr = md.as_ptr();
        let md_ref = &*(ptr as *const MdRef);
        ctx.set_rsa_oaep_md(md_ref)?;
        ctx.set_rsa_mgf1_md(md_ref)?;
    }

    let mut output = vec![];
    ctx.encrypt_to_vec(token_bytes, &mut output)?;

    Ok(general_purpose::STANDARD.encode(output))
}

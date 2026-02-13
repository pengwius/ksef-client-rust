use crate::client::KsefClient;
use crate::client::auth::auth_token_request::ContextIdentifierType;
use crate::client::auth::xades_auth::AuthTokens;
use crate::client::error::KsefError;
use crate::client::get_public_key_certificates::PublicKeyCertificateUsage;
use crate::client::routes;
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

pub fn submit_ksef_token_auth_request(client: &mut KsefClient) -> Result<AuthTokens, KsefError> {
    let challenge = match client.get_auth_challenge() {
        Ok(challenge) => challenge,
        Err(e) => return Err(e),
    };

    let token = client.ksef_token.token.clone();
    let context_type = client
        .ksef_token
        .context_type
        .clone()
        .ok_or_else(|| KsefError::ApplicationError(0, "Context type not set".to_string()))?;
    let context_value = client
        .ksef_token
        .context_value
        .clone()
        .ok_or_else(|| KsefError::ApplicationError(0, "Context value not set".to_string()))?;

    let certificates = client.get_public_key_certificates()?;
    let encryption_cert = certificates
        .iter()
        .find(|c| {
            c.usage
                .contains(&PublicKeyCertificateUsage::KsefTokenEncryption)
        })
        .ok_or_else(|| {
            KsefError::Unexpected("No KsefTokenEncryption certificate found".to_string())
        })?;

    let cert_der = general_purpose::STANDARD
        .decode(&encryption_cert.certificate)
        .map_err(|e| KsefError::Unexpected(format!("Base64 decode error: {}", e)))?;

    let x509 = openssl::x509::X509::from_der(&cert_der).map_err(KsefError::OpenSslError)?;

    let pkey = x509.public_key().map_err(KsefError::OpenSslError)?;

    let pem_bytes = pkey.public_key_to_pem().map_err(KsefError::OpenSslError)?;

    let pem = String::from_utf8(pem_bytes)
        .map_err(|e| KsefError::Unexpected(format!("UTF-8 error: {}", e)))?;

    let encrypted_token = encrypt_token(&token, challenge.timestamp_ms, &pem)
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

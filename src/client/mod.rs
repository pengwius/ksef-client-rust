use crate::AccessTokens;
use crate::AuthChallenge;
use crate::AuthTokens;
use crate::DetailedKsefToken;
use crate::KsefToken;
use crate::client::error::KsefError;
use base64::{Engine as _, engine::general_purpose};

pub mod error;

pub mod auth_challenge;
pub mod auth_token_request;
pub mod get_access_token;
mod get_auth_status;
pub mod get_public_key_certificates;
pub mod ksef_tokens;
mod routes;
pub mod submit_ksef_token_auth_request;
pub mod submit_xades_auth_request;
pub mod xades;

pub struct KsefClient {
    pub base_url: String,
    pub client: reqwest::Client,
    pub xades: xades::XadesSigner,
    pub auth_token: AuthTokens,
    pub access_token: AccessTokens,
    pub ksef_token: KsefToken,
}

impl KsefClient {
    pub fn new() -> Self {
        Self::new_with_base("https://api-test.ksef.mf.gov.pl")
    }

    pub fn new_with_base(base: &str) -> Self {
        KsefClient {
            base_url: base.trim_end_matches('/').to_string(),
            client: reqwest::Client::new(),
            xades: xades::XadesSigner::default(),
            auth_token: AuthTokens::default(),
            access_token: AccessTokens::default(),
            ksef_token: KsefToken::default(),
        }
    }

    pub fn get_auth_challenge(&self) -> Result<auth_challenge::AuthChallenge, KsefError> {
        auth_challenge::get_auth_challenge(self)
    }

    pub fn submit_xades_auth_request(&mut self, signed_xml: String) -> Result<(), KsefError> {
        match submit_xades_auth_request::submit_xades_auth_request(self, signed_xml) {
            Ok(tokens) => {
                self.auth_token = tokens;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn authenticate_by_ksef_token(&mut self) -> Result<AuthTokens, KsefError> {
        let challenge = match auth_challenge::get_auth_challenge(self) {
            Ok(challenge) => challenge,
            Err(e) => return Err(e),
        };

        let token = self.ksef_token.token.clone();
        let context_type =
            self.ksef_token.context_type.clone().ok_or_else(|| {
                KsefError::ApplicationError(0, "Context type not set".to_string())
            })?;
        let context_value =
            self.ksef_token.context_value.clone().ok_or_else(|| {
                KsefError::ApplicationError(0, "Context value not set".to_string())
            })?;

        let certificates = get_public_key_certificates::get_public_key_certificates(self)?;
        let encryption_cert = certificates
            .iter()
            .find(|c| {
                c.usage.contains(
                    &get_public_key_certificates::PublicKeyCertificateUsage::KsefTokenEncryption,
                )
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

        submit_ksef_token_auth_request::submit_ksef_token_auth_request(
            self,
            challenge,
            &token,
            context_type,
            &context_value,
            &pem,
        )
    }

    pub fn get_auth_status(&self) -> Result<bool, KsefError> {
        get_auth_status::get_auth_status(self)
    }

    pub fn get_access_token(&mut self) -> Result<(), KsefError> {
        match get_access_token::get_access_token(self) {
            Ok(tokens) => {
                self.access_token = tokens;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn refresh_access_token(&mut self) -> Result<(), KsefError> {
        match get_access_token::refresh_access_token(self) {
            Ok(token) => {
                self.access_token = token;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn new_ksef_token(&mut self) -> Result<(), KsefError> {
        match ksef_tokens::new_ksef_token::new_ksef_token(self) {
            Ok(token) => {
                self.ksef_token = token;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn get_ksef_tokens(&mut self) -> Result<Vec<DetailedKsefToken>, KsefError> {
        ksef_tokens::get_ksef_tokens::get_ksef_tokens(self)
    }

    pub fn get_ksef_token_status(
        &self,
        token_reference_number: &str,
    ) -> Result<DetailedKsefToken, KsefError> {
        ksef_tokens::get_ksef_token_status::get_ksef_token_status(self, token_reference_number)
    }

    pub fn revoke_ksef_token(&self, token_reference_number: &str) -> Result<(), KsefError> {
        ksef_tokens::revoke_ksef_token::revoke_ksef_token(self, token_reference_number)
    }

    pub fn auth_token(&self) -> &AuthTokens {
        &self.auth_token
    }

    pub fn access_token(&self) -> &AccessTokens {
        &self.access_token
    }

    pub fn ksef_token(&self) -> &KsefToken {
        &self.ksef_token
    }

    pub fn url_for(&self, path: &str) -> String {
        format!(
            "{}/{}",
            self.base_url.trim_end_matches('/'),
            path.trim_start_matches('/')
        )
    }
}

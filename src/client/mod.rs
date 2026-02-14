use crate::AccessTokens;
use crate::AuthChallenge;
use crate::AuthTokenRequest;
use crate::AuthTokenRequestBuilder;
use crate::AuthTokens;
use crate::ContextIdentifierType;
use crate::DetailedKsefToken;
use crate::KsefToken;
use crate::KsefTokenPermissions;
use crate::SubjectIdentifierType;
use crate::client::error::KsefError;
use crate::client::get_public_key_certificates::PublicKeyCertificate;

pub mod error;

pub mod auth;
pub mod get_public_key_certificates;
pub mod ksef_tokens;
mod routes;
pub mod sessions;
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

    pub fn get_auth_challenge(&self) -> Result<AuthChallenge, KsefError> {
        auth::auth_challenge::get_auth_challenge(self)
    }

    pub fn get_auth_token_request(
        &self,
        id: &str,
        id_type: ContextIdentifierType,
        subject_type: SubjectIdentifierType,
    ) -> Result<AuthTokenRequest, KsefError> {
        let challenge = match self.get_auth_challenge() {
            Ok(ch) => ch.challenge,
            Err(e) => {
                return Err(KsefError::ApplicationError(
                    0,
                    format!("Unable to get AuthChallenge: {}", e),
                ));
            }
        };

        let built = AuthTokenRequestBuilder::new()
            .with_challenge(&challenge)
            .with_context(id_type, id)
            .with_subject_type(subject_type)
            .build();

        let auth_token_request = match built {
            Ok(req) => req,
            Err(e) => {
                return Err(KsefError::ApplicationError(
                    0,
                    format!("Unable to build AuthTokenRequest: {}", e),
                ));
            }
        };

        Ok(auth_token_request)
    }

    pub fn authenticate_by_xades_signature(&mut self, signed_xml: String) -> Result<(), KsefError> {
        match auth::xades_auth::submit_xades_auth_request(self, signed_xml) {
            Ok(tokens) => {
                self.auth_token = tokens;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn authenticate_by_ksef_token(&mut self) -> Result<(), KsefError> {
        match auth::ksef_token_auth::submit_ksef_token_auth_request(self) {
            Ok(tokens) => {
                self.auth_token = tokens;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn get_auth_status(&mut self) -> Result<bool, KsefError> {
        auth::get_auth_status::get_auth_status(self)
    }

    pub fn get_access_token(&mut self) -> Result<(), KsefError> {
        match auth::get_access_token::get_access_token(self) {
            Ok(tokens) => {
                self.access_token = tokens;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn refresh_access_token(&mut self) -> Result<(), KsefError> {
        match auth::get_access_token::refresh_access_token(self) {
            Ok(token) => {
                self.access_token = token;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn get_public_key_certificates(&self) -> Result<Vec<PublicKeyCertificate>, KsefError> {
        get_public_key_certificates::get_public_key_certificates(self)
    }

    pub fn new_ksef_token(
        &mut self,
        load: bool,
        permissions: KsefTokenPermissions,
        description: &str,
    ) -> Result<KsefToken, KsefError> {
        match ksef_tokens::new_ksef_token::new_ksef_token(self, permissions, description) {
            Ok(token) => {
                if load {
                    self.ksef_token = token.clone();
                }
                Ok(token)
            }
            Err(e) => Err(e),
        }
    }

    pub fn load_ksef_token(&mut self, token: KsefToken) {
        self.ksef_token = token;
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

    pub fn get_active_sessions(
        &self,
        continuation_token: Option<&str>,
    ) -> Result<crate::client::sessions::QuerySessionsResponse, KsefError> {
        sessions::get_active_sessions::get_active_sessions(self, continuation_token)
    }

    pub fn revoke_current_session(&self) -> Result<(), KsefError> {
        sessions::revoke_current_session::revoke_current_session(self)
    }

    pub fn revoke_session(&self, reference_number: &str) -> Result<(), KsefError> {
        sessions::revoke_session::revoke_session(self, reference_number)
    }

    pub fn url_for(&self, path: &str) -> String {
        format!(
            "{}/{}",
            self.base_url.trim_end_matches('/'),
            path.trim_start_matches('/')
        )
    }
}

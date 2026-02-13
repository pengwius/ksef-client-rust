use crate::AccessTokens;
use crate::AuthTokens;
use crate::DetailedKsefToken;
use crate::KsefToken;
use crate::client::error::KsefError;

pub mod error;

mod auth_challenge;
pub mod auth_token_request;
pub mod get_access_token;
mod get_auth_status;
pub mod ksef_tokens;
mod routes;
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

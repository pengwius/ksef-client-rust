use crate::client::error::KsefError;

pub mod error;

mod auth_challenge;
mod routes;

pub struct KsefClient {
    pub base_url: String,
    pub client: reqwest::Client,
}

impl KsefClient {
    pub fn new() -> Self {
        Self::new_with_base("https://api-test.ksef.mf.gov.pl")
    }

    pub fn new_with_base(base: &str) -> Self {
        KsefClient {
            base_url: base.trim_end_matches('/').to_string(),
            client: reqwest::Client::new(),
        }
    }

    pub fn get_auth_challenge(&self) -> Result<auth_challenge::AuthChallenge, KsefError> {
        auth_challenge::get_auth_challenge(self)
    }

    pub fn url_for(&self, path: &str) -> String {
        format!("{}/{}", self.base_url.trim_end_matches('/'), path.trim_start_matches('/'))
    }
}

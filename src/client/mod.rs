use crate::client::auth::get_access_token::AccessTokens;
use crate::client::auth::xades_auth::AuthTokens;

pub use crate::client::error::KsefError;

use crate::client::ksef_tokens::new_ksef_token::KsefToken;
use crate::client::models::ContextIdentifier;
use crate::client::models::Environment;
use crate::client::xades::XadesSigner;
use std::time::Duration;

pub mod auth;
pub mod batch_session;
pub mod error;
pub mod fetching_invoices;
pub mod get_public_key_certificates;
pub mod ksef_certificates;
pub mod ksef_tokens;
pub mod models;
pub mod online_session;
pub mod peppol;
pub mod permissions;
pub mod qr;
mod routes;
pub mod sessions;
pub mod traits;
pub mod upo;
pub mod xades;

pub struct KsefClient {
    pub base_url: String,
    pub environment: Option<Environment>,
    pub context: ContextIdentifier,
    pub client: reqwest::Client,
    pub xades: XadesSigner,
    pub auth_token: AuthTokens,
    pub access_token: AccessTokens,
    pub ksef_token: KsefToken,
}

impl KsefClient {
    pub fn new(environment: Environment, context: ContextIdentifier) -> Self {
        Self::new_with_base(environment.base_url(), context).with_environment(environment)
    }

    pub fn new_with_base(base_url: &str, context: ContextIdentifier) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .unwrap();

        KsefClient {
            base_url: base_url.to_string(),
            environment: None,
            context,
            client,
            xades: XadesSigner::default(),
            auth_token: AuthTokens {
                authentication_token: String::new(),
                reference_number: String::new(),
            },
            access_token: AccessTokens::default(),
            ksef_token: KsefToken::default(),
        }
    }

    fn with_environment(mut self, environment: Environment) -> Self {
        self.environment = Some(environment);
        self
    }

    pub fn url_for(&self, path: &str) -> String {
        format!(
            "{}/{}",
            self.base_url.trim_end_matches('/'),
            path.trim_start_matches('/')
        )
    }
}

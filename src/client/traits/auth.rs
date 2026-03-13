use crate::client::KsefClient;
use crate::client::auth;
use crate::client::auth::auth_challenge::AuthChallenge;
use crate::client::auth::auth_token_request::{AuthTokenRequest, SubjectIdentifierType};
use crate::client::auth::get_access_token::AccessTokens;
use crate::client::auth::xades_auth::AuthTokens;
use crate::client::error::KsefError;
use async_trait::async_trait;

#[async_trait]
pub trait KsefAuth {
    async fn get_auth_challenge(&self) -> Result<AuthChallenge, KsefError>;

    async fn get_auth_token_request(
        &self,
        subject_type: SubjectIdentifierType,
    ) -> Result<AuthTokenRequest, KsefError>;

    async fn authenticate_by_xades_signature(
        &mut self,
        signed_xml: String,
    ) -> Result<(), KsefError>;

    async fn authenticate_by_ksef_token(&mut self) -> Result<(), KsefError>;

    async fn get_auth_status(&mut self) -> Result<bool, KsefError>;

    async fn get_access_token(&mut self) -> Result<(), KsefError>;

    async fn refresh_access_token(&mut self) -> Result<(), KsefError>;

    fn auth_token(&self) -> &AuthTokens;

    fn access_token(&self) -> &AccessTokens;
}

#[async_trait]
impl KsefAuth for KsefClient {
    async fn get_auth_challenge(&self) -> Result<AuthChallenge, KsefError> {
        auth::auth_challenge::get_auth_challenge(self).await
    }

    async fn get_auth_token_request(
        &self,
        subject_type: SubjectIdentifierType,
    ) -> Result<AuthTokenRequest, KsefError> {
        auth::get_auth_token_request::get_auth_token_request(self, subject_type).await
    }

    async fn authenticate_by_xades_signature(
        &mut self,
        signed_xml: String,
    ) -> Result<(), KsefError> {
        auth::xades_auth::submit_xades_auth_request_and_load(self, signed_xml).await
    }

    async fn authenticate_by_ksef_token(&mut self) -> Result<(), KsefError> {
        auth::ksef_token_auth::submit_ksef_token_auth_request(self)
            .await
            .map(|_| ())
    }

    async fn get_auth_status(&mut self) -> Result<bool, KsefError> {
        auth::get_auth_status::get_auth_status(self).await
    }

    async fn get_access_token(&mut self) -> Result<(), KsefError> {
        auth::get_access_token::get_access_token_and_load(self).await
    }

    async fn refresh_access_token(&mut self) -> Result<(), KsefError> {
        auth::get_access_token::refresh_access_token_and_load(self).await
    }

    fn auth_token(&self) -> &AuthTokens {
        &self.auth_token
    }

    fn access_token(&self) -> &AccessTokens {
        &self.access_token
    }
}

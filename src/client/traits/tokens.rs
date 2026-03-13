use crate::client::KsefClient;
use crate::client::error::KsefError;
use crate::client::ksef_tokens;
use crate::client::ksef_tokens::models::DetailedKsefToken;
use crate::client::ksef_tokens::new_ksef_token::{KsefToken, KsefTokenPermissions};
use async_trait::async_trait;

#[async_trait]
pub trait KsefTokens {
    async fn new_ksef_token(
        &mut self,
        load: bool,
        permissions: KsefTokenPermissions,
        description: &str,
    ) -> Result<KsefToken, KsefError>;

    fn load_ksef_token(&mut self, token: KsefToken);

    async fn get_ksef_tokens(&mut self) -> Result<Vec<DetailedKsefToken>, KsefError>;

    async fn get_ksef_token_status(
        &self,
        token_reference_number: &str,
    ) -> Result<DetailedKsefToken, KsefError>;

    async fn revoke_ksef_token(&self, token_reference_number: &str) -> Result<(), KsefError>;

    fn ksef_token(&self) -> &KsefToken;
}

#[async_trait]
impl KsefTokens for KsefClient {
    async fn new_ksef_token(
        &mut self,
        load: bool,
        permissions: KsefTokenPermissions,
        description: &str,
    ) -> Result<KsefToken, KsefError> {
        ksef_tokens::new_ksef_token::new_ksef_token_and_load(self, load, permissions, description)
            .await
    }

    fn load_ksef_token(&mut self, token: KsefToken) {
        self.ksef_token = token;
    }

    async fn get_ksef_tokens(&mut self) -> Result<Vec<DetailedKsefToken>, KsefError> {
        ksef_tokens::get_ksef_tokens::get_ksef_tokens(self).await
    }

    async fn get_ksef_token_status(
        &self,
        token_reference_number: &str,
    ) -> Result<DetailedKsefToken, KsefError> {
        ksef_tokens::get_ksef_token_status::get_ksef_token_status(self, token_reference_number)
            .await
    }

    async fn revoke_ksef_token(&self, token_reference_number: &str) -> Result<(), KsefError> {
        ksef_tokens::revoke_ksef_token::revoke_ksef_token(self, token_reference_number).await
    }

    fn ksef_token(&self) -> &KsefToken {
        &self.ksef_token
    }
}

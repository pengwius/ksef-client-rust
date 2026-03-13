use crate::client::KsefClient;
use crate::client::error::KsefError;
use crate::client::peppol;
use crate::client::peppol::get_peppol_providers::GetPeppolProvidersResponse;
use async_trait::async_trait;

#[async_trait]
pub trait KsefPeppol {
    async fn get_peppol_providers(
        &self,
        page_size: Option<i32>,
        page_offset: Option<i32>,
    ) -> Result<GetPeppolProvidersResponse, KsefError>;
}

#[async_trait]
impl KsefPeppol for KsefClient {
    async fn get_peppol_providers(
        &self,
        page_size: Option<i32>,
        page_offset: Option<i32>,
    ) -> Result<GetPeppolProvidersResponse, KsefError> {
        peppol::get_peppol_providers::get_peppol_providers(self, page_size, page_offset).await
    }
}

use crate::client::KsefClient;
use crate::client::error::KsefError;
use crate::client::fetching_invoices;
use crate::client::fetching_invoices::export_invoices::{
    ExportInvoicesRequest, ExportInvoicesResponse, ExportInvoicesStatusResponse, ExportResult,
};
use crate::client::fetching_invoices::fetch_invoice::FetchInvoiceResponse;
use crate::client::fetching_invoices::fetch_invoice_metadata::{
    FetchInvoiceMetadataRequest, FetchInvoiceMetadataResponse, QueryCriteria, SubjectType,
};
use crate::client::fetching_invoices::incremental_fetch::{FetchedInvoice, IncrementalFetchState};
use crate::client::types::{KsefNumber, ReferenceNumber};
use crate::client::upo;
use crate::client::upo::get_invoice_upo_by_ksef_number::{
    GetInvoiceUpoResponse, InvoiceIdentifier,
};
use async_trait::async_trait;

#[async_trait]
pub trait KsefInvoices {
    async fn get_invoice_upo(
        &self,
        reference_number: ReferenceNumber,
        identifier: InvoiceIdentifier,
    ) -> Result<GetInvoiceUpoResponse, KsefError>;

    async fn fetch_invoice_metadata(
        &self,
        request: FetchInvoiceMetadataRequest,
    ) -> Result<FetchInvoiceMetadataResponse, KsefError>;

    async fn fetch_invoice(
        &self,
        ksef_number: KsefNumber,
    ) -> Result<FetchInvoiceResponse, KsefError>;

    async fn start_export_invoices(
        &self,
        request: ExportInvoicesRequest,
    ) -> Result<ExportInvoicesResponse, KsefError>;

    async fn get_export_status(
        &self,
        reference_number: ReferenceNumber,
    ) -> Result<ExportInvoicesStatusResponse, KsefError>;

    async fn export_invoices(&self, query: QueryCriteria) -> Result<ExportResult, KsefError>;

    async fn export_invoices_incrementally(
        &self,
        state: &mut IncrementalFetchState,
        subject_types: Vec<SubjectType>,
        window_end: Option<chrono::DateTime<chrono::Utc>>,
        default_start: chrono::DateTime<chrono::Utc>,
    ) -> Result<Vec<FetchedInvoice>, KsefError>;
}

#[async_trait]
impl KsefInvoices for KsefClient {
    async fn get_invoice_upo(
        &self,
        reference_number: ReferenceNumber,
        identifier: InvoiceIdentifier,
    ) -> Result<GetInvoiceUpoResponse, KsefError> {
        upo::get_invoice_upo_by_ksef_number::get_invoice_upo(self, &reference_number, identifier)
            .await
    }

    async fn fetch_invoice_metadata(
        &self,
        request: FetchInvoiceMetadataRequest,
    ) -> Result<FetchInvoiceMetadataResponse, KsefError> {
        fetching_invoices::fetch_invoice_metadata::fetch_invoice_metadata(self, request).await
    }

    async fn fetch_invoice(
        &self,
        ksef_number: KsefNumber,
    ) -> Result<FetchInvoiceResponse, KsefError> {
        fetching_invoices::fetch_invoice::fetch_invoice(self, &ksef_number).await
    }

    async fn start_export_invoices(
        &self,
        request: ExportInvoicesRequest,
    ) -> Result<ExportInvoicesResponse, KsefError> {
        fetching_invoices::export_invoices::start_export_invoices(self, request).await
    }

    async fn get_export_status(
        &self,
        reference_number: ReferenceNumber,
    ) -> Result<ExportInvoicesStatusResponse, KsefError> {
        fetching_invoices::export_invoices::get_export_status(self, &reference_number).await
    }

    async fn export_invoices(&self, query: QueryCriteria) -> Result<ExportResult, KsefError> {
        fetching_invoices::export_invoices::export_invoices(self, query).await
    }

    async fn export_invoices_incrementally(
        &self,
        state: &mut IncrementalFetchState,
        subject_types: Vec<SubjectType>,
        window_end: Option<chrono::DateTime<chrono::Utc>>,
        default_start: chrono::DateTime<chrono::Utc>,
    ) -> Result<Vec<FetchedInvoice>, KsefError> {
        fetching_invoices::incremental_fetch::fetch_invoices_incrementally(
            self,
            state,
            subject_types,
            window_end,
            default_start,
        )
        .await
    }
}

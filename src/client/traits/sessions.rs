use crate::client::KsefClient;
use crate::client::batch_session;
use crate::client::batch_session::full_flow::BatchSubmissionResult;
use crate::client::batch_session::open_batch_session::{
    OpenBatchSessionRequest, OpenBatchSessionResponse,
};
use crate::client::batch_session::zip::EncryptedBatchPart;
use crate::client::error::KsefError;
use crate::client::models::InvoicePayload;
use crate::client::online_session;
use crate::client::online_session::encryption::EncryptionData;
use crate::client::online_session::full_flow::OnlineSubmissionResult;
use crate::client::online_session::get_invoice_status::GetInvoiceStatusResponse;
use crate::client::online_session::open_online_session::{
    OpenOnlineSessionRequest, OpenOnlineSessionResponse,
};
use crate::client::online_session::send_invoice::SendInvoiceResponse;
use crate::client::sessions;
use crate::client::sessions::QuerySessionsResponse;
use crate::client::types::ReferenceNumber;
use async_trait::async_trait;

#[async_trait]
pub trait KsefSessions {
    async fn get_active_sessions(
        &self,
        continuation_token: Option<&str>,
    ) -> Result<QuerySessionsResponse, KsefError>;

    async fn revoke_current_session(&self) -> Result<(), KsefError>;

    async fn revoke_session(&self, reference_number: ReferenceNumber) -> Result<(), KsefError>;

    async fn open_online_session(
        &self,
        request: OpenOnlineSessionRequest,
    ) -> Result<OpenOnlineSessionResponse, KsefError>;

    async fn open_batch_session(
        &self,
        request: OpenBatchSessionRequest,
    ) -> Result<OpenBatchSessionResponse, KsefError>;

    async fn upload_batch_parts(
        &self,
        response: &OpenBatchSessionResponse,
        parts: &[EncryptedBatchPart],
    ) -> Result<(), KsefError>;

    async fn close_batch_session(&self, reference_number: ReferenceNumber)
    -> Result<(), KsefError>;

    async fn submit_batch(
        &self,
        invoices: &[InvoicePayload],
        max_part_size_bytes: Option<usize>,
    ) -> Result<BatchSubmissionResult, KsefError>;

    async fn submit_online(&self, invoice: &[u8]) -> Result<OnlineSubmissionResult, KsefError>;

    async fn send_invoice(
        &self,
        reference_number: ReferenceNumber,
        invoice_xml: &[u8],
        encryption_data: &EncryptionData,
    ) -> Result<SendInvoiceResponse, KsefError>;

    async fn get_invoice_status(
        &self,
        session_reference_number: ReferenceNumber,
        invoice_reference_number: ReferenceNumber,
    ) -> Result<GetInvoiceStatusResponse, KsefError>;

    async fn close_online_session(
        &self,
        reference_number: ReferenceNumber,
    ) -> Result<(), KsefError>;

    async fn generate_encryption_data(&self) -> Result<EncryptionData, KsefError>;
}

#[async_trait]
impl KsefSessions for KsefClient {
    async fn get_active_sessions(
        &self,
        continuation_token: Option<&str>,
    ) -> Result<QuerySessionsResponse, KsefError> {
        sessions::get_active_sessions::get_active_sessions(self, continuation_token).await
    }

    async fn revoke_current_session(&self) -> Result<(), KsefError> {
        sessions::revoke_current_session::revoke_current_session(self).await
    }

    async fn revoke_session(&self, reference_number: ReferenceNumber) -> Result<(), KsefError> {
        sessions::revoke_session::revoke_session(self, reference_number.as_str()).await
    }

    async fn open_online_session(
        &self,
        request: OpenOnlineSessionRequest,
    ) -> Result<OpenOnlineSessionResponse, KsefError> {
        online_session::open_online_session::open_online_session(self, request).await
    }

    async fn open_batch_session(
        &self,
        request: OpenBatchSessionRequest,
    ) -> Result<OpenBatchSessionResponse, KsefError> {
        batch_session::open_batch_session::open_batch_session(self, request).await
    }

    async fn upload_batch_parts(
        &self,
        response: &OpenBatchSessionResponse,
        parts: &[EncryptedBatchPart],
    ) -> Result<(), KsefError> {
        batch_session::upload_batch_parts::upload_batch_parts(self, response, parts).await
    }

    async fn close_batch_session(
        &self,
        reference_number: ReferenceNumber,
    ) -> Result<(), KsefError> {
        batch_session::close_batch_session::close_batch_session(self, reference_number.as_str())
            .await
    }

    async fn submit_batch(
        &self,
        invoices: &[InvoicePayload],
        max_part_size_bytes: Option<usize>,
    ) -> Result<BatchSubmissionResult, KsefError> {
        batch_session::full_flow::submit_batch(self, invoices, max_part_size_bytes).await
    }

    async fn submit_online(&self, invoice: &[u8]) -> Result<OnlineSubmissionResult, KsefError> {
        online_session::full_flow::submit_online(self, invoice).await
    }

    async fn send_invoice(
        &self,
        reference_number: ReferenceNumber,
        invoice_xml: &[u8],
        encryption_data: &EncryptionData,
    ) -> Result<SendInvoiceResponse, KsefError> {
        online_session::send_invoice::send_invoice(
            self,
            &reference_number,
            invoice_xml,
            encryption_data,
        )
        .await
    }

    async fn get_invoice_status(
        &self,
        session_reference_number: ReferenceNumber,
        invoice_reference_number: ReferenceNumber,
    ) -> Result<GetInvoiceStatusResponse, KsefError> {
        online_session::get_invoice_status::get_invoice_status(
            self,
            session_reference_number.as_str(),
            invoice_reference_number.as_str(),
        )
        .await
    }

    async fn close_online_session(
        &self,
        reference_number: ReferenceNumber,
    ) -> Result<(), KsefError> {
        online_session::close_online_session::close_online_session(self, &reference_number).await
    }

    async fn generate_encryption_data(&self) -> Result<EncryptionData, KsefError> {
        online_session::encryption::generate_encryption_data(self).await
    }
}

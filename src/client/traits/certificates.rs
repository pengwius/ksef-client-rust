use crate::client::KsefClient;
use crate::client::error::KsefError;
use crate::client::get_public_key_certificates;
use crate::client::get_public_key_certificates::PublicKeyCertificate;
use crate::client::ksef_certificates;
use crate::client::ksef_certificates::csr::CsrResult;
use crate::client::ksef_certificates::enroll_certificate::{
    EnrollCertificateRequest, EnrollCertificateResponse,
};
use crate::client::ksef_certificates::get_certificate_metadata_list::{
    GetCertificateMetadataListRequest, GetCertificateMetadataListResponse,
};
use crate::client::ksef_certificates::get_certificates_limits::CertificateLimits;
use crate::client::ksef_certificates::get_enrollment_data::EnrollmentData;
use crate::client::ksef_certificates::get_enrollment_status::EnrollmentStatusResponse;
use crate::client::ksef_certificates::retrieve_certificates::RetrieveCertificatesListItem;
use crate::client::ksef_certificates::revoke_certificate::RevocationReason;
use async_trait::async_trait;

#[async_trait]
pub trait KsefCertificates {
    async fn get_public_key_certificates(&self) -> Result<Vec<PublicKeyCertificate>, KsefError>;

    async fn get_certificates_limits(&self) -> Result<CertificateLimits, KsefError>;

    async fn get_enrollment_data(&self) -> Result<EnrollmentData, KsefError>;

    async fn enroll_certificate(
        &self,
        request: EnrollCertificateRequest,
    ) -> Result<EnrollCertificateResponse, KsefError>;

    fn generate_csr(&self, enrollment_data: &EnrollmentData) -> Result<CsrResult, KsefError>;

    async fn get_enrollment_status(
        &self,
        reference_number: &str,
    ) -> Result<EnrollmentStatusResponse, KsefError>;

    async fn retrieve_certificates(
        &self,
        serial_numbers: Vec<String>,
    ) -> Result<Vec<RetrieveCertificatesListItem>, KsefError>;

    async fn get_certificate_metadata_list(
        &self,
        query: GetCertificateMetadataListRequest,
        page_size: Option<i32>,
        page_offset: Option<i32>,
    ) -> Result<GetCertificateMetadataListResponse, KsefError>;

    async fn revoke_certificate(
        &self,
        serial_number: &str,
        reason: RevocationReason,
    ) -> Result<(), KsefError>;
}

#[async_trait]
impl KsefCertificates for KsefClient {
    async fn get_public_key_certificates(&self) -> Result<Vec<PublicKeyCertificate>, KsefError> {
        get_public_key_certificates::get_public_key_certificates(self).await
    }

    async fn get_certificates_limits(&self) -> Result<CertificateLimits, KsefError> {
        ksef_certificates::get_certificates_limits::get_certificates_limits(self).await
    }

    async fn get_enrollment_data(&self) -> Result<EnrollmentData, KsefError> {
        ksef_certificates::get_enrollment_data::get_enrollment_data(self).await
    }

    async fn enroll_certificate(
        &self,
        request: EnrollCertificateRequest,
    ) -> Result<EnrollCertificateResponse, KsefError> {
        ksef_certificates::enroll_certificate::enroll_certificate(self, request).await
    }

    fn generate_csr(&self, enrollment_data: &EnrollmentData) -> Result<CsrResult, KsefError> {
        ksef_certificates::csr::generate_csr(enrollment_data)
    }

    async fn get_enrollment_status(
        &self,
        reference_number: &str,
    ) -> Result<EnrollmentStatusResponse, KsefError> {
        ksef_certificates::get_enrollment_status::get_enrollment_status(self, reference_number)
            .await
    }

    async fn retrieve_certificates(
        &self,
        serial_numbers: Vec<String>,
    ) -> Result<Vec<RetrieveCertificatesListItem>, KsefError> {
        ksef_certificates::retrieve_certificates::retrieve_certificates(self, serial_numbers).await
    }

    async fn get_certificate_metadata_list(
        &self,
        query: GetCertificateMetadataListRequest,
        page_size: Option<i32>,
        page_offset: Option<i32>,
    ) -> Result<GetCertificateMetadataListResponse, KsefError> {
        ksef_certificates::get_certificate_metadata_list::get_certificate_metadata_list(
            self,
            query,
            page_size,
            page_offset,
        )
        .await
    }

    async fn revoke_certificate(
        &self,
        serial_number: &str,
        reason: RevocationReason,
    ) -> Result<(), KsefError> {
        ksef_certificates::revoke_certificate::revoke_certificate(self, serial_number, reason).await
    }
}

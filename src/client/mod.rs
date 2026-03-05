use crate::AccessTokens;
use crate::AuthChallenge;
use crate::AuthTokenRequest;
use crate::AuthTokenRequestBuilder;
use crate::AuthTokens;
use crate::CertificateLimits;
use crate::ContextIdentifier;
use crate::CsrResult;
use crate::DetailedKsefToken;
use crate::EnrollmentData;
use crate::EnrollmentStatusResponse;
use crate::Environment;
use crate::InvoicePayload;
use crate::KsefToken;
use crate::KsefTokenPermissions;
use crate::QuerySessionsResponse;
use crate::RetrieveCertificatesListItem;
use crate::RevocationReason;
use crate::SubjectIdentifierType;
use crate::client::batch_session::full_flow::BatchSubmissionResult;
use crate::client::batch_session::open_batch_session::{
    OpenBatchSessionRequest, OpenBatchSessionResponse,
};
use crate::client::batch_session::zip::EncryptedBatchPart;
use crate::client::error::KsefError;
use crate::client::fetching_invoices::export_invoices::{
    ExportInvoicesRequest, ExportInvoicesResponse, ExportInvoicesStatusResponse,
};
use crate::client::fetching_invoices::fetch_invoice::FetchInvoiceResponse;
use crate::client::fetching_invoices::fetch_invoice_metadata::{
    FetchInvoiceMetadataRequest, FetchInvoiceMetadataResponse, QueryCriteria,
};
use crate::client::get_public_key_certificates::PublicKeyCertificate;
use crate::client::ksef_certificates::enroll_certificate::{
    EnrollCertificateRequest, EnrollCertificateResponse,
};
use crate::client::online_session::encryption::EncryptionData;
use crate::client::online_session::full_flow::OnlineSubmissionResult;
use crate::client::online_session::get_invoice_status::GetInvoiceStatusResponse;
use crate::client::online_session::open_online_session::{
    OpenOnlineSessionRequest, OpenOnlineSessionResponse,
};
use crate::client::online_session::send_invoice::SendInvoiceResponse;
use crate::client::permissions::get_operation_status::OperationStatusResponse;
use crate::client::permissions::grant_authorization_permissions::GrantAuthorizationPermissionsRequest;
use crate::client::permissions::grant_entity_permissions::GrantEntityPermissionsRequest;
use crate::client::permissions::grant_eu_entity_permissions::{
    GrantEuEntityPermissionsRequest, GrantEuEntityPermissionsResponse,
};
use crate::client::permissions::grant_eu_entity_representative_permissions::{
    GrantEuEntityRepresentativePermissionsRequest, GrantEuEntityRepresentativePermissionsResponse,
};
use crate::client::permissions::grant_indirect_entity_permissions::{
    GrantIndirectEntityPermissionsRequest, GrantIndirectEntityPermissionsResponse,
};
use crate::client::permissions::grant_person_permissions::GrantPersonPermissionsRequest;
use crate::client::permissions::grant_subunit_permissions::GrantSubunitPermissionsRequest;
use crate::{GetCertificateMetadataListRequest, GetCertificateMetadataListResponse};

pub mod error;

pub mod auth;
pub mod batch_session;
pub mod fetching_invoices;
pub mod get_public_key_certificates;
pub mod ksef_certificates;
pub mod ksef_tokens;
pub mod models;
pub mod online_session;
pub mod permissions;
mod routes;
pub mod sessions;
pub mod xades;

pub struct KsefClient {
    pub base_url: String,
    pub environment: Option<Environment>,
    pub context: ContextIdentifier,
    pub client: reqwest::Client,
    pub xades: xades::XadesSigner,
    pub auth_token: AuthTokens,
    pub access_token: AccessTokens,
    pub ksef_token: KsefToken,
}

impl KsefClient {
    pub fn new(environment: Environment, context: ContextIdentifier) -> Self {
        Self::new_with_base(environment.base_url(), context).with_environment(environment)
    }

    pub fn new_with_base(base: &str, context: ContextIdentifier) -> Self {
        KsefClient {
            base_url: base.trim_end_matches('/').to_string(),
            environment: None,
            context,
            client: reqwest::Client::new(),
            xades: xades::XadesSigner::default(),
            auth_token: AuthTokens::default(),
            access_token: AccessTokens::default(),
            ksef_token: KsefToken::default(),
        }
    }

    fn with_environment(mut self, environment: Environment) -> Self {
        self.environment = Some(environment);
        self
    }

    pub async fn get_auth_challenge(&self) -> Result<AuthChallenge, KsefError> {
        auth::auth_challenge::get_auth_challenge(self).await
    }

    pub async fn get_auth_token_request(
        &self,
        subject_type: SubjectIdentifierType,
    ) -> Result<AuthTokenRequest, KsefError> {
        let challenge = match self.get_auth_challenge().await {
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
            .with_context(self.context.id_type.clone(), &self.context.value)
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

    pub async fn authenticate_by_xades_signature(
        &mut self,
        signed_xml: String,
    ) -> Result<(), KsefError> {
        match auth::xades_auth::submit_xades_auth_request(self, signed_xml).await {
            Ok(tokens) => {
                self.auth_token = tokens;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub async fn authenticate_by_ksef_token(&mut self) -> Result<(), KsefError> {
        match auth::ksef_token_auth::submit_ksef_token_auth_request(self).await {
            Ok(tokens) => {
                self.auth_token = tokens;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub async fn get_auth_status(&mut self) -> Result<bool, KsefError> {
        auth::get_auth_status::get_auth_status(self).await
    }

    pub async fn get_access_token(&mut self) -> Result<(), KsefError> {
        match auth::get_access_token::get_access_token(self).await {
            Ok(tokens) => {
                self.access_token = tokens;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub async fn refresh_access_token(&mut self) -> Result<(), KsefError> {
        match auth::get_access_token::refresh_access_token(self).await {
            Ok(token) => {
                self.access_token = token;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub async fn get_public_key_certificates(
        &self,
    ) -> Result<Vec<PublicKeyCertificate>, KsefError> {
        get_public_key_certificates::get_public_key_certificates(self).await
    }

    pub async fn new_ksef_token(
        &mut self,
        load: bool,
        permissions: KsefTokenPermissions,
        description: &str,
    ) -> Result<KsefToken, KsefError> {
        match ksef_tokens::new_ksef_token::new_ksef_token(self, permissions, description).await {
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

    pub async fn get_ksef_tokens(&mut self) -> Result<Vec<DetailedKsefToken>, KsefError> {
        ksef_tokens::get_ksef_tokens::get_ksef_tokens(self).await
    }

    pub async fn get_ksef_token_status(
        &self,
        token_reference_number: &str,
    ) -> Result<DetailedKsefToken, KsefError> {
        ksef_tokens::get_ksef_token_status::get_ksef_token_status(self, token_reference_number)
            .await
    }

    pub async fn revoke_ksef_token(&self, token_reference_number: &str) -> Result<(), KsefError> {
        ksef_tokens::revoke_ksef_token::revoke_ksef_token(self, token_reference_number).await
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

    pub async fn get_active_sessions(
        &self,
        continuation_token: Option<&str>,
    ) -> Result<QuerySessionsResponse, KsefError> {
        sessions::get_active_sessions::get_active_sessions(self, continuation_token).await
    }

    pub async fn revoke_current_session(&self) -> Result<(), KsefError> {
        sessions::revoke_current_session::revoke_current_session(self).await
    }

    pub async fn revoke_session(&self, reference_number: &str) -> Result<(), KsefError> {
        sessions::revoke_session::revoke_session(self, reference_number).await
    }

    pub async fn grant_person_permissions(
        &self,
        request: GrantPersonPermissionsRequest,
    ) -> Result<OperationStatusResponse, KsefError> {
        permissions::grant_person_permissions::grant_person_permissions(self, request).await
    }

    pub async fn grant_entity_permissions(
        &self,
        request: GrantEntityPermissionsRequest,
    ) -> Result<crate::client::permissions::get_operation_status::OperationStatusResponse, KsefError>
    {
        permissions::grant_entity_permissions::grant_entity_permissions(self, request).await
    }

    pub async fn grant_authorization_permissions(
        &self,
        request: GrantAuthorizationPermissionsRequest,
    ) -> Result<crate::client::permissions::get_operation_status::OperationStatusResponse, KsefError>
    {
        permissions::grant_authorization_permissions::grant_authorization_permissions(self, request)
            .await
    }

    pub async fn get_authorizations_permissions(
        &self,
        page_offset: Option<i32>,
        page_size: Option<i32>,
        request: crate::client::permissions::get_authorizations_permissions::GetAuthorizationsPermissionsRequest,
    ) -> Result<
        crate::client::permissions::get_authorizations_permissions::GetAuthorizationsPermissionsResponse,
        KsefError,
    >{
        permissions::get_authorizations_permissions::get_authorizations_permissions(
            self,
            page_offset,
            page_size,
            request,
        )
        .await
    }

    pub async fn get_entities_permissions(
        &self,
        page_offset: Option<i32>,
        page_size: Option<i32>,
        request: Option<
            crate::client::permissions::get_entities_permissions::GetEntitiesPermissionsRequest,
        >,
    ) -> Result<
        crate::client::permissions::get_entities_permissions::GetEntitiesPermissionsResponse,
        KsefError,
    > {
        permissions::get_entities_permissions::get_entities_permissions(
            self,
            page_offset,
            page_size,
            request,
        )
        .await
    }

    pub async fn get_subordinate_entities_roles(
        &self,
        page_offset: Option<i32>,
        page_size: Option<i32>,
        request: Option<crate::client::permissions::get_subordinate_entities_roles::GetSubordinateEntitiesRolesRequest>,
    ) -> Result<
        crate::client::permissions::get_subordinate_entities_roles::GetSubordinateEntitiesRolesResponse,
        KsefError,
    >{
        permissions::get_subordinate_entities_roles::get_subordinate_entities_roles(
            self,
            page_offset,
            page_size,
            request,
        )
        .await
    }

    pub async fn grant_indirect_entity_permissions(
        &self,
        request: GrantIndirectEntityPermissionsRequest,
    ) -> Result<GrantIndirectEntityPermissionsResponse, KsefError> {
        permissions::grant_indirect_entity_permissions::grant_indirect_entity_permissions(
            self, request,
        )
        .await
    }

    pub async fn grant_subunit_permissions(
        &self,
        request: GrantSubunitPermissionsRequest,
    ) -> Result<crate::client::permissions::get_operation_status::OperationStatusResponse, KsefError>
    {
        permissions::grant_subunit_permissions::grant_subunit_permissions(self, request).await
    }

    pub async fn grant_eu_entity_permissions(
        &self,
        request: GrantEuEntityPermissionsRequest,
    ) -> Result<GrantEuEntityPermissionsResponse, KsefError> {
        permissions::grant_eu_entity_permissions::grant_eu_entity_permissions(self, request).await
    }

    pub async fn grant_eu_entity_representative_permissions(
        &self,
        request: GrantEuEntityRepresentativePermissionsRequest,
    ) -> Result<GrantEuEntityRepresentativePermissionsResponse, KsefError> {
        permissions::grant_eu_entity_representative_permissions::grant_eu_entity_representative_permissions(
            self, request,
        ).await
    }

    pub async fn revoke_authorizations_permission(
        &self,
        permission_id: &str,
    ) -> Result<crate::client::permissions::get_operation_status::OperationStatusResponse, KsefError>
    {
        permissions::revoke_authorizations_permission::revoke_authorizations_permission(
            self,
            permission_id,
        )
        .await
    }

    pub async fn revoke_common_permission(
        &self,
        permission_id: &str,
    ) -> Result<crate::client::permissions::get_operation_status::OperationStatusResponse, KsefError>
    {
        permissions::revoke_common_permission::revoke_common_permission(self, permission_id).await
    }

    pub async fn get_common_permissions(&self) -> Result<serde_json::Value, KsefError> {
        permissions::revoke_common_permission::get_common_permissions(self).await
    }

    pub async fn get_certificates_limits(&self) -> Result<CertificateLimits, KsefError> {
        ksef_certificates::get_certificates_limits::get_certificates_limits(self).await
    }

    pub async fn get_enrollment_data(&self) -> Result<EnrollmentData, KsefError> {
        ksef_certificates::get_enrollment_data::get_enrollment_data(self).await
    }

    pub async fn enroll_certificate(
        &self,
        request: EnrollCertificateRequest,
    ) -> Result<EnrollCertificateResponse, KsefError> {
        ksef_certificates::enroll_certificate::enroll_certificate(self, request).await
    }

    pub fn generate_csr(&self, enrollment_data: &EnrollmentData) -> Result<CsrResult, KsefError> {
        ksef_certificates::csr::generate_csr(enrollment_data)
    }

    pub async fn get_enrollment_status(
        &self,
        reference_number: &str,
    ) -> Result<EnrollmentStatusResponse, KsefError> {
        ksef_certificates::get_enrollment_status::get_enrollment_status(self, reference_number)
            .await
    }

    pub async fn retrieve_certificates(
        &self,
        serial_numbers: Vec<String>,
    ) -> Result<Vec<RetrieveCertificatesListItem>, KsefError> {
        ksef_certificates::retrieve_certificates::retrieve_certificates(self, serial_numbers).await
    }

    pub async fn get_certificate_metadata_list(
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

    pub async fn revoke_certificate(
        &self,
        serial_number: &str,
        reason: RevocationReason,
    ) -> Result<(), KsefError> {
        ksef_certificates::revoke_certificate::revoke_certificate(self, serial_number, reason).await
    }

    pub async fn open_online_session(
        &self,
        request: OpenOnlineSessionRequest,
    ) -> Result<OpenOnlineSessionResponse, KsefError> {
        online_session::open_online_session::open_online_session(self, request).await
    }

    pub async fn open_batch_session(
        &self,
        request: OpenBatchSessionRequest,
    ) -> Result<OpenBatchSessionResponse, KsefError> {
        batch_session::open_batch_session::open_batch_session(self, request).await
    }

    pub async fn upload_batch_parts(
        &self,
        response: &OpenBatchSessionResponse,
        parts: &[EncryptedBatchPart],
    ) -> Result<(), KsefError> {
        batch_session::upload_batch_parts::upload_batch_parts(self, response, parts).await
    }

    pub async fn close_batch_session(&self, reference_number: &str) -> Result<(), KsefError> {
        batch_session::close_batch_session::close_batch_session(self, reference_number).await
    }

    pub async fn submit_batch(
        &self,
        invoices: &[InvoicePayload],
        max_part_size_bytes: Option<usize>,
    ) -> Result<BatchSubmissionResult, KsefError> {
        batch_session::full_flow::submit_batch(self, invoices, max_part_size_bytes).await
    }

    pub async fn submit_online(&self, invoice: &[u8]) -> Result<OnlineSubmissionResult, KsefError> {
        online_session::full_flow::submit_online(self, invoice).await
    }

    pub async fn send_invoice(
        &self,
        reference_number: &str,
        invoice_xml: &[u8],
        encryption_data: &EncryptionData,
    ) -> Result<SendInvoiceResponse, KsefError> {
        online_session::send_invoice::send_invoice(
            self,
            reference_number,
            invoice_xml,
            encryption_data,
        )
        .await
    }

    pub async fn get_invoice_status(
        &self,
        session_reference_number: &str,
        invoice_reference_number: &str,
    ) -> Result<GetInvoiceStatusResponse, KsefError> {
        online_session::get_invoice_status::get_invoice_status(
            self,
            session_reference_number,
            invoice_reference_number,
        )
        .await
    }

    pub async fn close_online_session(&self, reference_number: &str) -> Result<(), KsefError> {
        online_session::close_online_session::close_online_session(self, reference_number).await
    }

    pub async fn generate_encryption_data(&self) -> Result<EncryptionData, KsefError> {
        online_session::encryption::generate_encryption_data(self).await
    }

    pub async fn fetch_invoice_metadata(
        &self,
        request: FetchInvoiceMetadataRequest,
    ) -> Result<FetchInvoiceMetadataResponse, KsefError> {
        fetching_invoices::fetch_invoice_metadata::fetch_invoice_metadata(self, request).await
    }

    pub async fn fetch_invoice(
        &self,
        ksef_number: &str,
    ) -> Result<FetchInvoiceResponse, KsefError> {
        fetching_invoices::fetch_invoice::fetch_invoice(self, ksef_number).await
    }

    pub async fn start_export_invoices(
        &self,
        request: ExportInvoicesRequest,
    ) -> Result<ExportInvoicesResponse, KsefError> {
        fetching_invoices::export_invoices::start_export_invoices(self, request).await
    }

    pub async fn get_export_status(
        &self,
        reference_number: &str,
    ) -> Result<ExportInvoicesStatusResponse, KsefError> {
        fetching_invoices::export_invoices::get_export_status(self, reference_number).await
    }

    pub async fn export_invoices(
        &self,
        query: QueryCriteria,
    ) -> Result<fetching_invoices::export_invoices::ExportResult, KsefError> {
        fetching_invoices::export_invoices::export_invoices(self, query).await
    }

    pub async fn export_invoices_incrementally(
        &self,
        state: &mut fetching_invoices::incremental_fetch::IncrementalFetchState,
        subject_types: Vec<fetching_invoices::fetch_invoice_metadata::SubjectType>,
        window_end: Option<chrono::DateTime<chrono::Utc>>,
        default_start: chrono::DateTime<chrono::Utc>,
    ) -> Result<Vec<fetching_invoices::incremental_fetch::FetchedInvoice>, KsefError> {
        fetching_invoices::incremental_fetch::fetch_invoices_incrementally(
            self,
            state,
            subject_types,
            window_end,
            default_start,
        )
        .await
    }

    pub fn url_for(&self, path: &str) -> String {
        format!(
            "{}/{}",
            self.base_url.trim_end_matches('/'),
            path.trim_start_matches('/')
        )
    }

    pub async fn get_personal_permissions(
        &self,
        page_offset: Option<i32>,
        page_size: Option<i32>,
        request_body: Option<permissions::get_personal_permissions::GetPersonalPermissionsRequest>,
    ) -> Result<permissions::get_personal_permissions::GetPersonalPermissionsResponse, KsefError>
    {
        permissions::get_personal_permissions::get_personal_permissions(
            self,
            page_offset,
            page_size,
            request_body,
        )
        .await
    }

    pub async fn get_persons_permissions(
        &self,
        page_offset: Option<i32>,
        page_size: Option<i32>,
        request_body: Option<permissions::get_persons_permissions::PersonsPermissionsRequest>,
    ) -> Result<permissions::get_persons_permissions::GetPersonsPermissionsResponse, KsefError>
    {
        permissions::get_persons_permissions::get_persons_permissions(
            self,
            page_offset,
            page_size,
            request_body,
        )
        .await
    }

    pub async fn get_subunits_permissions(
        &self,
        page_offset: Option<i32>,
        page_size: Option<i32>,
        request_body: Option<permissions::get_subunits_permissions::GetSubunitsPermissionsRequest>,
    ) -> Result<permissions::get_subunits_permissions::GetSubunitsPermissionsResponse, KsefError>
    {
        permissions::get_subunits_permissions::get_subunits_permissions(
            self,
            page_offset,
            page_size,
            request_body,
        )
        .await
    }
}

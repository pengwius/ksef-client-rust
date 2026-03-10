mod client;

pub use client::KsefClient;
pub use client::error::KsefError;
pub use secrecy;

pub mod auth {
    pub use crate::client::traits::auth::KsefAuth;

    pub use crate::client::auth::auth_challenge::AuthChallenge;
    pub use crate::client::auth::auth_token_request::{
        AuthTokenRequest, AuthTokenRequestBuilder, AuthenticationTokenAllowedIps,
        AuthenticationTokenAuthorizationPolicy, SubjectIdentifierType,
    };
    pub use crate::client::auth::get_access_token::AccessTokens;
    pub use crate::client::auth::xades_auth::AuthTokens;
}

pub mod certificates {
    pub use crate::client::traits::certificates::KsefCertificates;

    pub use crate::client::get_public_key_certificates::{
        PublicKeyCertificate, PublicKeyCertificateUsage,
    };
    pub use crate::client::ksef_certificates::csr::CsrResult;
    pub use crate::client::ksef_certificates::enroll_certificate::{
        CertificateType, EnrollCertificateRequest, EnrollCertificateResponse,
    };
    pub use crate::client::ksef_certificates::get_certificate_metadata_list::{
        CertificateListItem, CertificateStatus, CertificateSubjectIdentifier,
        CertificateSubjectIdentifierType, GetCertificateMetadataListRequest,
        GetCertificateMetadataListResponse,
    };
    pub use crate::client::ksef_certificates::get_certificates_limits::{
        CertificateLimits, LimitDetails,
    };
    pub use crate::client::ksef_certificates::get_enrollment_data::EnrollmentData;
    pub use crate::client::ksef_certificates::get_enrollment_status::{
        EnrollmentStatus, EnrollmentStatusResponse,
    };
    pub use crate::client::ksef_certificates::retrieve_certificates::RetrieveCertificatesListItem;
    pub use crate::client::ksef_certificates::revoke_certificate::RevocationReason;
}

pub mod invoices {
    pub use crate::client::traits::invoices::KsefInvoices;

    pub use crate::client::fetching_invoices::export_invoices::{
        EncryptionInfo, ExportInvoicesRequest, ExportInvoicesResponse,
        ExportInvoicesStatusResponse, ExportPackage, ExportPackagePart, ExportResult, ExportStatus,
        ExportedPart,
    };
    pub use crate::client::fetching_invoices::fetch_invoice::FetchInvoiceResponse;
    pub use crate::client::fetching_invoices::fetch_invoice_metadata::{
        AmountFilter, AmountType, AuthorizedSubjectMetadata, BuyerIdentifier,
        BuyerIdentifierMetadata, BuyerIdentifierType, BuyerMetadata, DateRange, DateRangeBuilder,
        DateType, FetchInvoiceMetadataRequest, FetchInvoiceMetadataRequestBuilder,
        FetchInvoiceMetadataResponse, FormType, InvoiceFormCode, InvoiceMetadata, InvoiceType,
        InvoicingMode, QueryCriteria, QueryCriteriaBuilder, SellerMetadata, SubjectType,
        ThirdSubjectIdentifier, ThirdSubjectMetadata,
    };
    pub use crate::client::fetching_invoices::incremental_fetch::{
        FetchedInvoice, IncrementalFetchState,
    };
    pub use crate::client::models::{FormCode, InvoicePayload};
}

pub mod models {
    pub use crate::client::models::{
        ContextIdentifier, ContextIdentifierType, Encryption, Environment,
    };
}

pub mod peppol {
    pub use crate::client::peppol::get_peppol_providers::{
        GetPeppolProvidersResponse, PeppolProvider,
    };
    pub use crate::client::traits::peppol::KsefPeppol;
}

pub mod permissions {
    pub use crate::client::traits::permissions::KsefPermissions;

    pub use crate::client::permissions::get_authorizations_permissions::{
        AuthorIdentifier, AuthorizationGrant, AuthorizedEntityIdentifier,
        AuthorizedIdentifier as AuthorizationAuthorizedIdentifier, AuthorizingEntityIdentifier,
        AuthorizingIdentifier, GetAuthorizationsPermissionsRequest,
        GetAuthorizationsPermissionsResponse, QueryType, SubjectEntityDetails,
    };
    pub use crate::client::permissions::get_entities_permissions::{
        EntitiesContextIdentifier, EntityPermissionItem, GetEntitiesPermissionsRequest,
        GetEntitiesPermissionsResponse,
    };
    pub use crate::client::permissions::get_eu_entities_permissions::{
        EuEntityPermission, GetEuEntitiesPermissionsRequest, GetEuEntitiesPermissionsResponse,
    };
    pub use crate::client::permissions::get_operation_status::OperationStatusResponse;
    pub use crate::client::permissions::get_personal_permissions::{
        GetPersonalPermissionsRequest, GetPersonalPermissionsResponse, PersonalPermission,
    };
    pub use crate::client::permissions::get_persons_permissions::{
        GetPersonsPermissionsResponse, Identifier, PersonPermission, PersonsPermissionsRequest,
    };
    pub use crate::client::permissions::get_subordinate_entities_roles::{
        GetSubordinateEntitiesRolesRequest, GetSubordinateEntitiesRolesResponse,
        SubordinateEntityIdentifier, SubordinateEntityRole,
    };
    pub use crate::client::permissions::get_subunits_permissions::{
        AuthorizedIdentifier, GetSubunitsPermissionsRequest, GetSubunitsPermissionsResponse,
        SubjectPersonDetails, SubunitIdentifier, SubunitPermission,
    };
    pub use crate::client::permissions::grant_authorization_permissions::{
        AuthorizationPermissionType, AuthorizationSubjectDetails, AuthorizationSubjectIdentifier,
        AuthorizationSubjectIdentifierType, GrantAuthorizationPermissionsRequest,
        GrantAuthorizationPermissionsRequestBuilder, GrantAuthorizationPermissionsResponse,
    };
    pub use crate::client::permissions::grant_entity_permissions::{
        EntityIdentifier, EntityIdentifierType, EntityPermission, EntityPermissionType,
        EntitySubjectDetails, GrantEntityPermissionsRequest, GrantEntityPermissionsRequestBuilder,
        GrantEntityPermissionsResponse,
    };
    pub use crate::client::permissions::grant_eu_entity_permissions::{
        EuEntityByFp, EuEntityContextIdentifier, EuEntityContextIdentifierType, EuEntityDetails,
        EuEntityIdDocument, EuEntityPersonByFpNoId, EuEntityPersonByFpWithId,
        EuEntityPersonIdentifier, EuEntityPersonIdentifierType, EuEntitySubjectDetails,
        EuEntitySubjectDetailsType, EuEntitySubjectIdentifier, EuEntitySubjectIdentifierType,
        GrantEuEntityPermissionsRequest, GrantEuEntityPermissionsRequestBuilder,
        GrantEuEntityPermissionsResponse,
    };
    pub use crate::client::permissions::grant_eu_entity_representative_permissions::{
        EuEntityRepresentativeEntityByFp, EuEntityRepresentativeIdDocument,
        EuEntityRepresentativePermissionType, EuEntityRepresentativePersonByFpNoId,
        EuEntityRepresentativePersonByFpWithId, EuEntityRepresentativePersonIdentifier,
        EuEntityRepresentativePersonIdentifierType, EuEntityRepresentativeSubjectDetails,
        EuEntityRepresentativeSubjectDetailsType, EuEntityRepresentativeSubjectIdentifier,
        EuEntityRepresentativeSubjectIdentifierType, GrantEuEntityRepresentativePermissionsRequest,
        GrantEuEntityRepresentativePermissionsRequestBuilder,
        GrantEuEntityRepresentativePermissionsResponse,
    };
    pub use crate::client::permissions::grant_indirect_entity_permissions::{
        GrantIndirectEntityPermissionsRequest, GrantIndirectEntityPermissionsRequestBuilder,
        GrantIndirectEntityPermissionsResponse, IndirectIdDocument, IndirectPermissionType,
        IndirectPersonByFpNoId, IndirectPersonByFpWithId, IndirectPersonById,
        IndirectPersonIdentifier, IndirectPersonIdentifierType, IndirectSubjectDetails,
        IndirectSubjectDetailsType, IndirectSubjectIdentifier, IndirectSubjectIdentifierType,
        IndirectTargetIdentifier, IndirectTargetIdentifierType,
    };
    pub use crate::client::permissions::grant_person_permissions::{
        GrantPersonPermissionsRequest, GrantPersonPermissionsRequestBuilder,
        GrantPersonPermissionsResponse, IdDocument, PersonByFpNoId, PersonByFpWithId, PersonById,
        PersonIdentifier, PersonIdentifierType, PersonPermissionType, SubjectDetails,
        SubjectDetailsType, SubjectIdentifier, SubjectIdentifierType as GrantSubjectIdentifierType,
    };
    pub use crate::client::permissions::grant_subunit_permissions::{
        GrantSubunitPermissionsRequest, GrantSubunitPermissionsRequestBuilder,
        SubunitContextIdentifier, SubunitContextIdentifierType, SubunitIdDocument,
        SubunitPersonByFpNoId, SubunitPersonByFpWithId, SubunitPersonById, SubunitPersonIdentifier,
        SubunitPersonIdentifierType, SubunitSubjectDetails, SubunitSubjectDetailsType,
        SubunitSubjectIdentifier, SubunitSubjectIdentifierType,
    };
}

pub mod sessions {
    pub use crate::client::traits::sessions::KsefSessions;

    pub use crate::client::sessions::{
        AuthenticationMethod, QuerySessionsResponse, Session, SessionStatus,
    };

    pub use crate::client::online_session::encryption::EncryptionData;
    pub use crate::client::online_session::full_flow::OnlineSubmissionResult;
    pub use crate::client::online_session::get_invoice_status::{
        GetInvoiceStatusResponse, InvoiceStatus,
    };
    pub use crate::client::online_session::open_online_session::{
        OpenOnlineSessionRequest, OpenOnlineSessionRequestBuilder, OpenOnlineSessionResponse,
    };
    pub use crate::client::online_session::send_invoice::SendInvoiceResponse;

    pub use crate::client::batch_session::full_flow::BatchSubmissionResult;
    pub use crate::client::batch_session::open_batch_session::{
        BatchFile, BatchFilePartInfo, OpenBatchSessionRequest, OpenBatchSessionRequestBuilder,
        OpenBatchSessionResponse, PartUploadRequest,
    };
    pub use crate::client::batch_session::zip::{
        BatchZipResult, EncryptedBatchPart, FileMetadata, calculate_invoice_hash, create_zip,
        encrypt_zip_parts, split_zip,
    };
}

pub mod tokens {
    pub use crate::client::ksef_tokens::models::{DetailedKsefToken, TokenStatus};
    pub use crate::client::ksef_tokens::new_ksef_token::{KsefToken, KsefTokenPermissions};
    pub use crate::client::traits::tokens::KsefTokens;
}

pub mod upo {
    pub use crate::client::upo::get_invoice_upo_by_ksef_number::{
        GetInvoiceUpoResponse, InvoiceIdentifier,
    };
}

pub mod traits {
    pub use crate::client::traits::auth::KsefAuth;
    pub use crate::client::traits::certificates::KsefCertificates;
    pub use crate::client::traits::invoices::KsefInvoices;
    pub use crate::client::traits::peppol::KsefPeppol;
    pub use crate::client::traits::permissions::KsefPermissions;
    pub use crate::client::traits::sessions::KsefSessions;
    pub use crate::client::traits::tokens::KsefTokens;
    pub use crate::client::traits::utils::KsefUtils;
}

pub mod prelude {
    pub use crate::client::KsefClient;
    pub use crate::client::error::KsefError;

    pub use crate::client::traits::auth::KsefAuth;
    pub use crate::client::traits::certificates::KsefCertificates;
    pub use crate::client::traits::invoices::KsefInvoices;
    pub use crate::client::traits::peppol::KsefPeppol;
    pub use crate::client::traits::permissions::KsefPermissions;
    pub use crate::client::traits::sessions::KsefSessions;
    pub use crate::client::traits::tokens::KsefTokens;
    pub use crate::client::traits::utils::KsefUtils;

    pub use crate::client::models::{ContextIdentifier, ContextIdentifierType, Environment};
}

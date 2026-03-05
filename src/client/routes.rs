pub const AUTH_PATH: &str = "/v2/auth";
pub const AUTH_CHALLENGE_PATH: &str = "/v2/auth/challenge";
pub const AUTH_XADES_SIGANTURE_PATH: &str = "/v2/auth/xades-signature";
pub const AUTH_KSEF_TOKEN_PATH: &str = "/v2/auth/ksef-token";
pub const AUTH_TOKEN_REDEEM_PATH: &str = "/v2/auth/token/redeem";
pub const AUTH_TOKEN_REFRESH_PATH: &str = "/v2/auth/token/refresh";

pub const TOKENS_PATH: &str = "/v2/tokens";
pub const PUBLIC_KEYS_PATH: &str = "/v2/security/public-key-certificates";

pub const AUTH_SESSIONS_PATH: &str = "/v2/auth/sessions";
pub const AUTH_SESSIONS_CURRENT_PATH: &str = "/v2/auth/sessions/current";

pub const PERMISSIONS_QUERY_PERSONAL_GRANTS_PATH: &str = "/v2/permissions/query/personal/grants";
pub const PERMISSIONS_PERSONS_GRANTS_PATH: &str = "/v2/permissions/persons/grants";
pub const PERMISSIONS_ENTITIES_GRANTS_PATH: &str = "/v2/permissions/entities/grants";
pub const PERMISSIONS_AUTHORIZATIONS_GRANTS_PATH: &str = "/v2/permissions/authorizations/grants";
pub const PERMISSIONS_QUERY_AUTHORIZATIONS_GRANTS_PATH: &str =
    "/v2/permissions/query/authorizations/grants";
pub const PERMISSIONS_QUERY_ENTITIES_GRANTS_PATH: &str = "/v2/permissions/query/entities/grants";
pub const PERMISSIONS_QUERY_ENTITIES_ROLES_PATH: &str = "/v2/permissions/query/entities/roles";
pub const PERMISSIONS_QUERY_EU_ENTITIES_GRANTS_PATH: &str =
    "/v2/permissions/query/eu-entities/grants";
pub const PERMISSIONS_INDIRECT_GRANTS_PATH: &str = "/v2/permissions/indirect/grants";
pub const PERMISSIONS_SUBUNITS_GRANTS_PATH: &str = "/v2/permissions/subunits/grants";
pub const PERMISSIONS_QUERY_SUBUNITS_GRANTS_PATH: &str = "/v2/permissions/query/subunits/grants";
pub const PERMISSIONS_COMMON_GRANTS_PATH: &str = "/v2/permissions/common/grants";
pub const PERMISSIONS_EU_ENTITIES_GRANTS_PATH: &str =
    "/v2/permissions/eu-entities/administration/grants";
pub const PERMISSIONS_EU_ENTITIES_REPRESENTATIVE_GRANTS_PATH: &str =
    "/v2/permissions/eu-entities/grants";
pub const PERMISSIONS_QUERY_PERSONS_GRANTS_PATH: &str = "/v2/permissions/query/persons/grants";
pub const PERMISSIONS_OPERATIONS_PATH: &str = "/v2/permissions/operations";
pub const PERMISSIONS_QUERY_SUBORDINATE_ENTITIES_ROLES_PATH: &str =
    "/v2/permissions/query/subordinate-entities/roles";

pub const CERTIFICATES_LIMITS_PATH: &str = "/v2/certificates/limits";
pub const CERTIFICATES_ENROLLMENT_DATA_PATH: &str = "/v2/certificates/enrollments/data";
pub const CERTIFICATES_ENROLLMENT_PATH: &str = "/v2/certificates/enrollments";
pub const CERTIFICATES_ENROLLMENT_STATUS_PATH: &str = "/v2/certificates/enrollments/";
pub const CERTIFICATES_RETRIEVE_PATH: &str = "/v2/certificates/retrieve";
pub const CERTIFICATES_QUERY_PATH: &str = "/v2/certificates/query";
pub const CERTIFICATES_PATH: &str = "/v2/certificates";

pub const SESSIONS_PATH: &str = "/v2/sessions";
pub const SESSIONS_ONLINE_PATH: &str = "/v2/sessions/online";
pub const SESSIONS_BATCH_PATH: &str = "/v2/sessions/batch";

pub const INVOICES_QUERY_METADATA_PATH: &str = "/v2/invoices/query/metadata";
pub const INVOICES_PATH: &str = "/v2/invoices/ksef";
pub const INVOICES_EXPORTS_PATH: &str = "/v2/invoices/exports";

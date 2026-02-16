mod client;

pub use client::KsefClient;
pub use client::auth::auth_challenge::AuthChallenge;
pub use client::auth::auth_token_request::{
    AuthTokenRequest, AuthTokenRequestBuilder, AuthenticationTokenAllowedIps,
    AuthenticationTokenAuthorizationPolicy, ContextIdentifierType, SubjectIdentifierType,
};
pub use client::auth::get_access_token::AccessTokens;
pub use client::auth::xades_auth::AuthTokens;
pub use client::ksef_tokens::models::{DetailedKsefToken, TokenStatus};
pub use client::ksef_tokens::new_ksef_token::{KsefToken, KsefTokenPermissions};

pub use client::permissions::grant_authorization_permissions::{
    AuthorizationPermissionType, AuthorizationSubjectDetails, AuthorizationSubjectIdentifier,
    AuthorizationSubjectIdentifierType, GrantAuthorizationPermissionsRequest,
    GrantAuthorizationPermissionsResponse,
};

pub use client::permissions::grant_entity_permissions::{
    EntityIdentifier, EntityIdentifierType, EntityPermission, EntityPermissionType,
    EntitySubjectDetails, GrantEntityPermissionsRequest, GrantEntityPermissionsResponse,
};

pub use client::permissions::grant_indirect_entity_permissions::{
    GrantIndirectEntityPermissionsRequest, GrantIndirectEntityPermissionsResponse,
    IndirectIdDocument, IndirectPermissionType, IndirectPersonByFpNoId, IndirectPersonByFpWithId,
    IndirectPersonById, IndirectPersonIdentifier, IndirectPersonIdentifierType,
    IndirectSubjectDetails, IndirectSubjectDetailsType, IndirectSubjectIdentifier,
    IndirectSubjectIdentifierType, IndirectTargetIdentifier, IndirectTargetIdentifierType,
};

pub use client::permissions::grant_subunit_permissions::{
    GrantSubunitPermissionsRequest, GrantSubunitPermissionsResponse, SubunitContextIdentifier,
    SubunitContextIdentifierType, SubunitIdDocument, SubunitPersonByFpNoId,
    SubunitPersonByFpWithId, SubunitPersonById, SubunitPersonIdentifier,
    SubunitPersonIdentifierType, SubunitSubjectDetails, SubunitSubjectDetailsType,
    SubunitSubjectIdentifier, SubunitSubjectIdentifierType,
};

pub use client::permissions::grant_eu_entity_permissions::{
    EuEntityByFp, EuEntityContextIdentifier, EuEntityContextIdentifierType, EuEntityDetails,
    EuEntityIdDocument, EuEntityPersonByFpNoId, EuEntityPersonByFpWithId, EuEntityPersonIdentifier,
    EuEntityPersonIdentifierType, EuEntitySubjectDetails, EuEntitySubjectDetailsType,
    EuEntitySubjectIdentifier, EuEntitySubjectIdentifierType, GrantEuEntityPermissionsRequest,
    GrantEuEntityPermissionsResponse,
};

pub use client::permissions::grant_person_permissions::{
    GrantPersonPermissionsRequest, GrantPersonPermissionsResponse, IdDocument, PersonByFpNoId,
    PersonByFpWithId, PersonById, PersonIdentifier, PersonIdentifierType, PersonPermissionType,
    SubjectDetails, SubjectDetailsType, SubjectIdentifier,
    SubjectIdentifierType as GrantSubjectIdentifierType,
};

pub use client::sessions::{AuthenticationMethod, QuerySessionsResponse, Session, SessionStatus};

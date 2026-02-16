use crate::common;

use ksef_client::{
    AuthorizationPermissionType, AuthorizationSubjectDetails, AuthorizationSubjectIdentifier,
    AuthorizationSubjectIdentifierType, GrantAuthorizationPermissionsRequest,
};

#[test]
fn test_grant_authorization_permissions() {
    let client = common::authorize_client();
    let target_nip = common::generate_random_nip();

    let request = GrantAuthorizationPermissionsRequest::builder()
        .with_subject_identifier(AuthorizationSubjectIdentifier {
            identifier_type: AuthorizationSubjectIdentifierType::Nip,
            value: target_nip,
        })
        .with_permission(AuthorizationPermissionType::SelfInvoicing)
        .with_description("Test authorization permission grant")
        .with_subject_details(AuthorizationSubjectDetails {
            full_name: "Test Entity Sp. z o.o.".to_string(),
        })
        .build()
        .expect("Failed to build request");

    match client.grant_authorization_permissions(request) {
        Ok(resp) => {
            println!(
                "Granted authorization permissions successfully. Reference number: {}",
                resp.reference_number
            );
            assert!(
                !resp.reference_number.is_empty(),
                "Reference number should not be empty"
            );
        }
        Err(e) => {
            panic!("Failed to grant authorization permissions: {:?}", e);
        }
    }
}

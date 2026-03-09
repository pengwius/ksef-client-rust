use ksef_client::*;
use crate::common;

use ksef_client::{
    AuthorizationPermissionType,
    AuthorizationSubjectDetails,
    AuthorizationSubjectIdentifier,
    AuthorizationSubjectIdentifierType,
    GrantAuthorizationPermissionsRequest,
};

#[tokio::test]
async fn test_grant_authorization_permissions() {
    let client: ksef_client::KsefClient = common::authorize_client().await;
    let target_nip: String = common::generate_random_nip().await;

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

    match client.grant_authorization_permissions(request).await {
        Ok(op_status) => {
            println!(
                "Grant authorization operation status (raw): {:?}",
                op_status.raw
            );
            match op_status.status_code() {
                Some(code) => {
                    assert_eq!(code, 200, "Expected final operation code 200, got {}", code);
                }
                None => {
                    panic!(
                        "Operation status did not contain a numeric code. Raw payload: {:?}",
                        op_status.raw
                    );
                }
            }
        }
        Err(e) => {
            panic!("Failed to grant authorization permissions: {:?}", e);
        }
    }
}

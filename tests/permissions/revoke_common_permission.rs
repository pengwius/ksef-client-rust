use crate::common;
use ksef_client::permissions::{
    GrantPersonPermissionsRequest, GrantSubjectIdentifierType, Identifier, PersonById,
    PersonPermissionType, PersonsPermissionsRequest, SubjectDetails, SubjectDetailsType,
    SubjectIdentifier,
};
use ksef_client::prelude::*;

#[tokio::test]
async fn test_grant_and_revoke_common_permission_workflow() {
    let client: ksef_client::KsefClient = common::authorize_client().await;
    let target_nip: String = common::generate_random_nip().await;

    let subject_identifier = SubjectIdentifier::builder()
        .with_type(GrantSubjectIdentifierType::Nip)
        .with_value(target_nip.clone())
        .build()
        .expect("Failed to build SubjectIdentifier");
    let request = GrantPersonPermissionsRequest::builder()
        .with_subject_identifier(subject_identifier)
        .with_permissions(vec![
            PersonPermissionType::InvoiceRead,
            PersonPermissionType::InvoiceWrite,
        ])
        .with_description("Integration test: grant and revoke common permission")
        .with_subject_details(
            SubjectDetails::builder()
                .with_subject_details_type(SubjectDetailsType::PersonByIdentifier)
                .with_person_by_id(
                    PersonById::builder()
                        .with_first_name("Test")
                        .with_last_name("User")
                        .build()
                        .expect("Failed to build PersonById"),
                )
                .build()
                .expect("Failed to build SubjectDetails"),
        )
        .build()
        .expect("Failed to build grant request");

    let op_resp = client
        .grant_person_permissions(request)
        .await
        .expect("Failed to grant person permissions");
    match op_resp.status_code() {
        Some(code) => {
            assert_eq!(code, 200, "Expected operation final code 200, got {}", code);
        }
        None => {
            panic!(
                "Operation status did not contain a numeric code. Raw payload: {:?}",
                op_resp.raw
            );
        }
    }

    let auth_id = Identifier::builder()
        .with_type("Nip")
        .with_value(target_nip.clone())
        .build()
        .expect("Failed to build Identifier");

    let req = PersonsPermissionsRequest::builder()
        .with_authorized_identifier(auth_id)
        .with_query_type("PermissionsGrantedInCurrentContext")
        .build()
        .expect("Failed to build PersonsPermissionsRequest");

    let list = match client
        .get_persons_permissions(Some(0), Some(100), Some(req))
        .await
    {
        Ok(list) => list,
        Err(ksef_client::KsefError::ApiError(code, _)) if code == 500 => {
            eprintln!("get_persons_permissions returned 500; skipping test");
            return;
        }
        Err(e) => panic!("Failed to fetch persons permissions: {:?}", e),
    };

    let permission_id = match list
        .permissions
        .iter()
        .find(|item| item.authorized_identifier.value == target_nip)
    {
        Some(item) => item.id.clone(),
        None => panic!(
            "Failed to find permission id for subject {} in persons grants",
            target_nip
        ),
    };

    let revoke_op = client
        .revoke_common_permission(&permission_id)
        .await
        .expect("Failed to revoke common permission");

    match revoke_op.status_code() {
        Some(code) => assert_eq!(
            code, 200,
            "Expected revoke operation to finish with code 200"
        ),
        None => panic!(
            "Revoke operation did not include numeric status: {:?}",
            revoke_op.raw
        ),
    }

    let auth_id = Identifier::builder()
        .with_type("Nip")
        .with_value(target_nip.clone())
        .build()
        .expect("Failed to build Identifier");

    let req = PersonsPermissionsRequest::builder()
        .with_authorized_identifier(auth_id)
        .with_query_type("PermissionsGrantedInCurrentContext")
        .build()
        .expect("Failed to build PersonsPermissionsRequest");

    let list = match client
        .get_persons_permissions(Some(0), Some(100), Some(req))
        .await
    {
        Ok(list) => list,
        Err(ksef_client::KsefError::ApiError(code, _)) if code == 500 => {
            eprintln!("get_persons_permissions returned 500 after revoke; skipping assertions");
            return;
        }
        Err(e) => panic!("Failed to fetch persons permissions after revoke: {:?}", e),
    };

    let still_present = list.permissions.iter().any(|item| item.id == permission_id);

    assert!(
        !still_present,
        "Permission id {} still present after revoke",
        permission_id
    );
}

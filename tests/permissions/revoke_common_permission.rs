use crate::common;
use ksef_client::{
    GrantPersonPermissionsRequest, GrantSubjectIdentifierType, Identifier, PersonById,
    PersonPermissionType, PersonsPermissionsRequest, SubjectDetails, SubjectDetailsType,
    SubjectIdentifier,
};
use tokio::time::{Duration, sleep};

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

    let grant_resp = client
        .grant_person_permissions(request)
        .await
        .expect("Failed to grant person permissions");
    assert!(
        !grant_resp.reference_number.is_empty(),
        "Grant response reference number should not be empty"
    );

    let mut permission_id: Option<String> = None;

    for _ in 0..10 {
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

        let list = client
            .get_persons_permissions(Some(0), Some(100), Some(req))
            .await
            .expect("Failed to fetch persons permissions");

        for item in list.permissions.iter() {
            if item.authorized_identifier.value == target_nip {
                permission_id = Some(item.id.clone());
                break;
            }
        }

        if permission_id.is_some() {
            break;
        }

        sleep(Duration::from_millis(500)).await;
    }

    let permission_id = match permission_id {
        Some(id) => id,
        None => panic!(
            "Failed to find permission id for subject {} in persons grants",
            target_nip
        ),
    };

    let revoke_resp = client
        .revoke_common_permission(&permission_id)
        .await
        .expect("Failed to revoke common permission");
    assert!(
        !revoke_resp.reference_number.is_empty(),
        "Revoke response reference number should not be empty"
    );

    let mut still_present = true;
    for _ in 0..10 {
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

        let list = client
            .get_persons_permissions(Some(0), Some(100), Some(req))
            .await
            .expect("Failed to fetch persons permissions after revoke");

        let mut found = false;
        for item in list.permissions.iter() {
            if item.id == permission_id {
                found = true;
                break;
            }
        }

        if !found {
            still_present = false;
            break;
        }

        sleep(Duration::from_millis(500)).await;
    }

    assert!(
        !still_present,
        "Permission id {} still present after revoke",
        permission_id
    );
}

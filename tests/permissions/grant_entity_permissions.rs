use crate::common;

use ksef_client::{
    EntityIdentifier, EntityIdentifierType, EntityPermission, EntityPermissionType,
    EntitySubjectDetails, GrantEntityPermissionsRequest,
};

#[tokio::test]
async fn test_grant_entity_permissions() {
    let client: ksef_client::KsefClient = common::authorize_client().await;
    let target_nip: String = common::generate_random_nip().await;

    let request = GrantEntityPermissionsRequest::builder()
        .with_subject_identifier(EntityIdentifier {
            identifier_type: EntityIdentifierType::Nip,
            value: target_nip,
        })
        .with_permissions(vec![
            EntityPermission {
                permission_type: EntityPermissionType::InvoiceRead,
                can_delegate: Some(false),
            },
            EntityPermission {
                permission_type: EntityPermissionType::InvoiceWrite,
                can_delegate: Some(true),
            },
        ])
        .with_description("Test entity permission grant")
        .with_subject_details(EntitySubjectDetails {
            full_name: "Test Entity Sp. z o.o.".to_string(),
        })
        .build()
        .expect("Failed to build request");

    match client.grant_entity_permissions(request).await {
        Ok(op_status) => {
            println!("Grant operation status: {:#?}", op_status);
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
            panic!("Failed to grant entity permissions: {:?}", e);
        }
    }
}

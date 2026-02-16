use crate::common;

use ksef_client::{
    EntityIdentifier, EntityIdentifierType, EntityPermission, EntityPermissionType,
    EntitySubjectDetails, GrantEntityPermissionsRequest,
};

#[test]
fn test_grant_entity_permissions() {
    let client = common::authorize_client();
    let target_nip = common::generate_random_nip();

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

    match client.grant_entity_permissions(request) {
        Ok(resp) => {
            println!(
                "Granted entity permissions successfully. Reference number: {}",
                resp.reference_number
            );
            assert!(
                !resp.reference_number.is_empty(),
                "Reference number should not be empty"
            );
        }
        Err(e) => {
            panic!("Failed to grant entity permissions: {:?}", e);
        }
    }
}

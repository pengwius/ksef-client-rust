mod common;

use ksef_client::{
    GrantPersonPermissionsRequest, GrantSubjectIdentifierType, PersonById, PersonPermissionType,
    SubjectDetails, SubjectDetailsType, SubjectIdentifier,
};

#[test]
fn test_grant_person_permissions() {
    let client = common::authorize_client();
    let target_nip = common::generate_random_nip();

    let request = GrantPersonPermissionsRequest {
        subject_identifier: SubjectIdentifier {
            identifier_type: GrantSubjectIdentifierType::Nip,
            value: target_nip,
        },
        permissions: vec![
            PersonPermissionType::InvoiceRead,
            PersonPermissionType::InvoiceWrite,
        ],
        description: "Test permission grant".to_string(),
        subject_details: SubjectDetails {
            subject_details_type: SubjectDetailsType::PersonByIdentifier,
            person_by_id: Some(PersonById {
                first_name: "Jan".to_string(),
                last_name: "Kowalski".to_string(),
            }),
            person_by_fp_with_id: None,
            person_by_fp_no_id: None,
        },
    };

    match client.grant_person_permissions(request) {
        Ok(resp) => {
            println!(
                "Granted permissions successfully. Reference number: {}",
                resp.reference_number
            );
            assert!(
                !resp.reference_number.is_empty(),
                "Reference number should not be empty"
            );
        }
        Err(e) => {
            panic!("Failed to grant person permissions: {:?}", e);
        }
    }
}

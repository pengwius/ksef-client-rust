use crate::common;

use ksef_client::{
    GrantIndirectEntityPermissionsRequest, IndirectPermissionType, IndirectPersonById,
    IndirectSubjectDetails, IndirectSubjectDetailsType, IndirectSubjectIdentifier,
    IndirectSubjectIdentifierType, IndirectTargetIdentifier, IndirectTargetIdentifierType,
};

#[test]
fn test_grant_indirect_entity_permissions() {
    let client = common::authorize_client();
    let target_nip = common::generate_random_nip();

    let request = GrantIndirectEntityPermissionsRequest::builder()
        .with_subject_identifier(IndirectSubjectIdentifier {
            identifier_type: IndirectSubjectIdentifierType::Nip,
            value: target_nip,
        })
        .with_target_identifier(IndirectTargetIdentifier {
            identifier_type: IndirectTargetIdentifierType::AllPartners,
            value: None,
        })
        .with_permissions(vec![
            IndirectPermissionType::InvoiceRead,
            IndirectPermissionType::InvoiceWrite,
        ])
        .with_description("Test indirect permission grant")
        .with_subject_details(IndirectSubjectDetails {
            subject_details_type: IndirectSubjectDetailsType::PersonByIdentifier,
            person_by_id: Some(IndirectPersonById {
                first_name: "Jan".to_string(),
                last_name: "Kowalski".to_string(),
            }),
            person_by_fp_with_id: None,
            person_by_fp_no_id: None,
        })
        .build()
        .expect("Failed to build request");

    match client.grant_indirect_entity_permissions(request) {
        Ok(resp) => {
            println!(
                "Granted indirect entity permissions successfully. Reference number: {}",
                resp.reference_number
            );
            assert!(
                !resp.reference_number.is_empty(),
                "Reference number should not be empty"
            );
        }
        Err(e) => {
            panic!("Failed to grant indirect entity permissions: {:?}", e);
        }
    }
}

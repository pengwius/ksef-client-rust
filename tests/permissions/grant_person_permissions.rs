use crate::common;
use ksef_client::{
    GrantPersonPermissionsRequest, GrantSubjectIdentifierType, PersonById, PersonPermissionType,
    SubjectDetails, SubjectDetailsType, SubjectIdentifier,
};

#[tokio::test]
async fn test_grant_person_permissions() {
    let client: ksef_client::KsefClient = common::authorize_client().await;
    let target_nip: String = common::generate_random_nip().await;

    let request = GrantPersonPermissionsRequest::builder()
        .with_subject_identifier(SubjectIdentifier {
            identifier_type: GrantSubjectIdentifierType::Nip,
            value: target_nip,
        })
        .with_permissions(vec![
            PersonPermissionType::InvoiceRead,
            PersonPermissionType::InvoiceWrite,
        ])
        .with_description("Test permission grant")
        .with_subject_details(SubjectDetails {
            subject_details_type: SubjectDetailsType::PersonByIdentifier,
            person_by_id: Some(PersonById {
                first_name: "Jan".to_string(),
                last_name: "Kowalski".to_string(),
            }),
            person_by_fp_with_id: None,
            person_by_fp_no_id: None,
        })
        .build()
        .expect("Failed to build request");

    match client.grant_person_permissions(request).await {
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

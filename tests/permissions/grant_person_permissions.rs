use crate::common;
use ksef_client::{
    GrantPersonPermissionsRequest, GrantSubjectIdentifierType, PersonById, PersonPermissionType,
    SubjectDetails, SubjectDetailsType, SubjectIdentifier,
};

#[tokio::test]
async fn test_grant_person_permissions() {
    let client: ksef_client::KsefClient = common::authorize_client().await;
    let target_nip: String = common::generate_random_nip().await;

    let subject_identifier = SubjectIdentifier::builder()
        .with_type(GrantSubjectIdentifierType::Nip)
        .with_value(target_nip.clone())
        .build()
        .expect("Failed to build SubjectIdentifier");

    let person_by_id = PersonById::builder()
        .with_first_name("Jan")
        .with_last_name("Kowalski")
        .build()
        .expect("Failed to build PersonById");

    let subject_details = SubjectDetails::builder()
        .with_subject_details_type(SubjectDetailsType::PersonByIdentifier)
        .with_person_by_id(person_by_id)
        .build()
        .expect("Failed to build SubjectDetails");

    let request = GrantPersonPermissionsRequest::builder()
        .with_subject_identifier(subject_identifier)
        .with_permissions(vec![
            PersonPermissionType::InvoiceRead,
            PersonPermissionType::InvoiceWrite,
        ])
        .with_description("Test permission grant")
        .with_subject_details(subject_details)
        .build()
        .expect("Failed to build request");

    match client.grant_person_permissions(request).await {
        Ok(op_status) => {
            println!("Granted permissions operation status: {:?}", op_status.raw);
            match op_status.status_code() {
                Some(code) => {
                    assert_eq!(code, 200, "Expected operation final code 200, got {}", code);
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
            panic!("Failed to grant person permissions: {:?}", e);
        }
    }
}

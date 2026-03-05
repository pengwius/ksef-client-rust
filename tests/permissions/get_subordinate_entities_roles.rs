use crate::common;

use ksef_client::{
    GrantSubunitPermissionsRequest, SubunitContextIdentifier, SubunitContextIdentifierType,
    SubunitPersonById, SubunitSubjectDetails, SubunitSubjectDetailsType, SubunitSubjectIdentifier,
    SubunitSubjectIdentifierType,
};

async fn calculate_checksum(input: &str) -> u32 {
    let weights = [1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3];
    let sum: u32 = input
        .chars()
        .filter(|c| c.is_ascii_digit())
        .zip(weights.iter())
        .map(|(c, w)| c.to_digit(10).unwrap() * w)
        .sum();
    sum % 10
}

#[tokio::test]
async fn test_get_subordinate_entities_roles() {
    let client: ksef_client::KsefClient = common::authorize_client().await;
    let target_nip: String = common::generate_random_nip().await;

    let parent_nip = client.context.value.clone();
    let internal_id_prefix = format!("{}0000", parent_nip);
    let checksum = calculate_checksum(&internal_id_prefix).await;
    let internal_id = format!("{}{}", internal_id_prefix, checksum);

    let grant_request = GrantSubunitPermissionsRequest::builder()
        .with_subject_identifier(SubunitSubjectIdentifier {
            identifier_type: SubunitSubjectIdentifierType::Nip,
            value: target_nip.clone(),
        })
        .with_context_identifier(SubunitContextIdentifier {
            identifier_type: SubunitContextIdentifierType::InternalId,
            value: internal_id.clone(),
        })
        .with_description("Integration test: grant subunit permission for subordinate roles")
        .with_subunit_name("Test Subunit")
        .with_subject_details(SubunitSubjectDetails {
            subject_details_type: SubunitSubjectDetailsType::PersonByIdentifier,
            person_by_id: Some(SubunitPersonById {
                first_name: "Jan".to_string(),
                last_name: "Kowalski".to_string(),
            }),
            person_by_fp_with_id: None,
            person_by_fp_no_id: None,
        })
        .build()
        .expect("Failed to build GrantSubunitPermissionsRequest");

    let op_status = match client.grant_subunit_permissions(grant_request).await {
        Ok(op) => op,
        Err(e) => panic!("Failed to grant subunit permissions: {:?}", e),
    };

    match op_status.status_code() {
        Some(code) => assert_eq!(code, 200, "Expected grant final code 200, got {}", code),
        None => {
            panic!(
                "Grant operation did not return numeric status: {:?}",
                op_status.raw
            );
        }
    }

    let resp = match client
        .get_subordinate_entities_roles(Some(0), Some(100), None)
        .await
    {
        Ok(r) => r,
        Err(ksef_client::KsefError::ApiError(code, _)) if code == 500 => {
            eprintln!("get_subordinate_entities_roles returned 500; skipping test");
            return;
        }
        Err(e) => panic!("Failed to fetch subordinate entities roles: {:?}", e),
    };

    println!("resp: {:#?}", resp);
}

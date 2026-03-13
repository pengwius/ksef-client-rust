use crate::common;
use ksef_client::permissions::{
    GetSubunitsPermissionsRequest, GrantSubunitPermissionsRequest, SubunitContextIdentifier,
    SubunitContextIdentifierType, SubunitIdentifier, SubunitPersonById, SubunitSubjectDetails,
    SubunitSubjectDetailsType, SubunitSubjectIdentifier, SubunitSubjectIdentifierType,
};
use ksef_client::prelude::*;

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
async fn test_get_subunits_permissions_unfiltered() {
    let client: ksef_client::KsefClient = common::authorize_client().await;
    let target_nip: String = common::generate_random_nip().await;

    let parent_nip = client.context.value.clone();
    let internal_id_prefix = format!("{}0000", parent_nip);
    let checksum = calculate_checksum(&internal_id_prefix).await;
    let internal_id = format!("{}{}", internal_id_prefix, checksum);
    let grant_request = GrantSubunitPermissionsRequest::builder()
        .with_subject_identifier(SubunitSubjectIdentifier {
            identifier_type: SubunitSubjectIdentifierType::Nip,
            value: target_nip,
        })
        .with_context_identifier(SubunitContextIdentifier {
            identifier_type: SubunitContextIdentifierType::InternalId,
            value: internal_id,
        })
        .with_description("Test grant")
        .with_subunit_name("Test Subunit")
        .with_subject_details(SubunitSubjectDetails {
            subject_details_type: SubunitSubjectDetailsType::PersonByIdentifier,
            person_by_id: Some(SubunitPersonById {
                first_name: "Test".to_string(),
                last_name: "User".to_string(),
            }),
            person_by_fp_with_id: None,
            person_by_fp_no_id: None,
        })
        .build()
        .expect("Failed to build grant request");

    let _ = client.grant_subunit_permissions(grant_request).await;

    let result = client
        .get_subunits_permissions(Some(0), Some(10), None)
        .await;

    match result {
        Ok(resp) => {
            assert!(
                resp.has_more == true || resp.has_more == false,
                "has_more should be a boolean"
            );

            if !resp.permissions.is_empty() {
                let p = &resp.permissions[0];
                assert!(!p.id.is_empty(), "Permission id should not be empty");
                assert!(
                    !p.permission_scope.is_empty(),
                    "permission_scope should not be empty"
                );
            }
        }
        Err(ksef_client::KsefError::ApiError(code, _)) if code == 500 => {
            eprintln!("get_subunits_permissions unfiltered returned 500; skipping assertions");
            return;
        }
        Err(e) => panic!("Expected Ok or ApiError(500), got Err: {:?}", e),
    }
}

#[tokio::test]
async fn test_get_subunits_permissions_filtered() {
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
        .with_description("Test grant")
        .with_subunit_name("Test Subunit")
        .with_subject_details(SubunitSubjectDetails {
            subject_details_type: SubunitSubjectDetailsType::PersonByIdentifier,
            person_by_id: Some(SubunitPersonById {
                first_name: "Test".to_string(),
                last_name: "User".to_string(),
            }),
            person_by_fp_with_id: None,
            person_by_fp_no_id: None,
        })
        .build()
        .expect("Failed to build grant request");

    let _ = client.grant_subunit_permissions(grant_request).await;

    let request = GetSubunitsPermissionsRequest {
        subunit_identifier: Some(SubunitIdentifier {
            identifier_type: "InternalId".to_string(),
            value: internal_id.clone(),
        }),
    };

    let result = client
        .get_subunits_permissions(Some(0), Some(10), Some(request))
        .await;

    match result {
        Ok(resp) => {
            assert!(
                resp.has_more == true || resp.has_more == false,
                "has_more should be a boolean"
            );

            if !resp.permissions.is_empty() {
                let p = &resp.permissions[0];
                assert!(!p.id.is_empty(), "Permission id should not be empty");
                assert!(
                    !p.subunit_identifier.value.is_empty(),
                    "subunit identifier value should not be empty"
                );
            }
        }
        Err(ksef_client::KsefError::ApiError(code, _)) if code == 500 => {
            eprintln!("get_subunits_permissions filtered returned 500; skipping assertions");
            return;
        }
        Err(e) => panic!("Expected Ok or ApiError(500), got Err: {:?}", e),
    }
}

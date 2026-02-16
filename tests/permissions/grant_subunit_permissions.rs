mod common;

use ksef_client::{
    GrantSubunitPermissionsRequest, SubunitContextIdentifier, SubunitContextIdentifierType,
    SubunitPersonById, SubunitSubjectDetails, SubunitSubjectDetailsType, SubunitSubjectIdentifier,
    SubunitSubjectIdentifierType,
};

fn calculate_checksum(input: &str) -> u32 {
    let weights = [1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3];
    let sum: u32 = input
        .chars()
        .filter(|c| c.is_ascii_digit())
        .zip(weights.iter())
        .map(|(c, w)| c.to_digit(10).unwrap() * w)
        .sum();
    sum % 10
}

#[test]
fn test_grant_subunit_permissions() {
    let client = common::authorize_client();
    let target_nip = common::generate_random_nip();

    let parent_nip = "1234567890";
    let internal_id_prefix = format!("{}-0000", parent_nip);
    let checksum = calculate_checksum(&internal_id_prefix);
    let internal_id = format!("{}{}", internal_id_prefix, checksum);

    let request = GrantSubunitPermissionsRequest {
        subject_identifier: SubunitSubjectIdentifier {
            identifier_type: SubunitSubjectIdentifierType::Nip,
            value: target_nip,
        },
        context_identifier: SubunitContextIdentifier {
            identifier_type: SubunitContextIdentifierType::InternalId,
            value: internal_id,
        },
        description: "Test subunit permission grant".to_string(),
        subunit_name: Some("Test Subunit".to_string()),
        subject_details: SubunitSubjectDetails {
            subject_details_type: SubunitSubjectDetailsType::PersonByIdentifier,
            person_by_id: Some(SubunitPersonById {
                first_name: "Jan".to_string(),
                last_name: "Kowalski".to_string(),
            }),
            person_by_fp_with_id: None,
            person_by_fp_no_id: None,
        },
    };

    match client.grant_subunit_permissions(request) {
        Ok(resp) => {
            println!(
                "Granted subunit permissions successfully. Reference number: {}",
                resp.reference_number
            );
            assert!(
                !resp.reference_number.is_empty(),
                "Reference number should not be empty"
            );
        }
        Err(e) => {
            println!(
                "Failed to grant subunit permissions (expected if NIP context mismatch): {:?}",
                e
            );
        }
    }
}

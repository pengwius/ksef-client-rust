mod common;

use ksef_client::{
    EuEntityByFp, EuEntityContextIdentifier, EuEntityContextIdentifierType, EuEntityDetails,
    EuEntityIdDocument, EuEntityPersonByFpNoId, EuEntityPersonByFpWithId, EuEntityPersonIdentifier,
    EuEntityPersonIdentifierType, EuEntitySubjectDetails, EuEntitySubjectDetailsType,
    EuEntitySubjectIdentifier, EuEntitySubjectIdentifierType, GrantEuEntityPermissionsRequest,
};

#[test]
fn test_grant_eu_entity_permissions_entity_by_fingerprint() {
    let client = common::authorize_client();

    let fingerprint = "0000000000000000000000000000000000000000000000000000000000000000";
    let parent_nip = "1111111111";
    let vat_ue = "DE123456789";
    let context_value = format!("{}-{}", parent_nip, vat_ue);

    let request = GrantEuEntityPermissionsRequest {
        subject_identifier: EuEntitySubjectIdentifier {
            identifier_type: EuEntitySubjectIdentifierType::Fingerprint,
            value: fingerprint.to_string(),
        },
        context_identifier: EuEntityContextIdentifier {
            identifier_type: EuEntityContextIdentifierType::NipVatUe,
            value: context_value.clone(),
        },
        description: "Test EU entity permission grant (Entity)".to_string(),
        eu_entity_name: "Test EU Company, Berlin, Germany".to_string(),
        subject_details: EuEntitySubjectDetails {
            subject_details_type: EuEntitySubjectDetailsType::EntityByFingerprint,
            person_by_fp_with_id: None,
            person_by_fp_no_id: None,
            entity_by_fp: Some(EuEntityByFp {
                full_name: "Test EU Company".to_string(),
                address: "Berlin, Germany".to_string(),
            }),
        },
        eu_entity_details: EuEntityDetails {
            full_name: "Test EU Company".to_string(),
            address: "Berlin, Germany".to_string(),
        },
    };

    match client.grant_eu_entity_permissions(request) {
        Ok(resp) => {
            println!(
                "Granted EU entity permissions (Entity) successfully. Reference number: {}",
                resp.reference_number
            );
        }
        Err(e) => {
            println!("Failed to grant EU entity permissions (Entity): {:?}", e);
        }
    }
}

#[test]
fn test_grant_eu_entity_permissions_person_with_nip() {
    let client = common::authorize_client();

    let fingerprint = "1111111111111111111111111111111111111111111111111111111111111111";
    let parent_nip = "1111111111";
    let vat_ue = "DE123456789";
    let context_value = format!("{}-{}", parent_nip, vat_ue);
    let person_nip = common::generate_random_nip();

    let request = GrantEuEntityPermissionsRequest {
        subject_identifier: EuEntitySubjectIdentifier {
            identifier_type: EuEntitySubjectIdentifierType::Fingerprint,
            value: fingerprint.to_string(),
        },
        context_identifier: EuEntityContextIdentifier {
            identifier_type: EuEntityContextIdentifierType::NipVatUe,
            value: context_value.clone(),
        },
        description: "Test EU entity permission grant (Person with NIP)".to_string(),
        eu_entity_name: "Test EU Company, Berlin, Germany".to_string(),
        subject_details: EuEntitySubjectDetails {
            subject_details_type: EuEntitySubjectDetailsType::PersonByFingerprintWithIdentifier,
            person_by_fp_with_id: Some(EuEntityPersonByFpWithId {
                first_name: "Jan".to_string(),
                last_name: "Kowalski".to_string(),
                identifier: EuEntityPersonIdentifier {
                    identifier_type: EuEntityPersonIdentifierType::Nip,
                    value: person_nip,
                },
            }),
            person_by_fp_no_id: None,
            entity_by_fp: None,
        },
        eu_entity_details: EuEntityDetails {
            full_name: "Test EU Company".to_string(),
            address: "Berlin, Germany".to_string(),
        },
    };

    match client.grant_eu_entity_permissions(request) {
        Ok(resp) => {
            println!(
                "Granted EU entity permissions (Person with NIP) successfully. Reference number: {}",
                resp.reference_number
            );
        }
        Err(e) => {
            println!(
                "Failed to grant EU entity permissions (Person with NIP): {:?}",
                e
            );
        }
    }
}

#[test]
fn test_grant_eu_entity_permissions_person_without_id() {
    let client = common::authorize_client();

    let fingerprint = "2222222222222222222222222222222222222222222222222222222222222222";
    let parent_nip = "1111111111";
    let vat_ue = "DE123456789";
    let context_value = format!("{}-{}", parent_nip, vat_ue);

    let request = GrantEuEntityPermissionsRequest {
        subject_identifier: EuEntitySubjectIdentifier {
            identifier_type: EuEntitySubjectIdentifierType::Fingerprint,
            value: fingerprint.to_string(),
        },
        context_identifier: EuEntityContextIdentifier {
            identifier_type: EuEntityContextIdentifierType::NipVatUe,
            value: context_value.clone(),
        },
        description: "Test EU entity permission grant (Person without ID)".to_string(),
        eu_entity_name: "Test EU Company, Berlin, Germany".to_string(),
        subject_details: EuEntitySubjectDetails {
            subject_details_type: EuEntitySubjectDetailsType::PersonByFingerprintWithoutIdentifier,
            person_by_fp_with_id: None,
            person_by_fp_no_id: Some(EuEntityPersonByFpNoId {
                first_name: "Anna".to_string(),
                last_name: "Nowak".to_string(),
                birth_date: "1990-01-01".to_string(),
                id_document: EuEntityIdDocument {
                    document_type: "PASSPORT".to_string(),
                    number: "ABC123456".to_string(),
                    country: "DE".to_string(),
                },
            }),
            entity_by_fp: None,
        },
        eu_entity_details: EuEntityDetails {
            full_name: "Test EU Company".to_string(),
            address: "Berlin, Germany".to_string(),
        },
    };

    match client.grant_eu_entity_permissions(request) {
        Ok(resp) => {
            println!(
                "Granted EU entity permissions (Person without ID) successfully. Reference number: {}",
                resp.reference_number
            );
        }
        Err(e) => {
            println!(
                "Failed to grant EU entity permissions (Person without ID): {:?}",
                e
            );
        }
    }
}

mod common;

use ksef_client::{
    EuEntityRepresentativeEntityByFp, EuEntityRepresentativeIdDocument,
    EuEntityRepresentativePermissionType, EuEntityRepresentativePersonByFpNoId,
    EuEntityRepresentativePersonByFpWithId, EuEntityRepresentativePersonIdentifier,
    EuEntityRepresentativePersonIdentifierType, EuEntityRepresentativeSubjectDetails,
    EuEntityRepresentativeSubjectDetailsType, EuEntityRepresentativeSubjectIdentifier,
    EuEntityRepresentativeSubjectIdentifierType, GrantEuEntityRepresentativePermissionsRequest,
};

#[test]
fn test_grant_eu_entity_representative_permissions_entity_by_fingerprint() {
    let client = common::authorize_client();

    let fingerprint = "0000000000000000000000000000000000000000000000000000000000000000";

    let request = GrantEuEntityRepresentativePermissionsRequest {
        subject_identifier: EuEntityRepresentativeSubjectIdentifier {
            identifier_type: EuEntityRepresentativeSubjectIdentifierType::Fingerprint,
            value: fingerprint.to_string(),
        },
        permissions: vec![
            EuEntityRepresentativePermissionType::InvoiceRead,
            EuEntityRepresentativePermissionType::InvoiceWrite,
        ],
        description: "Test EU entity representative permission grant (Entity)".to_string(),
        subject_details: EuEntityRepresentativeSubjectDetails {
            subject_details_type: EuEntityRepresentativeSubjectDetailsType::EntityByFingerprint,
            person_by_fp_with_id: None,
            person_by_fp_no_id: None,
            entity_by_fp: Some(EuEntityRepresentativeEntityByFp {
                full_name: "Test EU Company".to_string(),
                address: "Berlin, Germany".to_string(),
            }),
        },
    };

    match client.grant_eu_entity_representative_permissions(request) {
        Ok(resp) => {
            assert!(
                !resp.reference_number.is_empty(),
                "Reference number should not be empty"
            );
            println!(
                "Granted EU entity representative permissions (Entity) successfully. Reference number: {}",
                resp.reference_number
            );
        }
        Err(e) => {
            println!(
                "Failed to grant EU entity representative permissions (Entity): {:?}",
                e
            );
        }
    }
}

#[test]
fn test_grant_eu_entity_representative_permissions_person_with_nip() {
    let client = common::authorize_client();

    let fingerprint = "1111111111111111111111111111111111111111111111111111111111111111";
    let person_nip = common::generate_random_nip();

    let request = GrantEuEntityRepresentativePermissionsRequest {
        subject_identifier: EuEntityRepresentativeSubjectIdentifier {
            identifier_type: EuEntityRepresentativeSubjectIdentifierType::Fingerprint,
            value: fingerprint.to_string(),
        },
        permissions: vec![EuEntityRepresentativePermissionType::InvoiceRead],
        description: "Test EU entity representative permission grant (Person with NIP)".to_string(),
        subject_details: EuEntityRepresentativeSubjectDetails {
            subject_details_type: EuEntityRepresentativeSubjectDetailsType::PersonByFingerprintWithIdentifier,
            person_by_fp_with_id: Some(EuEntityRepresentativePersonByFpWithId {
                first_name: "Jan".to_string(),
                last_name: "Kowalski".to_string(),
                identifier: EuEntityRepresentativePersonIdentifier {
                    identifier_type: EuEntityRepresentativePersonIdentifierType::Nip,
                    value: person_nip,
                },
            }),
            person_by_fp_no_id: None,
            entity_by_fp: None,
        },
    };

    match client.grant_eu_entity_representative_permissions(request) {
        Ok(resp) => {
            assert!(
                !resp.reference_number.is_empty(),
                "Reference number should not be empty"
            );
            println!(
                "Granted EU entity representative permissions (Person with NIP) successfully. Reference number: {}",
                resp.reference_number
            );
        }
        Err(e) => {
            println!(
                "Failed to grant EU entity representative permissions (Person with NIP): {:?}",
                e
            );
        }
    }
}

#[test]
fn test_grant_eu_entity_representative_permissions_person_without_id() {
    let client = common::authorize_client();

    let fingerprint = "2222222222222222222222222222222222222222222222222222222222222222";

    let request = GrantEuEntityRepresentativePermissionsRequest {
        subject_identifier: EuEntityRepresentativeSubjectIdentifier {
            identifier_type: EuEntityRepresentativeSubjectIdentifierType::Fingerprint,
            value: fingerprint.to_string(),
        },
        permissions: vec![EuEntityRepresentativePermissionType::InvoiceWrite],
        description: "Test EU entity representative permission grant (Person without ID)".to_string(),
        subject_details: EuEntityRepresentativeSubjectDetails {
            subject_details_type: EuEntityRepresentativeSubjectDetailsType::PersonByFingerprintWithoutIdentifier,
            person_by_fp_with_id: None,
            person_by_fp_no_id: Some(EuEntityRepresentativePersonByFpNoId {
                first_name: "Anna".to_string(),
                last_name: "Nowak".to_string(),
                birth_date: "1990-01-01".to_string(),
                id_document: EuEntityRepresentativeIdDocument {
                    document_type: "PASSPORT".to_string(),
                    number: "ABC123456".to_string(),
                    country: "DE".to_string(),
                },
            }),
            entity_by_fp: None,
        },
    };

    match client.grant_eu_entity_representative_permissions(request) {
        Ok(resp) => {
            assert!(
                !resp.reference_number.is_empty(),
                "Reference number should not be empty"
            );
            println!(
                "Granted EU entity representative permissions (Person without ID) successfully. Reference number: {}",
                resp.reference_number
            );
        }
        Err(e) => {
            println!(
                "Failed to grant EU entity representative permissions (Person without ID): {:?}",
                e
            );
        }
    }
}

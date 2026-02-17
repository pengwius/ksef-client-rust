[Polish Version](permissions.md) / English version

# Permissions Management

Permission management in the KSeF system allows granting rights to perform specific operations to various entities.
A detailed description of each endpoint and business specification can be found in the official KSeF API documentation.

__[Official Documentation](https://github.com/CIRFMF/ksef-docs/blob/main/uprawnienia.md)__

### 1. Granting permissions to natural persons

The `grant_person_permissions` method is used to grant permissions to specific natural persons (identified e.g. by PESEL or NIP).

**Builder: `GrantPersonPermissionsRequestBuilder`**

*   `with_subject_identifier(SubjectIdentifier)` - **Required**. Specifies who is being authorized.
    *   `identifier_type`: `GrantSubjectIdentifierType::Nip`, `GrantSubjectIdentifierType::Pesel`, `GrantSubjectIdentifierType::Fingerprint`.
    *   `value`: Identifier value (e.g., NIP number, PESEL, or certificate hash).
*   `with_permissions(Vec<PersonPermissionType>)` - List of permissions.
    *   Available options: `InvoiceWrite`, `InvoiceRead`, `CredentialsManage`, `CredentialsRead`, `Introspection`, `SubunitManage`, `EnforcementOperations`.
    *   You can use the `with_permission(...)` method to add individually.
*   `with_description(String)` - **Required**. Description of the granted permission.
*   `with_subject_details(SubjectDetails)` - **Required**. Detailed data of the person.
    *   `subject_details_type`:
        *   `PersonByIdentifier`: For persons with NIP/PESEL. Requires `person_by_id` field (first name, last name).
        *   `PersonByFingerprintWithIdentifier`: For persons with a certificate without NIP/PESEL in the certificate, but possessing these numbers. Requires `person_by_fp_with_id` field.
        *   `PersonByFingerprintWithoutIdentifier`: For persons without NIP/PESEL (e.g., foreigners). Requires `person_by_fp_no_id` field (identity document data).

```rust
use ksef_client::{
    GrantPersonPermissionsRequest, GrantSubjectIdentifierType, PersonById, PersonPermissionType,
    SubjectDetails, SubjectDetailsType, SubjectIdentifier
};

let request = GrantPersonPermissionsRequest::builder()
    .with_subject_identifier(SubjectIdentifier {
        identifier_type: GrantSubjectIdentifierType::Nip,
        value: "1234567890".to_string(), // NIP of the person being granted permissions
    })
    .with_permissions(vec![
        PersonPermissionType::InvoiceRead,
        PersonPermissionType::InvoiceWrite,
    ])
    .with_description("Granting permissions to an employee")
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
    .expect("Failed to build request - missing required fields");

match client.grant_person_permissions(request) {
    Ok(resp) => println!("Permissions granted. Reference number: {}", resp.reference_number),
    Err(e) => eprintln!("Error: {:?}", e),
}
```

### 2. Granting invoice handling permissions to entities

The `grant_entity_permissions` method allows authorizing another business entity (e.g., an accounting office) to handle invoices.

**Builder: `GrantEntityPermissionsRequestBuilder`**

*   `with_subject_identifier(EntityIdentifier)` - **Required**. Identifies the authorized entity.
    *   `identifier_type`: `EntityIdentifierType::Nip`.
    *   `value`: NIP number of the entity.
*   `with_permissions(Vec<EntityPermission>)` - List of permissions.
    *   The `EntityPermission` structure contains the permission type (`EntityPermissionType::InvoiceRead` or `InvoiceWrite`) and a `can_delegate` flag (whether the entity can further delegate this permission).
*   `with_description(String)` - **Required**. Description.
*   `with_subject_details(EntitySubjectDetails)` - **Required**.
    *   Requires `full_name` field (full name of the entity).

```rust
use ksef_client::{
    GrantEntityPermissionsRequest, EntityIdentifier, EntityIdentifierType,
    EntityPermission, EntityPermissionType, EntitySubjectDetails
};

let request = GrantEntityPermissionsRequest::builder()
    .with_subject_identifier(EntityIdentifier {
        identifier_type: EntityIdentifierType::Nip,
        value: "1234567890".to_string(),
    })
    .with_permissions(vec![
        EntityPermission {
            permission_type: EntityPermissionType::InvoiceRead,
            can_delegate: Some(false), // No possibility of further delegation
        },
        EntityPermission {
            permission_type: EntityPermissionType::InvoiceWrite,
            can_delegate: Some(true), // Possibility of further delegation
        },
    ])
    .with_description("Authorization for accounting office")
    .with_subject_details(EntitySubjectDetails {
        full_name: "Sample Accounting Office Ltd.".to_string(),
    })
    .build()
    .expect("Error building request");

match client.grant_entity_permissions(request) {
    Ok(resp) => println!("Permissions granted. Reference number: {}", resp.reference_number),
    Err(e) => eprintln!("Error: {:?}", e),
}
```

### 3. Granting entity permissions (e.g., self-invoicing)

The `grant_authorization_permissions` method is used to grant specific permissions, such as self-invoicing.

**Builder: `GrantAuthorizationPermissionsRequestBuilder`**

*   `with_subject_identifier(AuthorizationSubjectIdentifier)` - **Required**.
    *   `identifier_type`: `AuthorizationSubjectIdentifierType::Nip` or `PeppolId`.
    *   `value`: Identifier value.
*   `with_permission(AuthorizationPermissionType)` - **Required**. Single permission.
    *   Available options: `SelfInvoicing`, `RRInvoicing`, `TaxRepresentative`, `PefInvoicing`.
*   `with_description(String)` - **Required**.
*   `with_subject_details(AuthorizationSubjectDetails)` - **Required**.
    *   Requires `full_name` field.

```rust
use ksef_client::{
    GrantAuthorizationPermissionsRequest, AuthorizationSubjectIdentifier,
    AuthorizationSubjectIdentifierType, AuthorizationPermissionType, AuthorizationSubjectDetails
};

let request = GrantAuthorizationPermissionsRequest::builder()
    .with_subject_identifier(AuthorizationSubjectIdentifier {
        identifier_type: AuthorizationSubjectIdentifierType::Nip,
        value: "1234567890".to_string(),
    })
    .with_permission(AuthorizationPermissionType::SelfInvoicing)
    .with_description("Consent for self-invoicing")
    .with_subject_details(AuthorizationSubjectDetails {
        full_name: "Trading Partner Ltd.".to_string(),
    })
    .build()
    .expect("Error building request");

match client.grant_authorization_permissions(request) {
    Ok(resp) => println!("Permissions granted. Reference number: {}", resp.reference_number),
    Err(e) => eprintln!("Error: {:?}", e),
}
```

### 4. Granting permissions indirectly

The `grant_indirect_entity_permissions` method allows granting permissions to handle invoices of another entity (client) to a person indicated in the request.

**Builder: `GrantIndirectEntityPermissionsRequestBuilder`**

*   `with_subject_identifier(IndirectSubjectIdentifier)` - **Required**. The person receiving the permission.
    *   Types: `Nip`, `Pesel`, `Fingerprint`.
*   `with_target_identifier(IndirectTargetIdentifier)` - Optional. Specifies the client concerned by the permissions.
    *   If omitted or type `AllPartners`, the permission is general.
    *   Types: `Nip`, `InternalId` (for a specific client), `AllPartners`.
*   `with_permissions(Vec<IndirectPermissionType>)` - List of permissions.
    *   Options: `InvoiceRead`, `InvoiceWrite`.
*   `with_description(String)` - **Required**.
*   `with_subject_details(IndirectSubjectDetails)` - **Required**. Analogous to point 1 (person data).

```rust
use ksef_client::{
    GrantIndirectEntityPermissionsRequest, IndirectSubjectIdentifier, IndirectSubjectIdentifierType,
    IndirectTargetIdentifier, IndirectTargetIdentifierType, IndirectPermissionType,
    IndirectSubjectDetails, IndirectSubjectDetailsType, IndirectPersonById
};

let request = GrantIndirectEntityPermissionsRequest::builder()
    .with_subject_identifier(IndirectSubjectIdentifier {
        identifier_type: IndirectSubjectIdentifierType::Nip,
        value: "1234567890".to_string(),
    })
    .with_target_identifier(IndirectTargetIdentifier {
        identifier_type: IndirectTargetIdentifierType::AllPartners, // General permissions
        value: None,
    })
    .with_permissions(vec![
        IndirectPermissionType::InvoiceRead,
        IndirectPermissionType::InvoiceWrite,
    ])
    .with_description("Indirect permission granting")
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
    .expect("Error building request");

match client.grant_indirect_entity_permissions(request) {
    Ok(resp) => println!("Permissions granted. Reference number: {}", resp.reference_number),
    Err(e) => eprintln!("Error: {:?}", e),
}
```

### 5. Granting subunit administrator permissions

The `grant_subunit_permissions` method is used to manage permissions in the context of subunits.

**Builder: `GrantSubunitPermissionsRequestBuilder`**

*   `with_subject_identifier(SubunitSubjectIdentifier)` - **Required**. Person/entity being the administrator.
    *   Types: `Nip`, `Pesel`, `Fingerprint`.
*   `with_context_identifier(SubunitContextIdentifier)` - **Required**. Subunit identifier.
    *   Types: `InternalId` (requires correct checksum), `Nip`.
*   `with_description(String)` - **Required**.
*   `with_subunit_name(String)` - Unit name (required for `InternalId`).
*   `with_subject_details(SubunitSubjectDetails)` - **Required**. Person/entity data.

```rust
use ksef_client::{
    GrantSubunitPermissionsRequest, SubunitSubjectIdentifier, SubunitSubjectIdentifierType,
    SubunitContextIdentifier, SubunitContextIdentifierType, SubunitSubjectDetails,
    SubunitSubjectDetailsType, SubunitPersonById
};

let parent_nip = "1234567890";
// Identifier consists of NIP + 5 digits (last one is checksum)
// Use the appropriate algorithm described in official KSeF documentation to calculate the checksum here
let internal_id = "123456789012345"; 

let request = GrantSubunitPermissionsRequest::builder()
    .with_subject_identifier(SubunitSubjectIdentifier {
        identifier_type: SubunitSubjectIdentifierType::Nip,
        value: "0987654321".to_string(),
    })
    .with_context_identifier(SubunitContextIdentifier {
        identifier_type: SubunitContextIdentifierType::InternalId,
        value: internal_id.to_string(),
    })
    .with_description("Branch administrator")
    .with_subunit_name("North Branch")
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
    .expect("Error building request");

match client.grant_subunit_permissions(request) {
    Ok(resp) => println!("Permissions granted. Reference number: {}", resp.reference_number),
    Err(e) => eprintln!("Error: {:?}", e),
}
```

### 6. Granting EU entity administrator permissions

The `grant_eu_entity_permissions` method allows granting administrative permissions in the context of an EU entity (identified by NIP and EU VAT).

**Builder: `GrantEuEntityPermissionsRequestBuilder`**

*   `with_subject_identifier(EuEntitySubjectIdentifier)` - **Required**.
    *   Type: `Fingerprint` (certificate fingerprint only).
*   `with_context_identifier(EuEntityContextIdentifier)` - **Required**.
    *   Type: `NipVatUe` (Format: `{NIP}-{VAT_UE}`).
*   `with_description(String)` - **Required**.
*   `with_eu_entity_name(String)` - **Required**. Name and address of the EU entity.
*   `with_subject_details(EuEntitySubjectDetails)` - **Required**. Data of the person/entity being authorized.
    *   Types: `PersonByFingerprintWithIdentifier`, `PersonByFingerprintWithoutIdentifier`, `EntityByFingerprint`.
*   `with_eu_entity_details(EuEntityDetails)` - **Required**. EU entity details (name, address).

```rust
use ksef_client::{
    GrantEuEntityPermissionsRequest, EuEntitySubjectIdentifier, EuEntitySubjectIdentifierType,
    EuEntityContextIdentifier, EuEntityContextIdentifierType, EuEntitySubjectDetails,
    EuEntitySubjectDetailsType, EuEntityByFp, EuEntityDetails
};

let fingerprint = "0000000000000000000000000000000000000000000000000000000000000000";
let context_value = "1234567890-DE123456789"; // Format: {NIP}-{VAT_UE}

let request = GrantEuEntityPermissionsRequest::builder()
    .with_subject_identifier(EuEntitySubjectIdentifier {
        identifier_type: EuEntitySubjectIdentifierType::Fingerprint,
        value: fingerprint.to_string(),
    })
    .with_context_identifier(EuEntityContextIdentifier {
        identifier_type: EuEntityContextIdentifierType::NipVatUe,
        value: context_value.to_string(),
    })
    .with_description("EU entity administrator")
    .with_eu_entity_name("Test EU Company, Berlin, Germany")
    .with_subject_details(EuEntitySubjectDetails {
        subject_details_type: EuEntitySubjectDetailsType::EntityByFingerprint,
        person_by_fp_with_id: None,
        person_by_fp_no_id: None,
        entity_by_fp: Some(EuEntityByFp {
            full_name: "Test EU Company".to_string(),
            address: "Berlin, Germany".to_string(),
        }),
    })
    .with_eu_entity_details(EuEntityDetails {
        full_name: "Test EU Company".to_string(),
        address: "Berlin, Germany".to_string(),
    })
    .build()
    .expect("Error building request");

match client.grant_eu_entity_permissions(request) {
    Ok(resp) => println!("Permissions granted. Reference number: {}", resp.reference_number),
    Err(e) => eprintln!("Error: {:?}", e),
}
```

### 7. Granting EU entity representative permissions

The `grant_eu_entity_representative_permissions` method is used to grant permissions to issue or view invoices in the context of an EU entity.

**Builder: `GrantEuEntityRepresentativePermissionsRequestBuilder`**

*   `with_subject_identifier(EuEntityRepresentativeSubjectIdentifier)` - **Required**.
    *   Type: `Fingerprint`.
*   `with_permissions(Vec<EuEntityRepresentativePermissionType>)` - List of permissions.
    *   Options: `InvoiceRead`, `InvoiceWrite`.
*   `with_description(String)` - **Required**.
*   `with_subject_details(EuEntityRepresentativeSubjectDetails)` - **Required**. Person/entity data.
    *   Types: `PersonByFingerprintWithIdentifier`, `PersonByFingerprintWithoutIdentifier`, `EntityByFingerprint`.

```rust
use ksef_client::{
    GrantEuEntityRepresentativePermissionsRequest, EuEntityRepresentativeSubjectIdentifier,
    EuEntityRepresentativeSubjectIdentifierType, EuEntityRepresentativePermissionType,
    EuEntityRepresentativeSubjectDetails, EuEntityRepresentativeSubjectDetailsType,
    EuEntityRepresentativeEntityByFp
};

let fingerprint = "0000000000000000000000000000000000000000000000000000000000000000";

let request = GrantEuEntityRepresentativePermissionsRequest::builder()
    .with_subject_identifier(EuEntityRepresentativeSubjectIdentifier {
        identifier_type: EuEntityRepresentativeSubjectIdentifierType::Fingerprint,
        value: fingerprint.to_string(),
    })
    .with_permissions(vec![
        EuEntityRepresentativePermissionType::InvoiceRead,
        EuEntityRepresentativePermissionType::InvoiceWrite,
    ])
    .with_description("EU entity representative")
    .with_subject_details(EuEntityRepresentativeSubjectDetails {
        subject_details_type: EuEntityRepresentativeSubjectDetailsType::EntityByFingerprint,
        person_by_fp_with_id: None,
        person_by_fp_no_id: None,
        entity_by_fp: Some(EuEntityRepresentativeEntityByFp {
            full_name: "Test EU Company".to_string(),
            address: "Berlin, Germany".to_string(),
        }),
    })
    .build()
    .expect("Error building request");

match client.grant_eu_entity_representative_permissions(request) {
    Ok(resp) => println!("Permissions granted. Reference number: {}", resp.reference_number),
    Err(e) => eprintln!("Error: {:?}", e),
}
```

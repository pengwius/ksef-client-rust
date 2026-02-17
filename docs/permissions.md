Polska Wersja / [English version](permissions.en.md)

# Zarządzanie Uprawnieniami

Zarządzanie uprawnieniami w systemie KSeF, pozwalając na nadawanie praw do wykonywania określonych operacji różnym podmiotom.
Szczegółowy opis działania każdego endpointu oraz specyfikacja biznesowa znajduje się w oficjalnej dokumentacji API KSeF.

__[Oficjalna Dokumentacja](https://github.com/CIRFMF/ksef-docs/blob/main/uprawnienia.md)__

### 1. Nadawanie uprawnień osobom fizycznym

Metoda `grant_person_permissions` służy do nadawania uprawnień konkretnym osobom fizycznym (identyfikowanym np. przez PESEL lub NIP).

**Builder: `GrantPersonPermissionsRequestBuilder`**

*   `with_subject_identifier(SubjectIdentifier)` - **Wymagane**. Określa kogo uprawniamy.
    *   `identifier_type`: `GrantSubjectIdentifierType::Nip`, `GrantSubjectIdentifierType::Pesel`, `GrantSubjectIdentifierType::Fingerprint`.
    *   `value`: Wartość identyfikatora (np. numer NIP, PESEL lub hash certyfikatu).
*   `with_permissions(Vec<PersonPermissionType>)` - Lista uprawnień.
    *   Dostępne opcje: `InvoiceWrite`, `InvoiceRead`, `CredentialsManage`, `CredentialsRead`, `Introspection`, `SubunitManage`, `EnforcementOperations`.
    *   Można użyć metody `with_permission(...)` do dodawania pojedynczo.
*   `with_description(String)` - **Wymagane**. Opis nadawanego uprawnienia.
*   `with_subject_details(SubjectDetails)` - **Wymagane**. Szczegółowe dane osoby.
    *   `subject_details_type`:
        *   `PersonByIdentifier`: Dla osób z NIP/PESEL. Wymaga pola `person_by_id` (imię, nazwisko).
        *   `PersonByFingerprintWithIdentifier`: Dla osób z certyfikatem bez NIP/PESEL w certyfikacie, ale posiadających te numery. Wymaga pola `person_by_fp_with_id`.
        *   `PersonByFingerprintWithoutIdentifier`: Dla osób bez NIP/PESEL (np. obcokrajowcy). Wymaga pola `person_by_fp_no_id` (dane dokumentu tożsamości).

```rust
use ksef_client::{
    GrantPersonPermissionsRequest, GrantSubjectIdentifierType, PersonById, PersonPermissionType,
    SubjectDetails, SubjectDetailsType, SubjectIdentifier
};

let request = GrantPersonPermissionsRequest::builder()
    .with_subject_identifier(SubjectIdentifier {
        identifier_type: GrantSubjectIdentifierType::Nip,
        value: "1234567890".to_string(), // NIP osoby, której nadajemy uprawnienia
    })
    .with_permissions(vec![
        PersonPermissionType::InvoiceRead,
        PersonPermissionType::InvoiceWrite,
    ])
    .with_description("Nadanie uprawnień pracownikowi")
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
    .expect("Nie udało się zbudować żądania - brak wymaganych pól");

match client.grant_person_permissions(request) {
    Ok(resp) => println!("Nadano uprawnienia. Numer referencyjny: {}", resp.reference_number),
    Err(e) => eprintln!("Błąd: {:?}", e),
}
```

### 2. Nadawanie podmiotom uprawnień do obsługi faktur

Metoda `grant_entity_permissions` pozwala na upoważnienie innego podmiotu gospodarczego (np. biura rachunkowego) do obsługi faktur.

**Builder: `GrantEntityPermissionsRequestBuilder`**

*   `with_subject_identifier(EntityIdentifier)` - **Wymagane**. Identyfikuje podmiot uprawniany.
    *   `identifier_type`: `EntityIdentifierType::Nip`.
    *   `value`: Numer NIP podmiotu.
*   `with_permissions(Vec<EntityPermission>)` - Lista uprawnień.
    *   Struktura `EntityPermission` zawiera typ uprawnienia (`EntityPermissionType::InvoiceRead` lub `InvoiceWrite`) oraz flagę `can_delegate` (czy podmiot może przekazywać to uprawnienie dalej).
*   `with_description(String)` - **Wymagane**. Opis.
*   `with_subject_details(EntitySubjectDetails)` - **Wymagane**.
    *   Wymaga pola `full_name` (pełna nazwa podmiotu).

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
            can_delegate: Some(false), // Brak możliwości dalszego delegowania
        },
        EntityPermission {
            permission_type: EntityPermissionType::InvoiceWrite,
            can_delegate: Some(true), // Możliwość dalszego delegowania
        },
    ])
    .with_description("Upoważnienie dla biura rachunkowego")
    .with_subject_details(EntitySubjectDetails {
        full_name: "Przykładowe Biuro Rachunkowe Sp. z o.o.".to_string(),
    })
    .build()
    .expect("Błąd budowania żądania");

match client.grant_entity_permissions(request) {
    Ok(resp) => println!("Nadano uprawnienia. Numer referencyjny: {}", resp.reference_number),
    Err(e) => eprintln!("Błąd: {:?}", e),
}
```

### 3. Nadanie uprawnień podmiotowych (np. samofakturowanie)

Metoda `grant_authorization_permissions` służy do nadawania specyficznych uprawnień, takich jak samofakturowanie.

**Builder: `GrantAuthorizationPermissionsRequestBuilder`**

*   `with_subject_identifier(AuthorizationSubjectIdentifier)` - **Wymagane**.
    *   `identifier_type`: `AuthorizationSubjectIdentifierType::Nip` lub `PeppolId`.
    *   `value`: Wartość identyfikatora.
*   `with_permission(AuthorizationPermissionType)` - **Wymagane**. Pojedyncze uprawnienie.
    *   Dostępne opcje: `SelfInvoicing`, `RRInvoicing`, `TaxRepresentative`, `PefInvoicing`.
*   `with_description(String)` - **Wymagane**.
*   `with_subject_details(AuthorizationSubjectDetails)` - **Wymagane**.
    *   Wymaga pola `full_name`.

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
    .with_description("Zgoda na samofakturowanie")
    .with_subject_details(AuthorizationSubjectDetails {
        full_name: "Partner Handlowy Sp. z o.o.".to_string(),
    })
    .build()
    .expect("Błąd budowania żądania");

match client.grant_authorization_permissions(request) {
    Ok(resp) => println!("Nadano uprawnienia. Numer referencyjny: {}", resp.reference_number),
    Err(e) => eprintln!("Błąd: {:?}", e),
}
```

### 4. Nadawanie uprawnień w sposób pośredni

Metoda `grant_indirect_entity_permissions` pozwala na nadanie uprawnień do obsługi faktur innego podmiotu (klienta) osobie wskazanej w żądaniu.

**Builder: `GrantIndirectEntityPermissionsRequestBuilder`**

*   `with_subject_identifier(IndirectSubjectIdentifier)` - **Wymagane**. Osoba otrzymująca uprawnienie.
    *   Typy: `Nip`, `Pesel`, `Fingerprint`.
*   `with_target_identifier(IndirectTargetIdentifier)` - Opcjonalne. Określa klienta, którego dotyczą uprawnienia.
    *   Jeśli pominięte lub typ `AllPartners`, uprawnienie jest generalne.
    *   Typy: `Nip`, `InternalId` (dla konkretnego klienta), `AllPartners`.
*   `with_permissions(Vec<IndirectPermissionType>)` - Lista uprawnień.
    *   Opcje: `InvoiceRead`, `InvoiceWrite`.
*   `with_description(String)` - **Wymagane**.
*   `with_subject_details(IndirectSubjectDetails)` - **Wymagane**. Analogicznie jak w pkt 1 (dane osoby).

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
        identifier_type: IndirectTargetIdentifierType::AllPartners, // Uprawnienia generalne
        value: None,
    })
    .with_permissions(vec![
        IndirectPermissionType::InvoiceRead,
        IndirectPermissionType::InvoiceWrite,
    ])
    .with_description("Pośrednie nadanie uprawnień")
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
    .expect("Błąd budowania żądania");

match client.grant_indirect_entity_permissions(request) {
    Ok(resp) => println!("Nadano uprawnienia. Numer referencyjny: {}", resp.reference_number),
    Err(e) => eprintln!("Błąd: {:?}", e),
}
```

### 5. Nadanie uprawnień administratora podmiotu podrzędnego

Metoda `grant_subunit_permissions` służy do zarządzania uprawnieniami w kontekście jednostek podrzędnych.

**Builder: `GrantSubunitPermissionsRequestBuilder`**

*   `with_subject_identifier(SubunitSubjectIdentifier)` - **Wymagane**. Osoba/podmiot będący administratorem.
    *   Typy: `Nip`, `Pesel`, `Fingerprint`.
*   `with_context_identifier(SubunitContextIdentifier)` - **Wymagane**. Identyfikator podmiotu podrzędnego.
    *   Typy: `InternalId` (wymaga poprawnej sumy kontrolnej), `Nip`.
*   `with_description(String)` - **Wymagane**.
*   `with_subunit_name(String)` - Nazwa jednostki (wymagane dla `InternalId`).
*   `with_subject_details(SubunitSubjectDetails)` - **Wymagane**. Dane osoby/podmiotu.

```rust
use ksef_client::{
    GrantSubunitPermissionsRequest, SubunitSubjectIdentifier, SubunitSubjectIdentifierType,
    SubunitContextIdentifier, SubunitContextIdentifierType, SubunitSubjectDetails,
    SubunitSubjectDetailsType, SubunitPersonById
};

let parent_nip = "1234567890";
// Identyfikator składa się z NIP + 5 cyfr (ostatnia to suma kontrolna)
// W tym miejscu należy użyć odpowiedniego algorytmu do wyliczenia sumy kontrolnej opisanego w oficjalnej dokumentacji KSeF
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
    .with_description("Administrator oddziału")
    .with_subunit_name("Oddział Północ")
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
    .expect("Błąd budowania żądania");

match client.grant_subunit_permissions(request) {
    Ok(resp) => println!("Nadano uprawnienia. Numer referencyjny: {}", resp.reference_number),
    Err(e) => eprintln!("Błąd: {:?}", e),
}
```

### 6. Nadanie uprawnień administratora podmiotu unijnego

Metoda `grant_eu_entity_permissions` pozwala na nadanie uprawnień administracyjnych w kontekście podmiotu unijnego (identyfikowanego przez NIP i VAT UE).

**Builder: `GrantEuEntityPermissionsRequestBuilder`**

*   `with_subject_identifier(EuEntitySubjectIdentifier)` - **Wymagane**.
    *   Typ: `Fingerprint` (tylko odcisk certyfikatu).
*   `with_context_identifier(EuEntityContextIdentifier)` - **Wymagane**.
    *   Typ: `NipVatUe` (Format: `{NIP}-{VAT_UE}`).
*   `with_description(String)` - **Wymagane**.
*   `with_eu_entity_name(String)` - **Wymagane**. Nazwa i adres podmiotu UE.
*   `with_subject_details(EuEntitySubjectDetails)` - **Wymagane**. Dane osoby/podmiotu uprawnianego.
    *   Typy: `PersonByFingerprintWithIdentifier`, `PersonByFingerprintWithoutIdentifier`, `EntityByFingerprint`.
*   `with_eu_entity_details(EuEntityDetails)` - **Wymagane**. Szczegóły podmiotu unijnego (nazwa, adres).

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
    .with_description("Administrator podmiotu UE")
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
    .expect("Błąd budowania żądania");

match client.grant_eu_entity_permissions(request) {
    Ok(resp) => println!("Nadano uprawnienia. Numer referencyjny: {}", resp.reference_number),
    Err(e) => eprintln!("Błąd: {:?}", e),
}
```

### 7. Nadanie uprawnień reprezentanta podmiotu unijnego

Metoda `grant_eu_entity_representative_permissions` służy do nadawania uprawnień do wystawiania lub przeglądania faktur w kontekście podmiotu unijnego.

**Builder: `GrantEuEntityRepresentativePermissionsRequestBuilder`**

*   `with_subject_identifier(EuEntityRepresentativeSubjectIdentifier)` - **Wymagane**.
    *   Typ: `Fingerprint`.
*   `with_permissions(Vec<EuEntityRepresentativePermissionType>)` - Lista uprawnień.
    *   Opcje: `InvoiceRead`, `InvoiceWrite`.
*   `with_description(String)` - **Wymagane**.
*   `with_subject_details(EuEntityRepresentativeSubjectDetails)` - **Wymagane**. Dane osoby/podmiotu.
    *   Typy: `PersonByFingerprintWithIdentifier`, `PersonByFingerprintWithoutIdentifier`, `EntityByFingerprint`.

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
    .with_description("Reprezentant podmiotu UE")
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
    .expect("Błąd budowania żądania");

match client.grant_eu_entity_representative_permissions(request) {
    Ok(resp) => println!("Nadano uprawnienia. Numer referencyjny: {}", resp.reference_number),
    Err(e) => eprintln!("Błąd: {:?}", e),
}
```

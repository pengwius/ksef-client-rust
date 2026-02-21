Polska wersja / [English version](ksef_certificates.en.md)

# Certyfikaty KSeF

__[Oficjalna Dokumentacja](https://github.com/CIRFMF/ksef-docs/blob/main/certyfikaty-KSeF.md)__

### 1. Sprawdzanie limitów

```rust
let cert_limits = match client.get_certificates_limits() {
    Ok(limits) => limits,
    Err(e) => panic!("Failed to retrieve certificate limits: {:?}", e),
};
```

### 2. Pobieranie danych do wniosku certyfikacyjnego

```rust
let enrollment_data = match client.get_enrollment_data() {
    Ok(data) => data,
    Err(e) => panic!("Failed to retrieve enrollment data: {:?}", e),
};
```

### 3. Przygotowanie żądania podpisania certyfikatu CSR (Certificate Signing Request)

`enrollment_data` uzyskujemy za pomocą `get_enrollment_data`

```rust
let csr_result = match client.generate_csr(&enrollment_data) {
    Ok(res) => res,
    Err(e) => panic!("Failed to generate CSR: {:?}", e),
};
```

### 4. Wysyłanie żądania

```rust
use ksef_client::{EnrollCertificateRequest, CertificateType};

let request = EnrollCertificateRequest {
    certificate_name: "Nazwa Certyfikatu".to_string(),
    certificate_type: CertificateType::Authentication,
    csr: csr_result.csr_base64, // CSR wygenerowane w punkcie 3
    valid_from: None,
};

let reference_number = match client.enroll_certificate(request) {
    Ok(response) => response.reference_number,
    Err(e) => panic!("Failed to enroll certificate: {:?}", e),
};
```

### 5. Sprawdzanie statusu żądania

`reference_number` zwracany jest przez `enroll_certificate` w punkcie 4. Metoda `get_enrollment_status` blokuje wykonanie i odpytuje API w pętli (pooling), dopóki status nie zmieni się na finalny (sukces lub błąd).

```rust
match client.get_enrollment_status(&reference_number) {
    Ok(status_resp) => {
        if let Some(serial) = status_resp.certificate_serial_number {
            println!("Certyfikat wygenerowany! Numer seryjny: {}", serial);
        } else {
             println!("Status: Code={}, Desc={}", status_resp.status.code, status_resp.status.description);
        }
    },
    Err(e) => panic!("Failed to check enrollment status: {:?}", e),
};
```

### 6. Pobieranie listy certyfikatów

Metoda pozwala pobrać treść certyfikatów na podstawie ich numerów seryjnych.

```rust
let serials = vec![ /* numery seryjne certyfikatów, które chcemy pobrać */ ];

let certificates = match client.retrieve_certificates(serials) {
    Ok(certs) => certs,
    Err(e) => panic!("Failed to retrieve certificates: {:?}", e),
};

for cert in certificates {
    println!("Nazwa: {}, Numer seryjny: {}", cert.certificate_name, cert.certificate_serial_number);
    // cert.certificate zawiera treść certyfikatu w Base64 (DER)
}
```

### 7. Pobieranie listy metadanych certyfikatów

Metoda pozwala na wyszukiwanie certyfikatów i pobieranie ich metadanych.

Parametry filtrowania w `GetCertificateMetadataListRequest` (wszystkie są opcjonalne):

- `certificate_serial_number` - numer seryjny certyfikatu
- `name` - nazwa certyfikatu
- `certificate_type` - typ certyfikatu jako `CertificateType`
- `status` - status certyfikatu jako `CertificateStatus`
- `expires_after` - data jako `DateTime<Utc>`

Argumenty metody `get_certificate_metadata_list`:

- `query` - obiekt `GetCertificateMetadataListRequest` zawierający kryteria filtrowania
- `page_size` - liczba wyników na stronę (opcjonalnie)
- `page_offset` - numer strony wyników (opcjonalnie)

```rust
use ksef_client::GetCertificateMetadataListRequest;

let query = GetCertificateMetadataListRequest {
    certificate_serial_number: Some("numer_seryjny".to_string()),
    ..Default::default()
};

let certificates_metadata_list = match client.get_certificate_metadata_list(query, Some(10), Some(0)) {
    Ok(response) => {
        if response.has_more {
            println!("There are more certificates to get.");
        }
        response.certificates
    },
    Err(e) => panic!("Failed to retrieve certificate metadata: {:?}", e),
};
```

### 8. Unieważnianie certyfikatów

Parametry metody `revoke_certificate`:

- `serial_number` - numer seryjny certyfikatu do unieważnienia
- `reason` - powód unieważnienia jako `RevocationReason`

Jako powód unieważnienia (`RevocationReason`) możemy podać:

- `Unspecified` - powód nieokreślony
- `KeyCompromise` - kompromitacja klucza
- `Superseded` - zastąpienie przez inny certyfikat

```rust
use ksef_client::RevocationReason;

match client.revoke_certificate(&serial, RevocationReason::Unspecified) {
    Ok(()) => {
        println!("Certificate revoked successfully.");
    }
    Err(e) => panic!("Failed to revoke certificate: {:?}", e),
}
```

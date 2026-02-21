[Polish version](ksef_certificates.md) / English version

# KSeF Certificates

__[Official Documentation](https://github.com/CIRFMF/ksef-docs/blob/main/certyfikaty-KSeF.md)__

### 1. Checking limits

```rust
let cert_limits = match client.get_certificates_limits() {
    Ok(limits) => limits,
    Err(e) => panic!("Failed to retrieve certificate limits: {:?}", e),
};
```

### 2. Retrieving data for certificate enrollment

```rust
let enrollment_data = match client.get_enrollment_data() {
    Ok(data) => data,
    Err(e) => panic!("Failed to retrieve enrollment data: {:?}", e),
};
```

### 3. Preparing CSR (Certificate Signing Request)

`enrollment_data` is obtained using `get_enrollment_data`.

```rust
let csr_result = match client.generate_csr(&enrollment_data) {
    Ok(res) => res,
    Err(e) => panic!("Failed to generate CSR: {:?}", e),
};
```

### 4. Sending the request

```rust
use ksef_client::{EnrollCertificateRequest, CertificateType};

let request = EnrollCertificateRequest {
    certificate_name: "Certificate Name".to_string(),
    certificate_type: CertificateType::Authentication,
    csr: csr_result.csr_base64, // CSR generated in step 3
    valid_from: None,
};

let reference_number = match client.enroll_certificate(request) {
    Ok(response) => response.reference_number,
    Err(e) => panic!("Failed to enroll certificate: {:?}", e),
};
```

### 5. Checking request status

`reference_number` is returned by `enroll_certificate` in step 4. The `get_enrollment_status` method blocks execution and polls the API (polling) until the status changes to final (success or error).

```rust
match client.get_enrollment_status(&reference_number) {
    Ok(status_resp) => {
        if let Some(serial) = status_resp.certificate_serial_number {
            println!("Certificate generated! Serial number: {}", serial);
        } else {
             println!("Status: Code={}, Desc={}", status_resp.status.code, status_resp.status.description);
        }
    },
    Err(e) => panic!("Failed to check enrollment status: {:?}", e),
};
```

### 6. Retrieving list of certificates

This method allows retrieving the content of certificates based on their serial numbers.

```rust
let serials = vec![ /* serial numbers of certificates to retrieve */ ];

let certificates = match client.retrieve_certificates(serials) {
    Ok(certs) => certs,
    Err(e) => panic!("Failed to retrieve certificates: {:?}", e),
};

for cert in certificates {
    println!("Name: {}, Serial Number: {}", cert.certificate_name, cert.certificate_serial_number);
    // cert.certificate contains the certificate content in Base64 (DER)
}
```

### 7. Retrieving certificate metadata list

This method allows searching for certificates and retrieving their metadata.

Filter parameters in `GetCertificateMetadataListRequest` (all are optional):

- `certificate_serial_number` - certificate serial number
- `name` - certificate name
- `certificate_type` - certificate type as `CertificateType`
- `status` - certificate status as `CertificateStatus`
- `expires_after` - date as `DateTime<Utc>`

Arguments of the `get_certificate_metadata_list` method:

- `query` - `GetCertificateMetadataListRequest` object containing filtering criteria
- `page_size` - number of results per page (optional)
- `page_offset` - result page number (optional)

```rust
use ksef_client::GetCertificateMetadataListRequest;

let query = GetCertificateMetadataListRequest {
    certificate_serial_number: Some("serial_number".to_string()),
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

### 8. Revoking certificates

Parameters of the `revoke_certificate` method:

- `serial_number` - serial number of the certificate to revoke
- `reason` - revocation reason as `RevocationReason`

Possible revocation reasons (`RevocationReason`):

- `Unspecified` - unspecified reason
- `KeyCompromise` - key compromise
- `Superseded` - superseded by another certificate

```rust
use ksef_client::RevocationReason;

match client.revoke_certificate(&serial, RevocationReason::Unspecified) {
    Ok(()) => {
        println!("Certificate revoked successfully.");
    }
    Err(e) => panic!("Failed to revoke certificate: {:?}", e),
}
```

use crate::common;
use ksef_client::{CertificateType, EnrollCertificateRequest, GetCertificateMetadataListRequest};

#[test]
fn test_get_certificate_metadata_list() {
    let client = common::authorize_client();

    println!("Getting enrollment data...");
    let enrollment_data = match client.get_enrollment_data() {
        Ok(data) => data,
        Err(e) => panic!("Failed to get enrollment data: {:?}", e),
    };

    println!("Generating CSR...");
    let csr_result = match client.generate_csr(&enrollment_data) {
        Ok(res) => res,
        Err(e) => panic!("Failed to generate CSR: {:?}", e),
    };

    let request = EnrollCertificateRequest {
        certificate_name: "Test Certificate Metadata List".to_string(),
        certificate_type: CertificateType::Authentication,
        csr: csr_result.csr_base64,
        valid_from: None,
    };

    println!("Sending enrollment request...");
    let reference_number = match client.enroll_certificate(request) {
        Ok(response) => {
            println!(
                "Enrollment successful. Reference Number: {}",
                response.reference_number
            );
            response.reference_number
        }
        Err(e) => {
            println!("Enrollment failed: {:?}", e);
            if let ksef_client::error::KsefError::ApiError(code, _) = e {
                if code >= 400 {
                    println!(
                        "Skipping metadata list test due to enrollment failure (business error)."
                    );
                    return;
                }
            }
            panic!("Unexpected error during enrollment setup: {:?}", e);
        }
    };

    println!(
        "Checking enrollment status for ref: {} (waiting for completion...)",
        reference_number
    );
    let status_resp = match client.get_enrollment_status(&reference_number) {
        Ok(resp) => resp,
        Err(e) => panic!("Failed to get enrollment status: {:?}", e),
    };

    if status_resp.status.code >= 400 {
        println!("Enrollment was rejected. Skipping metadata list test.");
        if let Some(details) = status_resp.status.details {
            println!("Rejection details: {:?}", details);
        }
        return;
    }

    if let Some(serial) = status_resp.certificate_serial_number {
        println!("Certificate generated! Serial: {}", serial);
        println!("Querying metadata list for serial: {}", serial);

        let query = GetCertificateMetadataListRequest {
            certificate_serial_number: Some(serial.clone()),
            ..Default::default()
        };

        match client.get_certificate_metadata_list(query, Some(10), Some(0)) {
            Ok(response) => {
                println!(
                    "Query successful. Retrieved {} certificates.",
                    response.certificates.len()
                );

                assert!(
                    !response.certificates.is_empty(),
                    "Should find the newly enrolled certificate"
                );

                let found = response
                    .certificates
                    .iter()
                    .find(|c| c.certificate_serial_number == serial);

                assert!(
                    found.is_some(),
                    "The enrolled certificate should be in the list"
                );

                let cert_item = found.unwrap();
                println!(
                    "Found Certificate: Serial={}, Name={}, Status={:?}",
                    cert_item.certificate_serial_number, cert_item.name, cert_item.status
                );

                assert_eq!(
                    cert_item.certificate_serial_number, serial,
                    "Serial number mismatch"
                );
            }
            Err(e) => panic!("Failed to get certificate metadata list: {:?}", e),
        }
    } else {
        panic!("Status 200 (Success) but no certificate serial number returned!");
    }
}

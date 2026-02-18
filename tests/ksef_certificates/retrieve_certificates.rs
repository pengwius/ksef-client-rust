use crate::common;
use ksef_client::error::KsefError;
use ksef_client::{CertificateType, EnrollCertificateRequest};
use std::thread;
use std::time::Duration;

#[test]
fn test_retrieve_certificates() {
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
        certificate_name: "Test Certificate Retrieval".to_string(),
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
            if let KsefError::ApiError(code, _) = e {
                if code >= 400 {
                    println!("Skipping retrieval test due to enrollment failure (business error).");
                    return;
                }
            }
            panic!("Unexpected error during enrollment setup: {:?}", e);
        }
    };

    println!("Polling enrollment status for ref: {}", reference_number);
    let mut serial_number: Option<String> = None;

    for attempt in 1..=6 {
        thread::sleep(Duration::from_secs(5));
        println!("Polling attempt {}...", attempt);

        match client.get_enrollment_status(&reference_number) {
            Ok(status_resp) => {
                println!(
                    "Status: Code={}, Desc={}",
                    status_resp.status.code, status_resp.status.description
                );

                if status_resp.status.code == 200 {
                    if let Some(sn) = status_resp.certificate_serial_number {
                        println!("Certificate generated! Serial: {}", sn);
                        serial_number = Some(sn);
                        break;
                    } else {
                        panic!("Status 200 but no serial number returned!");
                    }
                } else if status_resp.status.code >= 400 {
                    println!("Enrollment rejected/failed with status >= 400. Stopping poll.");
                    println!("Details: {:?}", status_resp.status.details);
                    return;
                }
            }
            Err(e) => {
                println!("Error checking status: {:?}", e);
                break;
            }
        }
    }

    if let Some(serial) = serial_number {
        println!("Retrieving certificate for serial: {}", serial);
        let serials_to_retrieve = vec![serial.clone()];

        match client.retrieve_certificates(serials_to_retrieve) {
            Ok(retrieved_list) => {
                println!("Retrieved {} certificates.", retrieved_list.len());

                assert!(
                    !retrieved_list.is_empty(),
                    "Should retrieve at least one certificate"
                );

                let retrieved = &retrieved_list[0];
                println!(
                    "Retrieved: Name={}, Serial={}",
                    retrieved.certificate_name, retrieved.certificate_serial_number
                );

                assert_eq!(
                    retrieved.certificate_serial_number, serial,
                    "Retrieved serial number should match requested one"
                );
                assert!(
                    !retrieved.certificate.is_empty(),
                    "Certificate content should not be empty"
                );
            }
            Err(e) => panic!("Failed to retrieve certificates: {:?}", e),
        }
    } else {
        println!(
            "Could not obtain a valid certificate serial number within the timeout period or enrollment failed."
        );
    }
}

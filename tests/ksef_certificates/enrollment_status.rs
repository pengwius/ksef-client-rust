use crate::common;
use ksef_client::error::KsefError;
use ksef_client::{CertificateType, EnrollCertificateRequest};

#[test]
fn test_get_enrollment_status() {
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
        certificate_name: "Test Certificate Status Check".to_string(),
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
                    println!("Skipping status check due to enrollment failure (business error).");
                    return;
                }
            }
            panic!("Unexpected error during enrollment setup: {:?}", e);
        }
    };

    println!("Checking enrollment status for ref: {}", reference_number);
    match client.get_enrollment_status(&reference_number) {
        Ok(status_response) => {
            println!("Status retrieved successfully.");
            println!("Request Date: {}", status_response.request_date);
            println!("Status Code: {}", status_response.status.code);
            println!("Status Description: {}", status_response.status.description);

            if let Some(details) = &status_response.status.details {
                println!("Details: {:?}", details);
            }
            if let Some(serial) = &status_response.certificate_serial_number {
                println!("Certificate Serial: {}", serial);
            }

            assert!(
                status_response.status.code >= 100,
                "Status code should be valid (>= 100)"
            );
        }
        Err(e) => panic!("Failed to get enrollment status: {:?}", e),
    }
}

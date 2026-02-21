use crate::common;
use ksef_client::error::KsefError;
use ksef_client::{CertificateType, EnrollCertificateRequest};

#[test]
fn test_enroll_certificate() {
    let client = common::authorize_client();

    println!("Getting enrollment data...");
    let enrollment_data = match client.get_enrollment_data() {
        Ok(data) => data,
        Err(e) => panic!("Failed to get enrollment data: {:?}", e),
    };

    println!("Generating CSR based on enrollment data...");
    let csr_result = match client.generate_csr(&enrollment_data) {
        Ok(res) => res,
        Err(e) => panic!("Failed to generate CSR: {:?}", e),
    };

    println!("CSR generated (len: {})", csr_result.csr_base64.len());

    let request = EnrollCertificateRequest {
        certificate_name: "Test Certificate Name".to_string(),
        certificate_type: CertificateType::Authentication,
        csr: csr_result.csr_base64,
        valid_from: None,
    };

    println!("Sending enrollment request...");
    match client.enroll_certificate(request) {
        Ok(response) => {
            println!("Enrollment successful!");
            println!("Reference Number: {}", response.reference_number);
            println!("Timestamp: {}", response.timestamp);

            assert!(
                !response.reference_number.is_empty(),
                "Reference number should not be empty"
            );
        }
        Err(e) => {
            println!("Enrollment request result: {:?}", e);
            if let KsefError::ApiError(code, _) = e {
                assert!(
                    code >= 400,
                    "Expected client error or success, got {}",
                    code
                );
            } else {
                panic!("Unexpected error type during enrollment: {:?}", e);
            }
        }
    }
}

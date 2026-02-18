use crate::common;
use ksef_client::{CertificateType, EnrollCertificateRequest, RevocationReason};

#[test]
fn test_revoke_certificate() {
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
        certificate_name: "Test Certificate Revocation".to_string(),
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
                        "Skipping revocation test due to enrollment failure (business error)."
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
        println!("Enrollment was rejected. Skipping revocation test.");
        if let Some(details) = status_resp.status.details {
            println!("Rejection details: {:?}", details);
        }
        return;
    }

    if let Some(serial) = status_resp.certificate_serial_number {
        println!("Certificate generated! Serial: {}", serial);

        println!("Revoking certificate {}...", serial);
        match client.revoke_certificate(&serial, RevocationReason::Unspecified) {
            Ok(()) => {
                println!("Certificate revoked successfully.");
            }
            Err(e) => panic!("Failed to revoke certificate: {:?}", e),
        }
    } else {
        panic!("Status 200 (Success) but no certificate serial number returned!");
    }
}

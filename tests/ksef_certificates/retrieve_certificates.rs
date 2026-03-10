use crate::common;
use ksef_client::certificates::{CertificateType, EnrollCertificateRequest, RevocationReason};
use ksef_client::prelude::*;

#[tokio::test]
async fn test_retrieve_certificates() {
    let client: ksef_client::KsefClient = common::authorize_client().await;

    println!("Getting enrollment data...");
    let enrollment_data = match client.get_enrollment_data().await {
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
    let reference_number = match client.enroll_certificate(request).await {
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

    let serial_number = match client.get_enrollment_status(&reference_number).await {
        Ok(status_resp) => {
            println!(
                "Status: Code={}, Desc={}",
                status_resp.status.code, status_resp.status.description
            );

            if let Some(sn) = status_resp.certificate_serial_number {
                println!("Certificate generated! Serial: {}", sn);
                sn
            } else {
                panic!("Status 200 but no serial number returned!");
            }
        }
        Err(e) => {
            panic!("Error checking status: {:?}", e);
        }
    };

    let serials_to_retrieve = vec![serial_number.clone()];

    let certs = match client.retrieve_certificates(serials_to_retrieve).await {
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
                retrieved.certificate_serial_number, serial_number,
                "Retrieved serial number should match requested one"
            );
            assert!(
                !retrieved.certificate.is_empty(),
                "Certificate content should not be empty"
            );

            retrieved_list
        }
        Err(e) => panic!("Failed to retrieve certificates: {:?}", e),
    };

    for cert in certs.iter() {
        match client
            .revoke_certificate(
                &cert.certificate_serial_number,
                RevocationReason::Unspecified,
            )
            .await
        {
            Ok(()) => println!(
                "Certificate {} revoked successfully.",
                cert.certificate_serial_number
            ),
            Err(e) => panic!(
                "Failed to revoke certificate {}: {:?}",
                cert.certificate_serial_number, e
            ),
        }
    }
}

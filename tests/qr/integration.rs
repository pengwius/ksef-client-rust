use crate::common;
use chrono::Utc;
use ksef_client::{CertificateType, EnrollCertificateRequest, KsefClient, RevocationReason};

async fn enroll_and_build_cert_qr(
    client: &KsefClient,
    seller_nip: &str,
    invoice_hash: &str,
) -> Result<(String, String), String> {
    let enrollment_data = client
        .get_enrollment_data()
        .await
        .map_err(|e| format!("Failed to get enrollment data: {:?}", e))?;

    let csr_result = client
        .generate_csr(&enrollment_data)
        .map_err(|e| format!("Failed to generate CSR: {:?}", e))?;

    let request = EnrollCertificateRequest {
        certificate_name: "Test Integration Certificate".to_string(),
        certificate_type: CertificateType::Authentication,
        csr: csr_result.csr_base64.clone(),
        valid_from: None,
    };

    let enroll_resp = client
        .enroll_certificate(request)
        .await
        .map_err(|e| format!("Failed to request enrollment: {:?}", e))?;

    let status_resp = client
        .get_enrollment_status(&enroll_resp.reference_number)
        .await
        .map_err(|e| format!("Failed to poll enrollment status: {:?}", e))?;

    if let Some(serial) = status_resp.certificate_serial_number {
        let private_pem = csr_result.private_key_pem;
        let url = client
            .build_certificate_verification_url(
                "Nip",
                seller_nip,
                seller_nip,
                &serial,
                invoice_hash,
                Some(private_pem.as_str()),
                true,
            )
            .map_err(|e| format!("Failed to build KOD II URL from enrolled cert: {:?}", e))?;

        Ok((url, serial))
    } else {
        Err("Enrollment finished but no certificate serial returned".to_string())
    }
}

#[tokio::test]
async fn test_send_invoice_and_build_qr() {
    let client: KsefClient = common::authorize_client().await;

    let seller_nip = client.context.value.clone();

    let invoice_xml: String = common::generate_fa2_invoice(&seller_nip).await;

    let issue_date_ddmmrrrr = invoice_xml
        .split("<P_6>")
        .nth(1)
        .and_then(|s| s.split("</P_6>").next())
        .and_then(|iso| {
            chrono::NaiveDate::parse_from_str(iso.trim(), "%Y-%m-%d")
                .ok()
                .map(|d| d.format("%d-%m-%Y").to_string())
        })
        .unwrap_or_else(|| Utc::now().format("%d-%m-%Y").to_string());

    println!(
        "Submitting invoice for seller NIP: {}, issue date: {}",
        seller_nip, issue_date_ddmmrrrr
    );

    let submit_result = client
        .submit_online(invoice_xml.as_bytes())
        .await
        .expect("Failed to submit online invoice");

    println!(
        "Submitted invoice: session_reference={}, invoice_reference={}",
        submit_result.session_reference_number, submit_result.invoice_reference_number
    );

    let status = client
        .get_invoice_status(
            &submit_result.session_reference_number,
            &submit_result.invoice_reference_number,
        )
        .await
        .expect("Failed to get invoice status");

    println!(
        "Invoice status: code={}, description={}",
        status.invoice_status.code, status.invoice_status.description
    );

    assert!(
        status.invoice_status.code == 200,
        "Expected invoice to be accepted (200), got {} ({})",
        status.invoice_status.code,
        status.invoice_status.description
    );

    let invoice_hash = status.invoice_hash;
    let invoice_url =
        client.build_invoice_verification_url(&seller_nip, &issue_date_ddmmrrrr, &invoice_hash);
    println!("Invoice verification URL: {}", invoice_url);

    assert!(
        invoice_url.contains(&format!("/invoice/{}/{}/", seller_nip, issue_date_ddmmrrrr)),
        "Invoice URL does not contain expected segments: {}",
        invoice_url
    );

    let cert_qr_result = enroll_and_build_cert_qr(&client, &seller_nip, &invoice_hash).await;

    match cert_qr_result {
        Ok((cert_qr, serial)) => {
            println!("Certificate verification URL: {}", cert_qr);
            assert!(
                cert_qr.contains("/certificate/"),
                "Cert QR URL should contain /certificate/"
            );

            match client
                .revoke_certificate(&serial, RevocationReason::Unspecified)
                .await
            {
                Ok(()) => println!("Certificate {} revoked successfully.", serial),
                Err(e) => panic!("Failed to revoke certificate {}: {:?}", serial, e),
            }
        }
        Err(e) => {
            panic!("Could not produce cert QR: {}", e);
        }
    }
}

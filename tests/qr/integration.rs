use crate::common;
use chrono::Utc;
use ksef_client::KsefClient;
use ksef_client::certificates::{CertificateType, EnrollCertificateRequest, RevocationReason};
use ksef_client::prelude::*;
use secrecy::ExposeSecret;

async fn submit_invoice_and_get_hash(
    client: &KsefClient,
    seller_nip: &str,
) -> Result<(String, String), String> {
    let invoice_xml: String = common::generate_fa2_invoice(seller_nip).await;

    let issue_date = invoice_xml
        .split("<P_6>")
        .nth(1)
        .and_then(|s| s.split("</P_6>").next())
        .and_then(|iso| {
            chrono::NaiveDate::parse_from_str(iso.trim(), "%Y-%m-%d")
                .ok()
                .map(|d| d.format("%d-%m-%Y").to_string())
        })
        .unwrap_or_else(|| Utc::now().format("%d-%m-%Y").to_string());

    let submit_result = client
        .submit_online(invoice_xml.as_bytes())
        .await
        .map_err(|e| format!("Failed to submit invoice: {:?}", e))?;

    let status = client
        .get_invoice_status(
            submit_result.session_reference_number.clone(),
            submit_result.invoice_reference_number.clone(),
        )
        .await
        .map_err(|e| format!("Failed to get invoice status: {:?}", e))?;

    if status.invoice_status.code != 200 {
        return Err(format!(
            "Invoice not accepted: {} ({})",
            status.invoice_status.code, status.invoice_status.description
        ));
    }

    let buyer_nip = invoice_xml
        .split("<Podmiot2>")
        .nth(1)
        .and_then(|s| s.split("</Podmiot2>").next())
        .and_then(|sec| sec.split("<NIP>").nth(1))
        .and_then(|s| s.split("</NIP>").next())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "UNKNOWN_BUYER_NIP".to_string());

    let invoice_number = invoice_xml
        .split("<P_2>")
        .nth(1)
        .and_then(|s| s.split("</P_2>").next())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "UNKNOWN_INVOICE_NUMBER".to_string());

    let total_amount = invoice_xml
        .split("<P_15>")
        .nth(1)
        .and_then(|s| s.split("</P_15>").next())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "UNKNOWN_AMOUNT".to_string());

    println!(
        "Submitted invoice with number: {}, \nbuyer NIP: {}, \ntotal amount: {}",
        invoice_number, buyer_nip, total_amount
    );

    Ok((issue_date, status.invoice_hash))
}

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
        certificate_type: CertificateType::Offline,
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

    let serial = status_resp
        .certificate_serial_number
        .ok_or_else(|| "Enrollment finished but no certificate serial returned".to_string())?;

    let url = client
        .build_certificate_verification_url(
            "Nip",
            seller_nip,
            seller_nip,
            &serial,
            invoice_hash,
            Some(csr_result.private_key_pem.expose_secret().as_str()),
        )
        .map_err(|e| format!("Failed to build qr url from enrolled cert: {:?}", e))?;

    Ok((url, serial))
}

#[tokio::test]
async fn test_generate_invoice_qr_code() {
    let client: KsefClient = common::authorize_client().await;
    let seller_nip = client.context.value.clone();

    let (issue_date, invoice_hash) = submit_invoice_and_get_hash(&client, &seller_nip)
        .await
        .expect("Invoice submission failed or was rejected");

    let url = client.build_invoice_verification_url(&seller_nip, &issue_date, &invoice_hash);
    println!("Invoice verification URL: {}", url);

    assert!(
        url.contains(&format!("/invoice/{}/{}/", seller_nip, issue_date)),
        "Invoice URL missing expected segments"
    );
}

#[tokio::test]
async fn test_generate_certificate_qr_code() {
    let client: KsefClient = common::authorize_client().await;
    let seller_nip = client.context.value.clone();

    let (_issue_date, invoice_hash) = submit_invoice_and_get_hash(&client, &seller_nip)
        .await
        .expect("Invoice submission failed or was rejected");

    let (url, serial) = match enroll_and_build_cert_qr(&client, &seller_nip, &invoice_hash).await {
        Ok((url, serial)) => (url, serial),
        Err(e) => panic!("Failed to enroll and build certificate QR: {}", e),
    };

    println!("Certificate QR URL: {}", url);

    assert!(
        url.contains("/certificate/"),
        "Certificate QR URL should contain /certificate/"
    );

    client
        .revoke_certificate(&serial, RevocationReason::Unspecified)
        .await
        .expect("Failed to revoke certificate used in test");
}

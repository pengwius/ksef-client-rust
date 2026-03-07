use crate::common;

use ksef_client::OpenOnlineSessionRequestBuilder;

#[tokio::test]
async fn test_online_session_flow() {
    let client: ksef_client::KsefClient = common::authorize_client().await;

    let encryption_data = client
        .generate_encryption_data()
        .await
        .expect("Failed to generate encryption data");

    let request = OpenOnlineSessionRequestBuilder::new()
        .with_encryption(
            &encryption_data.encrypted_symmetric_key,
            &encryption_data.initialization_vector,
        )
        .build()
        .expect("Failed to build OpenOnlineSessionRequest");

    let response = client
        .open_online_session(request)
        .await
        .expect("Failed to open online session");

    println!("Opened online session: {:?}", response);

    assert!(
        !response.reference_number.is_empty(),
        "Session reference number should not be empty"
    );

    let issuer_nip = "5261234567";
    let invoice_xml: String = common::generate_fa2_invoice(issuer_nip).await;

    let send_result = client
        .send_invoice(
            &response.reference_number,
            invoice_xml.as_bytes(),
            &encryption_data,
        )
        .await;

    let invoice_reference_number = match send_result {
        Ok(resp) => resp.reference_number,
        Err(e) => {
            panic!("Invoice send failed: {:?}", e);
        }
    };

    let status = client
        .get_invoice_status(&response.reference_number, &invoice_reference_number)
        .await
        .expect("Failed to get invoice status");

    println!("Final Invoice status: {:#?}", status);

    assert!(status.invoice_status.code == 200);

    match client
        .close_online_session(&response.reference_number)
        .await
    {
        Ok(()) => {}
        Err(e) => {
            panic!("Failed to get session status after close: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_submit_online_automated() {
    let client: ksef_client::KsefClient = common::authorize_client().await;

    let issuer_nip = "5261234567";
    let invoice_xml: String = common::generate_fa2_invoice(issuer_nip).await;

    let result = client
        .submit_online(invoice_xml.as_bytes())
        .await
        .expect("Failed to submit online session");

    println!("Online submission result: {:?}", result);

    assert!(
        !result.session_reference_number.is_empty(),
        "Session reference number should not be empty"
    );
    assert!(
        !result.invoice_reference_number.is_empty(),
        "Invoice reference number should not be empty"
    );

    let status = client
        .get_invoice_status(
            &result.session_reference_number,
            &result.invoice_reference_number,
        )
        .await
        .expect("Failed to get invoice status");

    println!(
        "Final Invoice status for {}: {:#?}",
        result.invoice_reference_number, status
    );
    assert!(status.invoice_status.code == 200);
}

use crate::common;

use ksef_client::OpenOnlineSessionRequestBuilder;

#[test]
fn test_online_session_flow() {
    let client = common::authorize_client();

    let encryption_data = client
        .generate_encryption_data()
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
        .expect("Failed to open online session");

    println!("Opened online session: {:?}", response);

    assert!(
        !response.reference_number.is_empty(),
        "Session reference number should not be empty"
    );

    let issuer_nip = "5264567890";
    let invoice_xml = common::generate_fa2_invoice(issuer_nip);

    let send_result = client.send_invoice(
        &response.reference_number,
        invoice_xml.as_bytes(),
        &encryption_data,
    );

    let invoice_reference_number = match send_result {
        Ok(resp) => resp.reference_number,
        Err(e) => {
            panic!("Invoice send failed: {:?}", e);
        }
    };

    let status = client
        .get_invoice_status(&response.reference_number, &invoice_reference_number)
        .expect("Failed to get invoice status");

    println!("Final Invoice status: {:#?}", status);

    assert!(status.invoice_status.code == 200);

    match client.close_online_session(&response.reference_number) {
        Ok(()) => {}
        Err(e) => {
            panic!("Failed to get session status after close: {:?}", e);
        }
    }
}

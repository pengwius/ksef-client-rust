use crate::common;

use base64::{Engine as _, engine::general_purpose};
use ksef_client::{GetInvoiceUpoResponse, InvoiceIdentifier, KsefClient};
use openssl::sha::sha256;

#[tokio::test]
async fn integration_upo_by_invoice_reference() {
    let client: KsefClient = common::authorize_client().await;
    let seller_nip = client.context.value.clone();

    let invoice_xml: String = common::generate_fa2_invoice(&seller_nip).await;
    let submit_result = client
        .submit_online(invoice_xml.as_bytes())
        .await
        .expect("Failed to submit invoice");

    let status = client
        .get_invoice_status(
            &submit_result.session_reference_number,
            &submit_result.invoice_reference_number,
        )
        .await
        .expect("Failed to get invoice status");

    if status.invoice_status.code != 200 {
        panic!(
            "Invoice not accepted: {} ({})",
            status.invoice_status.code, status.invoice_status.description
        );
    }

    let invoice_ref = submit_result.invoice_reference_number.clone();

    let upo_resp: GetInvoiceUpoResponse = client
        .get_invoice_upo(
            &submit_result.session_reference_number,
            InvoiceIdentifier::InvoiceReference(invoice_ref.clone()),
        )
        .await
        .expect("Failed to fetch invoice UPO by invoice reference (get_invoice_upo)");

    assert!(
        !upo_resp.content.is_empty(),
        "UPO content should not be empty"
    );
    assert!(
        !upo_resp.hash.is_empty(),
        "x-ms-meta-hash header should be present"
    );

    let digest = sha256(upo_resp.content.as_bytes());
    let digest_b64 = general_purpose::STANDARD.encode(digest);
    assert_eq!(
        digest_b64, upo_resp.hash,
        "UPO hash mismatch: computed vs header"
    );

    println!(
        "Fetched UPO for invoice ref {} (session {}) — hash OK",
        invoice_ref, submit_result.session_reference_number
    );

    println!("UPO content: {:#?}", upo_resp.content);
}

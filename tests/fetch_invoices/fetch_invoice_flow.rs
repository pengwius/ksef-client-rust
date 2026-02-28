use crate::common;
use ksef_client::{
    DateRange, DateType, FetchInvoiceMetadataRequest, IncrementalFetchState, InvoicePayload,
    QueryCriteria, SubjectType, fetch_invoices_incrementally,
};
use std::io::Read;

#[tokio::test]
async fn test_fetch_invoice_flow() {
    let client = common::authorize_client().await;
    let nip = "5264567890";

    let online_session_invoice_xml: String = common::generate_fa2_invoice(nip).await;

    let online_session_result = client
        .submit_online(online_session_invoice_xml.as_bytes())
        .await
        .expect("Failed to submit online session");

    let online_session_status = client
        .get_invoice_status(
            &online_session_result.session_reference_number,
            &online_session_result.invoice_reference_number,
        )
        .await
        .expect("Failed to get invoice status");

    assert!(online_session_status.invoice_status.code == 200);

    let batch_session_invoice_xml_1: String = common::generate_fa2_invoice(nip).await;
    let batch_session_invoice_xml_2: String = common::generate_fa2_invoice(nip).await;

    let invoices = vec![
        InvoicePayload {
            filename: "invoice1.xml".to_string(),
            content: batch_session_invoice_xml_1.as_bytes().to_vec(),
        },
        InvoicePayload {
            filename: "invoice2.xml".to_string(),
            content: batch_session_invoice_xml_2.as_bytes().to_vec(),
        },
    ];

    let batch_session_result = client
        .submit_batch(&invoices, Some(10 * 1024 * 1024))
        .await
        .expect("Failed to submit batch");

    assert!(
        batch_session_result.number_of_parts > 0,
        "Should have at least 1 part"
    );

    let now = chrono::Utc::now();
    let start_of_day = now.date_naive().and_hms_opt(0, 0, 0).unwrap().and_utc();
    let from_date = start_of_day.to_rfc3339();
    let to_date = (now + chrono::Duration::hours(1)).to_rfc3339();

    let query_req = FetchInvoiceMetadataRequest {
        query: QueryCriteria {
            subject_type: SubjectType::Subject1,
            date_range: DateRange {
                date_type: DateType::Invoicing,
                from: from_date,
                to: Some(to_date),
                restrict_to_permanent_storage_hwm_date: None,
            },
            ksef_number: None,
            invoice_number: None,
            amount: None,
            seller_nip: None,
            buyer_identifier: None,
            currency_codes: None,
            invoicing_mode: None,
            is_self_invoicing: None,
            form_type: None,
            invoice_types: None,
            has_attachment: None,
        },
        page_offset: Some(0),
        page_size: Some(100),
    };

    let metadata_resp = client.fetch_invoice_metadata(query_req.clone()).await;

    let mut ksef_number = "".to_string();

    match metadata_resp {
        Ok(resp) => {
            if !resp.invoices.is_empty() {
                println!("Fetched {} invoices", resp.invoices.len());

                if let Some(ksef_number) = &online_session_status.ksef_number {
                    if let Some(found_invoice) = resp
                        .invoices
                        .iter()
                        .find(|inv| inv.ksef_number == *ksef_number)
                    {
                        println!("Found online invoice: {}", ksef_number);
                        assert_eq!(found_invoice.seller.nip, nip, "Seller NIP should match");
                    }
                }

                ksef_number = resp.invoices.last().unwrap().ksef_number.clone();
            }
        }
        Err(e) => {
            println!("Error fetching metadata: {:?}", e);
        }
    }

    let fetch_resp = client
        .fetch_invoice(&ksef_number)
        .await
        .expect("Failed to fetch invoice content");

    println!("Fetched invoice content size: {}", fetch_resp.content.len());
    println!("Fetched invoice hash: {}", fetch_resp.hash);

    assert!(
        !fetch_resp.content.is_empty(),
        "Invoice content should not be empty"
    );
    assert!(
        !fetch_resp.hash.is_empty(),
        "Invoice hash should not be empty"
    );

    let content_str = String::from_utf8_lossy(&fetch_resp.content);
    assert!(
        content_str.contains("Faktura"),
        "Content should contain 'Faktura' tag"
    );

    println!("Starting invoice export...");

    let export_result = client
        .export_invoices(query_req.query.clone())
        .await
        .expect("Failed to export invoices and wait for completion");

    let exported_parts = export_result.parts;
    println!("Exported {} parts", exported_parts.len());

    for part in exported_parts.iter() {
        println!("Processing part: {}", part.metadata.part_name);
        let cursor = std::io::Cursor::new(&part.content);
        let mut archive = zip::ZipArchive::new(cursor).expect("Failed to open zip archive");

        let mut file = archive.by_index(0).expect("Failed to read file in zip");

        let mut xml_content = String::new();
        file.read_to_string(&mut xml_content)
            .expect("Failed to read XML content from zip");

        assert!(
            xml_content.contains("</Faktura>"),
            "Exported XML content should contain 'Faktura' tag"
        );
    }

    assert!(
        !exported_parts.is_empty(),
        "Should have exported at least one part"
    );

    println!("Starting incremental invoice fetch...");

    let mut state = IncrementalFetchState::new();
    let subject_types = vec![SubjectType::Subject1];
    let default_start = chrono::Utc::now() - chrono::Duration::days(1);
    let window_end = Some(chrono::Utc::now() + chrono::Duration::days(1));

    let fetched_invoices = fetch_invoices_incrementally(
        &client,
        &mut state,
        subject_types,
        window_end,
        default_start,
    )
    .await
    .expect("Failed to fetch invoices incrementally");

    println!("Incrementally fetched {} invoices", fetched_invoices.len());

    for invoice in &fetched_invoices {
        println!(
            "Fetched invoice: {} (KSeF: {})",
            invoice.metadata.invoice_number, invoice.metadata.ksef_number
        );
    }

    assert!(
        !fetched_invoices.is_empty(),
        "Should have fetched at least one invoice incrementally"
    );
}

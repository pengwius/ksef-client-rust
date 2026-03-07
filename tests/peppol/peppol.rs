use crate::common;

#[tokio::test]
async fn test_peppol_query() {
    let client = common::authorize_client().await;
    let resp = client
        .get_peppol_providers(Some(10), Some(0))
        .await
        .expect("Peppol providers request failed");

    assert!(resp.peppol_providers.len() >= 2);

    println!("Peppol providers: {:#?}", resp.peppol_providers);

    if let Some(p) = resp.peppol_providers.first() {
        assert!(!p.id.is_empty());
        assert!(!p.name.is_empty());
    }
}

[Polska wersja](peppol.md) / English version

# Fetching the list of Peppol service providers

__[Official Documentation](https://api-test.ksef.mf.gov.pl/docs/v2/index.html#tag/Peppol-Providers)__

```rust
let client = common::authorize_client().await;
let resp = client
    .get_peppol_providers(Some(10), Some(0))
    .await
    .expect("Peppol providers request failed");

println!("Peppol providers: {:#?}", resp.peppol_providers);
```

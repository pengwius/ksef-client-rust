Polska Wersja / [English version](peppol.en.md)

# Pobieranie listy dostawców usług Peppol

__[Oficjalna Dokumentacja](https://api-test.ksef.mf.gov.pl/docs/v2/index.html#tag/Uslugi-Peppol)__

```rust
let client = common::authorize_client().await;
let resp = client
    .get_peppol_providers(Some(10), Some(0))
    .await
    .expect("Peppol providers request failed");

println!("Peppol providers: {:#?}", resp.peppol_providers);
```

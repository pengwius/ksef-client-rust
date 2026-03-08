Polska Wersja / [English version](upo.en.md)

# Urzędowe Poświadczenie Odbioru (UPO) Faktur

__[Oficjalna Dokumentacja](https://api-test.ksef.mf.gov.pl/docs/v2/index.html#tag/Status-wysylki-i-UPO/paths/~1sessions~1%7BreferenceNumber%7D~1invoices~1ksef~1%7BksefNumber%7D~1upo/get)__

### Pobieranie UPO za pomocą numeru KSeF

```rust
let upo: GetInvoiceUpoResponse = client
    .get_invoice_upo(
        &session_reference_number, // numer referencyjny sesji
        InvoiceIdentifier::KsefNumber(ksef_number.clone()), // numer KSeF faktury
    )
    .await
    .expect("Failed to fetch invoice UPO by KSeF number");
```

### Pobieranie UPO za pomocą numeru referencyjnego

```rust
let upo: GetInvoiceUpoResponse = client
    .get_invoice_upo(
        &session_reference_number, // numer referencyjny sesji
        InvoiceIdentifier::InvoiceReference(invoice_ref.clone()), // numer referencyjny faktury
    )
    .await
    .expect("Failed to fetch invoice UPO by invoice reference");
```

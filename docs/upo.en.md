[Polska Wersja](upo.md) / English version

# Official Receipt Acknowledgment (UPO) for Invoices

__[Official Documentation](https://api-test.ksef.mf.gov.pl/docs/v2/index.html#tag/Status-wysylki-i-UPO/paths/~1sessions~1%7BreferenceNumber%7D~1invoices~1ksef~1%7BksefNumber%7D~1upo/get)__

### Fetching UPO by KSeF Number

```rust
let upo: GetInvoiceUpoResponse = client
    .get_invoice_upo(
        &session_reference_number, // session reference number 
        InvoiceIdentifier::KsefNumber(ksef_number.clone()), // invoice KSeF number
    )
    .await
    .expect("Failed to fetch invoice UPO by KSeF number");
```

### Fetching UPO by Invoice Reference

```rust
let upo: GetInvoiceUpoResponse = client
    .get_invoice_upo(
        &session_reference_number, // session reference number 
        InvoiceIdentifier::InvoiceReference(invoice_ref.clone()), // invoice reference number 
    )
    .await
    .expect("Failed to fetch invoice UPO by invoice reference");
```

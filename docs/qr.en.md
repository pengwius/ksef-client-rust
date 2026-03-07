[Polska wersja](qr.md) / English version

# QR Codes

__[Official Documentation](https://github.com/CIRFMF/ksef-docs/blob/main/kody-qr.md)__

The helpers for generating QR codes both for invoices (KOD I) and for certificates (KOD II) return a URL that can be used to display a QR code in a client application.

### Generating an invoice QR (KOD I)

You need:
- seller NIP — the seller's NIP
- issue date — the invoice issue date in DD-MM-YYYY format
- invoice hash — the invoice hash (Base64). You get this hash from the KSeF response after a successful invoice submission or by fetching the invoice

```rust
use ksef_client::KsefClient;
use ksef_client::{ContextIdentifier, ContextIdentifierType, Environment};

let context = ContextIdentifier {
    id_type: ContextIdentifierType::Nip,
    value: "5261234567".to_string(), // your NIP
};
    
let client = KsefClient::new(Environment::Test, context);

let seller_nip = "5261234567";
let issue_date = "07-03-2026";
let invoice_hash_b64 = "s8N6RaQsc8Fd19z...";

let url = client.build_invoice_verification_url(seller_nip, issue_date, invoice_hash_b64);
println!("Invoice QR URL: {}", url);
```

### Generating a certificate QR (KOD II) — from a KSeF certificate

You need:
- seller_nip — the seller's NIP
- cert_serial — the certificate serial number issued by KSeF; you receive it after enroll/issuing the certificate (in the server response or in PKCS#12 metadata)
- invoice_hash_b64url — the invoice hash (same as above)
- private_key_pem (optional) — if you want to sign the URL locally (KOD II includes a signature in the URL). You have the private key when:
  - you generated a CSR and kept the private key
  - or you have a PKCS#12 from KSeF/CA containing the private key

```rust
let context_type = "Nip";
let context_value = "5261234567";
let seller_nip = "5261234567";

let cert_serial = "01F20A5D352AE590";

let invoice_hash_b64url = "UtQp9Gpc51y-u3xApZjIjgkpZ01js-J8KflSPW8WzIE";

let private_key_pem: Option<&str> = Some("-----BEGIN PRIVATE KEY-----\n...\n-----END PRIVATE KEY-----");

let qr_url = client
    .build_certificate_verification_url(
        context_type,
        context_value,
        seller_nip,
        cert_serial,
        invoice_hash_b64url,
        private_key_pem,
    )
    .expect("failed to build certificate QR URL");

println!("Certificate QR URL: {}", qr_url);
```

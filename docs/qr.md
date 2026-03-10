Polska Wersja / [English version](qr.en.md)

# Kody QR

__[Oficjalna Dokumentacja](https://github.com/CIRFMF/ksef-docs/blob/main/kody-qr.md)__

Metody do generowania kodów QR zarówno dla faktur (KOD I) jak i dla certyfikatów (KOD II) zwracają URL, który można wykorzystać do wyświetlenia kodu QR w aplikacji klienckiej.

### Generowanie kodu QR dla faktury (KOD I)

Potrzebujesz:
- seller NIP — NIP sprzedawcy
- issue date — data wystawienia faktury w formacie DD-MM-YYYY
- invoice hash — skrót faktury (Base64). Ten hash otrzymujesz z odpowiedzi KSeF po poprawnym przesłaniu faktury lub pobierając fakturę

```rust
use ksef_client::prelude::{KsefClient, ContextIdentifier, ContextIdentifierType, Environment};

let context = ContextIdentifier {
    id_type: ContextIdentifierType::Nip,
    value: "5261234567".to_string(), // twój NIP
};
    
let client = KsefClient::new(Environment::Test, context);

let seller_nip = "5261234567";
let issue_date = "07-03-2026";
let invoice_hash_b64 = "s8N6RaQsc8Fd19z...";

let url = client.build_invoice_verification_url(seller_nip, issue_date, invoice_hash_b64);
println!("Invoice QR URL: {}", url);
```

### Generowanie kodu QR dla certyfikatu (KOD II) — z certyfikatu KSeF

Potrzebujesz:
- seller_nip — NIP sprzedawcy
- cert_serial — numer seryjny certyfikatu wydanego przez KSeF; otrzymujesz go po enroll/wyissuing certyfikatu (w odpowiedzi serwera lub z metadanych PKCS#12)
- invoice_hash_b64url — skrót faktury (tak jak wyżej)
- private_key_pem (opcjonalnie) — jeśli chcesz podpisać URL lokalnie (w KOD II sygnatura jest częścią URL) Private key możesz mieć gdy:
  - wygenerowałeś/aś CSR i zachowałeś/aś klucz prywatny
  - lub masz PKCS#12 od KSeF/CA zawierający prywatny klucz

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

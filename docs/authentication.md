Polska wersja / [English version](authentication.en.md)

# Uwierzytelnianie

Uwierzytelnianie może odbyć się za pomocą podpisu XAdES lub tokena KSeF. Token KSeF można wygenerować po pierwszym logowaniu podpisem kwalifikowanym XAdES. Do celów testowych można użyć certyfikatu samopodpisanego wygenerowanego za pomocą narzędzia [certgen](https://github.com/pengwius/ksef-client-rust/tree/main/certgen) lub metody `gen_selfsign_cert` w obiekcie `xades` w strukturze `KsefClient`.

### 1. Przygotowanie dokumentu XML

Należy przygotować dokument XML z identyfikatorem kontekstu, sposobem identyfikacji oraz podmiotu wywołując metodę `get_auth_token_request`.

| Parametr | Typ | Opis |
|---------|------|------|
| `id` | `&str` | Identyfikator kontekstu (np. NIP, identyfikator wewnętrzny lub identyfikator złożony VAT EU), przekazany jako `&str`. |
| `id_type` | `ContextIdentifierType` | Sposób identyfikacji kontekstu. Może przyjąć `ContextIdentifierType::Nip`, `ContextIdentifierType::InternalId` lub `ContextIdentifierType::NipVatUe`. |
| `subject_type` | `SubjectIdentifierType` | Sposób identyfikacji podmiotu. Może przyjąć `SubjectIdentifierType::CertificateSubject` lub `SubjectIdentifierType::CertificateFingerprint`. |

```rust
use ksef_client::{KsefClient, ContextIdentifierType, SubjectIdentifierType};

// Inicjalizacja klienta
let mut client = KsefClient::new();

// Budowa obiektu żądania 
let auth_request = client.get_auth_token_request(
    "1234567890",                             // NIP kontekstu
    ContextIdentifierType::Nip,               // Typ identyfikatora kontekstu
    SubjectIdentifierType::CertificateSubject // Logowanie certyfikatem
)?;

// Serializacja do XML
let unsigned_xml = auth_request.to_xml();
```

### 2. Podpisanie dokumentu (XAdES)

Przygotowany XML musi zostać podpisany formatem XAdES. W środowisku **Testowym** dopuszczalne jest użycie certyfikatów samopodpisanych (self-signed). W środowisku **Produkcyjnym** wymagany jest podpis kwalifikowany lub pieczęć.

**Opcja A: Certyfikat testowy (Self-signed)**
Biblioteka potrafi wygenerować taki certyfikat w locie dla celów testowych. 

```rust
let nip = "1234567890";
let given_name = "Jan";
let surname = "Kowalski";
let serial_prefix = "TINPL";
let common_name = "Jan Kowalski Sp. z o.o.";

client
    .xades
    .gen_selfsign_cert(given_name, surname, serial_prefix, nip, common_name)
    .expect("Failed to generate self-signed certificate");
```

**Opcja B: Własny certyfikat (PKCS#12)**
Można załadować istniejący plik `.p12` / `.pfx`.
```rust
let p12_data = std::fs::read("certyfikat.p12")?;
client.xades.load_pkcs12(&p12_data, "haslo_do_pliku")?; // hasło puste, jeżeli brak
```

**Złożenie podpisu:**
```rust
let signed_xml = match client.xades.sign(&unsigned_xml) {
    Ok(s) => s,
    Err(e) => {
        eprintln!("Unable to sign XML: {}", e);
        // obsługa błędu
        return;
    }
};
```

### 3. Wysłanie podpisanego XML
Podpisany dokument wysyłamy do API.
```rust
match client.authenticate_by_xades_signature(signed_xml) {
    Ok(()) => {}
    Err(e) => {
        eprintln!("Unable to submit signed XML for authentication: {}", e);
        return;
    }
};
``` 

W odpowiedzi otrzymujemy `authentication_token` oraz `reference_number`, które są przechowywane w stanie klienta i mogą być pobrane przez `client.auth_token()`.

### 4. Autoryzacja tokenem KSeF
Po pierwszym logowaniu podpisem XAdES można wygenerować długoterminowy token KSeF, który pozwala na logowanie bez konieczności podpisywania XML. Tokeny KSeF są udokumentowane w pliku [ksef_tokens.md](ksef_tokens.md).

**Generowanie nowego tokena (wymaga zalogowania przez XAdES):**
```rust
// Parametr load określa, czy wygenerowany token ma być automatycznie załadowany do stanu klienta (self.ksef_token). Funkcja też zawsze zwraca wygenerowany token.
let ksef_token = match client.new_ksef_token(true) {
    Ok(token) => {
        println!("    KSeF Token: {:?}", token.token);
        token
    }
    Err(e) => {
        eprintln!("Unable to get KSeF token: {}", e);
        return;
    }
};
```

**Logowanie przy użyciu istniejącego tokena:**

Jeżeli posiadamy już wygenerowany token:

```rust
// np. pobranie tokena z bazy danych
let my_token_str = "......"; 
let my_token = KsefToken {
    token: my_token_str.to_string(),
    reference_number: "....".to_string(),
    ..Default::default()
};

// Załaduj token do stanu klienta
client.load_ksef_token(my_token);

match client.authenticate_by_ksef_token() {
    Ok(()) => {
         let auth_token = client.auth_token();
         println!("    Auth Token: {}", auth_token.authentication_token);
    }
    Err(e) => {
        eprintln!("Unable to authenticate with KSeF token: {}", e);
    }
}
```

### 5. Sprawdzenie statusu
Proces weryfikacji podpisu jest asynchroniczny. Metoda `get_auth_status` blokuje wykonanie i odpytuje API w pętli (zaimplementowanej wewnętrznie), dopóki status nie zmieni się na pozytywny lub wystąpi błąd/timeout.

```rust
 let is_authenticated: bool = match client.get_auth_status() {
    Ok(status) => status,
    Err(e) => {
        eprintln!("Unable to get authentication status: {}", e);
        false
    }
};

if is_authenticated {
    println!("    Status: Authentication completed successfully.");
    // Po sukcesie access_token jest automatycznie pobierany i zapisywany w kliencie
} else {
    println!("    Status: Authentication failed.");
}
```

### Pełny przykład kodu

Poniżej znajduje się kompletny kod realizujący scenariusz:
1. Logowanie za pomocą certyfikatu XAdES (self-signed).
2. Wygenerowanie tokena KSeF.
3. Wylogowanie (nowa instancja klienta).
4. Logowanie za pomocą wygenerowanego tokena KSeF.

```rust
use ksef_client::{
    ContextIdentifierType, KsefClient, SubjectIdentifierType, KsefToken
};
use std::time::Duration;
use std::thread;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // === CZEŚĆ 1: LOGOWANIE XADES ===
    println!("--- 1. Logowanie XAdES ---");
    let mut client = KsefClient::new(); // Domyślnie środowisko testowe
    let nip = "1234567890";

    // 1. Generowanie certyfikatu self-signed (tylko dla testów!)
    client.xades.gen_selfsign_cert(
        "Jan", "Kowalski", "TINPL", nip, "Jan Kowalski Test"
    )?;

    // 2. Przygotowanie requestu autoryzacyjnego
    let auth_request = client.get_auth_token_request(
        nip,
        ContextIdentifierType::Nip,
        SubjectIdentifierType::CertificateSubject
    )?;
    let unsigned_xml = auth_request.to_xml();

    // 3. Podpisanie requestu
    let signed_xml = client.xades.sign(&unsigned_xml)?;

    // 4. Wysłanie signed XML
    client.authenticate_by_xades_signature(signed_xml)?;
    println!("Wysłano żądanie logowania. Reference number: {}", client.auth_token().reference_number);

    // 5. Czekanie na zakończenie procesu logowania (get_auth_status odpytuje cyklicznie)
    if client.get_auth_status()? {
        println!("Zalogowano pomyślnie przez XAdES.");
        println!("Access Token: {}", client.access_token().access_token);
    } else {
        println!("Błąd logowania.");
        return Ok(());
    }

    // === CZEŚĆ 2: GENEROWANIE TOKENA KSEF ===
    println!("\n--- 2. Generowanie Tokena KSeF ---");
    // Generujemy token i ładujemy go od razu do klienta
    let new_token = client.new_ksef_token(true)?;
    println!("Wygenerowano nowy token KSeF: {}", new_token.token);
    println!("Reference number tokena: {}", new_token.reference_number);

    // Tutaj normalnie zapisalibyśmy token do bazy danych
    let saved_token_string = new_token.token.clone();
    let saved_token_ref = new_token.reference_number.clone();


    // === CZEŚĆ 3: LOGOWANIE TOKENEM KSEF ===
    println!("\n--- 3. Logowanie Tokenem KSeF (nowa sesja) ---");
    
    // Symulujemy nową sesję - nowa instancja klienta
    let mut client2 = KsefClient::new();

    // Odtwarzamy obiekt tokena (normalnie z bazy danych)
    let ksef_token = KsefToken {
        token: saved_token_string,
        reference_number: saved_token_ref,
        ..Default::default()
    };

    // Ładujemy token
    client2.load_ksef_token(ksef_token);

    // Logujemy się tokenem
    client2.authenticate_by_ksef_token()?;
    println!("Wysłano żądanie logowania tokenem. Reference number sesji: {}", client2.auth_token().reference_number);

    // Sprawdzamy status
    if client2.get_auth_status()? {
        println!("Zalogowano pomyślnie przez Token KSeF.");
        println!("Access Token (sesja z tokena): {}", client2.access_token().access_token);
    } else {
        println!("Błąd logowania tokenem.");
    }

    Ok(())
}
```

Polska Wersja / [English version](ksef_tokens.en.md)

# Tokeny KSeF
Token KSeF to unikalny, generowany identyfikator uwierzytelniający, który — na równi z kwalifikowanym podpisem elektronicznym — umożliwia uwierzytelnienie się do API KSeF.

Token KSeF jest wydawany z niezmiennym zestawem uprawnień określonych przy jego tworzeniu; każda modyfikacja tych uprawnień wymaga wygenerowania nowego tokena.

__[Oficjalna Dokumentacja](https://github.com/CIRFMF/ksef-docs/blob/main/tokeny-ksef.md)__

## Zarządzanie Tokenami KSeF

Wszystkie poniższe operacje (generowanie, pobieranie listy, unieważnianie) wymagają wcześniejszego uwierzytelnienia się (np. certyfikatem XAdES) z odpowiednimi uprawnieniami (zazwyczaj `CredentialsManage`).

### 1. Generowanie Tokena KSeF

```rust
use ksef_client::KsefTokenPermissions;

let permissions = KsefTokenPermissions {
    invoice_read: true,
    invoice_write: true,
    credentials_read: true,
    credentials_manage: false,
    subunit_manage: false,
    enforcement_operations: false,
};

let description = "Opis mojego tokena";

// Parametr 'true' oznacza, że wygenerowany token zostanie automatycznie załadowany do klienta,
// umożliwiając późniejsze logowanie się nim (w nowej sesji) lub wykonywanie operacji w jego kontekście.
let ksef_token = match client.new_ksef_token(true, permissions, description) {
    Ok(token) => {
        println!("    KSeF Token: {}", token.token);
        token
    }
    Err(e) => {
        eprintln!("Unable to get KSeF token: {}", e);
        return;
    }
};
```

### 2. Logowanie przy użyciu Tokena KSeF

Aby zalogować się tokenem, musi on być załadowany do klienta (metodą `load_ksef_token` lub automatycznie przy generowaniu).

```rust
match client.authenticate_by_ksef_token() {
    Ok(()) => {
        println!("    Authentication request sent successfully!");
        let auth_token = client.auth_token();
        println!("    Auth Token Ref: {}", auth_token.reference_number);
        
        // Należy pamiętać o sprawdzeniu statusu (get_auth_status), 
        // aby uzyskać access_token.
    }
    Err(e) => {
        eprintln!("Unable to authenticate with KSeF token: {}", e);
        return;
    }
}
```

### 3. Pobieranie listy wygenerowanych tokenów

Zwraca listę tokenów powiązanych z podmiotem.

```rust
match client.get_ksef_tokens() {
    Ok(tokens) => {
        println!("Znaleziono {} tokenów.", tokens.len());
        for t in tokens {
             println!(" - Ref: {}, Opis: {}", t.reference_number, t.description);
        }
    }
    Err(e) => {
        eprintln!("Unable to get list of KSeF tokens: {}", e);
        return;
    }
};
```

### 4. Pobieranie szczegółów konkretnego tokena

```rust
let ksef_token_reference_number = &ksef_token.reference_number;

match client.get_ksef_token_status(ksef_token_reference_number.as_str()) {
    Ok(token_status) => {
        println!(
            "    KSeF Token Status\n{}",
            serde_json::to_string_pretty(&token_status).unwrap_or_default()
        );
    }
    Err(e) => {
        eprintln!("Unable to get KSeF token status: {}", e);
        return;
    }
};
```

### 5. Unieważnianie tokena KSeF

Tokeny, które nie są już potrzebne lub zostały skompromitowane, powinny zostać unieważnione.

```rust
let ksef_token_reference_number = &ksef_token.reference_number;

match client.revoke_ksef_token(ksef_token_reference_number.as_str()) {
    Ok(()) => {
        println!("    KSeF token revoked successfully.");
    }
    Err(e) => {
        eprintln!("Unable to revoke KSeF token: {}", e);
        return;
    }
};
```

### Pełny przykład cyklu życia tokena

Poniższy kod zakłada, że klient (`client`) jest już zalogowany (np. certyfikatem XAdES) i posiada uprawnienia do zarządzania poświadczeniami (`CredentialsManage`).

```rust
use ksef_client::{KsefClient, KsefTokenPermissions};

fn token_lifecycle_example(client: &mut KsefClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Rozpoczęcie cyklu życia tokena ---");

    // 1. Generowanie nowego tokena
    println!("\n1. Generowanie nowego tokena...");

    let permissions = KsefTokenPermissions {
        invoice_read: true,
        invoice_write: true,
        credentials_read: true,
        credentials_manage: false,
        subunit_manage: false,
        enforcement_operations: false,
    };

    let new_token = client.new_ksef_token(false, permissions, "Token testowy cyklu życia")?; // false - nie ładujemy go jako aktywnego tokena autoryzacyjnego tej sesji
    println!("   Wygenerowano token: {}", new_token.token);
    println!("   Numer referencyjny: {}", new_token.reference_number);

    // 2. Sprawdzenie statusu nowo utworzonego tokena
    println!("\n2. Pobieranie statusu tokena...");
    let status = client.get_ksef_token_status(&new_token.reference_number)?;
    println!("   Status tokena: aktywny? {}", status.active);
    println!("   Opis: {}", status.description);

    // 3. Pobranie listy wszystkich tokenów
    println!("\n3. Lista wszystkich tokenów...");
    let tokens_list = client.get_ksef_tokens()?;
    println!("   Liczba tokenów w systemie: {}", tokens_list.len());
    
    let exists = tokens_list.iter().any(|t| t.reference_number == new_token.reference_number);
    println!("   Czy nowy token jest na liście? {}", exists);

    // 4. Unieważnienie tokena
    println!("\n4. Unieważnianie tokena...");
    client.revoke_ksef_token(&new_token.reference_number)?;
    println!("   Token unieważniony.");

    // 5. Weryfikacja unieważnienia (opcjonalnie)
    // Uwaga: Aktualizacja statusu w systemie KSeF może zająć chwilę.
    println!("\n5. Weryfikacja statusu po unieważnieniu...");
    let final_status = client.get_ksef_token_status(&new_token.reference_number)?;
    println!("   Status tokena: aktywny? {}", final_status.active);

    println!("\n--- Zakończono cykl życia tokena ---");
    Ok(())
}
```

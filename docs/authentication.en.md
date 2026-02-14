[Polska wersja](authentication.md) / English version

# Authentication

Authentication can be performed using an XAdES signature or a KSeF token. A KSeF token can be generated after the first login with a qualified XAdES signature. for testing purposes, you can use a self-signed certificate generated with the [certgen](https://github.com/pengwius/ksef-client-rust/tree/main/certgen) tool or the `gen_selfsign_cert` method in the `xades` object of the `KsefClient` structure.

__[Official Documentation](https://github.com/CIRFMF/ksef-docs/blob/main/uwierzytelnianie.md)__

### 1. Preparing the XML Document

You must prepare an XML document with the context identifier, identification method, and entity type by calling the `get_auth_token_request` method.

| Parameter | Type | Description |
|---------|------|------|
| `id` | `&str` | Context identifier (e.g., NIP, internal identifier, or composite VAT EU identifier), passed as `&str`. |
| `id_type` | `ContextIdentifierType` | Context identification method. Can be `ContextIdentifierType::Nip`, `ContextIdentifierType::InternalId`, or `ContextIdentifierType::NipVatUe`. |
| `subject_type` | `SubjectIdentifierType` | Entity identification method. Can be `SubjectIdentifierType::CertificateSubject` or `SubjectIdentifierType::CertificateFingerprint`. |

```rust
use ksef_client::{KsefClient, ContextIdentifierType, SubjectIdentifierType};

// Initialize client
let mut client = KsefClient::new();

// Build request object
let auth_request = client.get_auth_token_request(
    "1234567890",                             // Context NIP
    ContextIdentifierType::Nip,               // Context identifier type
    SubjectIdentifierType::CertificateSubject // Login with certificate
)?;

// Serialize to XML
let unsigned_xml = auth_request.to_xml();
```

### 2. Signing the Document (XAdES)

The prepared XML must be signed in XAdES format. In the **Test** environment, self-signed certificates are allowed. In the **Production** environment, a qualified certificate or seal is required.

**Option A: Test Certificate (Self-signed)**
The library can generate such a certificate on the fly for testing purposes.

```rust
let nip = "1234567890";
let given_name = "John";
let surname = "Doe";
let serial_prefix = "TINPL";
let common_name = "John Doe Sp. z o.o.";

client
    .xades
    .gen_selfsign_cert(given_name, surname, serial_prefix, nip, common_name)
    .expect("Failed to generate self-signed certificate");
```

**Option B: Custom Certificate (PKCS#12)**
You can load an existing `.p12` / `.pfx` file.
```rust
let p12_data = std::fs::read("certificate.p12")?;
client.xades.load_pkcs12(&p12_data, "file_password")?; // empty password if none
```

**Signing:**
```rust
let signed_xml = match client.xades.sign(&unsigned_xml) {
    Ok(s) => s,
    Err(e) => {
        eprintln!("Unable to sign XML: {}", e);
        // handle error
        return;
    }
};
```

### 3. Sending the Signed XML
The signed document is sent to the API.
```rust
match client.authenticate_by_xades_signature(signed_xml) {
    Ok(()) => {}
    Err(e) => {
        eprintln!("Unable to submit signed XML for authentication: {}", e);
        return;
    }
};
``` 

In response, we receive `authentication_token` and `reference_number`, which are stored in the client's state and can be retrieved via `client.auth_token()`.

### 4. Authorization with KSeF Token
After the first login with an XAdES signature, you can generate a long-term KSeF token, which allows logging in without signing XML. KSeF tokens are documented in [ksef_tokens.md](ksef_tokens.en.md).

**Generating a new token (requires XAdES login):**
```rust
// The load parameter determines if the generated token should be automatically loaded into the client state (self.ksef_token). The function always returns the generated token.
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

**Login using an existing token:**

If you already have a generated token:

```rust
// e.g., retrieve token from database
let my_token_str = "......"; 
let my_token = KsefToken {
    token: my_token_str.to_string(),
    reference_number: "....".to_string(),
    ..Default::default()
};

// Load token into client state
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

### 5. Checking Status
The signature verification process is asynchronous. The `get_auth_status` method blocks execution and polls the API in a loop (implemented internally) until the status changes to positive or an error/timeout occurs.

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
    // Upon success, access_token is automatically retrieved and saved in the client
} else {
    println!("    Status: Authentication failed.");
}
```

### Full Code Example

Below is a complete code implementing the scenario:
1. Login using an XAdES certificate (self-signed).
2. Generating a KSeF token.
3. Logout (new client instance).
4. Login using the generated KSeF token.

```rust
use ksef_client::{
    ContextIdentifierType, KsefClient, SubjectIdentifierType, KsefToken
};
use std::time::Duration;
use std::thread;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // === PART 1: XADES LOGIN ===
    println!("--- 1. XAdES Login ---");
    let mut client = KsefClient::new(); // Default test environment
    let nip = "1234567890";

    // 1. Generate self-signed certificate (only for testing!)
    client.xades.gen_selfsign_cert(
        "John", "Doe", "TINPL", nip, "John Doe Test"
    )?;

    // 2. Prepare auth request
    let auth_request = client.get_auth_token_request(
        nip,
        ContextIdentifierType::Nip,
        SubjectIdentifierType::CertificateSubject
    )?;
    let unsigned_xml = auth_request.to_xml();

    // 3. Sign request
    let signed_xml = client.xades.sign(&unsigned_xml)?;

    // 4. Send signed XML
    client.authenticate_by_xades_signature(signed_xml)?;
    println!("Login request sent. Reference number: {}", client.auth_token().reference_number);

    // 5. Wait for login completion (get_auth_status polls cyclically)
    if client.get_auth_status()? {
        println!("Successfully logged in via XAdES.");
        println!("Access Token: {}", client.access_token().access_token);
    } else {
        println!("Login failed.");
        return Ok(());
    }

    // === PART 2: GENERATE KSEF TOKEN ===
    println!("\n--- 2. Generate KSeF Token ---");
    // Generate token and load it into client immediately
    let new_token = client.new_ksef_token(true)?;
    println!("Generated new KSeF token: {}", new_token.token);
    println!("Token reference number: {}", new_token.reference_number);

    // Here you would normally save the token to a database
    let saved_token_string = new_token.token.clone();
    let saved_token_ref = new_token.reference_number.clone();


    // === PART 3: KSEF TOKEN LOGIN ===
    println!("\n--- 3. KSeF Token Login (new session) ---");
    
    // Simulate new session - new client instance
    let mut client2 = KsefClient::new();

    // Reconstruct token object (normally from database)
    let ksef_token = KsefToken {
        token: saved_token_string,
        reference_number: saved_token_ref,
        ..Default::default()
    };

    // Load token
    client2.load_ksef_token(ksef_token);

    // Login with token
    client2.authenticate_by_ksef_token()?;
    println!("Token login request sent. Session reference number: {}", client2.auth_token().reference_number);

    // Check status
    if client2.get_auth_status()? {
        println!("Successfully logged in via KSeF Token.");
        println!("Access Token (token session): {}", client2.access_token().access_token);
    } else {
        println!("Token login failed.");
    }

    Ok(())
}
```

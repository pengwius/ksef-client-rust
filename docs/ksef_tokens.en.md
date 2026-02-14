[Polska Wersja](ksef_tokens.md) / English version

# KSeF Tokens
A KSeF Token is a unique, generated authentication identifier which—on par with a qualified electronic signature—allows authentication to the KSeF API.

A KSeF token is issued with an immutable set of permissions defined at its creation; any modification of these permissions requires generating a new token.

__[Official Documentation](https://github.com/CIRFMF/ksef-docs/blob/main/tokeny-ksef.md)__

## Managing KSeF Tokens

All operations below (generating, listing, revoking) require prior authentication (e.g., using an XAdES certificate) with appropriate permissions (usually `CredentialsManage`).

### 1. Generating a KSeF Token

```rust
// The 'true' parameter means the generated token will be automatically loaded into the client,
// enabling subsequent login with it (in a new session) or performing operations in its context.
let ksef_token = match client.new_ksef_token(true) {
    Ok(token) => {
        println!("    KSeF Token: {}", token.token);
        println!("    Reference Number: {}", token.reference_number);
        token
    }
    Err(e) => {
        eprintln!("Unable to get KSeF token: {}", e);
        return;
    }
};
```

### 2. Logging in with a KSeF Token

To log in with a token, it must be loaded into the client (via the `load_ksef_token` method or automatically upon generation).

```rust
match client.authenticate_by_ksef_token() {
    Ok(()) => {
        println!("    Authentication request sent successfully!");
        let auth_token = client.auth_token();
        println!("    Auth Token Ref: {}", auth_token.reference_number);
        
        // Remember to check the status (get_auth_status), 
        // to obtain the access_token.
    }
    Err(e) => {
        eprintln!("Unable to authenticate with KSeF token: {}", e);
        return;
    }
}
```

### 3. Retrieving a list of generated tokens

Returns a list of tokens associated with the entity.

```rust
match client.get_ksef_tokens() {
    Ok(tokens) => {
        println!("Found {} tokens.", tokens.len());
        for t in tokens {
             println!(" - Ref: {}, Description: {}", t.reference_number, t.description);
        }
    }
    Err(e) => {
        eprintln!("Unable to get list of KSeF tokens: {}", e);
        return;
    }
};
```

### 4. Retrieving details of a specific token

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

### 5. Revoking a KSeF token

Tokens that are no longer needed or have been compromised should be revoked.

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

### Full Token Lifecycle Example

The code below assumes that the client (`client`) is already logged in (e.g., using an XAdES certificate) and possesses permissions to manage credentials (`CredentialsManage`).

```rust
use ksef_client::KsefClient;

fn token_lifecycle_example(client: &mut KsefClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Starting Token Lifecycle ---");

    // 1. Generate new token
    println!("\n1. Generating new token...");
    let new_token = client.new_ksef_token(false)?; // false - do not load as active auth token for this session
    println!("   Generated token: {}", new_token.token);
    println!("   Reference number: {}", new_token.reference_number);

    // 2. Check status of newly created token
    println!("\n2. Getting token status...");
    let status = client.get_ksef_token_status(&new_token.reference_number)?;
    println!("   Token status: active? {}", status.active);
    println!("   Description: {}", status.description);

    // 3. List all tokens
    println!("\n3. Listing all tokens...");
    let tokens_list = client.get_ksef_tokens()?;
    println!("   Number of tokens in system: {}", tokens_list.len());
    
    let exists = tokens_list.iter().any(|t| t.reference_number == new_token.reference_number);
    println!("   Is new token on the list? {}", exists);

    // 4. Revoke token
    println!("\n4. Revoking token...");
    client.revoke_ksef_token(&new_token.reference_number)?;
    println!("   Token revoked.");

    // 5. Verify revocation (optional)
    // Note: Status update in KSeF system might take a moment.
    println!("\n5. Verifying status after revocation...");
    let final_status = client.get_ksef_token_status(&new_token.reference_number)?;
    println!("   Token status: active? {}", final_status.active);

    println!("\n--- Token Lifecycle Completed ---");
    Ok(())
}
```

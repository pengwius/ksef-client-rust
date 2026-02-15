[Polska wersja](sessions.md) / English Version

# Session management

__[Official Documentation](https://github.com/CIRFMF/ksef-docs/blob/main/auth/sesje.md)__

### 1. Retrieving active sessions

The function `get_active_sessions` returns a list of active sessions. It accepts an optional `continuation_token` parameter that enables paginating results when there are many sessions. The `continuation_token` is included in the `QuerySessionsResponse` object returned by the function and can be used to fetch the next page of results. If you do not have a continuation token, call `get_active_sessions(None)` to fetch the first page of results.

```rust
match client.get_active_sessions(None) {
    Ok(resp) => {
        let sessions = resp.items;
        println!("Retrieved {} active sessions", sessions.len());
    }
    Err(e) => {
        panic!("Failed to retrieve active sessions: {:?}", e);
    }
}
```

### 2. Revoking the current session

The function `revoke_current_session` revokes the current session — i.e. the session associated with the token used to call this method. After it completes, the linked refresh token becomes invalid; active access tokens remain valid until they expire.

```rust
match client.revoke_current_session() {
    Ok(()) => println!("Current session revoked successfully"),
    Err(e) => panic!("Failed to revoke current session: {:?}", e),
}
```

### 3. Revoking a selected session

The function `revoke_session` revokes the specified session using its reference number (`reference_number`). You can obtain the reference number from the list of active sessions (`Session.reference_number`). Executing this operation may require appropriate permissions.

```rust
match client.revoke_session("session_reference_number") {
    Ok(()) => println!("Session revoked successfully"),
    Err(e) => panic!("Failed to revoke session: {:?}", e),
}
```

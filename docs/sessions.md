Polska Wersja / [English version](sessions.en.md)

# Zarządzanie sesjami

__[Oficjalna Dokumentacja](https://github.com/CIRFMF/ksef-docs/blob/main/auth/sesje.md)__

### 1. Uzyskiwanie listy aktywnych sesji

Funkcja `get_active_sessions` zwraca listę aktywnych sesji. Przyjmuje ona opcjonalny parametr `continuation_token`, który umożliwia paginację wyników w przypadku dużej liczby sesji. `continuation_token` znajduje się w obiekcie `QuerySessionsResponse` zwracanym przez funkcję i można go użyć do pobrania kolejnej strony wyników. Jeśli nie posiadasz tokena kontynuacji, wywołaj `get_active_sessions(None)`, aby pobrać pierwszą stronę wyników.

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

### 2. Unieważnianie aktualnej sesji

Funkcja `revoke_current_session` unieważnia bieżącą sesję — tj. sesję powiązaną z tokenem użytym do wywołania tej metody. Po jej wykonaniu powiązany refresh token przestaje działać; aktywne access tokeny pozostają ważne do momentu wygaśnięcia.

```rust
match client.revoke_current_session() {
    Ok(()) => println!("Current session revoked successfully"),
    Err(e) => panic!("Failed to revoke current session: {:?}", e),
}
```

### 3. Unieważnianie wybranej sesji

Funkcja `revoke_session` unieważnia wskazaną sesję na podstawie numeru referencyjnego (`reference_number`). Numer referencyjny można pobrać z listy aktywnych sesji (`Session.reference_number`). Wykonanie tej operacji może wymagać odpowiednich uprawnień.

```rust
match client.revoke_session("session_reference_number") {
    Ok(()) => println!("Session revoked successfully"),
    Err(e) => panic!("Failed to revoke session: {:?}", e),
}
```

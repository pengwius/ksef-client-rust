Polska wersja / [English version](README.en.md)

# KSeF SDK w Rust

SDK do integracji z Krajowym Systemem e-Faktur (KSeF).

[![Crates.io](https://img.shields.io/crates/v/ksef-client.svg)](https://crates.io/crates/ksef-client)

Biblioteka jest w fazie eksperymentalnej. Nie wszystkie funkcje muszą działać poprawnie.

---

# Instalacja
Dodaj do swojego `Cargo.toml`:

```toml
[dependencies]
ksef-client = "0.2"
```

## Dokumentacja

Szczegółowa dokumentacja znajduje się w katalogu `docs/`.

*   [Proces Uwierzytelniania](docs/authentication.md)
*   [Zarządzanie Tokenami KSeF](docs/ksef_tokens.md)
*   [Zarządzanie Sesjami](docs/sessions.md)
*   [Zarządzanie Uprawnieniami](docs/permissions.md)
*   [Certyfikaty KSeF](docs/ksef_certificates.md)
*   [Wysyłanie Faktury w Sesji Interaktywnej](docs/online_session.md)
*   [Wysyłanie Faktury w Sesji Wsadowej](docs/batch_session.md)
*   [Pobieranie Faktur](docs/fetching_invoices.md)
*   [Kody QR](docs/qr.md)
*   [Pobieranie Dostawców Usług Peppol](docs/peppol.md)
*   [Urzędowe Poświadczenie Odbioru (UPO) Faktur](docs/upo.md)

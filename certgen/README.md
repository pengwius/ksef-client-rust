Polska wersja / [English version](README.en.md)

## certgen

Program do generowania self-signed certyfikatów dla KSeF.

Użycie:
```bash
cargo run --bin certgen -- --output file --nip 1234567890
```

Opcje:
- --output — tryby: `file` (zapisuje certyfikat do pliku) lub `screen` (wypisuje na ekran).
- --nip — numer NIP (np. `1234567890`) używany w danych certyfikatu (CN/subject).

Więcej informacji:
```bash
cargo run --bin certgen -- --help
```

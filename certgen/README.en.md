[Polska wersja](README.md) / English version

## certgen

A tool to generate self-signed certificates for KSeF.

Usage:
```bash
cargo run --bin certgen -- --output file --nip 1234567890
```

Options:
- --output — modes: `file` (writes certificate to a file) or `screen` (prints to stdout).
- --nip — NIP number (e.g. `1234567890`) used in the certificate subject.

More info:
```bash
cargo run --bin certgen -- --help
```

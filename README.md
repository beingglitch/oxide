# oxide

A file encryption CLI built from scratch in Rust.

**⚠️ Do not use this to protect anything real.** This is a learning project.

## Install

```
cargo install --path .
```

## Usage

```
oxide encrypt myfile.txt
oxide decrypt encrypted.txt
```

## How it works

Currently: a single-byte XOR cipher applied byte-by-byte across the file's contents (`fs::read` into a `Vec<u8>`, XOR each byte against a fixed key byte, `fs::write` the result back out). This proves the fundamental reversibility property XOR-based ciphers rely on (`a ^ key ^ key == a`), but is not yet a real cipher (a single repeating byte is trivially breakable via frequency analysis).

This is a work in progress, being built incrementally. See Roadmap below for what's coming.

## Known limitations

- No tests yet

## Roadmap

[x] CLI scaffold (clap, derive macros, encrypt/decrypt subcommands)
- [x] File I/O module (read/write, `Result`-based errors)
- [x] Core XOR cipher, round-trip verified
- [x] Keystream generation (cycling password bytes across an arbitrary-length file)
- [x] Magic-byte prefix for wrong-password detection
- [x] Custom error type (`thiserror`), `?`-based propagation, no `.unwrap()`
- [x] Proper key derivation (mixing/hash step to fix the prefix-collision weakness above)
- [ ] Streamed file I/O for large files (`BufReader`/`BufWriter`)
- [ ] Unit tests (encrypt→decrypt round-trip, edge cases)
- [ ] `clippy`-clean, documented (`///`) public API
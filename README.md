# oxide

A file encryption CLI built from scratch in Rust. No `RustCrypto`, no `aes`/`chacha20` crates. A hand-rolled streaming cipher and key derivation, implemented from the byte level up, built as a vehicle to learn Rust deeply, not as a production security tool.

**⚠️ Do not use this to protect anything real.** This is a learning project.

## Install

```
cargo install oxide-cipher
```

## Usage

```
oxide encrypt myfile.txt --password hunter2 --output encrypted.bin
oxide decrypt encrypted.bin --password hunter2 --output decrypted.txt
```

`--output` is optional and defaults to `encrypted_file.txt` / `decrypted_file.txt`.

## How it works

A password is mixed into a fixed-size seed via a chained rotate-and-XOR process, where each byte of the seed depends on the accumulated state from every byte before it rather than a fresh computation per position. This avoids the keystream predictability that a naive per-position reset produces. The seed is cycled into a keystream and XORed against the file's bytes.

Files are streamed in fixed-size chunks (`BufReader`/`BufWriter`) rather than loaded entirely into memory, so encryption and decryption work on files of any size with constant memory use. A 5-byte magic prefix (`OXIDE`) is encrypted once at the start of the output and checked on decrypt, so a wrong password is detected explicitly instead of silently producing garbage.

All fallible operations (file I/O, wrong password) return a custom `OxideError` type and propagate with `?`, no panics on expected failures.

## Roadmap

- [x] CLI scaffold (clap, derive macros, encrypt/decrypt subcommands)
- [x] File I/O module (read/write, `Result`-based errors)
- [x] Core XOR cipher, round-trip verified
- [x] Keystream generation (cycling seed bytes across an arbitrary-length file)
- [x] Magic-byte prefix for wrong-password detection
- [x] Custom error type (`thiserror`), `?`-based propagation, no `.unwrap()`
- [x] Chained key derivation (`derive_seed`), fixing prefix-collision and predictability weaknesses
- [x] Streamed file I/O for large files (`BufReader`/`BufWriter`), constant memory use
- [x] Unit tests (round-trip, wrong-key, empty-input, keystream-collision regression)
- [x] `clippy`-clean, documented (`///`) public API

## Known limitations

- The mixing function is a hand-rolled rotate-and-XOR chain, not a real cryptographic hash. It resists the specific weaknesses found during development (prefix collisions, simple two-equation recovery) but has none of the formal guarantees of an actual KDF like PBKDF2, scrypt, or Argon2.
- No authenticated encryption. The magic-byte check detects a wrong password but doesn't protect against tampering with the ciphertext itself.

## License

[MIT](LICENSE)
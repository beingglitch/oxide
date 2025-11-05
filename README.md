# 🧪 Oxide

A **blazingly fast file encryption CLI** built in Rust — powered by **AES-256-GCM** and **PBKDF2-HMAC-SHA256** for secure, authenticated encryption.

---

## ⚙️ Usage

```bash
oxide encrypt --input <file> --password <password>
oxide decrypt --input <file.oxide> --password <password>
```

Optional:

```bash
--output <path>   # specify custom output file
```

Example:

```bash
oxide encrypt -i secrets.txt -p "hunter2"
oxide decrypt -i secrets.txt.oxide -p "hunter2"
```

---

## 📦 Installation

Three ways to install Oxide as a system binary:

---

### 🧩 1. Manual Install (via `cp` to `/usr/local/bin`)

For direct installation after building locally:

```bash
git clone https://github.com/beingglitch/oxide.git
cd oxide
cargo build --release

# copy binary to PATH
sudo cp target/release/oxide /usr/local/bin/
sudo chown root:root /usr/local/bin/oxide
sudo chmod 755 /usr/local/bin/oxide
```

**Explanation**

* `cp` copies the compiled binary into `/usr/local/bin`, which is already on `PATH`.
* `chown root:root` makes it owned by root (standard for system binaries).
* `chmod 755` allows everyone to run it, but only root can modify it.

After this, run:

```bash
oxide --help
```

---

### 📦 2. Install from `.deb` (Debian/Ubuntu)

If you’ve built or downloaded a `.deb` package:

```bash
sudo dpkg -i target/debian/oxide_0.1.0_amd64.deb
sudo apt-get -f install   # fix dependencies if needed
```

Once installed, `oxide` will be available system-wide:

```bash
oxide encrypt --help
```

---

### 🧰 3. Install via APT (coming soon)

Oxide will soon be available via a signed APT repository.

Planned usage:

```bash
sudo apt update
sudo apt install oxide
```

Stay tuned — APT repository configuration will be announced soon.

---

## 🔒 Features

* AES-256-GCM authenticated encryption
* PBKDF2-HMAC-SHA256 key derivation
* Random salt + nonce for each encryption
* Simple CLI built with `clap`
* Cross-platform Rust binary

---

## 🛠 Build (for development)

```bash
cargo build --release
```

Binary output: `target/release/oxide`

---

## 🧹 Uninstall

```bash
# Manual install
sudo rm /usr/local/bin/oxide

# .deb or apt install
sudo apt remove oxide
```

---

## 📝 License

MIT © 2025 [LICENSE](LICENSE)

---

## ✨ Summary

Fast • Secure • Minimal — Oxide encrypts files with modern cryptography in a single command.

---

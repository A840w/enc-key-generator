#  GSRC  Rust Encryption Key Generator

> Generate cryptographically secure encryption keys for AES, ChaCha20, and custom lengths directly from your terminal.

[![rust](https://img.shields.io/badge/Rust-Stable-orange?style=for-the-badge&logo=rust)](https://rust-lang.org/)
![cli tool](https://img.shields.io/badge/CLI-Tool-blue?style=for-the-badge)
[![MIT LICENCE](https://img.shields.io/badge/License-MIT-green?style=for-the-badge)](https://mit-license.org/)

---

##  Features

✅ **Cryptographically Secure**
- Uses `rand::thread_rng()`
- Backed by the operating system's CSPRNG:
  - Linux: `/dev/urandom`
  - Unix: `getrandom()`
  - Windows: `BCryptGenRandom`

✅ **Multiple Key Types**
- AES-128 (16 bytes)
- AES-192 (24 bytes)
- AES-256 (32 bytes)
- ChaCha20 (32 bytes)
- Custom byte lengths

✅ **Multiple Output Formats**
- Hexadecimal
- Base64
- Raw Byte Arrays (`[0x00, 0x01, ...]`)

✅ **Generate Multiple Keys**
- Create one or many keys in a single command.

✅ **Colorized Terminal Output**
- Styled output powered by `colored`.

---

#  Quick Start

## 1. Clone the Repository

```bash
git clone https://github.com/yourusername/your-repo.git
cd your-repo
```

## 2. Run the Tool

```bash
cargo run
```

### Default Behavior

Running without arguments generates:

```text
AES-256 Key
Format: Hex
Count: 1
```

Example:

```text
7d65a46fe1a5e6d1c4f4fb1028f48e44f6ea84ed947fc258fd1d2397a69e8b39
```

---

#  Interactive Command Builder

Choose what you want:

| Option | Value |
|----------|----------|
| Key Type | `aes128`, `aes192`, `aes256`, `cha-cha20`, `custom` |
| Format | `hex`, `base64`, `raw-bytes` |
| Count | `-n <number>` |
| Custom Length | `-l <bytes>` |

### Formula

```bash
cargo run -- -k <key-type> -f <format> -n <count>
```

---

##  Supported Key Types

| Algorithm | Size |
|------------|---------|
| AES-128 | 16 Bytes (128-bit) |
| AES-192 | 24 Bytes (192-bit) |
| AES-256 | 32 Bytes (256-bit) |
| ChaCha20 | 32 Bytes (256-bit) |
| Custom | User Defined |

---

##  Supported Output Formats

### Hex

```text
7d65a46fe1a5e6d1...
```

### Base64

```text
fWWkb+Gl5tHE9PsQKPSORP...
```

### Raw Bytes

```rust
[
    0x7D, 0x65, 0xA4, 0x6F,
    0xE1, 0xA5, 0xE6, 0xD1
]
```

---

#  Examples

## Generate a Base64 ChaCha20 Key

```bash
cargo run -- -k cha-cha20 -f base64
```

---

## Generate 5 AES-128 Keys as Raw Bytes

```bash
cargo run -- -k aes128 -f raw-bytes -n 5
```

---

## Generate an AES-192 Key in Hex

```bash
cargo run -- -k aes192 -f hex
```

---

## Generate an AES-256 Key in Base64

```bash
cargo run -- -k aes256 -f base64
```

---

## Generate a Custom 64-Byte (512-bit) Key

```bash
cargo run -- -k custom -l 64 -f hex
```

---

## Generate 10 Custom Keys

```bash
cargo run -- -k custom -l 128 -f base64 -n 10
```

---

#  CLI Arguments

| Argument | Description |
|------------|------------|
| `-k` | Key type |
| `-f` | Output format |
| `-n` | Number of keys to generate |
| `-l` | Custom key length (required for custom mode) |

---

## Key Types

```text
aes128
aes192
aes256
cha-cha20
custom
```

---

## Formats

```text
hex
base64
raw-bytes
```

---

#  Usage Cheat Sheet

### Default AES-256 (Hex)

```bash
cargo run
```

### AES-128

```bash
cargo run -- -k aes128
```

### AES-192

```bash
cargo run -- -k aes192
```

### ChaCha20

```bash
cargo run -- -k cha-cha20
```

### Base64 Output

```bash
cargo run -- -f base64
```

### Raw Byte Output

```bash
cargo run -- -f raw-bytes
```

### Multiple Keys

```bash
cargo run -- -n 25
```

### Custom 256-Byte Key

```bash
cargo run -- -k custom -l 256
```

---

#  Security Notes

- Keys are generated using a cryptographically secure random number generator.
- No deterministic seeds are used.
- No key material is written to disk.
- Generated keys exist only in memory and terminal output.
- Suitable for encryption, testing, development, and cryptographic key generation workflows.

---

#  Built With

- Rust
- rand
- base64
- colored

---

#  License

MIT License

Feel free to use, modify, and contribute.

---

##  Example Output

```text
 GSRC Key Generator
============================================
Algorithm: Aes256 | Length: 32 bytes (256 bits) | Format: Hex
============================================
 fbe92f4cc261af5901349d7ad8c477df3687732cd36e89cb39df634c4cb8fa21
```

---

### If this project helped you, consider giving it a ⭐ on GitHub.
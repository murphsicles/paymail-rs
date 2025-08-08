# paymail-rs 🚀

[![Rust](https://img.shields.io/badge/rust-1.86%2B-orange?logo=rust)](https://www.rust-lang.org)
[![Dependencies](https://deps.rs/repo/github/murphsicles/paymail-rs/status.svg)](https://deps.rs/repo/github/murphsicles/paymail-rs)
[![CI](https://github.com/murphsicles/paymail-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/murphsicles/paymail-rs/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-BSV-blue)](LICENSE)

A fast, asynchronous Rust library for the [BSV PayMail protocol](https://bsvalias.org/), enabling seamless integration with Bitcoin SV services. ⚡

## Features ✨

- **Host and Capability Discovery**: Resolves PayMail domains via DNS SRV and A/AAAA records, fetching capabilities over HTTP. 🔍
- **PKI Resolution**: Retrieves public keys for PayMail addresses (BRFC 759684b1a19a). 🔑
- **Payment Address Resolution**: Supports signed payment destination requests (BRFC 759684b1a19a). 💸
- **P2P Transactions**: Implements P2P payment destinations and transaction submission (BRFCs 2a40af698840, 5f1323cddf31). 🔗
- **Extensible**: Handles custom BRFC extensions via the `call_extension` method. 🛠️
- **Fully Asynchronous**: Built with Tokio for high-performance async operations. 🕒
- **BSV Primitives**: Leverages `rust-sv` for robust cryptographic operations. 🔒

## Installation 📦

Add to your `Cargo.toml`:

```toml
[dependencies]
paymail-rs = { git = "https://github.com/your-repo/paymail-rs.git", tag = "v0.1.2" }
```

Ensure you have Rust 1.86 or later installed.

## Getting Started 🏁

1. **Install Rust**: Follow the [official Rust installation guide](https://www.rust-lang.org/tools/install).
2. **Clone the Repository**:
   ```sh
   git clone https://github.com/murphsicles/paymail-rs.git
   cd paymail-rs
   ```
3. **Build the Project**:
   ```sh
   cargo build
   ```
4. **Run Examples**:
   ```sh
   cargo run --example client
   ```
   See `examples/client.rs` for a sample client implementation that demonstrates fetching public keys and payment destinations.

## Usage 📝

Create a `PaymailClient` with a private key and use it to interact with PayMail services:

```rust
use paymail_rs::PaymailClient;
use secp256k1::SecretKey;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load a valid secp256k1 private key (replace with your own)
    let priv_key = SecretKey::from_slice(&[
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10,
        0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18,
        0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20,
    ])?;

    let client = PaymailClient::builder().build(priv_key);

    // Get public key
    let pubkey = client.get_pubkey("alice@wallet.com").await?;
    println!("Pubkey: {pubkey}");

    // Get payment destination
    use paymail_rs::models::PaymentRequest;
    let req = PaymentRequest {
        sender_name: Some("Sender".to_string()),
        sender_handle: "sender@wallet.com".to_string(),
        dt: "".to_string(),
        amount: Some(10000),
        purpose: Some("Test".to_string()),
        signature: "".to_string(),
    };
    let output = client.get_payment_destination("alice@wallet.com", req).await?;
    println!("Output: {output}");

    // P2P transaction
    let tx_resp = client.send_p2p_tx("alice@wallet.com", "txhex", json!({}), "ref").await?;
    println!("Tx: {tx_resp:?}");

    Ok(())
}
```

## Testing 🧪

The library includes integration tests for core functionality:

- **Capabilities Resolution**: Tests fetching PayMail capabilities (`tests/integration.rs`).
- **Public Key Resolution**: Tests retrieving public keys for PayMail addresses (`tests/integration.rs`).
- Tests use `wiremock` and `mockall` to mock HTTP and DNS responses, ensuring reliability.

Run tests with:

```sh
cargo test
```

For benchmarks:

```sh
cargo bench
```

## Contributing 🤝

Contributions are welcome! Please:
1. Fork the repository.
2. Create a feature branch (`git checkout -b feature/my-feature`).
3. Commit changes (`git commit -am 'Add my feature'`).
4. Push to the branch (`git push origin feature/my-feature`).
5. Open a pull request.

Ensure all code passes `cargo fmt`, `cargo clippy`, and `cargo test`.

## License 📄

This project is licensed under the Open BSV License. See the [LICENSE](LICENSE) file for details.

## Acknowledgments 🙌

- Built with [rust-sv](https://github.com/murphsicles/rust-sv) for BSV primitives.
- Inspired by the [BSV Alias (PayMail) specifications](https://bsvalias.org/).

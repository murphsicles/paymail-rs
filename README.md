# paymail-rs 🚀

A fast, asynchronous Rust library for the BSV PayMail protocol. ⚡

## Features ✨

- Host and capability discovery via DNS and HTTPS. 🔍
- PKI resolution for public keys. 🔑
- Payment address resolution with signed requests. 💸
- Extensible for P2P transactions and other BRFCs. 🔗
- Fully async with Tokio. 🕒
- Uses rust-sv for BSV primitives. 🛠️

## Installation 📦

Add to your Cargo.toml:

```toml
[dependencies]
paymail-rs = { git = "https://github.com/your-repo/paymail-rs.git" }  # Until published
```

## Usage 📝

See examples/client.rs for basic usage.

```rust
use paymail_rs::PaymailClient;
use secp256k1::SecretKey;

// Load your private key
let priv_key = SecretKey::from_slice(&[0; 32]).unwrap(); // Dummy

let client = PaymailClient::builder().build(priv_key);

// Get pubkey
let pubkey = client.get_pubkey("alice@wallet.com").await.unwrap();

// Get payment destination
use paymail_rs::models::PaymentRequest;
let req = PaymentRequest {
    sender_handle: "sender@wallet.com".to_string(),
    // ... other fields
};
let output = client.get_payment_destination("alice@wallet.com", req).await.unwrap();
```

## Building and Testing 🧪

```sh
cargo build
cargo test
cargo bench
```

## License 📄

Ooen BSV

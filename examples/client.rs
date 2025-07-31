use paymail_rs::{models::PaymentRequest, PaymailClient};
use secp256k1::SecretKey;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Dummy private key; in real use, load from wallet
    let priv_key_bytes = [0u8; 32]; // Replace with actual key bytes
    let priv_key = SecretKey::from_slice(&priv_key_bytes)?;

    let client = PaymailClient::builder().build(priv_key);

    // Example: Get pubkey
    let pubkey = client.get_pubkey("alice@walletprovider.com").await?;
    println!("Pubkey: {}", pubkey);

    // Example: Get payment destination
    let mut req = PaymentRequest {
        sender_name: Some("Sender".to_string()),
        sender_handle: "sender@wallet.com".to_string(),
        dt: "".to_string(), // Set automatically
        amount: Some(10000),
        purpose: Some("Test payment".to_string()),
        signature: "".to_string(), // Set automatically
    };
    let output = client.get_payment_destination("alice@walletprovider.com", req).await?;
    println!("Output script hex: {}", output);

    Ok(())
}

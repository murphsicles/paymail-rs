use paymail_rs::{PaymailClient, models::PaymentRequest};
use secp256k1::SecretKey;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let priv_key = SecretKey::from_slice(&[0; 32])?;
    let client = PaymailClient::builder().build(priv_key);

    let pubkey = client.get_pubkey("alice@wallet.com").await?;
    println!("Pubkey: {pubkey}");

    let req = PaymentRequest {
        sender_name: Some("Sender".to_string()),
        sender_handle: "sender@wallet.com".to_string(),
        dt: "".to_string(),
        amount: Some(10000),
        purpose: Some("Test".to_string()),
        signature: "".to_string(),
    };
    let output = client
        .get_payment_destination("alice@wallet.com", req)
        .await?;
    println!("Output: {output}");

    let p2p_resp = client
        .get_p2p_payment_destination("alice@wallet.com", 10000)
        .await?;
    println!("P2P: {p2p_resp:?}");

    let tx_resp = client
        .send_p2p_tx("alice@wallet.com", "txhex", json!({}), "ref")
        .await?;
    println!("Tx: {tx_resp:?}");

    Ok(())
}

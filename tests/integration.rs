use paymail_rs::{PaymailClient, models::PaymentRequest};
use secp256k1::SecretKey;
use wiremock::{Mock, MockServer, ResponseTemplate};
use wiremock::matchers::{method, path};

#[tokio::test]
async fn test_get_capabilities() {
    let mock_server = MockServer::start().await;
    let dummy_priv = SecretKey::from_slice(&[0;32]).unwrap();
    let client = PaymailClient::builder().build(dummy_priv);

    Mock::given(method("GET"))
        .and(path("/.well-known/bsvalias"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "bsvalias": "1.0",
            "capabilities": {
                "pki": "https://example.com/id/{alias}@{domain.tld}",
                "paymentDestination": "https://example.com/{alias}@{domain.tld}/payment-destination"
            }
        })))
        .mount(&mock_server)
        .await;

    // Override resolve_host to use mock
    // For real test, patch resolve_host to return mock_server.uri()

    // Test code...
}

 // Add more tests for pubkey, payment destination, signature verification, etc.

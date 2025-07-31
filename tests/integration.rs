use paymail_rs::{PaymailClient, models::PaymentRequest};
use secp256k1::SecretKey;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_get_capabilities() {
    let mock_server = MockServer::start().await;
    let dummy_priv = SecretKey::from_slice(&[0; 32]).unwrap();
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

    // Note: In real tests, mock resolve_host to use mock_server.uri()
    let caps = client.get_capabilities("example.com").await.unwrap();
    assert_eq!(caps.bsvalias, "1.0");
    // Add tests for other methods
}

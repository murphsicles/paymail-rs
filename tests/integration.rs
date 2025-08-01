use paymail_rs::PaymailClient;
use secp256k1::SecretKey;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_get_capabilities() {
    let mock_server = MockServer::start().await;
    // Use a valid secp256k1 private key (example: 32 bytes, non-zero, valid scalar)
    let dummy_priv = SecretKey::from_slice(&[
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10,
        0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18,
        0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20,
    ]).unwrap();
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

    // TODO: Mock resolve_host to return mock_server.uri() to avoid real DNS calls
    let caps = client.get_capabilities("example.com").await.unwrap();
    assert_eq!(caps.bsvalias, "1.0");
}

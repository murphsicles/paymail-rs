use mockall::mock;
use paymail_rs::PaymailClient;
use paymail_rs::resolver::Resolver;
use secp256k1::SecretKey;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

mock! {
    Resolver {}
    #[async_trait::async_trait]
    impl Resolver for Resolver {
        async fn resolve_host(&self, domain: &str) -> Result<(String, u16), paymail_rs::errors::PaymailError>;
    }
}

#[tokio::test]
async fn test_get_capabilities() {
    let mock_server = MockServer::start().await;
    // Use a valid secp256k1 private key (example: 32 bytes, non-zero, valid scalar)
    let dummy_priv = SecretKey::from_slice(&[
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e,
        0x1f, 0x20,
    ])
    .unwrap();

    // Mock resolver to return mock server's host and port
    let mut mock_resolver = MockResolver::new();
    let mock_uri = mock_server.uri();
    let mock_host = mock_uri
        .strip_prefix("http://")
        .unwrap_or(&mock_uri)
        .strip_suffix('/')
        .unwrap_or(&mock_uri)
        .to_string();
    mock_resolver
        .expect_resolve_host()
        .with(mockall::predicate::eq("example.com"))
        .times(1)
        .returning(move |_| Ok((mock_host.clone(), 80)));

    let client = PaymailClient::builder()
        .resolver(Arc::new(mock_resolver))
        .build(dummy_priv);

    Mock::given(method("GET"))
        .and(path("/.well-known/bsvalias"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "bsvalias": "1.0",
            "capabilities": {
                "pki": "/id/{alias}@{domain.tld}",
                "paymentDestination": "/{alias}@{domain.tld}/payment-destination"
            }
        })))
        .mount(&mock_server)
        .await;

    let caps = client
        .get_capabilities("example.com")
        .await
        .expect("Failed to get capabilities");
    assert_eq!(caps.bsvalias, "1.0");
}

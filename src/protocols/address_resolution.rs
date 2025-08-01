use crate::{models::PaymentRequest, PaymailClient, PaymailError};

pub async fn resolve_address(
    client: &PaymailClient,
    paymail: &str,
    sender_handle: &str,
    amount: Option<u64>,
    purpose: Option<String>,
) -> Result<String, PaymailError> {
    let req = PaymentRequest {
        sender_name: None,
        sender_handle: sender_handle.to_string(),
        dt: "".to_string(),
        amount,
        purpose,
        signature: "".to_string(),
    };
    client.get_payment_destination(paymail, req).await
}

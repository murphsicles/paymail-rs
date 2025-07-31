use crate::{models::PaymentRequest, PaymailClient, PaymailError, utils};

pub async fn resolve_address(
    client: &PaymailClient,
    paymail: &str,
    sender_handle: &str,
    amount: Option<u64>,
    purpose: Option<String>,
) -> Result<String, PaymailError> {
    let mut req = PaymentRequest {
        sender_name: None,  // Optional
        sender_handle: sender_handle.to_string(),
        dt: "".to_string(),  // Set in client
        amount,
        purpose,
        signature: "".to_string(),  // Set in client
    };
    client.get_payment_destination(paymail, req).await
}

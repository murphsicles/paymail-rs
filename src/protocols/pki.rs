use crate::{PaymailClient, PaymailError};

pub async fn fetch_pubkey(client: &PaymailClient, paymail: &str) -> Result<String, PaymailError> {
    client.get_pubkey(paymail).await
}

use crate::{
    models::{P2PPaymentDestinationResponse, P2PTxResponse},
    PaymailClient, PaymailError,
};
use serde_json::Value;

pub async fn resolve_p2p_address(
    client: &PaymailClient,
    paymail: &str,
    satoshis: u64,
) -> Result<P2PPaymentDestinationResponse, PaymailError> {
    client.get_p2p_payment_destination(paymail, satoshis).await
}

pub async fn submit_p2p_tx(
    client: &PaymailClient,
    paymail: &str,
    hex: &str,
    metadata: Value,
    reference: &str,
) -> Result<P2PTxResponse, PaymailError> {
    client.send_p2p_tx(paymail, hex, metadata, reference).await
}

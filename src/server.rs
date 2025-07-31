use async_trait::async_trait;

use crate::errors::PaymailError;
use crate::models::{PkiResponse, PaymentDestinationResponse};
use crate::utils;

#[async_trait]
pub trait PaymailHandler {
    async fn handle_pki(&self, alias: &str, domain: &str) -> Result<PkiResponse, PaymailError>;

    async fn handle_payment_destination(
        &self,
        alias: &str,
        domain: &str,
        sender_handle: &str,
        dt: &str,
        amount: Option<u64>,
        purpose: Option<String>,
        signature: &str,
    ) -> Result<PaymentDestinationResponse, PaymailError>;

    // TODO: Add handlers for P2P, verifiable messages, etc.
}

// Example impl would be provided by user, e.g., struct MyHandler { ... } impl PaymailHandler { ... }
// In handle_pki, return pubkey from wallet or db, using rust-sv for key gen if needed.
// In handle_payment_destination, verify signature using utils::verify_signature, then generate single-use script via rust-sv Script.

use async_trait::async_trait;

use crate::errors::PaymailError;
use crate::models::{
    P2PPaymentDestinationResponse, P2PTxResponse, PaymentDestinationResponse, PkiResponse,
};
use crate::utils;
use serde_json::Value;

#[async_trait]
pub trait PaymailHandler {
    async fn handle_pki(&self, alias: &str, domain: &str) -> Result<PkiResponse, PaymailError>;

    async fn handle_payment_destination(
        &self,
        _alias: &str,
        _domain: &str,
        sender_handle: &str,
        dt: &str,
        amount: Option<u64>,
        purpose: Option<String>,
        signature: &str,
        sender_pubkey: &str,
    ) -> Result<PaymentDestinationResponse, PaymailError> {
        let message = format!(
            "{}|{}|{}|{}",
            sender_handle,
            dt,
            amount.unwrap_or(0),
            purpose.as_deref().unwrap_or("")
        );
        if !utils::verify_signature(sender_pubkey, signature, &message)? {
            return Err(PaymailError::InvalidSignature(
                "Signature verification failed".to_string(),
            ));
        }
        let script = "76a914deadbeef88ac".to_string();
        Ok(PaymentDestinationResponse { output: script })
    }

    async fn handle_p2p_payment_destination(
        &self,
        _alias: &str,
        _domain: &str,
        satoshis: u64,
    ) -> Result<P2PPaymentDestinationResponse, PaymailError> {
        let outputs = vec![Value::Object(serde_json::Map::from_iter(vec![
            (
                "script".to_string(),
                Value::String("76a914deadbeef88ac".to_string()),
            ),
            ("satoshis".to_string(), Value::Number(satoshis.into())),
        ]))];
        Ok(P2PPaymentDestinationResponse {
            outputs,
            reference: "unique-ref".to_string(),
        })
    }

    async fn handle_p2p_tx(
        &self,
        _alias: &str,
        _domain: &str,
        hex: &str,
        metadata: Value,
        reference: &str,
        signature: &str,
        sender_pubkey: &str,
    ) -> Result<P2PTxResponse, PaymailError> {
        let message = format!("{}|{}", hex, reference);
        if !utils::verify_signature(sender_pubkey, signature, &message)? {
            return Err(PaymailError::InvalidSignature(
                "Signature verification failed".to_string(),
            ));
        }
        let txid = "txid-from-broadcast".to_string();
        Ok(P2PTxResponse {
            txid,
            note: Some("Received".to_string()),
        })
    }
}

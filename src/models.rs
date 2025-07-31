use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Capabilities {
    pub bsvalias: String,
    pub capabilities: HashMap<String, Value>,
}

#[derive(Serialize, Debug, Clone)]
pub struct PaymentRequest {
    pub sender_name: Option<String>,
    pub sender_handle: String,
    pub dt: String,
    pub amount: Option<u64>,
    pub purpose: Option<String>,
    pub signature: String,
}

impl PaymentRequest {
    pub fn signable_message(&self) -> String {
        format!(
            "{}|{}|{}|{}",
            self.sender_handle,
            self.dt,
            self.amount.unwrap_or(0),
            self.purpose.as_deref().unwrap_or("")
        )
    }
}

#[derive(Deserialize, Debug)]
pub struct PaymentDestinationResponse {
    pub output: String,  // Hex-encoded script
}

#[derive(Deserialize, Debug)]
pub struct PkiResponse {
    pub bsvalias: String,
    pub handle: String,
    pub pubkey: String,
}

// TODO: Add models for P2P tx, etc.

use thiserror::Error;
use sv::util::Error as SvError;

#[derive(Error, Debug)]
pub enum PaymailError {
    #[error("Invalid PayMail format: {0}")]
    InvalidFormat(String),
    #[error("DNS resolution failed: {0}")]
    DnsFailure(String),
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),
    #[error("JSON serialization/deserialization failed: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("Capability missing: {0}")]
    CapabilityMissing(String),
    #[error("Invalid signature: {0}")]
    InvalidSignature(String),
    #[error("Bitcoin SV error: {0}")]
    SvError(#[from] SvError),
    #[error("Other error: {0}")]
    Other(String),
}

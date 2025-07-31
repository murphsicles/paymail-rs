use base64;
use crate::errors::PaymailError;
use hex;
use ring::digest::{digest, SHA256};
use secp256k1::{ecdsa, Message, PublicKey, RecoveryId, Secp256k1, SecretKey};
use sv::script::Script;
use sv::util::hash256::sha256d;
use sv::util::var_int;

pub fn generate_signature(priv_key: &SecretKey, message: &str) -> Result<String, PaymailError> {
    let prefix = b"Bitcoin Signed Message:\n";
    let mut prefixed_message = Vec::new();
    var_int::write(message.len() as u64, &mut prefixed_message)?;
    prefixed_message.extend_from_slice(message.as_bytes());
    let mut full_message = Vec::new();
    full_message.extend_from_slice(prefix);
    full_message.extend_from_slice(&prefixed_message);
    let hash = sha256d(&full_message);
    let msg = Message::from_slice(&hash.0).map_err(|e| PaymailError::Other(e.to_string()))?;
    let secp = Secp256k1::new();
    let recoverable_sig = secp.sign_ecdsa_recoverable(&msg, priv_key);
    let (recovery_id, compact) = recoverable_sig.serialize_compact();
    let mut full_sig = [0u8; 65];
    full_sig[0] = 31 + recovery_id.to_i32() as u8;
    full_sig[1..].copy_from_slice(&compact);
    Ok(base64::engine::general_purpose::STANDARD.encode(&full_sig))
}

pub fn verify_signature(pub_key_hex: &str, signature: &str, message: &str) -> Result<bool, PaymailError> {
    let pub_key_bytes = hex::decode(pub_key_hex).map_err(|e| PaymailError::Other(e.to_string()))?;
    let pub_key = PublicKey::from_slice(&pub_key_bytes).map_err(|e| PaymailError::Other(e.to_string()))?;

    let sig_bytes = base64::engine::general_purpose::STANDARD.decode(signature).map_err(|e| PaymailError::Other(e.to_string()))?;
    if sig_bytes.len() != 65 {
        return Err(PaymailError::InvalidSignature("Invalid signature length".to_string()));
    }
    let header = sig_bytes[0];
    if !(31..=34).contains(&header) {
        return Err(PaymailError::InvalidSignature("Invalid recovery header".to_string()));
    }
    let recovery_id = RecoveryId::from_i32((header - 31) as i32).map_err(|e| PaymailError::Other(e.to_string()))?;
    let compact_sig = ecdsa::RecoverableSignature::from_compact(&sig_bytes[1..], recovery_id).map_err(|e| PaymailError::Other(e.to_string()))?;
    let standard_sig = compact_sig.to_standard();

    let prefix = b"Bitcoin Signed Message:\n";
    let mut prefixed_message = Vec::new();
    var_int::write(message.len() as u64, &mut prefixed_message)?;
    prefixed_message.extend_from_slice(message.as_bytes());
    let mut full_message = Vec::new();
    full_message.extend_from_slice(prefix);
    full_message.extend_from_slice(&prefixed_message);
    let hash = sha256d(&full_message);
    let msg = Message::from_slice(&hash.0).map_err(|e| PaymailError::Other(e.to_string()))?;

    let secp = Secp256k1::new();
    Ok(secp.verify_ecdsa(&msg, &standard_sig, &pub_key).is_ok())
}

pub fn parse_script(hex_str: &str) -> Result<Script, PaymailError> {
    let bytes = hex::decode(hex_str).map_err(|e| PaymailError::Other(e.to_string()))?;
    Ok(Script(bytes))
}

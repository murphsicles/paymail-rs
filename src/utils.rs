use crate::errors::PaymailError;
use base64::Engine;
use hex;
use ring::digest::SHA256;
use secp256k1::{Message, PublicKey, Secp256k1, SecretKey, ecdsa};
use sv::script::Script;

pub fn generate_signature(priv_key: &SecretKey, message: &str) -> Result<String, PaymailError> {
    let hash = ring::digest::digest(&SHA256, message.as_bytes());
    let msg_array: [u8; 32] = hash
        .as_ref()
        .try_into()
        .map_err(|_| PaymailError::Other("Invalid hash length".to_string()))?;
    let msg = Message::from_digest(msg_array);
    let secp = Secp256k1::new();
    let recoverable_sig = secp.sign_ecdsa_recoverable(msg, priv_key);
    let (recovery_id, compact) = recoverable_sig.serialize_compact();
    let mut full_sig = [0u8; 65];
    full_sig[0] = 31 + i32::from(recovery_id) as u8;
    full_sig[1..].copy_from_slice(&compact);
    Ok(base64::engine::general_purpose::STANDARD.encode(full_sig))
}

pub fn verify_signature(
    pub_key_hex: &str,
    signature: &str,
    message: &str,
) -> Result<bool, PaymailError> {
    let pub_key_bytes = hex::decode(pub_key_hex).map_err(|e| PaymailError::Other(e.to_string()))?;
    let pub_key =
        PublicKey::from_slice(&pub_key_bytes).map_err(|e| PaymailError::Other(e.to_string()))?;

    let sig_bytes = base64::engine::general_purpose::STANDARD
        .decode(signature)
        .map_err(|e| PaymailError::Other(e.to_string()))?;
    if sig_bytes.len() != 65 {
        return Err(PaymailError::InvalidSignature(
            "Invalid signature length".to_string(),
        ));
    }
    let header = sig_bytes[0];
    if !(31..=34).contains(&header) {
        return Err(PaymailError::InvalidSignature(
            "Invalid recovery header".to_string(),
        ));
    }
    let recovery_id = ecdsa::RecoveryId::try_from((header - 31) as i32)
        .map_err(|e| PaymailError::Other(e.to_string()))?;
    let compact_sig = ecdsa::RecoverableSignature::from_compact(&sig_bytes[1..], recovery_id)
        .map_err(|e| PaymailError::Other(e.to_string()))?;
    let standard_sig = compact_sig.to_standard();

    let hash = ring::digest::digest(&SHA256, message.as_bytes());
    let msg_array: [u8; 32] = hash
        .as_ref()
        .try_into()
        .map_err(|_| PaymailError::Other("Invalid hash length".to_string()))?;
    let msg = Message::from_digest(msg_array);

    let secp = Secp256k1::new();
    Ok(secp.verify_ecdsa(msg, &standard_sig, &pub_key).is_ok())
}

pub fn parse_script(hex_str: &str) -> Result<Script, PaymailError> {
    let bytes = hex::decode(hex_str).map_err(|e| PaymailError::Other(e.to_string()))?;
    Ok(Script(bytes))
}

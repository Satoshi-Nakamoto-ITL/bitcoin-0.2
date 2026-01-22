use secp256k1::{Secp256k1, Message, PublicKey};
use secp256k1::ecdsa::Signature;

pub fn verify_signature(
    sighash: &[u8],
    signature_bytes: &[u8],
    pubkey_bytes: &[u8],
) -> bool {
    let secp = Secp256k1::verification_only();

    let msg = match Message::from_digest_slice(sighash) {
        Ok(m) => m,
        Err(_) => return false,
    };

    let sig = match Signature::from_der(signature_bytes) {
        Ok(s) => s,
        Err(_) => return false,
    };

    let pubkey = match PublicKey::from_slice(pubkey_bytes) {
        Ok(p) => p,
        Err(_) => return false,
    };

    secp.verify_ecdsa(&msg, &sig, &pubkey).is_ok()
}

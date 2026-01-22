use sha2::{Sha256, Digest};
use ed25519_dalek::{
    Signature,
    SigningKey,
    VerifyingKey,
    Signer,
    Verifier,
};

/// SHA256 helper
pub fn sha256(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

/// 🔐 Sign a message using ed25519
///
/// This derives a deterministic signing key from `private_key_seed`
/// (educational model, Bitcoin-style ownership via pubkey)
pub fn sign(private_key_seed: &[u8], message: &[u8]) -> Vec<u8> {
    // Derive a deterministic 32-byte seed
    let seed = sha256(private_key_seed);

    let signing_key = SigningKey::from_bytes(
        seed[..32]
            .try_into()
            .expect("seed must be 32 bytes"),
    );

    signing_key.sign(message).to_bytes().to_vec()
}

/// Verify an ed25519 signature
///
/// Returns true if and only if:
/// - pubkey is valid
/// - signature is valid
/// - signature matches message
pub fn verify_signature(
    pubkey_bytes: &[u8],
    message: &[u8],
    signature_bytes: &[u8],
) -> bool {
    // Public key must be exactly 32 bytes
    let pubkey_bytes: &[u8; 32] = match pubkey_bytes.try_into() {
        Ok(b) => b,
        Err(_) => return false,
    };

    // Signature must be exactly 64 bytes
    let signature_bytes: &[u8; 64] = match signature_bytes.try_into() {
        Ok(b) => b,
        Err(_) => return false,
    };

    let pubkey = match VerifyingKey::from_bytes(pubkey_bytes) {
        Ok(k) => k,
        Err(_) => return false,
    };

    let signature = Signature::from_bytes(signature_bytes);

    pubkey.verify(message, &signature).is_ok()
}

use crate::core::block::Block;

/// Consensus PoW rule:
///
/// - hash and target are 32-byte BIG-ENDIAN values
/// - numeric comparison is lexicographic
/// - hash <= target is VALID
///
/// ⚠️ CHANGING ENDIANNESS WILL FORK THE CHAIN
pub fn valid_pow(hash: &[u8], target: &[u8; 32]) -> bool {
    if hash.len() != 32 {
        return false;
    }

    let mut h = [0u8; 32];
    h.copy_from_slice(hash);

    h <= *target
}

pub fn mine(block: &mut Block) {
    loop {
        let hash = block.hash_header();

        if valid_pow(&hash, &block.header.target) {
            block.hash = hash;
            break;
        }

        block.header.nonce += 1;
    }
}

use crate::block::Block;

fn valid_pow(hash: &[u8], difficulty: u32) -> bool {
    let mut remaining = difficulty;

    for byte in hash {
        let zeros = byte.leading_zeros();

        if zeros >= remaining {
            return true;
        }

        if zeros < 8 {
            return false;
        }

        remaining -= zeros;
    }

    false
}

pub fn mine(block: &mut Block) {
    loop {
        let hash = block.hash_header();

        if valid_pow(&hash, block.header.difficulty) {
            block.hash = hash;
            break;
        }

        block.header.nonce += 1;
    }
}

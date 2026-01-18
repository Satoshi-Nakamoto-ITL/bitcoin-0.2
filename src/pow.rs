use crate::block::Block;

fn valid_pow(hash: &[u8], difficulty: u32) -> bool {
    let mut count = 0;

    for byte in hash {
        let z = byte.leading_zeros();
        count += z;

        if z < 8 {
            break;
        }
        if count >= difficulty {
            return true;
        }
    }

    count >= difficulty
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

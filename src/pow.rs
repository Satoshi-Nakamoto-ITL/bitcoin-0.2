use crate::block::Block;
use crate::crypto::sha256;

const SCRATCH_SIZE: usize = 32 * 1024 * 1024;

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

        remaining -= 8;
    }

    false
}

pub fn mine(block: &mut Block) {
    let mut scratch = vec![0u8; SCRATCH_SIZE];

    loop {
        let mut hash = block.hash_header();

        for i in 0..block.header.difficulty {
            let idx = (
                (hash[i as usize % hash.len()] as usize) << 8
                | hash[(i + 1) as usize % hash.len()] as usize
            ) % (SCRATCH_SIZE - 64);

            for j in 0..64 {
                scratch[idx + j] ^= hash[j % hash.len()];
            }
            hash = sha256(&[hash, scratch[idx..idx + 64].to_vec()].concat());
        }

        if valid_pow(&hash, block.header.difficulty) {
            block.hash = hash;
            break;
        }

        block.header.nonce += 1;
    }
}

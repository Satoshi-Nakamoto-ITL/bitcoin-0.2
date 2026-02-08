use std::fs;
use std::path::Path;
use std::time::Instant;

use rand::{rngs::OsRng, RngCore};
use memsec::mlock;

use aes_gcm::{
    Aes256Gcm,
    aead::{Aead, KeyInit},
};
use aes_gcm::aead::generic_array::GenericArray;

use sha2::{Sha256, Digest};
use pbkdf2::pbkdf2_hmac;
use bip39::{Mnemonic, Language};

use crate::crypto::{
    secret_key_from_seed,
    public_key,
    pubkey_hash,
    sign,
};

use crate::core::transaction::{Transaction, TxInput, TxOutput};
use crate::core::utxo::UTXOSet;

/* ───────── Encrypted Wallet File ───────── */

#[derive(serde::Serialize, serde::Deserialize)]
struct WalletFile {
    version: u32,
    encrypted_master_seed: Vec<u8>,
    password_salt: Vec<u8>,
    nonce: Vec<u8>,
    next_index: u32,
}

/* ───────── Memory Lock ───────── */

fn lock_memory(bytes: &mut [u8]) {
    unsafe {
        mlock(bytes.as_mut_ptr(), bytes.len());
    }
}

/* ───────── HD Derivation ───────── */

fn derive_child_seed(master: &[u8; 32], index: u32) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(master);
    hasher.update(index.to_be_bytes());
    let hash = hasher.finalize();

    let mut out = [0u8; 32];
    out.copy_from_slice(&hash[..32]);
    out
}

/* ───────── Wallet Struct ───────── */

pub struct Wallet {
    master_seed: Option<[u8; 32]>,
    last_unlock: Option<Instant>,
    next_index: u32,
}

/* ───────── Wallet Impl ───────── */

impl Wallet {
    pub fn load_or_create(password: &str) -> Self {
        fs::create_dir_all("data").unwrap();

        if Path::new("data/wallet.dat").exists() {
            let mut w = Wallet {
                master_seed: None,
                last_unlock: None,
                next_index: 0,
            };

            w.unlock(password).expect("wallet unlock failed");
            w
        } else {
            Self::create_new(password)
        }
    }

    fn create_new(password: &str) -> Self {
        let mut entropy = [0u8; 16];
        OsRng.fill_bytes(&mut entropy);

        let mnemonic =
            Mnemonic::from_entropy_in(Language::English, &entropy)
                .expect("mnemonic generation failed");

        println!("\n⚠️ WRITE THIS DOWN — WALLET RECOVERY PHRASE ⚠️");
        println!("{}", mnemonic);
        println!("⚠️ ANYONE WITH THESE WORDS CAN SPEND YOUR COINS ⚠️\n");

        Self::create_from_mnemonic(password, &mnemonic.to_string())
            .expect("wallet creation failed")
    }

    pub fn create_from_mnemonic(
        password: &str,
        mnemonic_phrase: &str,
    ) -> Result<Self, &'static str> {
        let mnemonic =
            Mnemonic::parse_in_normalized(Language::English, mnemonic_phrase)
                .map_err(|_| "invalid mnemonic")?;

        let seed = mnemonic.to_seed("");
        let mut master_seed = [0u8; 32];
        master_seed.copy_from_slice(&seed[..32]);

        let mut salt = [0u8; 16];
        OsRng.fill_bytes(&mut salt);

        let mut enc_key = [0u8; 32];
        pbkdf2_hmac::<Sha256>(
            password.as_bytes(),
            &salt,
            300_000,
            &mut enc_key,
        );

        let cipher = Aes256Gcm::new(GenericArray::from_slice(&enc_key));

        let mut nonce = [0u8; 12];
        OsRng.fill_bytes(&mut nonce);

        let encrypted = cipher
            .encrypt(GenericArray::from_slice(&nonce), &master_seed[..])
            .map_err(|_| "encryption failed")?;

        let wf = WalletFile {
            version: 3,
            encrypted_master_seed: encrypted,
            password_salt: salt.to_vec(),
            nonce: nonce.to_vec(),
            next_index: 0,
        };

        fs::write("data/wallet.dat", bincode::serialize(&wf).unwrap()).unwrap();
        lock_memory(&mut master_seed);

        Ok(Wallet {
            master_seed: Some(master_seed),
            last_unlock: Some(Instant::now()),
            next_index: 0,
        })
    }

    pub fn unlock(&mut self, password: &str) -> Result<(), ()> {
        let data = fs::read("data/wallet.dat").map_err(|_| ())?;
        let wf: WalletFile = bincode::deserialize(&data).map_err(|_| ())?;

        let mut enc_key = [0u8; 32];
        pbkdf2_hmac::<Sha256>(
            password.as_bytes(),
            &wf.password_salt,
            300_000,
            &mut enc_key,
        );

        let cipher = Aes256Gcm::new(GenericArray::from_slice(&enc_key));

        let seed = cipher
            .decrypt(
                GenericArray::from_slice(&wf.nonce),
                wf.encrypted_master_seed.as_ref(),
            )
            .map_err(|_| ())?;

        let mut master_seed = [0u8; 32];
        master_seed.copy_from_slice(&seed[..32]);
        lock_memory(&mut master_seed);

        self.master_seed = Some(master_seed);
        self.last_unlock = Some(Instant::now());
        self.next_index = wf.next_index;

        Ok(())
    }

    pub fn address(&self) -> Result<Vec<u8>, &'static str> {
        let master = self.master_seed.ok_or("wallet locked")?;
        let child = derive_child_seed(&master, 0);
        let sk = secret_key_from_seed(&child);
        let pk = public_key(&sk);
        Ok(pubkey_hash(&pk))
    }

    pub fn create_transaction(
        &mut self,
        utxos: &UTXOSet,
        to: Vec<u8>,
        amount: u64,
        _current_height: u64,
    ) -> Result<Transaction, &'static str> {
        let master = self.master_seed.ok_or("wallet locked")?;

        let mut collected = 0u64;
        let mut inputs = Vec::new();

        for (key, utxo) in utxos {
            for index in 0..20 {
                let child = derive_child_seed(&master, index);
                let sk = secret_key_from_seed(&child);
                let pk = public_key(&sk);

                if pubkey_hash(&pk) == utxo.pubkey_hash {
                    let parts: Vec<&str> = key.split(':').collect();
                    let txid = hex::decode(parts[0]).unwrap();
                    let vout = parts[1].parse().unwrap();

                    inputs.push((txid, vout, index, utxo.value));
                    collected += utxo.value;

                    if collected >= amount {
                        break;
                    }
                }
            }
            if collected >= amount {
                break;
            }
        }

        if collected < amount {
            return Err("insufficient funds");
        }

        let mut outputs = vec![TxOutput {
            value: amount,
            pubkey_hash: to,
        }];

        if collected > amount {
            outputs.push(TxOutput {
                value: collected - amount,
                pubkey_hash: self.address()?,
            });
        }

        let mut tx = Transaction {
            inputs: Vec::new(),
            outputs,
        };

        let sighash = tx.sighash();

        for (txid, vout, index, _) in inputs {
            let child = derive_child_seed(&master, index);
            let sk = secret_key_from_seed(&child);
            let pk = public_key(&sk);

            tx.inputs.push(TxInput {
                txid,
                index: vout,
                signature: sign(&sighash, &sk),
                pubkey: pk.serialize().to_vec(),
                address_index: index,
            });
        }

        Ok(tx)
    }
}

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash,Hasher};
use ed25519_dalek::{Keypair};
use rand::rngs::OsRng;
use hex::encode;
use crate::wallet::Wallet;


pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub fn create_random_keypair() -> Keypair {
    let mut csprng = OsRng{};
    Keypair::generate(&mut csprng)
}

#[allow(dead_code)]
pub fn get_address_from_keypair(keypair: &Keypair) -> String {
    let address: String = encode(keypair.public);
    address
}

#[allow(dead_code)]
pub fn get_public_key_from_address(address: &String) -> Vec<u8> {
    let public_key = &address[2..];
    let public_key = hex::decode(public_key).unwrap();
    public_key
}

pub fn new_random_wallet() -> Wallet {
    let Keypair = create_random_keypair();
    let wallet = Wallet::new(Keypair);
    wallet
}


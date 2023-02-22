use sha2::{Sha256, Digest};
use ed25519_dalek::{Signature, Signer};
use hex::encode;

mod helpers;
mod block;
mod transaction;
mod wallet;
mod blockchain;
mod node;
mod state;

fn main() {
    let our_difficulty: usize = 2;
    let keypair = helpers::create_random_keypair();
    let message: &[u8] = b"Hello, world!";
    let signature: Signature = keypair.sign(message);
    let mut our_hash: String;

    println!("Signature: {}", encode(signature.to_bytes()));
    println!("Signature verification: {}", keypair.verify(message, &signature).is_ok());
    
    let mut genesis = block::Block::new(1, 1, "0".to_string(), "0".to_string(), 0, our_difficulty as u64, 0);
    
    

    loop {
        
        our_hash = format!("{:x}", Sha256::digest(&helpers::calculate_hash(&genesis).to_string().as_bytes()));
        println!("Our hash: {}, nonce: {}", our_hash, genesis.get_nonce());
        if  our_hash.to_string()[..our_difficulty] == "0".repeat(our_difficulty) {
            println!("Found it! nonce: {}", genesis.get_nonce());
            break;
        }
        genesis.set_nonce(genesis.get_nonce() + 1)

    }  
  
    
    
}
    

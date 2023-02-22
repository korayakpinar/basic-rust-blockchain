use ed25519_dalek::{Keypair,Signer};
use crate::helpers;
use crate::transaction::Transaction;

#[allow(dead_code)]
pub struct Wallet {
    keypair: Keypair,
    address: String,
}

impl Wallet {

    #[allow(dead_code)]
    pub fn new(keypair: Keypair) -> Wallet {
        //let keypair = helpers::create_random_keypair();
        let address = helpers::get_address_from_keypair(&keypair);
        Wallet {
            keypair,
            address,
        }
    }

    #[allow(dead_code)]
    pub fn get_address(&self) -> &String {
        &self.address
    }

    #[allow(dead_code)]
    pub fn get_keypair(&self) -> &Keypair {
        &self.keypair
    }

    #[allow(dead_code)]
    pub fn sign_transaction(&self, transaction: &Transaction) -> String {
        let signature = self.keypair.sign(&helpers::calculate_hash(transaction).to_string().as_bytes());
        hex::encode(signature.to_bytes())
    }

    
}


#[cfg(test)]
mod tests {
    
    use crate::{transaction::Transaction, helpers::{new_random_wallet, get_address_from_keypair}, helpers::create_random_keypair};
    use ed25519_dalek::{Keypair};
    use rand::rngs::OsRng;
    use super::Wallet;

    #[test]
    fn test_wallet() {
        let wallet = new_random_wallet();
        assert_eq!(wallet.get_address().len(), 64);
    }

    #[test]
    fn test_sign_transaction() {
        let tx = Transaction::new(1, "0".to_string(), "0".to_string(), 1, 1, "0".to_string());
        let wallet = new_random_wallet();
        let signature = wallet.sign_transaction(&tx);
        assert_eq!(signature.len(), 128);
    }
}
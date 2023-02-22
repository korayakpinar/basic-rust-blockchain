use sha2::{Sha256, Digest};



#[derive(Clone, Hash)]
pub struct Transaction {
    version: u64,
    from: String,
    recipient : String,
    amount: u64,
    timestamp: u64,
    signature: String,
}


impl Transaction {

    #[allow(dead_code)]
    pub fn new(version: u64, from: String, recipient: String, amount: u64, timestamp: u64, signature: String) -> Transaction {
        Transaction {
            version,
            from,
            recipient,
            amount,
            timestamp,
            signature
        }
    }

    #[allow(dead_code)]
    pub fn get_version(&self) -> &u64 {
        &self.version
    }

    #[allow(dead_code)]
    pub fn get_from(&self) -> &String {
        &self.from
    }

    #[allow(dead_code)]
    pub fn get_recipient (&self) -> &String {
        &self.recipient 
    }

    #[allow(dead_code)]
    pub fn get_amount(&self) -> &u64 {
        &self.amount
    }

    #[allow(dead_code)]
    pub fn get_timestamp(&self) -> &u64 {
        &self.timestamp
    }

    #[allow(dead_code)]
    pub fn get_signature(&self) -> &String {
        &self.signature
    }

    #[allow(dead_code)]
    pub fn set_signature(&mut self, signature: String) {
        self.signature = signature;
    }

    #[allow(dead_code)]
    pub fn to_hash(&self) -> Vec<u8> {
        let data = serde_json::json!({
            "version": self.get_version(),
            "from": self.get_from(),
            "recipient": self.get_recipient(),
            "amount": self.get_amount(),
            "timestamp": self.get_timestamp(),
        }); 
    
        let mut hasher = Sha256::new();
        hasher.update(data.to_string().as_bytes());
        hasher.finalize().as_slice().to_owned()
    }
        
    
}


#[cfg(test)]
mod tests {
    use std::time;
    use super::*;
    use crate::{ helpers::new_random_wallet};

    #[test]
    fn test_transaction() {
        let sender = new_random_wallet();
        let recipient = new_random_wallet();
        let now = time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap().as_secs();
        let mut transaction = Transaction::new(1, sender.get_address().to_owned(), recipient.get_address().to_owned(), 1, now, "0".to_string());
        let signature = sender.sign_transaction(&transaction);
        let our_signature = signature.clone();

        transaction.set_signature(signature);

        
        assert_eq!(transaction.get_version(), &1);
        assert_eq!(transaction.get_from(), sender.get_address());
        assert_eq!(transaction.get_recipient(), recipient.get_address());
        assert_eq!(transaction.get_amount(), &1);
        assert_eq!(transaction.get_timestamp(), &now);
        assert_eq!(transaction.get_signature(), &our_signature);
    }
}
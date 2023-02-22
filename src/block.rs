use crate::transaction::Transaction;
use sha2::{Sha256, Digest};



#[derive(Clone,Hash)]
pub struct Block {
    header: BlockHeader,
    transactions: Vec<Transaction>,
    signature: String,
}

#[derive(Clone,Hash)]
pub struct BlockHeader {
    version: u64,
    height: u64,
    prev_hash: String,
    merkle_root: String,
    timestamp: u64,
    difficulty: u64,
    nonce: u64,
}


impl Block {
    
    pub fn new(version: u64, height: u64, prev_hash: String, merkle_root: String, timestamp: u64, difficulty: u64, nonce: u64 ) -> Block {
        
        Block {
            header: BlockHeader {
                version,
                height,
                prev_hash,
                merkle_root,
                timestamp,
                difficulty,
                nonce,
            },
            transactions: Vec::new(),
            signature: String::new(),
        }
    }

    #[allow(dead_code)]
    pub fn get_version(&self) -> &u64 {
        &self.header.version
    }

    pub fn get_height(&self) -> &u64 {
        &self.header.height
    }

    #[allow(dead_code)]
    pub fn get_prev_hash(&self) -> &String {
        &self.header.prev_hash
    }

    #[allow(dead_code)]
    pub fn get_merkle_root(&self) -> &String {
        &self.header.merkle_root
    }

    #[allow(dead_code)]
    pub fn get_timestamp(&self) -> &u64 {
        &self.header.timestamp
    }

    #[allow(dead_code)]
    pub fn get_difficulty(&self) -> &u64 {
        &self.header.difficulty
    }

    #[allow(dead_code)]
    pub fn get_transactions(&self) -> &Vec<Transaction> {
        &self.transactions
    }

    #[allow(dead_code)]
    pub fn get_nonce(&self) -> &u64 {
        &self.header.nonce
    }

    #[allow(dead_code)]
    pub fn get_signature(&self) -> &String {
        &self.signature
    }

    #[allow(dead_code)]
    pub fn set_merkle_root(&mut self, merkle_root: String) {
        self.header.merkle_root = merkle_root;
    }
    
    #[allow(dead_code)]
    pub fn set_nonce(&mut self, nonce: u64) {
        self.header.nonce = nonce;
    }

    #[allow(dead_code)]
    pub fn set_signature(&mut self, signature: String) {
        self.signature = signature;
    }

    #[allow(dead_code)]
    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
    }

    #[allow(dead_code)]
    pub fn to_hash(&self) -> Vec<u8> {
        let data = serde_json::json!({
            "version": self.get_version(),
            "height": self.get_height(),
            "previous_hash": self.get_prev_hash(),
            "timestamp": self.get_timestamp(),
            "nonce": self.get_nonce(),
        }); 
    
        let mut hasher = Sha256::new();
        hasher.update(data.to_string().as_bytes());
        hasher.finalize().as_slice().to_owned()
    }



}


struct Mempool {
    verified_txs: Vec<Transaction>,
    unverified_txs: Vec<Transaction>,
}

impl Mempool {
    pub fn new() -> Mempool {
        Mempool {
            verified_txs: Vec::new(),
            unverified_txs: Vec::new(),
        }
    }

    pub fn get_verified_txs(&self) -> &Vec<Transaction> {
        &self.verified_txs
    }

    pub fn get_unverified_txs(&self) -> &Vec<Transaction> {
        &self.unverified_txs
    }

    pub fn add_tx(&mut self, tx: Transaction) {
        self.unverified_txs.push(tx);
    }

    pub fn verify_tx(&mut self, tx: Transaction) {
        let public_key = PublicKey::from_bytes(&hex::decode(transaction.get_from()).unwrap()).unwrap();
        let signature = Signature::from_bytes(&hex::decode(&transaction.get_signature()).unwrap()).unwrap();
        let message = String::from_utf8_lossy(&transaction.to_hash()).to_string();

        if transaction.get_amount() > 0 &&
        transaction.get_from() == transaction.get_to() &&
        transaction.get_timestamp() < self.blockchain.get_last_block().get_timestamp() &&
        public_key.verify(message.as_bytes(), &signature).is_ok() &&
        transaction.get_amount() > self.get_balance_for_address(transaction.get_from().clone()) {
            self.unverified_txs.retain(|x| x != &tx);
            self.verified_txs.push(tx);
        }

        if transaction.get_amount() < 0 {
            return false;
        };

        if transaction.get_from() == transaction.get_to() {
            return false;
        };
        
        if transaction.get_timestamp() < self.blockchain.get_last_block().get_timestamp() {
            return false;
        };

        if !public_key.verify(message.as_bytes(), &signature).is_ok() {
            return false;
        };

        if transaction.get_amount() > self.get_balance_for_address(transaction.get_from().clone()) {
            return false;
        };
        
        self.unverified_txs.retain(|x| x != &tx);
        self.verified_txs.push(tx);
    }
} 
       
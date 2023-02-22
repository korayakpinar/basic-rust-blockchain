use ed25519_dalek::{Keypair,Signer,PublicKey,Signature,Verifier};
use crate::helpers::{self};
use crate::transaction::{Transaction};
use crate::block::Block;
use crate::blockchain::Blockchain;


#[allow(dead_code)]
enum Errors {
    BlockNotFound,
    TxNotFound,
}

#[allow(dead_code)]
struct Node {
    keypair: Keypair,
    blockchain: Blockchain,
    peers: Vec<Node>,
    id: u64,
    mempool: Vec<Transaction>,
}


impl Node {
    
    #[allow(dead_code)]
    fn new(keypair: Keypair, blockchain: Blockchain, peers: Vec<Node>, id: u64, mempool: Vec<Transaction>) -> Node {
        Node {
            keypair,
            blockchain,
            peers,
            id,
            mempool,
        }
    }

    #[allow(dead_code)]
    fn get_keypair(&self) -> &Keypair {
        &self.keypair
    }

    #[allow(dead_code)]
    fn get_blockchain(&self) -> &Blockchain {
        &self.blockchain
    }

    #[allow(dead_code)]
    fn get_peers(&self) -> &Vec<Node> {
        &self.peers
    }

    #[allow(dead_code)]
    fn get_id(&self) -> &u64 {
        &self.id
    }

    #[allow(dead_code)]
    fn get_mempool(&self) -> &Vec<Transaction> {
        &self.mempool
    }

    #[allow(dead_code)]
    fn get_balance_for_address(&self, address: String) -> &u64 {
        self.blockchain.get_balance_from_address(address)
    }

    #[allow(dead_code)]
    fn get_transaction_by_hash(&self, hash: String) -> Result<&Transaction,Errors> {
        for block in self.blockchain.get_chain() {
            for transaction in block.get_transactions() {
                if String::from_utf8_lossy(&transaction.to_hash()).to_string() == hash  {
                    return Ok(&transaction);
                }
            }
        }
        Err(Errors::TxNotFound)
    }

    // TODO: get_block_by_hash

    #[allow(dead_code)]
    fn add_peer(&mut self, peer: Node) {
        self.peers.push(peer);
    }

    #[allow(dead_code)]
    fn send_transaction(&mut self, transaction: Transaction) {
        self.mempool.push(transaction);
    }

    #[allow(dead_code)]
    fn add_block(&mut self, block: Block) {
        self.blockchain.add_block(block);
    }

    #[allow(dead_code)]
    fn sign_block(&self, block: &Block) -> String {
        let signature = self.keypair.sign(&helpers::calculate_hash(block).to_string().as_bytes());
        hex::encode(signature.to_bytes())
    }
    
    #[allow(dead_code)]
    fn verify_block(&mut self, block: &Block) -> bool {
        let public_key = PublicKey::from_bytes(&hex::decode(&block.get_transactions()[0].get_from()).unwrap()).unwrap();
        let signature = Signature::from_bytes(&hex::decode(&block.get_signature()).unwrap()).unwrap();
        let message = String::from_utf8_lossy(&block.to_hash()).to_string();

        if block.get_timestamp() < self.blockchain.get_last_block().get_timestamp() {
            return false;
        };

        if !public_key.verify(message.as_bytes(), &signature).is_ok()  {
           return false; 
        };

        if block.get_prev_hash() != &String::from_utf8_lossy(&self.blockchain.get_last_block().to_hash()).to_string() {
            return false;
        };

        if *block.get_height() != self.blockchain.get_height() + 1 {
            return false;
        };

        true
              
        
    }

    #[allow(dead_code)]
    async fn run(&mut self) {
        //let difficulty = 4;
        

        loop {
            if self.mempool.len() > 0 {
                break;  
            }
                
        }
    }
        

}

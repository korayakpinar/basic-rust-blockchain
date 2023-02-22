
use crate::block::Block;
use crate::state::State;


pub struct Blockchain {
    chain: Vec<Block>,
    chain_id: u64,
    state: State,
    difficulty: u64,
}

impl Blockchain {
    
    #[allow(dead_code)]
    pub fn new(genesis: Block, id: u64, difficulty: u64) -> Blockchain {
        Blockchain {
            chain: vec![genesis],
            chain_id: id,
            state: State::new(),
            difficulty,
        }
    }

    #[allow(dead_code)]
    pub fn add_block(&mut self, block: Block) {
        self.chain.push(block);
    }

    #[allow(dead_code)]
    pub fn get_height(&mut self) -> &u64 {
        &self.chain.last().unwrap().get_height()
        
    } 

    #[allow(dead_code)]
    pub fn get_chain_id(&mut self) -> &u64 {
        &self.chain_id
    }

    #[allow(dead_code)]
    pub fn get_last_block(&mut self) -> &Block {
        &self.chain.last().unwrap()
    }

    #[allow(dead_code)]
    pub fn get_chain(&self) -> &Vec<Block> {
        &self.chain
    }

    #[allow(dead_code)]
    pub fn get_balance_from_address(&self, address: String) -> &u64 {
        &self.state.get_balance(address)
    }

    #[allow(dead_code)]
    pub fn get_difficulty(&self) -> &u64 {
        &self.difficulty
    }

    #[allow(dead_code)]
    pub fn get_state(&self) -> &State {
        &self.state
    }

    #[allow(dead_code)]
    pub fn set_difficulty(&mut self, diff: u64) {
        self.difficulty = diff;
    }
}




